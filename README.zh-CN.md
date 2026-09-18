# 剪映剪辑（partme-jianying-plugin）

把**剪辑计划 JSON**（`jianying-plan/v1`）生成**剪映专业版原生草稿**——多轨视频/文字/音频、非破坏式生成、交付前校验。生成的是可在剪映里继续编辑的原生项目，导出仍在剪映内完成。

状态：**v0.2.0**（skills-only 分发；确定性引擎 `scripts/jy_headless/` 逐字 vendor 自 [partme-ai/jy-headless](https://github.com/partme-ai/jy-headless) v0.1.0）

## 快速开始

1. 剪映专业版已安装且启动过至少一次。
2. 预检：`python3 scripts/jy_headless/cli.py detect`
3. 准备 `plan.json`（`jianying-plan/v1`：video/text/audio 轨道 + 素材路径 + 微秒时间）
4. 生成：`python3 scripts/jy_headless/cli.py generate --plan plan.json`
5. 打开剪映专业版 → 开始页 → 该草稿，继续编辑并导出。

## 三平台安装

| 平台 | 清单 | 说明 |
|---|---|---|
| Codex | `.codex-plugin/plugin.json` | partme-ai 市场条目 |
| ZCode | `.zcode-plugin/plugin.json` | 含 `userConfig`（JY_DRAFT_ROOT 可在设置里指定） |
| Kimi | `kimi.plugin.json` | sessionStart 预载路由技能 + 命令 |

## 流水线位置

```text
公有领域经典名场面 → 镜头表 → Blender 白模 → MiniMax/H3 逐镜生成
→ detect_shots 实测切点 → 解说配音 → **jianying-edit 生成剪映原生草稿** → 剪映内导出
```

## 文档

- 引擎与验证状态：`docs/`
- 方法论参考：mcncarl/yichen-skills（yichen-jianying-edit）与 mcncarl/jianying-headless 私有预览版——仅设计参考，无代码或文本复制（见 THIRD_PARTY_NOTICES.md）

## License

Apache-2.0（引擎基于 pyJianYingDraft 0.3.0，Apache-2.0）。
