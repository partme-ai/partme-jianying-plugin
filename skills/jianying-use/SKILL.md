---
name: jianying-use
description: "Route JianYing (剪映专业版) draft-generation requests: environment preflight first (vendored pyJianYingDraft engine, MediaInfo probe, draft root), then route to pyJianYingDraft script generation (primary), the built-in Rust jianying-cli (full-capability fast path), narration condensing, subtitles, audio, motion, transitions, inspection, recovery, or the optional fork pro-tier. Drafts open as fully editable native JianYing projects."
license: Apache-2.0
---

# JianYing Edit Router

Entry point. Run the capability preflight (step 0) before any draft work —
the vendored engine needs pymediainfo/MediaInfo for media probing, and a draft
root must exist. Missing pieces must surface as actionable guidance, never as
a raw traceback.

## Engine tiers (pick per request)

| 层 | 引擎 | 何时用 |
|---|---|---|
| **主力** | vendored [pyJianYingDraft](https://github.com/GuanYixuan/pyJianYingDraft)（Apache-2.0，插件内 `scripts/vendor/`，逐文件 SHA-256 钉扎） | 任何"计划/需求 → 原生草稿"生成：多轨、转场、关键帧、蒙版、样式文本、SRT 字幕、入场出场动画、滤镜/特效元数据 |
| 快路径 | 内置 Rust CLI **jianying-cli**（`jianying-cli-plan/v1`，全能力目录：转场 453/滤镜 1052/特效/动画/蒙版/混音等，repo `full-aigc-plugins/jianying-cli`） | 已有结构化计划 JSON、要确定性批量构建时 |
| 专业档（可选） | 用户自己的 [partme-ai/jianying-headless](https://github.com/partme-ai/jianying-headless) fork 检出（NC 许可，零改动驱动） | 仅原生 MP4 导出、已有草稿编辑、ASR 记账 |

## Step 0 — capability preflight (always first)

```bash
python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jydraft_check.py"
```

Read the JSON and act:

- `engine` 非 `ok (vendored)` → vendor 损坏或缺 pip 依赖：如实报告，禁止改
  hash 钉扎"修好"。
- `media_probe` 缺 pymediainfo/MediaInfo → `pip install pymediainfo` +
  macOS `brew install mediainfo`（Windows 装 MediaInfo 加 PATH）。
- 所有 draft_roots 不存在 → 让用户启动一次剪映专业版新建任意草稿。
- `ffprobe` 缺 → `brew install ffmpeg`（素材实测与合成需要）。

## Routing

| 请求 | 技能/入口 |
|---|---|
| 需求/分镜 → 原生草稿（核心工作流） | `jianying-edit` |
| pyJianYingDraft API 速查/脚本配方 | `jianying-draft` |
| 口播长视频 → 精剪草稿（keep/drop） | `jianying-narration` |
| 字幕/标题/描边/花字背景 | `jianying-subtitles` |
| 音频/BGM/音量/场景音 | `jianying-audio` |
| 关键帧/入场出场动画 | `jianying-motion` |
| 转场（453 目录纪律） | `jianying-transitions` |
| 素材/草稿探测 | `jianying-inspect` |
| 导出前检查 | `jianying-export-prep` |
| 同名冲突/恢复 | `jianying-recover` |
| 原生导出/改已有草稿/ASR 记账（NC pro 档） | `jianying-harness` |
| 环境/安装/许可边界 | `jianying-setup` |

## Never do

- Never 修改 vendor 目录内容或其 hash 钉扎（升级走仓库同步 + 重打 pin）。
- Never 修改 fork 检出或绕过其哈希钉扎（pro 档零改动纪律）。
- Never 声称可无人值守导出 MP4（pyJianYingDraft 的导出自动化仅 Windows
  旧版剪映 UIA；导出由用户在剪映内完成）。
- Never 把会员标记资源当可用授权。
