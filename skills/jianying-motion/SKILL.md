---
name: jianying-motion
description: "Motion design with pyJianYingDraft: add_keyframe (KeyframeProperty, linear) for Ken Burns/push-in/drift, and add_animation (Intro/Outro/Group) for fades — alpha keyframes are ignored at render time, animations are the fade mechanism."
license: Apache-2.0
---

# JianYing Motion（关键帧与动画）

## 关键帧（属性动画）

```python
seg.add_keyframe(KeyframeProperty.scale_x, 0, 1.0)
seg.add_keyframe(KeyframeProperty.scale_x, 4_000_000, 1.12)
```

属性目录：`position_x` / `position_y`（半幅坐标）、`scale_x` / `scale_y`、
`rotation`（顺时针度）、`alpha`、`saturation`、`contrast`、`brightness`；
音频专用 `AudioSegment.add_keyframe(time_offset, volume)`。
线性插值；`time_offset` 为段内相对微秒（0 到段长），每属性 ≥2 点。

**坑：alpha 关键帧在视频/文本段预览可见但渲染被 app 忽略**——淡入淡出
一律用入场/出场动画（下节）。

## 经典配方（段长 L 微秒）

| 效果 | 配方 |
|---|---|
| 缓推（Ken Burns） | scale_x+scale_y：(0 → 1.0)，(L → 1.12) |
| 缓拉 | scale_x+scale_y：(0 → 1.12)，(L → 1.0) |
| 横移 | position_x：(0 → -0.05)，(L → 0.05)，配静态 scale 1.1 防露边 |
| 微旋转（手持感） | rotation：(0 → 0.8)，(L → -0.8) |
| 呼吸感 | scale：(0 → 1.0)，(L/2 → 1.04)，(L → 1.0) |
| 淡入/淡出 | `add_animation(IntroType.渐显)` / `(OutroType.渐隐)` |

## 入场/出场/循环动画

```python
seg.add_animation(IntroType.渐显, duration=500_000)     # VideoSegment
txt.add_animation(TextIntro.渐显, duration=500_000)     # 文本段
```

目录：视频入场 155 / 出场 124 / 组合 123；文本入场 145 / 出场 97 / 循环 93
（中文名枚举，带 resource_id 元数据）。每段 in/out/group 各最多一个。

## 规则

1. 放大超过 1.15 明显变软——缩放/位移前确认素材分辨率 ≥ 输出。
2. 横移幅度配缩放：`scale ≥ 1 + 2×|位移幅度|`，否则露边。
3. 每段 ≤ 2 条属性链；更多层次交给剪映后期。
4. 静态机位素材优先缓推——观众感知"动"而不察觉"剪"。
5. 带转场的段，关键帧避开段尾转场吃掉的画面窗。

## Never do

- Never 用 alpha 关键帧做淡入淡出（渲染被忽略——用入场/出场动画）。
- Never 对同一段叠 3 条以上属性链（落盘成功但观感不可控）。
