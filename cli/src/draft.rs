//! Draft assembly: `jycut-plan/v1` → a real 剪映/CapCut draft directory.
//!
//! Layout written (superset that opens across layouts):
//! - `draft_content.json` + `draft_info.json` (identical timeline mirrors)
//! - `draft_meta_info.json` (registration sidecar)
//! - `assets/<video|audio>/…` (media copied next to the timeline)
//!
//! Format facts synthesized from: GuanYixuan/pyJianYingDraft (Apache-2.0
//! skeleton assets), renezander030/capcut-cli (MIT — mirror/registration
//! discipline, render_index conventions, companion materials), and
//! partme-ai/jianying-headless (contract facts only).

use crate::plan::{hex_to_rgb01, KeyPoint, Plan, Segment};
use crate::probe::MediaInfo;
use anyhow::{bail, Context, Result};
use serde::Serialize;
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub const CONTENT_TEMPLATE: &str = include_str!("../assets/draft_content_template.json");
pub const META_TEMPLATE: &str = include_str!("../assets/draft_meta_info.json");
pub const TRANSITIONS: &str = include_str!("../assets/transitions.json");

const RENDER_INDEX_VIDEO: i64 = 14000;
const RENDER_INDEX_AUDIO: i64 = 11000;
const RENDER_INDEX_TEXT: i64 = 15000;

#[derive(Serialize)]
pub struct BuildReport {
    pub name: String,
    pub duration_us: i64,
    pub tracks: usize,
    pub segments: usize,
    pub out_dir: String,
    pub files: Vec<String>,
}

fn hex_id() -> String {
    uuid::Uuid::new_v4().simple().to_string()
}

fn uuid_upper() -> String {
    uuid::Uuid::new_v4().to_string().to_uppercase()
}

fn now_us() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_micros() as i64)
        .unwrap_or(0)
}

fn canvas_ratio(width: u64, height: u64) -> &'static str {
    match (width, height) {
        (1920, 1080) => "16:9",
        (1080, 1920) => "9:16",
        (1080, 1080) => "1:1",
        (1440, 1080) => "4:3",
        (1080, 1440) => "3:4",
        _ => "original",
    }
}

fn host_os() -> &'static str {
    if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "mac"
    } else {
        "linux"
    }
}

fn transition_lookup(name: &str) -> Result<Value> {
    let catalog: BTreeMap<String, Value> = serde_json::from_str(TRANSITIONS)?;
    let entry = catalog.get(name).ok_or_else(|| {
        let mut names: Vec<&String> = catalog.keys().collect();
        names.sort();
        anyhow::anyhow!(
            "transition {name:?} is not in the bundled catalog (available: {})",
            names
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        )
    })?;
    if entry["vip"].as_bool().unwrap_or(false) {
        bail!("transition {name:?} is a VIP resource; jycut only bundles non-VIP entries");
    }
    Ok(entry.clone())
}

/// The text `content` field is a JSON-in-string with UTF-16 code-unit ranges.
fn build_text_content(seg: &Segment) -> Result<String> {
    let text = seg.text.as_deref().unwrap_or_default();
    let utf16_len: usize = text.encode_utf16().count();
    let color = hex_to_rgb01(seg.color.as_deref().unwrap_or("#FFFFFF"))?;
    let size = seg.size.unwrap_or(8.0);
    let mut style = json!({
        "range": [0, utf16_len as i64],
        "size": size,
        "bold": seg.bold.unwrap_or(false),
        "italic": false,
        "underline": false,
        "fill": {"alpha": 1.0, "content": {"render_type": "solid",
                 "solid": {"alpha": 1.0, "color": color}}},
    });
    let border = seg.border_width.unwrap_or(0.0);
    if border > 0.0 {
        let bc = hex_to_rgb01(seg.border_color.as_deref().unwrap_or("#000000"))?;
        let relative = (border / 15.0).clamp(0.01, 1.0);
        style["strokes"] = json!([{"content": {"solid": {"alpha": 1.0, "color": bc}},
                                   "width": relative}]);
    }
    Ok(json!({"text": text, "styles": [style]}).to_string())
}

