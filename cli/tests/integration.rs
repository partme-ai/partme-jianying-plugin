use anyhow::Result;
use jycut::{draft, plan::Plan, probe::MediaInfo, store};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};

fn stub_probe(expected_name: &str, dur_us: i64) -> impl Fn(&Path) -> Result<MediaInfo> {
    let expected = expected_name.to_string();
    move |p: &Path| {
        if !p.ends_with(&expected) {
            anyhow::bail!("unexpected source {}", p.display());
        }
        Ok(MediaInfo {
            path: p.to_string_lossy().into_owned(),
            duration_us: dur_us,
            width: 320,
            height: 240,
            has_video: true,
            has_audio: true,
        })
    }
}

const TWO_SHOT_PLAN: &str = r##"
{
  "schema": "jycut-plan/v1",
  "name": "integration-draft",
  "canvas": {"width": 1920, "height": 1080, "fps": 30},
  "tracks": [
    {"type": "video", "name": "主视频", "segments": [
      {"start_us": 0, "duration_us": 2000000, "source": "shot-a.mp4",
       "transition_out": {"name": "叠化", "duration_us": 400000}},
      {"start_us": 2000000, "duration_us": 2000000, "source": "shot-b.mp4",
       "keyframes": {"scale": [{"at_us": 0, "value": 1.0}, {"at_us": 2000000, "value": 1.12}]}}
    ]},
    {"type": "audio", "segments": [
      {"start_us": 0, "duration_us": 4000000, "source": "narration.mp3", "volume": 0.8}
    ]},
    {"type": "text", "segments": [
      {"start_us": 200000, "duration_us": 1500000, "text": "上半场：白模预演", "size": 8,
       "color": "#FF8800", "border_color": "#000000", "border_width": 4}
    ]}
  ]
}
"##;

fn write_plan(plan: &str, dir: &Path) -> PathBuf {
    let p = dir.join("plan.json");
    std::fs::write(&p, plan).unwrap();
    // placeholder media bodies; the stub prober and copy step only need real files
    for name in ["shot-a.mp4", "shot-b.mp4", "narration.mp3"] {
        std::fs::write(dir.join(name), b"fake media bytes").unwrap();
    }
    p
}

fn probe_stub(_p: &Path) -> Result<MediaInfo> {
    Ok(MediaInfo {
        path: _p.to_string_lossy().into_owned(),
        duration_us: 10_000_000,
        width: 320,
        height: 240,
        has_video: true,
        has_audio: true,
    })
}

#[test]
fn plan_validation_rejects_bad_input() {
    let good = json!({
        "schema": "jycut-plan/v1", "name": "x",
        "canvas": {"width": 1920, "height": 1080, "fps": 30},
        "tracks": [{"type": "video", "segments": [
            {"start_us": 0, "duration_us": 1000000, "source": "a.mp4"}]}]
    });
    let plan: Plan = serde_json::from_value(good.clone()).unwrap();
    plan.validate().expect("good plan validates");

    let mut bad_fps = good.clone();
    bad_fps["canvas"]["fps"] = json!(60.5);
    assert!(serde_json::from_value::<Plan>(bad_fps).is_err());

    let mut no_main = good.clone();
    no_main["tracks"][0]["type"] = json!("text");
    no_main["tracks"][0]["segments"][0] = json!({"start_us": 0, "duration_us": 1, "text": "t"});
    let parsed: Plan = serde_json::from_value(no_main).unwrap();
    assert!(
        parsed.validate().is_err(),
        "plan without video track must fail"
    );

    let mut overlap = good.clone();
    overlap["tracks"][0]["segments"]
        .as_array_mut()
        .unwrap()
        .push(json!({"start_us": 500000, "duration_us": 100000, "source": "b.mp4"}));
    let parsed: Plan = serde_json::from_value(overlap).unwrap();
    assert!(parsed.validate().is_err(), "overlap must fail");
}

