---
description: 整集流水线剪映收口：逐镜成片 + 解说 + 字幕 → 剪映原生草稿
argument-hint: "<episode 目录（含 generated/ 与 narration/）>"
---

把整集流水线的产物装配进剪映：

1. 收集 episode 目录的 generated/*.mp4（逐镜成片）、narration/*.mp3（解说）、
   字幕文本（shot 表或 decisions）。
2. ffprobe 实测每个素材时长（生成会漂移，不用计划值），写
   `jy14-headless-plan/v1`：主视频轨按实测切点顺序无缝拼接（第一轨、首段 0 起）、
   解说与 BGM 放 audio 轨、text 轨对齐解说；字段细节按 `jianying-edit` 的
   references/plan-format.md。
3. 走 `jianying-edit` 链：build → verify-build → publish（剪映关闭）→ verify，
   草稿名 `episode-<名>-<日期>`。
4. 可选 `headless_draft.py export --build WORK/build --out WORK/export` 出
   render.mp4（会员特效被隔离时如实报告）。
5. 提示用户在剪映里通览后定稿；导出参数（分辨率/码率）按发布平台调整。