struct Companions {
    refs: Vec<String>,
}

/// Companion materials every media segment needs (speed + registration stubs).
fn make_companions(materials: &mut Value, speed: f64, video: bool) -> Companions {
    let speed_id = hex_id();
    materials["speeds"].as_array_mut().unwrap().push(json!({
        "id": speed_id, "type": "speed", "speed": speed, "mode": 0, "curve_speed": null
    }));
    let placeholder_id = hex_id();
    materials["placeholders"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "id": placeholder_id, "type": "placeholder_info", "error_path": "", "error_text": "",
            "meta_type": "none", "res_path": "", "res_text": ""
        }));
    let scm_id = hex_id();
    materials["sound_channel_mappings"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "id": scm_id, "type": "none", "audio_channel_mapping": 0, "is_config_open": false
        }));
    let vocal_id = hex_id();
    materials["vocal_separations"]
        .as_array_mut()
        .unwrap()
        .push(json!({
            "id": vocal_id, "type": "vocal_separation", "choice": 0, "enter_from": "",
            "final_algorithm": "", "production_path": "", "removed_sounds": [], "time_range": null
        }));
    let mut refs = vec![speed_id, placeholder_id, scm_id, vocal_id];
    if video {
        let canvas_id = hex_id();
        materials["canvases"].as_array_mut().unwrap().push(json!({
            "id": canvas_id, "type": "canvas_color", "album_image": "", "blur": 0, "color": "",
            "image": "", "image_id": "", "image_name": "", "source_platform": 0, "team_id": ""
        }));
        let color_id = hex_id();
        materials["material_colors"]
            .as_array_mut()
            .unwrap()
            .push(json!({
                "id": color_id, "type": "material_color", "gradient_angle": 90,
                "gradient_colors": [], "gradient_percents": [], "height": 0,
                "is_color_clip": false, "is_gradient": false, "solid_color": "", "width": 0
            }));
        refs.push(canvas_id);
        refs.push(color_id);
    }
    Companions { refs }
}

fn base_segment(material_id: &str, seg: &Segment, render_index: i64) -> Value {
    let speed = seg.speed.unwrap_or(1.0);
    let src_duration = seg
        .source_duration_us
        .unwrap_or((seg.duration_us as f64 * speed).round() as i64);
    let mut v = json!({
        "id": hex_id(),
        "material_id": material_id,
        "target_timerange": {"start": seg.start_us, "duration": seg.duration_us},
        "source_timerange": {"start": seg.source_start_us, "duration": src_duration},
        "speed": speed,
        "volume": seg.volume.unwrap_or(1.0),
        "visible": true,
        "reverse": false,
        "enable_adjust": true,
        "enable_color_correct_adjust": false,
        "enable_color_curves": true,
        "enable_color_match_adjust": false,
        "enable_color_wheels": true,
        "enable_lut": true,
        "enable_smart_color_adjust": false,
        "last_nonzero_volume": 1.0,
        "dur": seg.duration_us,
        "render_index": render_index,
        "track_render_index": 0,
        "track_attribute": 0,
        "extra_material_refs": [],
        "common_keyframes": [],
        "keyframe_refs": [],
    });
    if seg.source_duration_us.is_some() {
        v["source_timerange"]["duration"] = json!(src_duration);
    }
    v
}

fn apply_visuals(seg_v: &mut Value, seg: &Segment, video: bool) {
    seg_v["clip"] = json!({
        "alpha": seg.opacity.unwrap_or(1.0),
        "rotation": seg.rotation.unwrap_or(0.0),
        "scale": {"x": seg.scale.unwrap_or(1.0), "y": seg.scale.unwrap_or(1.0)},
        "transform": {"x": seg.x.unwrap_or(0.0), "y": seg.y.unwrap_or(0.0)},
        "flip": {"horizontal": false, "vertical": false}
    });
    if video {
        seg_v["uniform_scale"] = json!({"on": true, "value": 1.0});
        seg_v["hdr_settings"] = json!({"intensity": 1.0, "mode": 1, "nits": 1000});
    }
}

