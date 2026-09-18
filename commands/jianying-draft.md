---
description: 按需求写 pyJianYingDraft 脚本并生成剪映原生草稿
argument-hint: "<需求描述或分镜/素材清单> [草稿名]"
---

1. 预检：`python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jydraft_check.py"`。
2. ffprobe 实测每个素材时长/分辨率（生成会漂移，不用设计值）。
3. 按 `jianying-draft` 的 API 手册写生成脚本（gen.py）：主视频轨连续叙事、
   字幕对齐语音、音频分层音量、转场/关键帧按能力技能纪律。
4. 运行：`python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jydraft_run.py" gen.py`。
5. 读回 draft_content.json 核验轨道/段数/时长；同名冲突换名不覆盖。
6. 报告草稿名与结构摘要，提示用户在剪映开始页打开检查后导出。
