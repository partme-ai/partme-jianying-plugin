---
name: jianying-inspect
description: "Inspect materials, existing drafts, and generated draft JSON before planning: ffprobe media durations, list draft-root contents, verify draft JSON structure. Read-only."
---

# JianYing Inspect（素材与草稿探测）

计划前必做：**用实测时长，不用计划时长**——生成模型会漂移（白模切点漂移教训）。

## 素材探测

```bash
ffprobe -v error -show_entries format=duration -of csv=p=0 <media>
ffprobe -v error -select_streams v:0 -show_entries stream=width,height,r_frame_rate -of csv=p=0 <media>
```

每个进入 plan 的素材都要探测：实际时长决定 `duration_us` 上限，分辨率决定是否
需要统一转码（剪映可容纳混合分辨率，但同轨混分辨率会被缩放）。

## 已有草稿探测

```bash
python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jy_headless/cli.py" detect
```

`existing_drafts` 列出草稿名。生成前检查同名冲突：同名且用户可能编辑过 →
换名并提示；确需覆盖必须用户明说。

## 生成结果校验

```bash
python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jy_headless/cli.py" verify --draft-dir <dir>
```

校验 draft_content.json 可解析并报告轨道/段数。深度核对（段起点是否对齐切点）：

```python
import json
d = json.load(open("<draft>/draft_content.json"))
for tr in d["tracks"]:
    for s in tr.get("segments", []):
        t = s["target_timerange"]
        print(tr["type"], t["start"] / 1e6, (t["start"] + t["duration"]) / 1e6)
```

## Never do

- Never 依据计划时长而非 ffprobe 实测值放置后续轨（对齐漂移是字幕错位的头号原因）。
- Never 修改已有草稿目录里的任何文件。
