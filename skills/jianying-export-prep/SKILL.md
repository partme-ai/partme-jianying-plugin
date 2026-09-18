---
name: jianying-export-prep
description: "Pre-export checklist plus native export for a verified jianying-headless build: verify-build, member-effect isolation, headless_draft.py export to render.mp4, and the in-app export fallback."
---

# JianYing Export Prep（导出前检查 + 原生导出）

## 1. 构建完整性（全部只读）

```bash
HD="$JIANYING_HEADLESS_ROOT/skills/yichen-jianying-edit/scripts/headless_draft.py"
python3 "$HD" verify-build --build WORK/build --report WORK/vb.json
```

- 结构核验通过、轨道/段数与计划一致；发布后跑 `verify --build WORK/build`
  做活体回读。

## 2. 素材可用性

- 遍历计划每个 `source`：文件存在、可读、非空。
- 素材在**生成后被移动**是头号翻车原因——剪映打开会标红丢失素材。

## 3. 内容核对

- 总时长 = 主轨段时长之和（主轨连续，无黑场）。
- 字幕无重叠、未越安全区；音频解说无重叠；转场只在章节边界。

## 4. 原生导出（可选，无需打开剪映）

```bash
python3 "$HD" export --build WORK/build --out WORK/export \
  [--bitrate 4000000] [--timeout 600]
```

产出 `WORK/export/render.mp4`。纪律：

- 只吃**已 verify 的 build 快照**，不是活体编辑器状态——先 verify 再导出。
- **会员特效隔离**：计划含会员标记资源时导出被拦，如实报告——改用已采集的
  普通资源，或让用户在剪映内导出（会员账号下导出是用户自己的授权行为）。
- 导出超时先降 `--bitrate` 或分段，不要盲目加大 `--timeout`。

## 5. 交付说明模板

```text
草稿：<名称>（<时长>秒，<轨道数>轨）
已发布到剪映开始页。render.mp4 位于 WORK/export/（码率 <b>）。
建议在剪映里通览一遍：字幕对齐、音量层次、章节转场。
```

## Never do

- Never 跳过 verify-build 直接导出（export 会拒绝未验证快照）。
- Never 声称已绕过会员隔离导出会员效果；隔离是授权边界不是故障。
