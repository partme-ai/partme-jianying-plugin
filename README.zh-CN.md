# 剪映剪辑（partme-jianying-plugin）

把**剪辑计划**一键构建并发布为**剪映专业版原生草稿**——多轨视频/文字/音频、转场/关键帧/蒙版、口播精剪、原生 MP4 导出。生成的是可在剪映里继续编辑的原生项目；导出可走引擎原生渲染或在剪映内完成。

状态：**v0.6.0**（fork 直连架构：插件只带技能/命令/钩子，引擎直连用户自己的 [partme-ai/jianying-headless](https://github.com/partme-ai/jianying-headless) 检出，**零改动驱动**——不 vendor、不改 fork、不绕过哈希钉扎）

## 快速开始

1. clone 引擎（用户自己的 fork）：`git clone https://github.com/partme-ai/jianying-headless.git`
   并 `export JIANYING_HEADLESS_ROOT=<检出绝对路径>`
2. 剪映专业版已安装且启动过至少一次（macOS `com.lemon.lvpro`，运行时钉扎 11.4.x）
3. 预检：`python3 "$JIANYING_HEADLESS_ROOT/skills/yichen-jianying-edit/scripts/headless_draft.py" doctor`
4. 写 `plan.json`（`jy14-headless-plan/v1`，字段手册见
   `skills/jianying-edit/references/plan-format.md`；口播精剪走
   `jianying-edit-plan/v1` → `edit_plan.py compile`）
5. 构建链：`headless_draft.py build → verify-build → publish（剪映关闭）→ verify`
6. 打开剪映开始页该草稿继续编辑；或 `headless_draft.py export` 直接出 render.mp4

详细英文说明见 [README.md](README.md)；技能家族 13 个（路由/核心/字幕/音频/动效/转场/导出/恢复/环境）见英文表。

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

## 方法论与许可

- 方法论参考：mcncarl/yichen-skills（yichen-jianying-edit）与 mcncarl/jianying-headless——仅设计参考，无代码或文本复制。
- fork 引擎继承上游**个人学习与非商用**许可边界（见 THIRD_PARTY_NOTICES.md）；会员特效视为授权边界。
- 插件自身 Apache-2.0；引擎为外部依赖，许可见其仓库。
