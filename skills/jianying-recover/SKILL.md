---
name: jianying-recover
description: "Draft conflict and recovery discipline for pyJianYingDraft workflows: allow_replace as the explicit overwrite gate, duplicate_as_template for copy-based iteration, name-conflict etiquette, and what never gets touched."
license: Apache-2.0
---

# JianYing Recover（草稿冲突与恢复）

## 同名冲突

`create_draft` / `duplicate_as_template` 同名默认直接失败（`allow_replace=False`
是非破坏纪律的落点）。目标名已存在时：

1. **用户在剪映里编辑过它** → 换新名（加日期后缀 `-v2`），绝不覆盖。
2. **确认是同一流水线的废弃产物** → 也建议换名重生成，旧稿由用户在剪映里
   自行删除。
3. 判断依据问一句用户，不要猜；`allow_replace=True` 只在用户明确同意后使用。

## 迭代副本

改已有草稿 = `DraftFolder.duplicate_as_template(template_name, new_name)`：
独立副本迭代，源稿永不被触碰。

## 常见恢复场景

| 场景 | 处置 |
|---|---|
| 生成的草稿打不开 | 读 draft_content.json 校验 JSON；损坏则修脚本重生成（素材在就不会丢内容） |
| 用户改过草稿想回 AI 版 | 换名重新生成新草稿，旧版留对比 |
| 素材被移动导致标红 | 新路径回填脚本重生成，或在剪映里手动重链 |
| 误删生成脚本 | 草稿 JSON 即事实源——按结构读回推脚本，或直接改脚本重跑 |

## 边界

- Never 删除/移动草稿目录里的任何文件（含剪映自己的备份/回收结构）。
- Never 用文件系统快照覆盖用户在剪映里的编辑。
- 插件的边界是"生成自己的新草稿"，不做草稿内修复；已有草稿编辑走 fork
  专业档（`jianying-harness`）或用户手动。
