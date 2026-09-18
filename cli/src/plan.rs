//! `jycut-plan/v1` — the CLI's input contract.
//!
//! Field semantics follow the interoperability facts established across
//! pyJianYingDraft / capcut-cli / jianying-headless: all times are microseconds
//! on the timeline, `start_us` positions a segment on its track, sources are
//! local files, the first video track is the continuous main track.

use anyhow::{bail, Result};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub const SCHEMA: &str = "jycut-plan/v1";
pub const FPS_VALUES: [u64; 5] = [24, 25, 30, 50, 60];

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    pub schema: String,
    pub name: String,
    pub canvas: Canvas,
    pub tracks: Vec<Track>,
    /// Directory of the plan file, set by `Plan::load`; relative sources
    /// resolve against it.
    #[serde(skip)]
    pub parent: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Canvas {
    pub width: u64,
    pub height: u64,
    pub fps: u64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Track {
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub name: Option<String>,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Segment {
    pub start_us: i64,
    pub duration_us: i64,
    #[serde(default)]
    pub source: Option<PathBuf>,
    #[serde(default)]
    pub source_start_us: i64,
    #[serde(default)]
    pub source_duration_us: Option<i64>,
    #[serde(default)]
    pub speed: Option<f64>,
    #[serde(default)]
    pub volume: Option<f64>,
    // video-only visuals
    #[serde(default)]
    pub scale: Option<f64>,
    #[serde(default)]
    pub x: Option<f64>,
    #[serde(default)]
    pub y: Option<f64>,
    #[serde(default)]
    pub rotation: Option<f64>,
    #[serde(default)]
    pub opacity: Option<f64>,
    #[serde(default)]
    pub keyframes: Option<BTreeMap<String, Vec<KeyPoint>>>,
    #[serde(default)]
    pub transition_out: Option<TransitionOut>,
    // text-only
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub size: Option<f64>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub border_color: Option<String>,
    #[serde(default)]
    pub border_width: Option<f64>,
    #[serde(default)]
    pub bold: Option<bool>,
    #[serde(default)]
    pub alignment: Option<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyPoint {
    pub at_us: i64,
    pub value: f64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransitionOut {
    pub name: String,
    #[serde(default)]
    pub duration_us: Option<i64>,
}

fn check_range(v: f64, lo: f64, hi: f64, what: &str) -> Result<()> {
    if !(lo..=hi).contains(&v) {
        bail!("{what} {v} outside {lo}..{hi}");
    }
    Ok(())
}

const KEY_CHANNELS_VIDEO: &[(&str, f64, f64)] = &[
    ("scale", 0.01, 10.0),
    ("x", -2.0, 2.0),
    ("y", -2.0, 2.0),
    ("rotation", -360.0, 360.0),
    ("opacity", 0.0, 1.0),
];
const KEY_CHANNELS_TEXT: &[(&str, f64, f64)] = &[
    ("scale", 0.01, 10.0),
    ("x", -2.0, 2.0),
    ("y", -2.0, 2.0),
    ("rotation", -360.0, 360.0),
];
const KEY_CHANNELS_AUDIO: &[(&str, f64, f64)] = &[("volume", 0.0, 4.0)];

impl Plan {
    pub fn load(path: &Path) -> Result<Plan> {
        let raw = std::fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("cannot read plan {}: {e}", path.display()))?;
        let mut plan: Plan = serde_json::from_str(&raw)
            .map_err(|e| anyhow::anyhow!("plan is not a valid {SCHEMA}: {e}"))?;
        plan.parent = Some(path.parent().map(Path::to_path_buf).unwrap_or_default());
        plan.validate()?;
        Ok(plan)
    }

    pub fn validate(&self) -> Result<()> {
        if self.schema != SCHEMA {
            bail!(
                "unsupported plan schema {} (expected {SCHEMA})",
                self.schema
            );
        }
        let name = &self.name;
        if name.is_empty()
            || name.len() >= 200
            || name.starts_with('.')
            || name.chars().any(|c| "/\\\n\r\0".contains(c))
        {
            bail!("draft name must be a visible single directory component");
        }
        if !(16..=8192).contains(&self.canvas.width) || !(16..=8192).contains(&self.canvas.height) {
            bail!("canvas dimensions must be 16..8192");
        }
        if !FPS_VALUES.contains(&self.canvas.fps) {
            bail!("fps must be one of {FPS_VALUES:?}");
        }
        if self.tracks.is_empty() {
            bail!("plan needs at least one track");
        }
        let mut seen_main = false;
        for (ti, track) in self.tracks.iter().enumerate() {
            if track.kind == "video" && !seen_main {
                if ti != 0 {
                    bail!("the main video track must be the first track");
                }
                seen_main = true;
            }
            if !matches!(track.kind.as_str(), "video" | "audio" | "text") {
                bail!(
                    "unsupported track kind {} (jycut supports video/audio/text)",
                    track.kind
                );
            }
            if track.segments.is_empty() {
                bail!("track {} needs at least one segment", ti);
            }
            let mut prior_end: i64 = 0;
            for (si, seg) in track.segments.iter().enumerate() {
                let last = ti == 0 && si + 1 == track.segments.len();
                self.validate_segment(track, seg, prior_end, last)?;
                prior_end = seg.start_us + seg.duration_us;
            }
            if track.kind == "video" && ti == 0 && track.segments[0].start_us != 0 {
                bail!("main video must start at 0; place gaps on overlay tracks only");
            }
        }
        if !seen_main {
            bail!("plan needs a video track (the main track)");
        }
        Ok(())
    }

    fn validate_segment(
        &self,
        track: &Track,
        seg: &Segment,
        prior_end: i64,
        last_on_main: bool,
    ) -> Result<()> {
        if seg.start_us < prior_end {
            bail!(
                "segments on track {} must be ordered and non-overlapping ({} < {})",
                track.name_deref(),
                seg.start_us,
                prior_end
            );
        }
        if seg.duration_us <= 0 {
            bail!("duration_us must be positive");
        }
        let speed = seg.speed.unwrap_or(1.0);
        check_range(speed, 0.1, 8.0, "speed")?;
        let volume = seg.volume.unwrap_or(1.0);
        check_range(volume, 0.0, 4.0, "volume")?;

        match track.kind.as_str() {
            "video" | "audio" => {
                let src = seg
                    .source
                    .as_ref()
                    .ok_or_else(|| anyhow::anyhow!("segment at {} needs a source", seg.start_us))?;
                if src.as_os_str().is_empty() {
                    bail!("segment at {} has an empty source path", seg.start_us);
                }
                if seg.text.is_some() || seg.size.is_some() || seg.color.is_some() {
                    bail!("{}-track segments cannot carry text fields", track.kind);
                }
            }
            "text" => {
                if seg.text.as_deref().map(str::trim).unwrap_or("").is_empty() {
                    bail!("text segment at {} has empty text", seg.start_us);
                }
                if seg.source.is_some() {
                    bail!("text segments cannot carry a source");
                }
            }
            _ => unreachable!(),
        }

        if let Some(t) = &seg.transition_out {
            if track.kind != "video" {
                bail!("transitions are only supported on video tracks");
            }
            if last_on_main {
                bail!("a transition needs a following segment (cannot be on the last one)");
            }
            let dur = t.duration_us.unwrap_or(0);
            if dur <= 0 || dur > 1_000_000 {
                bail!("transition duration_us must be within 1..1000000");
            }
        }

        // keyframes: linear, ordered, first point at 0; trimmed/speed-adjusted
        // sources have an unverified time mapping, so keep the jy14 rule.
        if let Some(kfs) = &seg.keyframes {
            let allowed: &[(&str, f64, f64)] = match track.kind.as_str() {
                "video" => KEY_CHANNELS_VIDEO,
                "text" => KEY_CHANNELS_TEXT,
                _ => KEY_CHANNELS_AUDIO,
            };
            if speed != 1.0 || seg.source_start_us != 0 {
                bail!("keyframes require speed == 1 and source_start_us == 0");
            }
            for (channel, points) in kfs {
                let (_, lo, hi) =
                    allowed
                        .iter()
                        .find(|(c, _, _)| c == channel)
                        .ok_or_else(|| {
                            anyhow::anyhow!(
                                "unsupported keyframe channel {channel} on {} track",
                                track.kind
                            )
                        })?;
                if points.len() < 2 {
                    bail!("keyframe channel {channel} needs at least two points");
                }
                let mut prev: i64 = -1;
                for p in points {
                    if p.at_us <= prev || p.at_us > seg.duration_us {
                        bail!("keyframe channel {channel} times must ascend within the segment");
                    }
                    prev = p.at_us;
                    check_range(p.value, *lo, *hi, &format!("keyframe {channel}"))?;
                }
                if points[0].at_us != 0 {
                    bail!("keyframe channel {channel} must start at at_us 0");
                }
                let static_conflict = match channel.as_str() {
                    "scale" => seg.scale,
                    "x" => seg.x,
                    "y" => seg.y,
                    "rotation" => seg.rotation,
                    "opacity" => seg.opacity,
                    "volume" => seg.volume,
                    _ => None,
                };
                if let Some(v) = static_conflict {
                    if (v - points[0].value).abs() > 1e-7 {
                        bail!("static {channel} conflicts with its first keyframe value");
                    }
                }
            }
        }

        if track.kind != "text" {
            if let Some(v) = seg.scale {
                check_range(v, 0.01, 10.0, "scale")?;
            }
            if let Some(v) = seg.x {
                check_range(v, -2.0, 2.0, "x")?;
            }
            if let Some(v) = seg.y {
                check_range(v, -2.0, 2.0, "y")?;
            }
            if let Some(v) = seg.rotation {
                check_range(v, -360.0, 360.0, "rotation")?;
            }
            if let Some(v) = seg.opacity {
                check_range(v, 0.0, 1.0, "opacity")?;
            }
        } else if let Some(w) = seg.border_width {
            check_range(w, 0.0, 15.0, "border_width")?;
        }
        Ok(())
    }

    pub fn total_duration(&self) -> i64 {
        self.tracks
            .iter()
            .flat_map(|t| t.segments.iter())
            .map(|s| s.start_us + s.duration_us)
            .max()
            .unwrap_or(0)
    }
}

impl Track {
    fn name_deref(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.kind)
    }
}

/// Parse `#RRGGBB` (6 hex digits) into 0..1 floats. Returns alpha and the
/// material-level hex form separately when needed.
pub fn hex_to_rgb01(hex: &str) -> Result<[f64; 3]> {
    let h = hex.trim().trim_start_matches('#');
    if h.len() != 6 || !h.chars().all(|c| c.is_ascii_hexdigit()) {
        bail!("color must be #RRGGBB, got {hex:?}");
    }
    let ch = |i: usize| -> Result<f64> {
        u8::from_str_radix(&h[i * 2..i * 2 + 2], 16)
            .map(|v| v as f64 / 255.0)
            .map_err(|e| anyhow::anyhow!("bad color {hex:?}: {e}"))
    };
    Ok([ch(0)?, ch(1)?, ch(2)?])
}
