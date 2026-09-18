---
name: jianying-subtitles
description: "Design text/subtitle tracks for a JianYing plan: per-sentence placement timed to measured speech, full TextStyle (size/color/bold/align/letter_spacing), safe-area rules, and no-overlap windows."
---

# JianYing Subtitles（字幕轨设计）

字幕轨 = text track + TextSegment（引擎 `TextSegment(text, timerange=, style=)`）。

## 放置规则

1. **逐句对齐语音**：每条字幕的起止 = 该句语音的实测起止（来自 ASR 词级时间戳或
   ffprobe 实测），不是镜头切点。一句跨切点时字幕保持连续（观众读字优先于切点）。
2. **安全区**：底部字幕 `start_us` 离画面底 ≥ 高度 8%；避免与已有贴字/花字重叠。
3. **可读时长**：中文 ≥ 0.25s/字；一行 ≤ 16 字，超长拆两条或分行。
4. **样式基线**：`TextStyle(size=8.0, color=(1,1,1), bold=True)` + 黑描边可读性最稳；
   强调句用 `color` 提亮，整轨保持同一字体家族（剪映内可再统一替换）。

## 计划片段示例

```json
{"type": "text", "clips": [
  {"text": "上半是 AI 排的方块片", "start_us": 200000, "duration_us": 2200000,
   "size": 8.0, "color": [1, 1, 1], "bold": true},
  {"text": "下半是 AI 拍的成片", "start_us": 2600000, "duration_us": 2400000,
   "size": 8.0, "color": [1, 1, 1], "bold": true}
]}
```

## 与转写联动

长口播的逐句字幕：用 ASR 词级时间戳把 transcript 按标点切句，每句一个 clip；
禁止手工估时间。ASR 不可用时（CLI 身份墙）如实报告，不手估。

## Never do

- Never 让两条字幕在同时刻重叠（读者只能读一条）。
- Never 用字幕复述画面已有的文字（片名/花字）。
