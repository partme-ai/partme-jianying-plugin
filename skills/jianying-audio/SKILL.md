---
name: jianying-audio
description: "Design audio tracks for a JianYing plan: narration/voiceover placement, BGM layering with volume ducking, multi-track ordering, and measured-duration alignment."
---

# JianYing Audio（音频轨设计）

音频轨 = audio track + AudioSegment（`AudioMaterial(path)`, `Timerange`,
`volume`, `speed`）。多轨并存：解说轨 + BGM 轨 + 原声（视频段自带）。

## 分层与音量基线

| 轨 | 音量基线 | 说明 |
|---|---|---|
| 解说/口播 | 1.0 | 主导轨，任何时刻不允许被 BGM 盖过 |
| BGM | 0.15-0.25 | 循环铺底；解说句间可稍抬，句中压低 |
| 原声（视频自带） | 跟随 video clip volume | 采访/同期声段落抬到 1.0，其余 0.3 |

BGM 时长不足：`AudioSegment` 放循环素材或用多段拼接；`speed` 可微调但会变调。

## 放置规则

1. **解说对齐语义**：每段解说起于它描述的画面起点前 0.2-0.4s（ leading 一点点），
   不与上一段解说重叠（引擎不做自动混音交接，重叠=两段人声叠加）。
2. **BGM 从 0 铺到片尾**，独立轨；淡入淡出 v0.2 引擎未暴露——需要时用 ffmpeg
   预处理 BGM（afade）后作为素材进来。
3. **实测对齐**：放之前 ffprobe 每段素材的真实时长；`duration_us` 用实测值。

## 与口播精剪联动

`jianying-narration` 产出的 keep 段自带原声——音频轨只需放解说与 BGM，
不要重复铺一层原声。

## Never do

- Never 让解说段互相重叠（引擎不混音，重叠=两段人声叠加）。
- Never 在解说期间把 BGM 提到 0.4 以上。
- Never 引用不存在的音频文件；放置前 ffprobe 实测。
