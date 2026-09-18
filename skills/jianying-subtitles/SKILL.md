---
name: jianying-subtitles
description: "Subtitle/title design with pyJianYingDraft: TextSegment + TextStyle/TextBorder/TextBackground, per-sentence placement timed to measured speech, SRT batch import via import_srt, safe-area and no-overlap rules."
license: Apache-2.0
---

# JianYing Subtitles（字幕轨设计）

字幕轨 = `TrackSpec(TrackType.text)` + `TextSegment`。

## 放置规则

1. **逐句对齐语音**：每条字幕起止 = 该句语音实测起止（ASR 词级时间戳或
   ffprobe），不是镜头切点。一句跨切点时字幕保持连续。
2. **安全区**：底部字幕 `ClipSettings(transform_y=-0.8)`（半幅坐标，向上为
   正）；留出平台 UI 遮挡区；水平居中 `transform_x=0`。
3. **可读时长**：中文 ≥ 0.25s/字；`auto_wrapping=True` + `max_line_width=0.82`
   自动换行，一行 ≤ 16 字。
4. **样式基线**（可读性最稳）：`TextStyle(size=8, align=1, color=(1,1,1))` +
   `TextBorder(color=(0,0,0), width=40)` 描边；强调句换 color，整轨同一家族。
   背景条用 `TextBackground(color="#RRGGBB", style=1)`。

## 逐句写入

```python
t = script.append_track(TrackSpec(TrackType.text))
txt = TextSegment("上半是白模预演", timerange=Timerange(200_000, 2_200_000),
                  style=TextStyle(size=8, align=1),
                  border=TextBorder(color=(0.0, 0.0, 0.0)),
                  clip_settings=ClipSettings(transform_y=-0.78))
script.add_segment(txt, t)
```

整段 SRT 一次导入（含样式与安全区）：

```python
script.import_srt("subs.srt", "字幕",
                  text_style=TextStyle(size=5, align=1, auto_wrapping=True),
                  clip_settings=ClipSettings(transform_y=-0.8))
```

## 与转写联动

长口播逐句字幕：ASR 词级时间戳按标点切句 → 生成 SRT → `import_srt`；
禁止手工估时间。ASR 不可用时如实报告。

## Never do

- Never 让两条字幕同时刻重叠（读者只能读一条）。
- Never 用字幕复述画面已有的文字（片名/花字）。
- Never 手估字幕时间——全部来自实测转写。
