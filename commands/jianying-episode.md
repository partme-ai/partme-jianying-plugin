---
description: 固定模式整集流水线的剪映收口：生成/解说产物 → 剪映原生草稿
argument-hint: "<episode 目录（含 generated/ 与 narration/）>"
---

把 classic-reels/episode 流水线的产物装配进剪映：

1. 从 episode 目录收集 generated/*.mp4（逐镜成片）、narration/*.mp3（解说）、
   字幕文本（shot 表或 decisions）。
2. 用 ffprobe 实测每个素材时长（生成会漂移，不用计划值），生成 jianying-plan/v1：
   video 轨按 detect_shots 实测切点顺序拼接，text 轨对齐解说，audio 轨放解说配音。
3. jianying-edit 生成原生草稿（命名 episode-<名>-<日期>）。
4. 提示用户在剪映里检查后导出 MP4；不声称已自动化导出。
