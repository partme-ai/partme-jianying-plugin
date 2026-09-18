---
name: jianying-export-prep
description: "Pre-export checklist for a generated JianYing draft: draft JSON integrity, material availability, track/segment sanity, subtitle coverage, and app-version sensitivity. Run before handing the draft to the user for export."
---

# JianYing Export Prep（导出前检查清单）

生成的草稿交用户导出前，逐项核对（全部只读）：

## 1. 草稿完整性

```bash
python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jy_headless/cli.py" verify --draft-dir <dir>
```

- draft_content.json 可解析、轨道/段数与计划一致。
- draft_meta_info.json 存在。

## 2. 素材可用性

- 遍历 plan 的每个 `material` 路径：文件存在、可读、非空。
- 素材在**生成后被移动过**是头号翻车原因——剪映打开草稿时会标红丢失素材。

## 3. 内容核对

- 总时长 = 各段时长之和（微秒级误差可接受）。
- 字幕无重叠、无超出画面底部安全区。
- 音频：解说段无重叠；BGM 铺满全程。
- 首帧不是黑场/空镜头（开始页封面观感）。

## 4. 版本敏感

引擎按固定剪映版本验证（见 jy-headless 的 docs/VERIFICATION）。剪映大版本
升级后：老草稿一般兼容，但新生成草稿若打不开，先降级怀疑再报告。

## 5. 交付说明模板

```text
草稿：<名称>（<时长>秒，<轨道数>轨）
已在剪映开始页可见。建议：打开后先通览一遍，
确认字幕与语音对齐，再调整导出参数（分辨率/帧率/码率）。
```

## Never do

- Never 声称"已导出 MP4"——导出动作在剪映里由用户完成。
- Never 跳过素材存在性检查（素材丢失是用户打开后才发现的头号问题）。
