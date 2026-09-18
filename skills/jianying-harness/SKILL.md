---
name: jianying-harness
description: "Deep-dive on the vendored jy-headless engine: CLI surface, plan contract (jianying-plan/v1), draft-root resolution, vendor pinning, and troubleshooting for every known failure mode. Use when a generation command fails, paths behave oddly, or the plan contract needs clarification."
---

# JianYing Harness（引擎深用与排障）

引擎 = `scripts/jy_headless/`（vendored 自 partme-ai/jy-headless v0.2.0，Apache-2.0）。
本技能是它的完整操作手册。

## CLI 全surface

```bash
python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jy_headless/cli.py" detect
python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jy_headless/cli.py" generate --plan plan.json [--root <root>] [--no-replace]
python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jy_headless/cli.py" verify --draft-dir <dir>
```

## 计划契约 `jianying-plan/v1`（v0.2 起含可选扩展字段）

顶层：`schema` / `draft{name,width,height,fps}` / `tracks[]`。
轨道类型：`video` / `text` / `audio`；每 clip 微秒时间（`start_us`/`duration_us`）。

- video clip：`material`（本地路径，必填存在）、`volume`、`speed`、
  可选 `transition_out{type,duration_us}`（叠化/推拉/翻页等中文目录，
  见 `jianying-transitions`）、可选 `keyframes[]`（uniform_scale/position_x/
  position_y/rotation + time_offset_us + value，见 `jianying-motion`）。
- text clip：`text`、`size`、`color[r,g,b]`、`bold/italic/underline`、
  `align`、`letter_spacing`（见 `jianying-subtitles`）。
- audio clip：`material`、`volume`、`speed`。

## 草稿根解析顺序

`--root` 参数 → `JY_DRAFT_ROOT` 环境变量 → 平台默认路径（macOS
`~/Movies/JianyingPro/User Data/Projects/com.lveditor.draft`；Windows
`%LOCALAPPDATA%\JianyingPro\...`）。根目录不存在时 generate 会自动创建。

## 故障对照表

| 症状 | 根因 | 处置 |
|---|---|---|
| `根文件夹 ... 不存在` | 剪映未安装/未启动过 | 装 剪映专业版 并启动一次；或 `--root` 显式指定 |
| `plan schema 必须是 jianying-plan/v1` | schema 字段错/漏 | 改 plan.json |
| `unknown variant` / `KeyError: '叠化'` | 转场名不在 TransitionType 目录 | 查 `jianying-transitions` 的目录表 |
| 同名草稿报错 | 未加 allow_replace | 换名（推荐）或显式允许替换 |
| 引擎文件漂移 | vendor 被手改 | `python3 scripts/vendor_engine.py update` 重锁 |

## Never do

- Never 手改 `scripts/jy_headless/` 下的 vendored 文件——改引擎去
  partme-ai/jy-headless 仓打 tag，再 `vendor_engine.py update`。
- Never 在 plan 里引用不存在的素材路径。
