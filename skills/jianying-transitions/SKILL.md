---
name: jianying-transitions
description: "Place transition_out on the main video track of a jy14-headless-plan/v1: dissolve-only catalog, edge_policy require-handles vs repeat-edge, duration discipline, and rhythm pairing."
---

# JianYing Transitions（转场）

video 段的 `transition_out` = `{name, duration_us, edge_policy}`。
当前引擎只支持一种：**`name: "dissolve"`（叠化）**。转场作用于段尾——放在
前一段上，向它的后继段过渡；**末段不能加**（引擎要求有后继段）。

仅限**主视频轨**（tracks[0]）；画中画轨上的过渡在剪映内手动加或换设计。
`edge_policy` 二选一：

- `require-handles`：要求两段素材在切点处各有富余手柄帧（推荐，观感最正）。
- `repeat-edge`：素材不够手柄时重复边缘帧补位（有冻结感，快切素材才用）。
- 居中转场要求时间轴帧数为偶数——fps 给定时注意 duration_us 的帧对齐。

## 时长与节奏纪律

1. `duration_us` ≤ 1_000_000（1s）——转场吃两段的画面时间，超 1s 拖节奏。
2. 口播/解说类：只在**章节边界**用（开场→正题、正题→总结），镜头间硬切；
   一分钟 ≤ 2 个转场。
3. 卡点视频：转场对齐音乐节拍，时长 = 节拍间隔。
4. 相邻段有一段是静态机位时慎用——画面不动 + 叠化 = 观感发闷。

## 计划片段示例

```json
{"start_us": 0, "duration_us": 4000000, "source": "shot-01.mp4",
 "source_start_us": 0,
 "transition_out": {"name": "dissolve", "duration_us": 500000,
                    "edge_policy": "require-handles"}}
```

## Never do

- Never 每个切点都加转场（默认硬切；转场是标点不是逗号）。
- Never 用转场掩盖内容断裂——先修 keep-list/分镜再谈转场。
- Never 在画中画轨或末段写 transition_out（引擎直接拒绝）。
