---
name: jianying-motion
description: "Design keyframe motion on video segments: uniform_scale/position_x/position_y/rotation keyframes for Ken Burns zooms, push-ins, and drift effects. Linear interpolation; plan values against clip duration."
---

# JianYing Motion（关键帧动效）

`VideoSegment.add_keyframe(KeyframeProperty.<prop>, time_offset_us, value)`。
属性目录：`uniform_scale`（等比缩放）、`position_x` / `position_y`（位移，
归一化坐标）、`rotation`（旋转）、`scale_x` / `scale_y`（拉伸）。
线性插值，`time_offset_us` 为段内相对时间（从 0 到段长）。

## 经典动效配方

| 效果 | 配方（段长 L 微秒） |
|---|---|
| 缓推（Ken Burns） | uniform_scale: (0 → 1.0)，(L → 1.12) |
| 缓拉 | uniform_scale: (0 → 1.12)，(L → 1.0) |
| 横移 | position_x: (0 → -0.05)，(L → 0.05)（配 uniform_scale 1.1 防露边） |
| 微旋转（复古手持感） | rotation: (0 → 0.8)，(L → -0.8) |
| 呼吸感 | uniform_scale: (0 → 1.0)，(L/2 → 1.04)，(L → 1.0) |

## 规则

1. 缩放/位移前先确认素材分辨率 ≥ 输出分辨率：放大超过 1.15 会明显软。
2. position 用归一化坐标（0-1，相对画面）；露边=缩放不足，位移幅度配
   `uniform_scale ≥ 1 + 2×|位移幅度|`。
3. 每段 ≤ 2 条关键帧链（一条属性一条链）；再多交给剪映后期。
4. 静态机位素材优先用缓推——观众感知"动"而不察觉"剪"。

## 计划片段示例

```json
{"material": "shot-02.mp4", "start_us": 0, "duration_us": 4000000,
 "keyframes": [
   {"property": "uniform_scale", "time_offset_us": 0, "value": 1.0},
   {"property": "uniform_scale", "time_offset_us": 4000000, "value": 1.12}
 ]}
```

## Never do

- Never 对同一段叠 3 条以上属性链（引擎会落盘，观感不可控）。
- Never 在带转场的段首/段尾 0.3s 内放关键帧（转场期画面被叠化吃掉）。
