---
name: jianying-transitions
description: "Transition discipline with pyJianYingDraft: the 453-name Chinese TransitionType catalog (VIP-flagged metadata), add_transition on the outgoing segment, duration bounds, and rhythm pairing."
license: Apache-2.0
---

# JianYing Transitions（转场）

```python
seg.add_transition(TransitionType.叠化, duration=500_000)
```

转场作用于**段尾**——放在前一段上，向它的后继段过渡；末段不能加（库会
拒绝）。目录是 **453 个中文名枚举**（pyJianYingDraft 内置元数据，带
resource_id/effect_id/is_vip），常用：叠化、闪黑、闪白、模糊、推移/擦除/
翻页类、百叶窗、水墨、故障、快速缩放等。**`is_vip=True` 的成员（如叠化扭曲、
心形叠化）需要会员授权——默认只用非 VIP 成员**，用户明确要求并自担授权时
才可用。

## 时长与节奏纪律

1. `duration` ≤ 1_000_000（1s）——转场吃两段的画面时间，超 1s 拖节奏；
   默认 0.5s。
2. 口播/解说类：只在**章节边界**用（开场→正题、正题→总结），镜头间硬切；
   一分钟 ≤ 2 个转场。
3. 卡点视频：转场对齐音乐节拍，时长 = 节拍间隔。
4. 相邻段有一段是静态机位时慎用运动类转场——画面不动 + 运动转场 = 观感割裂。
5. 首选枚举：拿不准时用 `TransitionType.叠化`（非 VIP、通用、观感最稳）。

## Never do

- Never 每个切点都加转场（默认硬切；转场是标点不是逗号）。
- Never 用转场掩盖内容断裂——先修 keep-list/分镜再谈转场。
- Never 擅自使用 VIP 目录成员（授权边界）。
