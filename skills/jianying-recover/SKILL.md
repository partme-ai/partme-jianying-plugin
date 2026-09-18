---
name: jianying-recover
description: "Publish conflict and recovery discipline for jianying-headless: publish requires 剪映 closed, resume-publish idempotent, same-name collisions, draft name rules, and what never to touch. Non-destructive by contract."
---

# JianYing Recover（发布冲突与恢复）

## 发布前置与中断

- `publish --build B --audit A` 要求**剪映完全关闭**（进程级，不是窗口关闭）。
- 发布中断/失败：先关剪映，再 `resume-publish --build B --audit A`（幂等，
  从审计记录续做）——不要重跑 publish 从头开始。
- 一步到位的 `create --plan P --work W` 失败时同样按上述续做。

## 同名冲突

草稿名在草稿根内唯一。发布前 `doctor` 看草稿根；目标名已存在时：

1. **用户在剪映里编辑过它** → 换新名（加日期后缀 `-v2`），绝不覆盖。
2. **确认是同一流水线的废弃产物** → 仍建议换名重发布，旧草稿由用户在剪映里
   自行删除；引擎不提供删除。

判断依据问一句用户，不要猜。

## 常见恢复场景

| 场景 | 处置 |
|---|---|
| publish 报剪映未关闭 | 退出剪映（含后台进程）后 resume-publish |
| build 目录损坏 | 保留现场，从计划重新 `build`（素材在就不会丢内容） |
| verify 活体回读不一致 | 读 `--report` 定位差异段；核对计划值与实测值后重建 |
| 素材被移动导致标红 | 新路径回填计划的 `source` 重新 build/publish，或剪映内手动重链 |
| 用户改过草稿想回 AI 版 | 换名重新发布新草稿，旧版留对比 |

## 边界

- Never 删除/移动草稿目录里的任何文件（含剪映自己的备份/回收结构）。
- Never 用文件系统快照覆盖用户在剪映里的编辑。
- 草稿冲突属剪映数据，插件的边界是"发布自己的新草稿"，不做草稿内修复。
