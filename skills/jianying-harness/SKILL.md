---
name: jianying-harness
description: "OPTIONAL pro-tier reference for the user's jianying-headless fork checkout (NC license): doctor / edit_plan compile+render-audio / asr_once / headless_draft from-compiled+build+publish+verify / resume-publish, the jy14-headless-plan/v1 contract, and known failure modes. Use when wiring, debugging, or extending automation flows."
license: Apache-2.0
---

# JianYing Harness（fork 专业档 CLI 全参考——可选）

> **定位**：普通生成不需要本技能（走 vendored pyJianYingDraft，见
> `jianying-edit`）。本参考只在需要 fork 独有能力时使用：原生 MP4 导出、
> 已有草稿编辑（独立副本）、ASR 记账。引擎是用户自己的 fork 检出
> （Personal Learning and Non-Commercial，零改动驱动）。

引擎位置：`$JIANYING_HEADLESS_ROOT`。Skill 入口脚本在其
`skills/yichen-jianying-edit/scripts/`；核心代码在其 `engine/`。
所有命令以绝对路径调用；输出必须是本次工作区 `work/` 下的新目录。

## 命令面

```bash
# 环境与依赖体检（版本/签名/hash 全核）
python3 $JY/skills/yichen-jianying-edit/scripts/headless_draft.py doctor

# 口播计划编译（jianying-edit-plan/v1 -> compiled.json）
python3 $JY/skills/yichen-jianying-edit/scripts/edit_plan.py compile \
  --plan WORK/edit-plan.json --out WORK/compiled-v1
python3 $JY/skills/yichen-jianying-edit/scripts/edit_plan.py render-audio \
  --plan WORK/compiled-v1/compiled.json --out WORK/voice-v1.wav --work WORK/audio-render-v1

# ASR（执行器不随 Skill 分发；YICHEN_ASR_EXECUTOR 或用户目录 scripts/transcribe.py）
python3 $JY/skills/yichen-jianying-edit/scripts/asr_once.py run --source SOURCE --ledger WORK/asr-ledger
python3 $JY/skills/yichen-jianying-edit/scripts/asr_once.py adopt --source SOURCE --cache CACHE --expect-source-sha HASH --ledger WORK/asr-ledger

# 草稿构建与登记
python3 $JY/skills/yichen-jianying-edit/scripts/headless_draft.py from-compiled \
  --compiled WORK/compiled/compiled.json --name <草稿名> --out WORK/headless-plan.json
python3 $JY/skills/yichen-jianying-edit/scripts/headless_draft.py build \
  --plan WORK/headless-plan.json --out WORK/headless-build
python3 $JY/skills/yichen-jianying-edit/scripts/headless_draft.py verify-build \
  --build WORK/headless-build
python3 $JY/skills/yichen-jianying-edit/scripts/headless_draft.py publish \
  --build WORK/headless-build --audit WORK/headless-publish
python3 $JY/skills/yichen-jianying-edit/scripts/headless_draft.py verify \
  --build WORK/headless-build --report WORK/after-native-save.json
# 剪映关闭 + 计划已审时，build+publish 一次完成：
python3 $JY/skills/yichen-jianying-edit/scripts/headless_draft.py create --plan PLAN --work NEW_DIR
# 首页登记失败后的幂等续跑：
python3 $JY/skills/yichen-jianying-edit/scripts/headless_draft.py resume-publish \
  --build WORK/headless-build --audit WORK/headless-resume
```

## `jy14-headless-plan/v1` 契约要点

- 时间全部整数微秒；`tracks[]`: video 主轨（首段从 0 起、同轨有序不重叠、无黑场）
  → video 画中画/B-roll 轨（可有间隙）→ text 字幕/标题 → audio BGM/音效
  → filter/effect 轨（已采集资源）。
- video segment：`source`（绝对路径）、`start_us`、`duration_us`、
  `source_start_us`（默认 0）、`source_duration_us`（默认=目标时长）、
  `speed` 0.1-8、`volume` 0-4、`scale`/`x`/`y`/`rotation`/`opacity`。
  `source_duration_us / speed` 必须等于 `duration_us`。
- text segment：`size`、`x/y`（y 默认 -0.78）、`color`/`border_color`/`border_width`
  （#RRGGBB）、`text_effect`（orange-outline 等已采集花字；与 color/border 互斥）。
- keyframes（段内相对时间，线性，首点 0 严格递增）：
  video `x/y/scale/rotation/opacity/volume`，text `x/y/scale/rotation`，audio `volume`；
  当前限制：源起点必须 0、速度 1（剪过源或变速的映射未验收，会拒绝）。
- mask：`{"shape": "circle|rectangle|line|mirror|star|heart", "width", "height",
  "x", "y", "rotation", "feather", "invert", "round_corner"}`（归一化包围框）。
- transition_out（主视频相邻段）：`{"name": "dissolve", "duration_us": 400000,
  "edge_policy": "require-handles"|"repeat-edge"}`；末段不能加。
- 输入视频：H.264/HEVC、≤1 音频流；图片 PNG（含 alpha）/JPEG/GIF；
  拒绝旋转元数据与未知像素格式；不偷偷转码。

## 资源与许可边界

已采集资源（黑白滤镜/light-shake 抖动/orange-outline 花字/六种几何蒙版）
逐文件 hash 固定、复制进草稿 `Resources/headless-native/`；原生保存回指剪映
自身缓存（`native_cache_dependency: true`）。高清黑白与该花字带会员/不可商用
UI 标记——**缓存存在 ≠ 取得商用授权**。未知资源、参数、字节变化会被拒绝。

## 故障对照表

| 症状 | 处置 |
|---|---|
| doctor 报核心缺失/hash 不符 | 检出 fork 并设 `JIANYING_HEADLESS_ROOT`；不改固定版本和 hash |
| `from-compiled` 报保护词无解 | 调整低能量处边界，不删保护词 |
| publish 失败但目录已写入 | `resume-publish --build ... --audit ...`（幂等） |
| 剪映升级后拒绝 | 引擎按精确 runtime profile 钉死（11.4.x）；升级是用户决策 |
| keyframes 被拒 | 源起点必须 0、速度 1；线性插值；严格递增不超段长 |