fn apply_keyframes(seg_v: &mut Value, seg: &Segment) {
    let kfs = match &seg.keyframes {
        Some(k) if !k.is_empty() => k,
        _ => return,
    };
    let points_json = |points: &[KeyPoint]| -> Value {
        json!(points
            .iter()
            .map(|p| json!({
                "id": hex_id(),
                "curveType": "Line",
                "graphID": "",
                "left_control": {"x": 0.0, "y": 0.0},
                "right_control": {"x": 0.0, "y": 0.0},
                "time_offset": p.at_us,
                "values": [p.value]
            }))
            .collect::<Vec<_>>())
    };
    let mut push = |property: &str, points: &[KeyPoint]| {
        seg_v["common_keyframes"]
            .as_array_mut()
            .unwrap()
            .push(json!({
                "id": hex_id(),
                "property_type": property,
                "keyframe_list": points_json(points),
                "material_id": ""
            }));
    };
    for (channel, points) in kfs {
        match channel.as_str() {
            "scale" => {
                push("KFTypeScaleX", points);
                push("KFTypeScaleY", points);
            }
            "x" => push("KFTypePositionX", points),
            "y" => push("KFTypePositionY", points),
            "rotation" => push("KFTypeRotation", points),
            "opacity" => push("KFTypeAlpha", points),
            "volume" => push("KFTypeVolume", points),
            other => debug_assert!(false, "validated channel slipped through: {other}"),
        }
    }
}

fn copy_asset(out_dir: &Path, kind: &str, src: &Path) -> Result<String> {
    let dir = out_dir.join("assets").join(kind);
    std::fs::create_dir_all(&dir)?;
    let name = src
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "media.bin".to_string());
    let mut dest = dir.join(&name);
    let mut n = 0;
    let src_len = std::fs::metadata(src)?.len();
    while dest.exists() && std::fs::metadata(&dest)?.len() != src_len {
        n += 1;
        let stem = Path::new(&name)
            .file_stem()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| "media".to_string());
        let ext = Path::new(&name)
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();
        dest = dir.join(format!("{stem}_{n}{ext}"));
    }
    if !dest.exists() {
        std::fs::copy(src, &dest)
            .with_context(|| format!("copying {} into the draft", src.display()))?;
    }
    Ok(dest.to_string_lossy().into_owned())
}

/// Resolve a plan source path: absolute stays, relative resolves against the
/// plan file's directory (not the process CWD).
fn resolve_source(plan_dir: &Path, src: &Path) -> PathBuf {
    if src.is_absolute() {
        src.to_path_buf()
    } else {
        plan_dir.join(src)
    }
}

