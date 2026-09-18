---
name: jianying-use
description: "Route JianYing (剪映专业版) editing requests: capability preflight first (native app, draft root, vendored engine), then edit-plan to native draft generation, narration condensing, subtitles, audio, motion, transitions, inspection, recovery, or environment setup. Drafts open as fully editable native JianYing projects."
---

# JianYing Edit Router

Use this as the entry point. Before any draft generation, run the capability
preflight (step 0) — the workflow needs 剪映专业版 installed, a draft root that
exists, and the vendored engine; missing pieces must surface as actionable
guidance, never as a raw traceback.

## Step 0 — capability preflight (always first)

```bash
python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jy_headless/cli.py" detect
```

Read the output and act on it:

- `roots: []` → 剪映专业版未安装或未启动过。Guidance: install 剪映专业版
  (macOS /Applications/剪映专业版.app, Windows 官网安装包), launch it once and
  create/open any draft so the draft root is created, then re-run. Windows
  users can also set `JY_DRAFT_ROOT` to a custom draft root.
- Drafts listed → ready. The primary root is where generated drafts land.

## Routing

| 请求 | 技能 |
|---|---|
| 剪辑计划 → 原生草稿（核心工作流） | `jianying-edit` |
| 长口播 → 精剪版草稿（ASR + 选段） | `jianying-narration` |
| 字幕轨设计与放置 | `jianying-subtitles` |
| 音频轨/解说/BGM 分层 | `jianying-audio` |
| 关键帧动效（推拉/横移/呼吸） | `jianying-motion` |
| 转场选型与放置 | `jianying-transitions` |
| 素材/草稿/生成结果探测 | `jianying-inspect` |
| 同名冲突/恢复/备份 | `jianying-recover` |
| 导出前检查清单 | `jianying-export-prep` |
| 环境/安装/排障 | `jianying-setup` + `jianying-harness` |

## Never do

- Never modify or delete an existing user draft; generation only creates new
  drafts (allow_replace is opt-in).
- Never claim native MP4 export is automated: the deliverable is the draft;
  export happens in 剪映 by the user (automated export is a future engine
  capability).
- Never invent materials: every `material` path in the plan must exist and be
  a real media file the user provided or our pipeline generated.
