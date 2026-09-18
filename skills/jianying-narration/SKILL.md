---
name: jianying-narration
description: "口播长视频精剪流水线: ASR (asr_once) -> unit-level keep/protect decisions -> edit-plan compile -> from-compiled -> build -> publish -> verify -> 原生剪映草稿. Millisecond-precision cuts from word-level timestamps; human fine-tunes in 剪映."
---

# JianYing Narration（口播精剪）

长口播 → 精剪版原生草稿。与 `jianying-edit` 的关系：本技能是**上游**——
它产出 edit-plan（keeps/protect/subtitles），交给 jianying-edit 的编译构建链。

## 流程

1. **ASR**：`asr_once.py run --source <口播> --ledger WORK/asr-ledger`
   （豆包执行器经 `YICHEN_ASR_EXECUTOR`；一次请求按素材内容 hash 记账，
   完成结果复用，不明状态阻止新提交——不用 force 绕过）。
2. **逐字稿分析**（你来做）：按语义标 keeps（含 protect 有效发音区间）、
   subtitles（一条可跨多段）、decisions（DELETE 区间与理由）。
3. **compile**：`edit_plan.py compile`（口播计划 → 编译结果，
   整帧区间 + 字幕余量 + 相邻去重）。
4. **render-audio**（可选）：编译结果的音频渲染。
5. **from-compiled → build → verify-build → publish → verify**
   （见 `jianying-harness` 的命令面）。
6. **交付**：草稿名 + 前后时长对比；用户在剪映内微调（毫秒级切点支持
   丝滑手工调整）并导出。

## 决策纪律

- 保留完整语义，删气口与跑题；不删难以确认的数字、产品名或核心判断。
- `protect` 防止切进有效发音；编译器不自动延长保护区间——无解时调边界。
- 一条字幕可跨多个保留片段，文字仅包含保留语义。
- 音效事件落删除区间默认报错；确认应随下一段开始时 `"snap": "next"`。

## 成本提示

- ASR 按调用量计费（豆包/火山）；已有同源逐词稿时直接使用，不重复转写。
- AI/LLM 的选段决策是本流程的智力环节——逐字稿读两遍再下刀，比返工便宜。
