---
name: jianying-inspect
description: "Inspect materials, draft root, and engine health before planning: ffprobe media durations, headless_draft.py doctor, edit-mode inspect of existing drafts. Read-only."
---

# JianYing Inspect（素材与草稿探测）

计划前必做：**用实测时长，不用计划时长**——生成模型会漂移（白模切点漂移教训）。

```bash
HD="$JIANYING_HEADLESS_ROOT/skills/yichen-jianying-edit/scripts/headless_draft.py"
```

## 引擎与草稿根

```bash
python3 "$HD" doctor
```

输出运行时（剪映 11.4.x 钉扎）、草稿根路径、资源目录健康度。任何一项异常，
按 `jianying-setup` 修复后再继续；不要带着坏环境生成。

## 素材探测

```bash
ffprobe -v error -show_entries format=duration -of csv=p=0 <media>
ffprobe -v error -select_streams v:0 -show_entries stream=width,height,r_frame_rate -of csv=p=0 <media>
```

每个进入计划的 `source` 都要探测：实测时长决定 `duration_us` 与
`source_start_us` 的合法上限；混合分辨率可同剪映工程但同轨混分辨率会被缩放。

## 已有草稿探测（独立副本，只读）

```bash
python3 "$HD" edit inspect --draft <草稿目录> --out WORK/inspect.json
```

读 `WORK/inspect.json` 拿轨道/段结构。计划前检查同名冲突：目标名已存在且用户
可能编辑过 → 换名并提示；确需覆盖必须用户明说。**绝不修改原草稿目录内文件。**

## 生成产物核验

- `verify-build --build WORK/build --report WORK/vb.json`：发布前的结构核验。
- `verify --build WORK/build`：发布后的活体回读。
- 深度核对段切点（对齐预期时间轴）读 verify 报告/审计 JSON，不在草稿目录里改东西。
