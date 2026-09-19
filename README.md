# JianYing Edit (partme-jianying-plugin)

把**剪辑计划**一键构建并发布为**剪映专业版原生草稿**——多轨视频/文字/音频、转场/关键帧/蒙版、口播精剪、原生 MP4 导出。生成的是可在剪映里继续编辑的原生项目；导出可走引擎原生渲染或在剪映内完成。

Status: **v0.6.0**（fork 直连架构：插件只带技能/命令/钩子，引擎直连用户自己的 [partme-ai/jianying-headless](https://github.com/partme-ai/jianying-headless) 检出，**零改动驱动**——不 vendor、不改 fork、不绕过哈希钉扎）

## Quick start

1. clone 引擎（用户自己的 fork）：`git clone https://github.com/partme-ai/jianying-headless.git`
   并 `export JIANYING_HEADLESS_ROOT=<检出绝对路径>`
2. 剪映专业版已安装且启动过至少一次（macOS `com.lemon.lvpro`，运行时钉扎 11.4.x）
3. 预检：`python3 "$JIANYING_HEADLESS_ROOT/skills/yichen-jianying-edit/scripts/headless_draft.py" doctor`
4. 写 `plan.json`（`jy14-headless-plan/v1`，字段手册见
   `skills/jianying-edit/references/plan-format.md`；口播精剪走
   `jianying-edit-plan/v1` → `edit_plan.py compile`）
5. 构建链：`headless_draft.py build → verify-build → publish（剪映关闭）→ verify`
6. 打开剪映开始页该草稿继续编辑；或 `headless_draft.py export` 直接出 render.mp4

## 内置引擎（v0.9.0 起：pyJianYingDraft 直驱 + jycut 快路径）

- **主力引擎：vendored [pyJianYingDraft](https://github.com/GuanYixuan/pyJianYingDraft)**（Apache-2.0，`scripts/vendor/` 逐文件 SHA-256 钉扎）——技能家族直驱其完整 API：多轨、453 转场目录、关键帧、6 形蒙版、样式文本/描边/背景、入场出场动画、1052 滤镜与画面特效元数据、SRT 一键导入。运行器：`scripts/jydraft_run.py`；环境自检：`scripts/jydraft_check.py`。
- **快路径：Rust 引擎 [jianying-cli](https://github.com/full-aigc-plugins/jianying-cli)**（vendored 快照 `scripts/vendor/jianying-cli/`，v1.0.0 全能力：453 转场/1052 滤镜/特效/动画/蒙版/混音/SRT/代理渲染，`jianying-cli-plan/v1`）——构建 `cargo build --release` 后 `jianying build plan.json --out d1`。
- **专业档（可选）：fork 检出**（`JIANYING_HEADLESS_ROOT`，NC 许可零改动驱动）——仅原生 MP4 导出、已有草稿编辑、ASR 记账。

```bash
python3 scripts/jydraft_run.py gen.py        # 计划脚本 → 原生草稿
cd cli && cargo build --release && target/release/jycut build plan.json --out d1
```

细节见 [cli/README.md](cli/README.md) 与 `skills/jianying-draft`。

## 三平台安装

| 平台 | 清单 | 说明 |
|---|---|---|
| Codex | `.codex-plugin/plugin.json` | partme-ai 市场条目（skills-only 面） |
| ZCode | `.zcode-plugin/plugin.json` | 含 `userConfig`（JY_DRAFT_ROOT 可在设置里指定）+ hooks |
| Kimi | `kimi.plugin.json` | sessionStart 预载路由技能 + 命令 + SessionStart 钩子 |

## 流水线位置

```text
公有领域经典名场面 → 镜头表 → Blender 白模 → AI 逐镜生成
→ detect_shots 实测切点 → 解说配音 → **jianying-edit 发布剪映原生草稿** → 原生导出/剪映内导出
```

## 文档

- [中文说明](README.zh-CN.md)
- 计划字段手册：`skills/jianying-edit/references/plan-format.md`（权威原文在 fork 检出内）
- 方法论参考：mcncarl/yichen-skills（yichen-jianying-edit）与 mcncarl/jianying-headless——仅设计参考，无代码或文本复制；fork 引擎继承上游个人学习与非商用许可边界（见 THIRD_PARTY_NOTICES.md）

## License

Apache-2.0（插件自身；引擎为外部依赖，许可见其仓库）。

## Skills（13，按 blender 家族粒度）

| Skill | 职责 |
|---|---|
| `jianying-use` | 路由 + Step 0 能力预检（doctor） |
| `jianying-edit` | 计划 → 原生草稿（build/publish 链 + edit 入口 + 原生导出） |
| `jianying-harness` | fork CLI 全命令面/契约/失败表 |
| `jianying-draft` | pyJianYingDraft 底层原理（进阶参考） |
| `jianying-inspect` | 素材/草稿根/已有草稿探测（ffprobe + doctor + edit inspect） |
| `jianying-narration` | 口播精剪流水线（ASR → keeps/protect → 编译 → 草稿） |
| `jianying-subtitles` | 字幕轨设计（对齐语音/安全区/花字互斥） |
| `jianying-audio` | 音频轨分层（解说/BGM/原声，volume 0-4） |
| `jianying-motion` | 关键帧动效（通道字典/scale/x/y/rotation/opacity） |
| `jianying-transitions` | 叠化转场 + edge_policy 放置纪律 |
| `jianying-export-prep` | 导出前检查 + 原生导出（会员特效隔离） |
| `jianying-recover` | 发布冲突/resume-publish/恢复边界 |
| `jianying-setup` | fork 检出/剪映钉扎/ASR 执行器安装与排障 |
