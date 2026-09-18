---
name: jianying-edit
description: "Turn an edit-plan JSON (jianying-plan/v1: video/text/audio tracks with per-clip material, start, duration, volume, speed) into a native, fully editable JianYing Pro draft via the vendored jy-headless engine. Deterministic, offline, non-destructive to existing drafts."
---

# JianYing Edit（剪辑计划 → 原生草稿）

Deterministic engine: `scripts/jy_headless/cli.py`（vendored from
partme-ai/jy-headless, Apache-2.0）. You design the edit plan; the engine
generates the native draft; 剪映 opens it as a normal editable project.

## Edit-plan contract (`jianying-plan/v1`)

```json
{
  "schema": "jianying-plan/v1",
  "draft": {"name": "episode-01", "width": 1280, "height": 720, "fps": 24},
  "tracks": [
    {"type": "video", "clips": [
      {"material": "generated/shot-01.mp4", "start_us": 0,
       "duration_us": 6000000, "volume": 1.0, "speed": 1.0}
    ]},
    {"type": "text", "clips": [
      {"text": "解说字幕", "start_us": 500000, "duration_us": 3000000,
       "size": 8.0, "color": [1.0, 1.0, 1.0]}
    ]},
    {"type": "audio", "clips": [
      {"material": "narration.mp3", "start_us": 0,
       "duration_us": 6000000, "volume": 1.0}
    ]}
  ]
}
```

Rules: `start_us`/`duration_us` are integers in microseconds; every `material`
must be an existing local media file; draft name must be unique in the draft
root unless replacement is explicitly accepted; video and text/audio tracks
are assembled bottom-up in the listed order.

## Workflow

1. **Design the edit plan** from the user's brief or our pipeline artifacts
   (shot table from `blender-previs`, generated clips from
   `minimax-video-generation`, narration from the speech layer). Plan the
   multi-track layout: main video track first, then text/subtitle, then audio.
2. **Validate materials**: every referenced file must exist and be a real
   media file; probe durations with `ffprobe` so text/audio placement matches
   actual clip lengths instead of planned lengths.
3. **Write the plan** as `plan.json` following the contract, then:

```bash
python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jy_headless/cli.py" generate \
  --plan plan.json
```

4. **Verify**: `verify --draft-dir <输出目录>` parses the generated
   draft_content.json; report track/segment counts.
5. **Deliver**: the draft name and root — the user opens 剪映专业版 and the
   draft appears in the start page, fully editable (multi-track, speed,
   volume, text styles are all native).

## Iteration

A rejected edit is a plan change: adjust `plan.json` (or the upstream shot
table), regenerate with a new draft name (keep the rejected draft for
comparison unless the user asks to clean up). Never overwrite a draft the user
may have edited in 剪映: `--no-replace` makes the engine fail instead of
replacing.

## Never do

- Never touch, move, or delete an existing draft in the root; generation
  creates new drafts only.
- Never reference materials that do not exist, and never approximate a brand
  logo, UI, or identity asset inside generated text or media.
- Never promise automated MP4 export: the draft opens in 剪映, and the user
  exports there (headless export automation is a future engine capability).
- Never proceed past material validation with placeholder media the user has
  not approved.