/// Build a full draft directory from a validated plan.
pub fn build(
    plan: &Plan,
    plan_dir: &Path,
    out_dir: &Path,
    prober: &dyn Fn(&Path) -> Result<MediaInfo>,
) -> Result<BuildReport> {
    if out_dir.exists() {
        bail!("output directory {} already exists", out_dir.display());
    }
    std::fs::create_dir_all(out_dir)?;

    let mut content: Value = serde_json::from_str(CONTENT_TEMPLATE)?;
    let mut meta: Value = serde_json::from_str(META_TEMPLATE)?;
    let now = now_us();

    content["id"] = json!(uuid_upper());
    content["name"] = json!(plan.name);
    content["duration"] = json!(plan.total_duration());
    content["fps"] = json!(plan.canvas.fps as f64);
    content["canvas_config"] = json!({
        "width": plan.canvas.width, "height": plan.canvas.height,
        "ratio": canvas_ratio(plan.canvas.width, plan.canvas.height)
    });
    content["create_time"] = json!(now / 1_000_000);
    content["update_time"] = json!(now / 1_000_000);
    content["platform"]["os"] = json!(host_os());
    content["last_modified_platform"]["os"] = json!(host_os());
    content["extra_info"] = json!({"created_via": "jycut"});

    let materials = &mut content["materials"];

    let mut tracks_json: Vec<Value> = Vec::with_capacity(plan.tracks.len());
    let mut segment_count = 0usize;

    for track in &plan.tracks {
        let mut segments_json: Vec<Value> = Vec::with_capacity(track.segments.len());

        for seg in track.segments.iter() {
            segment_count += 1;
            let mut seg_v: Value;
            let mut refs: Vec<String>;

            match track.kind.as_str() {
                "video" | "audio" => {
                    let src = resolve_source(
                        plan_dir,
                        seg.source.as_deref().unwrap_or_else(|| Path::new("")),
                    );
                    let info = prober(&src)?;
                    let kind = if track.kind == "video" {
                        "video"
                    } else {
                        "audio"
                    };
                    let stored = copy_asset(out_dir, kind, &src)?;

                    let (material_id, companions) = if track.kind == "video" {
                        let id = hex_id();
                        materials["videos"].as_array_mut().unwrap().push(json!({
                            "audio_fade": null, "category_id": "", "category_name": "local",
                            "check_flag": 63487,
                            "crop": {"lower_left_x": 0.0, "lower_left_y": 1.0,
                                      "lower_right_x": 1.0, "lower_right_y": 1.0,
                                      "upper_left_x": 0.0, "upper_left_y": 0.0,
                                      "upper_right_x": 1.0, "upper_right_y": 0.0},
                            "crop_ratio": "free", "crop_scale": 1.0,
                            "duration": info.duration_us, "height": info.height, "width": info.width,
                            "id": id, "local_material_id": "", "material_id": id,
                            "material_name": src.file_name().map(|n| n.to_string_lossy()).unwrap_or_default(),
                            "media_path": "", "path": stored, "type": "video"
                        }));
                        let c = make_companions(materials, seg.speed.unwrap_or(1.0), true);
                        (id, c)
                    } else {
                        let id = hex_id();
                        materials["audios"].as_array_mut().unwrap().push(json!({
                            "app_id": 0, "category_id": "", "category_name": "local",
                            "check_flag": 3, "copyright_limit_type": "none",
                            "duration": info.duration_us, "effect_id": "", "formula_id": "",
                            "id": id, "local_material_id": id, "music_id": id,
                            "name": src.file_name().map(|n| n.to_string_lossy()).unwrap_or_default(),
                            "path": stored, "source_platform": 0, "type": "extract_music",
                            "wave_points": []
                        }));
                        let c = make_companions(materials, seg.speed.unwrap_or(1.0), false);
                        (id, c)
                    };

                    let render_index = if track.kind == "video" {
                        RENDER_INDEX_VIDEO
                    } else {
                        RENDER_INDEX_AUDIO
                    };
                    seg_v = base_segment(&material_id, seg, render_index);
                    if track.kind == "video" {
                        apply_visuals(&mut seg_v, seg, true);
                    } else {
                        seg_v["clip"] = json!(null);
                        seg_v["hdr_settings"] = json!(null);
                    }
                    refs = companions.refs;
                    apply_keyframes(&mut seg_v, seg);
                }
                "text" => {
                    let material_id = hex_id();
                    let content_str = build_text_content(seg)?;
                    let border = seg.border_width.unwrap_or(0.0);
                    let mut text_material = json!({
                        "id": material_id, "type": "text", "content": content_str,
                        "alignment": seg.alignment.unwrap_or(1),
                        "font_size": seg.size.unwrap_or(8.0),
                        "text_color": seg.color.clone().unwrap_or_else(|| "#FFFFFF".into()).to_uppercase(),
                        "typesetting": 0, "letter_spacing": 0, "line_spacing": 0.02, "line_feed": 1,
                        "line_max_width": 0.82, "force_apply_line_max_width": false,
                        "check_flag": if border > 0.0 { 15 } else { 7 },
                        "fixed_width": -1, "fixed_height": -1
                    });
                    if border > 0.0 {
                        text_material["has_border"] = json!(true);
                        text_material["border_color"] = json!(seg
                            .border_color
                            .clone()
                            .unwrap_or_else(|| "#000000".into())
                            .to_uppercase());
                        text_material["border_width"] = json!(border);
                        text_material["border_alpha"] = json!(1);
                    }
                    materials["texts"]
                        .as_array_mut()
                        .unwrap()
                        .push(text_material);

                    seg_v = base_segment(&material_id, seg, RENDER_INDEX_TEXT);
                    seg_v["source_timerange"] = json!({"start": 0, "duration": seg.duration_us});
                    apply_visuals(&mut seg_v, seg, false);
                    // Subtitle baseline sits below center; jy14's -0.78 default
                    if seg.y.is_none() {
                        seg_v["clip"]["transform"]["y"] = json!(-0.78);
                    }
                    apply_keyframes(&mut seg_v, seg);
                    refs = Vec::new();
                }
                _ => unreachable!("validated track kind"),
            }

            if let Some(t) = &seg.transition_out {
                let entry = transition_lookup(&t.name)?;
                let transition_id = hex_id();
                materials["transitions"].as_array_mut().unwrap().push(json!({
                    "category_id": "", "category_name": "",
                    "duration": t.duration_us.unwrap_or(entry["default_duration_us"].as_i64().unwrap_or(500_000)),
                    "effect_id": entry["effect_id"],
                    "id": transition_id,
                    "is_overlap": entry["is_overlap"],
                    "name": entry["name"],
                    "platform": "all",
                    "resource_id": entry["resource_id"],
                    "type": "transition"
                }));
                refs.push(transition_id);
            }

            seg_v["extra_material_refs"] = json!(refs);
            segments_json.push(seg_v);
        }

        tracks_json.push(json!({
            "attribute": 0, "flag": 0, "id": hex_id(), "is_default_name": false,
            "name": track.name.clone().unwrap_or_else(|| track.kind.clone()),
            "segments": segments_json, "type": track.kind
        }));
    }

    content["tracks"] = json!(tracks_json);

    // meta sidecar
    meta["draft_id"] = json!(uuid::Uuid::new_v4().to_string());
    meta["draft_name"] = json!(plan.name);
    meta["draft_fold_path"] = json!(out_dir
        .canonicalize()
        .unwrap_or_else(|_| out_dir.to_path_buf())
        .to_string_lossy());
    meta["draft_root_path"] = json!(out_dir
        .parent()
        .map(|p| p.to_string_lossy())
        .unwrap_or_default());
    meta["draft_json_file"] = json!(out_dir.join("draft_content.json").to_string_lossy());
    meta["tm_draft_create"] = json!(now);
    meta["tm_draft_modified"] = json!(now);
    meta["tm_draft_removed"] = json!(0);
    meta["tm_duration"] = json!(plan.total_duration());

    let content_str = serde_json::to_string_pretty(&content)?;
    std::fs::write(out_dir.join("draft_content.json"), &content_str)?;
    std::fs::write(out_dir.join("draft_info.json"), &content_str)?;
    std::fs::write(
        out_dir.join("draft_meta_info.json"),
        serde_json::to_string_pretty(&meta)?,
    )?;

    Ok(BuildReport {
        name: plan.name.clone(),
        duration_us: plan.total_duration(),
        tracks: plan.tracks.len(),
        segments: segment_count,
        out_dir: out_dir.to_string_lossy().into_owned(),
        files: vec![
            "draft_content.json".into(),
            "draft_info.json".into(),
            "draft_meta_info.json".into(),
        ],
    })
}

