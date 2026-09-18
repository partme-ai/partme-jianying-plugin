---
name: jianying-narration
description: "口播精剪流水线: long-form talking-head footage -> ASR transcript -> unit-level keep/drop decisions -> jianying-plan/v1 -> native JianYing draft. The fixed-format account pipeline close-out, adapted from the validated koubo-condense flow."
---

# JianYing Narration（口播精剪）

长口播 → 精剪版剪映草稿。全链已在 koubo-condense-demo 验证（227s→112s，49%）。

## 前置

- 口播源视频（用户提供；本机无则先 TTS 合成测试素材）
- video-agent-kit 官方 ASR/TTS 通道（MCP 面内免 key）

## 流程

1. **转写**：`speech_transcribe(input_path=<口播>, output_json=out/transcript.json)`。
   空转录合法（无语音素材不适用本技能）。
2. **建索引**：`condense_index(video_path, transcript_path, silence_db=-45)`。
   合成/低噪音频的 auto 阈值会误判（threshold_too_high）——手动压到 -45 或更低，
   直到 verdict=plausible。产出 47 类语义单元表。
3. **选段决策**（你来做）：读单元表，按「钩子→框架→三段展开→总结→CTA」保留
   干货链，弃问候/跑题/碎片/重复。**连续 run 优先**（相邻单元不产生跳切），
   目标时长决定取舍。写出 keep 列表。
4. **condense_plan**：`condense_plan(index_path, keep=[...], tighten_pauses=true,
   drop_fillers="hard")` → 13 clip 级计划 + 边界吸附报告。
5. **QC + 裁决**：`condense_render` 出 ffmpeg 预览 → `condense_qc`（0 错基线；
   连续性警告逐条裁决写入 out/condense_verify.md）。
6. **转剪映草稿**：读 condense_plan.json 的 clips（start/end 秒）转
   `jianying-plan/v1`（video 轨顺序拼接）→ `jy_headless generate` → 原生草稿。
7. **交付**：草稿名 + 前后时长对比；用户在剪映里微调后导出。

## Never do

- Never 凭计划时长放置字幕/音轨——用 ffprobe 实测。
- Never 把「轮询超时/部分失败」当完成；逐单元核对 QC 警告。
- Never 伪造转录内容（ASR 不可用时如实报告并停止）。
