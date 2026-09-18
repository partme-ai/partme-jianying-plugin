---
name: jianying-transitions
description: "Choose and place transitions between video segments: the TransitionType catalog (Chinese-named), duration discipline, and pairing with content rhythm. Applied via the engine's per-clip transition_out field."
---

# JianYing Transitions（转场）

引擎在 video 段上支持 `transition_out`：本段出点向下一段过渡的效果。
**转场作用于段的尾部**——放在前一段的 clip 上，不是后一段。

## 目录（常用，全部中文名）

| 类别 | TransitionType 名 |
|---|---|
| 溶解类 | 叠化、闪黑、闪白 |
| 运动类 | 上移、下移、左移、右移、推近、拉远 |
| 翻页类 | 上下翻页、左右翻页、中心旋转 |
| 光效类 | 云朵、星闪、光线 |

完整目录以引擎 `TransitionType` 枚举为准（`list(draft.TransitionType)`）。

## 时长与放置纪律

1. `duration_us` ≤ 1_000_000（1 秒）——转场吃的是两段的画面时间，超过 1s
   会明显拖节奏。
2. 口播/解说类视频：只在**章节边界**用（如 开场→正题、正题→总结），
   镜头间保持硬切；一分钟 ≤ 2 个转场。
3. 卡点视频：转场对齐音乐节拍，转场时长 = 节拍间隔。
4. 相邻段有一段是静态机位时慎用运动类转场——画面不动 + 运动转场 = 观感割裂。

## 计划片段示例

```json
{"material": "shot-01.mp4", "start_us": 0, "duration_us": 4000000,
 "transition_out": {"type": "叠化", "duration_us": 500000}}
```

## Never do

- Never 每个切点都加转场（默认硬切；转场是标点不是逗号）。
- Never 用转场掩盖内容断裂——先修 keep-list 再谈转场。
