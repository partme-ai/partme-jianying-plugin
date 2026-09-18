---
name: jianying-motion
description: "Design keyframe motion for a jy14-headless-plan/v1: per-channel keyframe dicts (scale/x/y/rotation/opacity, linear, first point at 0), Ken Burns/push-in/drift recipes, and the speed=1 + source_start_us=0 constraint."
---

# JianYing Motion（关键帧动效）

关键帧是**按通道的字典**：`{"keyframes": {"<通道>": [{"at_us": 0, "value": v},
{"at_us": L, "value": v2}]}}`。通道与值域：

| 通道 | 适用轨 | 值域 |
|---|---|---|
| scale | video/text | 0.01-10 |
| x / y | video/text | ±5（归一化） |
| rotation | video/text | ±360 |
| opacity | video | 0-1 |
| volume | audio | 0-4 |

线性插值；每通道 ≥2 点、`at_us` 严格递增、**首点必须 at_us=0**；同段若有同
名字段（如 `scale`），其值必须等于首点值。**带关键帧的段 `speed` 必须为 1 且
`source_start_us` 必须为 0**（引擎直接拒绝裁剪/变速源的关键帧）。

## 经典配方（段长 L 微秒）

| 效果 | 配方 |
|---|---|
| 缓推（Ken Burns） | scale: (0 → 1.0)，(L → 1.12) |
| 缓拉 | scale: (0 → 1.12)，(L → 1.0) |
| 横移 | x: (0 → -0.05)，(L → 0.05)，配静态 scale 1.1 防露边 |
| 微旋转（手持感） | rotation: (0 → 0.8)，(L → -0.8) |
| 呼吸感 | scale: (0 → 1.0)，(L/2 → 1.04)，(L → 1.0) |
| 淡入 | opacity: (0 → 0)，(0.3s → 1) |

## 规则

1. 放大超过 1.15 会明显变软——缩放/位移前确认素材分辨率 ≥ 输出分辨率。
2. 横移幅度配缩放：`scale ≥ 1 + 2×|位移幅度|`，否则露边。
3. 每段 ≤ 2 条通道链；更多层次交给剪映后期。
4. 静态机位素材优先缓推——观众感知"动"而不察觉"剪"。
5. 需要变速/裁剪后再动效的段：先分两段（各自 source_start_us/speed），
   关键帧只放在满足约束的段上。

## 计划片段示例

```json
{"start_us": 0, "duration_us": 4000000, "source": "shot-02.mp4",
 "source_start_us": 0,
 "keyframes": {"scale": [{"at_us": 0, "value": 1.0},
                          {"at_us": 4000000, "value": 1.12}]}}
```

## Never do

- Never 对同一段叠 3 条以上通道链（落盘成功但观感不可控）。
- Never 在带转场的段尾 0.3s 内放关键帧（转场期画面被叠化吃掉）。
- Never 给 speed≠1 或有入点的段配关键帧（引擎会拒绝）。
