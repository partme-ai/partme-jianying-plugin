---
name: jianying-inspect
description: "Inspect materials and drafts before/after generation: ffprobe measured durations, draft JSON structure read-back, and the built-in jycut verify/inspect as lint tooling. Read-only."
license: Apache-2.0
---

# JianYing Inspect（素材与草稿探测）

计划前必做：**用实测时长，不用设计时长**——生成模型会漂移（白模切点漂移教训）。

## 素材探测

```bash
ffprobe -v error -show_entries format=duration -of csv=p=0 <media>
ffprobe -v error -select_streams v:0 -show_entries stream=width,height,r_frame_rate -of csv=p=0 <media>
```

每个进入脚本的素材都要探测：实测时长决定 `Timerange` 合法上限（越界库会
ValueError）；同轨混分辨率会被缩放。内置 Rust CLI 亦可：
`cli/target/release/jycut probe <media>`（同源 ffprobe）。

## 草稿根与已有草稿

- 草稿根：`jydraft_check.py` 的 `draft_roots` 列出路径与存在性。
- 计划前检查同名冲突：目标名已存在且用户可能编辑过 → 换名并提示；确需覆盖
  必须用户明说（`create_draft(allow_replace=True)` 是显式门禁）。
- 读已有草稿结构（只读）：直接读 `draft_content.json` 或
  `jycut inspect <草稿目录>`。

## 生成产物核验

- 读回 `draft_content.json`：轨道/段数与设计一致、`duration` = 各段末端
  最大值、转场挂在前段 `extra_material_refs`、关键帧按属性成组。
- `jycut verify <草稿目录>`：引用完整性 + 主轨连续 + 时长一致性 lint。

## Never do

- Never 依据计划时长而非实测值放置后续轨（对齐漂移是字幕错位的头号原因）。
- Never 修改已有草稿目录里的任何文件。
