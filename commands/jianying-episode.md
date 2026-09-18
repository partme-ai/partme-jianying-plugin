---
description: 整集流水线剪映收口：逐镜成片 + 解说 + 字幕 → 原生草稿
argument-hint: "<episode 目录（含 generated/ 与 narration/）>"
---

把整集流水线的产物装配进剪映：

1. 收集 episode 目录的 generated/*.mp4（逐镜成片）、narration/*.mp3（解说）、
   字幕文本（shot 表或 decisions）。
2. ffprobe 实测每个素材时长（生成会漂移，不用计划值）。
3. 按 `jianying-edit` 工作流写 pyJianYingDraft 脚本：主视频轨按实测切点顺序
   无缝拼接、解说/BGM 放音频轨、文本轨对齐解说（或直接 import_srt）；
   草稿名 `episode-<名>-<日期>`。
4. 运行生成 → 读回核验（轨道/段数/时长）。
5. 提示用户在剪映里通览后定稿导出；导出参数按发布平台调整。
