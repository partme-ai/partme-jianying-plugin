---
name: jianying-subtitles
description: "Design text tracks for a jy14-headless-plan/v1: per-sentence placement timed to measured speech, text/size/x/y/color/border fields, 花字 text_effect (mutually exclusive with custom colors), and no-overlap windows."
---

# JianYing Subtitles（字幕轨设计）

字幕轨 = `{type: "text", name, segments[]}`；段字段：`start_us, duration_us,
text, size, x, y, color(#RRGGBB), border_color, border_width, opacity,
keyframes, text_effect`。

## 放置规则

1. **逐句对齐语音**：每条字幕起止 = 该句语音的实测起止（ASR 词级时间戳或
   ffprobe），不是镜头切点。一句跨切点时字幕保持连续（观众读字优先于切点）。
2. **安全区**：底部字幕 `y` 默认 -0.78（归一化，向上为正）；离画面底留出
   平台 UI 遮挡区；`x` 居中用 0。
3. **可读时长**：中文 ≥ 0.25s/字；一行 ≤ 16 字，超长拆两条。
4. **样式基线**：`size` 8 上下、`color` #FFFFFF + `border_color` #000000 +
   适度 `border_width` 描边可读性最稳；强调句换 `color`，整轨保持同一家族。
5. **花字**：`text_effect: {"name": "<已采集花字名>"}`——花字自带填充与描边，
   **与 `color`/`border_color`/`border_width` 互斥**，引擎直接拒绝同用。
   可用名单读 fork 内 `engine/native-resource-catalog.json`。

## 计划片段示例

```json
{"type": "text", "name": "字幕", "segments": [
  {"start_us": 200000, "duration_us": 2200000, "text": "上半是白模预演",
   "size": 8, "x": 0, "y": -0.78, "color": "#FFFFFF",
   "border_color": "#000000", "border_width": 4},
  {"start_us": 2600000, "duration_us": 2400000, "text": "下半是 AI 成片",
   "size": 8, "x": 0, "y": -0.78, "text_effect": {"name": "orange-outline"}}
]}
```

## 与转写联动

长口播逐句字幕：ASR 词级时间戳按标点切句，每句一段；禁止手工估时间。
ASR 不可用时如实报告，不手估。

## Never do

- Never 让两条字幕在同时刻重叠（读者只能读一条）。
- Never 用字幕复述画面已有的文字（片名/花字）。
- Never 把未采集的花字名写进计划——读资源目录，不猜名字。