#[test]
fn build_produces_registered_shape_and_self_verifies() {
    let tmp = std::env::temp_dir().join(format!("jycut-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let plan_path = write_plan(TWO_SHOT_PLAN, &tmp);
    let out = tmp.join("integration-draft");

    let plan = Plan::load(&plan_path).unwrap();
    let report = draft::build(&plan, &tmp, &out, &probe_stub).expect("build succeeds");

    assert_eq!(report.duration_us, 4_000_000);
    assert_eq!(report.tracks, 3);
    assert_eq!(report.segments, 4);

    let content: Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("draft_content.json")).unwrap())
            .unwrap();
    let info: Value =
        serde_json::from_str(&std::fs::read_to_string(out.join("draft_info.json")).unwrap())
            .unwrap();
    assert_eq!(content, info, "both timeline mirrors must be identical");

    let tracks = content["tracks"].as_array().unwrap();
    assert_eq!(tracks[0]["type"], json!("video"));
    assert_eq!(tracks[1]["type"], json!("audio"));
    assert_eq!(tracks[2]["type"], json!("text"));

    // transition attached to the first segment's refs and registered
    let first_refs = tracks[0]["segments"][0]["extra_material_refs"]
        .as_array()
        .unwrap();
    let transitions = content["materials"]["transitions"].as_array().unwrap();
    assert_eq!(transitions.len(), 1);
    assert_eq!(transitions[0]["name"], json!("叠化"));
    assert!(first_refs.iter().any(|r| r == &transitions[0]["id"]));

    // text material content parses and carries UTF-16 range + stroke
    let texts = content["materials"]["texts"].as_array().unwrap();
    assert_eq!(texts.len(), 1);
    let parsed: Value = serde_json::from_str(texts[0]["content"].as_str().unwrap()).unwrap();
    let utf16_len = "上半场：白模预演".encode_utf16().count() as i64;
    assert_eq!(parsed["styles"][0]["range"], json!([0, utf16_len]));
    assert!(
        parsed["styles"][0].get("strokes").is_some(),
        "border_width>0 must add strokes"
    );
    let fill_color = &parsed["styles"][0]["fill"]["content"]["solid"]["color"];
    assert_eq!(fill_color, &json!([1.0, 0.5333333333333333, 0.0]));

    // main track continuous + duration
    let v = tracks[0]["segments"].as_array().unwrap();
    assert_eq!(v[0]["target_timerange"]["start"], json!(0));
    assert_eq!(
        v[1]["target_timerange"]["start"],
        json!(v[0]["target_timerange"]["duration"])
    );
    assert_eq!(content["duration"], json!(4_000_000));

    // self-verify + inspect
    let verdict = draft::verify(&out).unwrap();
    assert_eq!(verdict["ok"], json!(true), "issues: {}", verdict["issues"]);
    let summary = draft::inspect(&out).unwrap();
    assert_eq!(summary["tracks"].as_array().unwrap().len(), 3);
}

#[test]
fn publish_registers_into_root_meta() {
    let tmp = std::env::temp_dir().join(format!("jycut-pub-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&tmp);
    std::fs::create_dir_all(&tmp).unwrap();
    let plan_path = write_plan(TWO_SHOT_PLAN, &tmp);
    let out = tmp.join("integration-draft");
    let plan = Plan::load(&plan_path).unwrap();
    draft::build(&plan, &tmp, &out, &probe_stub).unwrap();

    let root = tmp.join("store-root");
    std::fs::create_dir_all(&root).unwrap();
    let result = store::publish(&out, &root, true).expect("publish succeeds");
    assert_eq!(result["status"], json!("published"));

    let dest = root.join("integration-draft");
    assert!(dest.join("draft_content.json").is_file());
    let root_meta: Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("root_meta_info.json")).unwrap())
            .unwrap();
    let store = root_meta["all_draft_store"].as_array().unwrap();
    assert_eq!(store.len(), 1);
    assert_eq!(store[0]["draft_name"], json!("integration-draft"));

    // second publish with the same name must refuse
    assert!(
        store::publish(&out, &root, true).is_err(),
        "name clash must refuse"
    );
}