/// Load the timeline from a draft directory (either mirror).
pub fn load_timeline(draft_dir: &Path) -> Result<Value> {
    for name in ["draft_content.json", "draft_info.json"] {
        let p = draft_dir.join(name);
        if p.is_file() {
            let v: Value = serde_json::from_str(&std::fs::read_to_string(&p)?)?;
            if v.get("tracks").map(Value::is_array).unwrap_or(false) {
                return Ok(v);
            }
        }
    }
    bail!(
        "no readable timeline in {} (expected draft_content.json or draft_info.json)",
        draft_dir.display()
    );
}

/// Structural lint over a built/published draft.
pub fn verify(draft_dir: &Path) -> Result<Value> {
    let tl = load_timeline(draft_dir)?;
    let mut issues: Vec<String> = Vec::new();

    let tracks = tl["tracks"].as_array().context("tracks must be an array")?;
    if tracks.is_empty() {
        issues.push("no tracks".into());
    }
    let first_video = tracks.iter().position(|t| t["type"] == "video");
    if first_video != Some(0) {
        issues.push("the main video track must be the first track".into());
    }
    let mut max_end: i64 = 0;
    for (ti, t) in tracks.iter().enumerate() {
        let segs = t["segments"]
            .as_array()
            .context("segments must be an array")?;
        let mut prior_end: i64 = 0;
        for s in segs {
            let start = s["target_timerange"]["start"].as_i64().unwrap_or(0);
            let dur = s["target_timerange"]["duration"].as_i64().unwrap_or(0);
            if start < prior_end {
                issues.push(format!("track {ti}: overlapping segment at {start}"));
            }
            prior_end = start + dur;
            max_end = max_end.max(prior_end);
            let mid = s["material_id"].as_str().unwrap_or_default();
            let bucket = match t["type"].as_str() {
                Some("video") => Some("videos"),
                Some("audio") => Some("audios"),
                Some("text") => Some("texts"),
                _ => None,
            };
            if let Some(bucket) = bucket {
                let found = tl["materials"][bucket]
                    .as_array()
                    .map(|a| a.iter().any(|m| m["id"] == json!(mid)))
                    .unwrap_or(false);
                if !found {
                    issues.push(format!("track {ti}: material {mid} missing from {bucket}"));
                }
            }
            for r in s["extra_material_refs"].as_array().unwrap_or(&vec![]) {
                let r = r.as_str().unwrap_or_default();
                let known = [
                    "speeds",
                    "placeholders",
                    "sound_channel_mappings",
                    "vocal_separations",
                    "canvases",
                    "material_colors",
                    "transitions",
                ]
                .iter()
                .any(|b| {
                    tl["materials"][b]
                        .as_array()
                        .map(|a| a.iter().any(|m| m["id"] == json!(r)))
                        .unwrap_or(false)
                });
                if !known {
                    issues.push(format!("track {ti}: dangling extra_material_ref {r}"));
                }
            }
        }
        if ti == 0 && t["type"] == "video" {
            let first = t["segments"][0]["target_timerange"]["start"]
                .as_i64()
                .unwrap_or(-1);
            if first != 0 {
                issues.push("main video must start at 0".into());
            }
        }
    }
    let declared = tl["duration"].as_i64().unwrap_or(0);
    if declared != max_end {
        issues.push(format!(
            "top duration {declared} != max segment end {max_end}"
        ));
    }
    Ok(json!({
        "ok": issues.is_empty(),
        "name": tl["name"],
        "duration_us": declared,
        "tracks": tracks.len(),
        "issues": issues,
    }))
}

/// Human/agent-readable summary of a draft.
pub fn inspect(draft_dir: &Path) -> Result<Value> {
    let tl = load_timeline(draft_dir)?;
    let tracks: Vec<Value> = tl["tracks"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|t| {
            json!({
                "type": t["type"], "name": t["name"],
                "segments": t["segments"].as_array().map(|a| a.len()).unwrap_or(0),
            })
        })
        .collect();
    Ok(json!({
        "name": tl["name"], "duration_us": tl["duration"],
        "canvas": tl["canvas_config"], "fps": tl["fps"],
        "platform": tl["platform"], "tracks": tracks,
    }))
}
