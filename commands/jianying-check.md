---
description: 剪映编辑环境预检：vendored 引擎、MediaInfo、ffprobe、草稿根
---

运行 `python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jydraft_check.py"` 并按 JSON 逐项汇报：
vendored pyJianYingDraft 引擎健康度、pymediainfo/MediaInfo 探测、ffprobe/ffmpeg、
草稿根路径。缺什么按 `jianying-setup` 的指引给出安装命令（pip/brew）。
不做任何草稿写入，不改 vendor 内容。
