---
name: jianying-narration
description: "Narration condensing with pyJianYingDraft: ASR word-level transcript -> keep/protect decisions -> trimmed VideoSegments (target_timerange + source_timerange per keep) -> subtitles via import_srt. Millisecond cuts, human fine-tunes in 剪映."
license: Apache-2.0
---

# JianYing Narration（口播精剪）

长口播 → 精剪版原生草稿。引擎 = vendored pyJianYingDraft；keep/drop 通过
**截取区间**表达，一条主轨放多个裁剪段，天然无黑场。

## 流程

1. **ASR**：优先可用通道（如 video-agent-kit 的 speech_transcribe）拿词级
   时间戳；fork 专业档的 `asr_once.py`（豆包执行器）是备选（按素材 hash
   记账，不明状态阻止重提交）。都没有时如实报告，不手估。
2. **选段决策**（你来做）：按语义标 keeps（起止秒）、protect（有效发音
   词边界）、subtitles——保留完整语义，删气口与跑题；不删难以确认的数字、
   产品名或核心判断。
3. **写生成脚本**：每个 keep = 主轨一个 `VideoSegment`，`target_timerange`
   与 `source_timerange` 等长且相继（成品无黑场）：

```python
cursor = 0
for (src_start, src_end) in keeps:            # 原片微秒
    dur = src_end - src_start
    seg = VideoSegment("talk.mp4",
                       target_timerange=Timerange(cursor, dur),
                       source_timerange=Timerange(src_start, dur))
    script.add_segment(seg, v)
    cursor += dur
```

4. **字幕**：保留语义切句生成 SRT → `script.import_srt(...)`（安全区与样式
   见 `jianying-subtitles`）。
5. **核验与交付**：duration = keeps 总长；用户在剪映内微调（毫秒级切点支持
   丝滑手工调整）并导出。

## 决策纪律

- protect 防止切进有效发音；边界一律落在词间隙，不自作主张延长语义单元。
- 一条字幕可跨多个 keep，文字仅包含保留语义。
- 音效/BGM 事件落在删除区间时，把它移到下一个 keep 的开头，不要硬塞。

## 成本提示

ASR 按调用量计费；已有同源逐词稿直接复用。选段决策是智力环节——逐字稿
读两遍再下刀，比返工便宜。
