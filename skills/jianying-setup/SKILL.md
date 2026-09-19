---
name: jianying-setup
description: "Diagnose and fix the JianYing automation environment: vendored pyJianYingDraft engine health, pymediainfo + MediaInfo library, ffmpeg/ffprobe, draft root, and the optional fork pro-tier checkout. Advisory per-OS guidance."
license: Apache-2.0
---

# JianYing Setup（环境诊断与准备）

## 检查清单（按序）

```bash
python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jydraft_check.py"
```

1. **引擎**：vendored pyJianYingDraft 在插件 `scripts/vendor/`（Apache-2.0，
   `VENDOR.json` 逐文件 SHA-256 钉扎）。`engine: ok (vendored)` 即就绪——
   无需 pip 安装。显示 `shadowed by pip` 说明引导丢失，报告不修。
2. **媒体探测**：`pip install pymediainfo` + macOS `brew install mediainfo`
   （Windows 装 MediaInfo 并加 PATH）。没有它 VideoMaterial/AudioMaterial
   无法探测素材时长。
3. **ffmpeg/ffprobe**：素材实测与预处理（`brew install ffmpeg`）。
4. **草稿根**：启动一次剪映专业版并新建任意草稿（macOS
   `~/Movies/JianyingPro/User Data/Projects/com.lveditor.draft`）。
5. **fork 专业档（可选）**：仅原生导出/已有草稿编辑/ASR 记账需要——
   `git clone https://github.com/partme-ai/jianying-headless` 并
   `export JIANYING_HEADLESS_ROOT=<绝对路径>`。普通生成完全不依赖它。

## 许可与边界

- vendored 引擎与 jianying-cli（repo `full-aigc-plugins/jianying-cli`）均为 Apache-2.0，可商用分发；出处见
  `THIRD_PARTY_NOTICES.md` 与 `cli/assets/ASSETS-PROVENANCE.md`。
- fork 引擎（专业档）是 **Personal Learning and Non-Commercial**——继承上游
  边界，本插件不代为声明商用许可；会员资源是授权边界不是障碍。
- 生成的是原生草稿；最终导出由用户在剪映内完成（详见 `jianying-export-prep`）。

## Never do

- Never 修改 vendor 内容或 hash 钉扎来"通过"检查。
- Never 从非官方镜像安装剪映/MediaInfo。
- Never 自动充值或提交付费资源请求。
