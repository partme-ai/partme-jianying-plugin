---
name: jianying-audio
description: "Audio track design with pyJianYingDraft: AudioSegment (pure-audio materials only), narration/BGM/原声 layering with volume baselines, volume keyframes, scene-audio effects, measured-duration placement."
license: Apache-2.0
---

# JianYing Audio（音频轨设计）

音频轨 = `TrackSpec(TrackType.audio)` + `AudioSegment`。多轨并存：解说轨 +
BGM 轨 + 原声（video 段自带）。

**注意**：`AudioSegment` 的素材必须是纯音频文件（含视频轨会 ValueError）——
需要视频里的声音时直接调 video 段的 volume，不要另铺一层。

## 分层与音量基线

| 轨 | 音量基线 | 说明 |
|---|---|---|
| 解说/口播 | 1.0 | 主导轨，任何时刻不允许被 BGM 盖过 |
| BGM | 0.15-0.25 | 循环铺底；句间起伏用音量关键帧 |
| 原声（video 段） | 跟随段 volume | 采访/同期声段 1.0，其余 0.3 左右 |

BGM 时长不足：多段拼接同轨递增放置；变速用 `speed` 但会变调，微调 ≤1.05。

## 放置规则

1. **解说对齐语义**：每段解说起于它描述的画面起点前 0.2-0.4s；同轨段递增
   不重叠（引擎不混音，重叠=两段人声叠加）。
2. **BGM 从 0 铺到片尾**，独立轨；淡入淡出用 ffmpeg 预处理（afade）后作
   素材进来，或铺 `AudioSegment.add_keyframe(time_offset, volume)` 音量
   关键帧。
3. **实测对齐**：放置前 ffprobe 每段素材真实时长；`target_timerange` 用实测值。
4. 场景音/音色：`aud.add_effect(AudioSceneEffectType.<名>)`（85 个目录成员）。

## 与口播精剪联动

`jianying-narration` 的 keep 段自带原声（video 段 volume 承载）——音频轨只
放解说与 BGM，不要重复铺原声。

## Never do

- Never 让解说段互相重叠（引擎不混音）。
- Never 在解说期间把 BGM 提到 0.4 以上。
- Never 拿含视频轨的文件构造 AudioSegment。
