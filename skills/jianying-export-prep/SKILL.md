---
name: jianying-export-prep
description: "Pre-export checklist for a generated JianYing draft: JSON integrity, material availability, subtitle/audio sanity, cold-reopen advice, and honest export boundaries (in-app by the user; pyJianYingDraft's UIA export is Windows-only on legacy JianYing)."
license: Apache-2.0
---

# JianYing Export Prep（导出前检查清单）

## 1. 草稿完整性（只读）

- `draft_content.json` + `draft_meta_info.json` 可解析；轨道/段数与设计一致。
- `jianying verify <草稿目录>`：引用完整、主轨连续、时长一致。

## 2. 素材可用性

- 遍历脚本里每个素材路径：存在、可读、非空。
- 素材在**生成后被移动**是头号翻车原因——剪映打开会标红丢失素材。
  pyJianYingDraft 不复制素材文件（引用绝对路径），移动素材 = 断链。

## 3. 内容核对

- 总时长 = 主轨段时长之和（主轨连续无黑场）。
- 字幕无重叠、未越安全区；解说无重叠；BGM 铺满；转场只在章节边界。
- 首帧不是黑场/空镜头（开始页封面观感）。

## 4. 导出边界（如实）

- **默认出口：用户在剪映内导出**（分辨率/帧率/码率自选）。
- pyJianYingDraft 自带的 UIA 自动导出**仅 Windows + 剪映 ≤6.8**（新版剪映
  隐藏了控件树）；macOS 无自动化导出。
- 需要无人值守出 MP4 的专业路径 = fork 检出的原生导出（`jianying-harness`，
  NC 许可边界 + 会员特效隔离照旧）。

## 5. 交付说明模板

```text
草稿：<名称>（<时长>秒，<轨道数>轨），已写入 <草稿根>。
建议：打开后先通览一遍，确认字幕与语音对齐、切口与转场符合预期，
再在剪映内设置导出参数（分辨率/帧率/码率）。
```

## Never do

- Never 声称"已导出 MP4"（默认导出动作在剪映里由用户完成）。
- Never 跳过素材存在性检查（素材丢失是用户打开后才发现的头号问题）。
