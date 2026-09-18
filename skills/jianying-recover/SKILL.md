---
name: jianying-recover
description: "Draft conflict and recovery discipline: same-name collisions, backup-before-regenerate, restoring an overwritten draft name, and reading draft_meta_info for provenance. Non-destructive by contract."
---

# JianYing Recover（草稿冲突与恢复）

## 同名冲突

生成前 detect 列出已有草稿；目标名已存在时：

1. **用户在剪映里编辑过它** → 换新名（加日期后缀 `-v2`），绝不覆盖。
2. **确认是同一流水线的废弃产物** → `generate --no-replace` 会报错——
   改名重生成，旧草稿留给用户在剪映里自行删除。

判断依据问一句用户，不要猜。

## 误删/覆盖的恢复边界

- 剪映自身有云备份/本地回收：让用户在剪映内操作，插件不代劳。
- 引擎侧保证：`allow_replace=False`（默认）下同名生成直接失败，
  `duplicate_as_template` 不会碰源草稿。
- 草稿目录里的 `draft_meta_info.json` 含 tm_draft_create/update 时间戳——
  排查"哪个版本被覆盖"时读它。

## 常见恢复场景

| 场景 | 处置 |
|---|---|
| 生成的草稿打不开 | 读 draft_content.json 校验 JSON；损坏则重新 generate（素材在就不会丢内容） |
| 用户在剪映里改过又想回到 AI 版 | 用同名 plan 重新 generate（allow_replace=false 会报错→加日期后缀新名） |
| 素材被移动导致标红 | 新路径回填 plan 的 material 字段重新生成，或在剪映里手动重链 |

## Never do

- Never 删除/移动草稿目录里的任何文件（包括 .recycle_bin）。
- Never 用文件系统快照覆盖用户在剪映里的编辑。
