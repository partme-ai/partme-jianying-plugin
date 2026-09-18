---
name: jianying-audio
description: "Design audio tracks for a jy14-headless-plan/v1: narration/BGM/原声 layering, volume 0-4 discipline, speed limits, and measured-duration placement. Audio segments share the common plan schema."
---

# JianYing Audio（音频轨设计）

音频轨 = `{type: "audio", name, segments[]}`；段 = `{start_us, duration_us,
source, source_start_us, source_duration_us, speed, volume, keyframes}`。
多轨并存：解说轨 + BGM 轨 + 原声（video 段自带）。

## 分层与音量基线（volume 合法域 0-4）

| 轨 | 音量基线 | 说明 |
|---|---|---|
| 解说/口播 | 1.0 | 主导轨，任何时刻不允许被 BGM 盖过 |
| BGM | 0.15-0.25 | 循环铺底；可对 volume 通道做关键帧做句间起伏 |
| 原声（video 段） | 跟随段 volume | 采访/同期声段 1.0，其余 0.3 左右 |

BGM 时长不足：多段拼接同轨递增放置；`speed` 0.1-8 可调但会变调，微调 ≤1.05。

## 放置规则

1. **解说对齐语义**：每段解说起于它描述的画面起点前 0.2-0.4s；同轨段递增
   不重叠（重叠=两段人声叠加，引擎不混音）。
2. **BGM 从 0 铺到片尾**，独立轨；淡入淡出引擎未暴露——需要时用 ffmpeg
   预处理（afade）后作为素材进来。
3. **实测对齐**：放置前 ffprobe 每段素材真实时长；`duration_us` 用实测值。
4. 带 `keyframes` 的音频段 `speed` 必须为 1 且 `source_start_us` 必须为 0。

## 与口播精剪联动

`jianying-narration` 产出的 keep 段自带原声——音频轨只放解说与 BGM，
不要重复铺一层原声。

## Never do

- Never 让解说段互相重叠（引擎不混音）。
- Never 在解说期间把 BGM 提到 0.4 以上。
- Never 引用不存在的音频文件；放置前 ffprobe 实测。
