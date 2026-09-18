---
name: jianying-use
description: "Route JianYing (剪映专业版) automation requests: capability preflight first (fork checkout, 剪映 11.4.x runtime, draft root, ASR executor), then route to edit-plan compilation, native draft build/publish, narration condensing, subtitles, audio, motion, masks, transitions, effects, or export. Drafts open as fully editable native JianYing projects."
---

# JianYing Edit Router

Use this as the entry point. Before any draft operation, run the capability
preflight (step 0) — the engine requires 剪映专业版 11.4.x on this Mac, a fork
checkout, and (for ASR) an executor; missing pieces must surface as actionable
guidance, never as a raw traceback.

## Step 0 — capability preflight (always first)

```bash
python3 "$JIANYING_HEADLESS_ROOT/skills/yichen-jianying-edit/scripts/headless_draft.py" doctor
```

Resolve `$JIANYING_HEADLESS_ROOT` in this order: the `JIANYING_HEADLESS_ROOT`
environment variable → `~/workspaces/workspace-partme-ai/jianying-headless` →
ask the user for their fork checkout path (never search the whole disk).

Read the doctor output and act:

- Core checkout missing or hash mismatch → guide the user to
  `git clone https://github.com/partme-ai/jianying-headless.git` (their fork)
  and set `JIANYING_HEADLESS_ROOT`. Do not "fix" hashes or versions to pass.
- 剪映 app missing / version not 11.4.x → the engine is version-pinned;
  installing a different major version is a user decision, not a workaround.
- Draft root missing → launch 剪映 once and create any draft.

## Routing

| 请求 | 技能/入口 |
|---|---|
| 口播长视频 → 精剪版草稿（ASR + 选段） | `jianying-narration` |
| 剪辑计划 → 原生草稿（build/publish） | `jianying-edit` |
| 计划字段/蒙版/资源名速查 | `jianying-edit` 的 references/plan-format.md |
| 修改已有多轨草稿（独立副本） | `jianying-edit` 的 edit 入口 |
| 字幕/标题/花字设计 | `jianying-subtitles` |
| 音频/BGM/音量分层 | `jianying-audio` |
| 关键帧动效 | `jianying-motion` |
| 转场（叠化 + edge_policy） | `jianying-transitions` |
| 滤镜/特效（已采集目录） | `jianying-edit` + fork 的 native-resource-catalog.json |
| 导出 MP4（原生） | `jianying-export-prep` |
| 同名冲突/恢复/审计 | `jianying-recover` |
| pyJianYingDraft 底层原理 | `jianying-draft`（进阶参考） |
| 环境/安装/排障 | `jianying-setup` + `jianying-harness` |

## Never do

- Never modify the fork checkout: the plugin drives it as-is; upstream updates
  land via GitHub sync on the user's fork.
- Never bypass signature/version checks (they protect against draft corruption).
- Never promise automated cloud features (在线模板/资源下载/账号权益 are out of
  scope by the engine's own contract).
