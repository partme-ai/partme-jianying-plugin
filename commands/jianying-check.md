---
description: 剪映编辑环境预检：fork 检出、剪映 11.4.x、草稿根、资源目录
---

解析 `$JIANYING_HEADLESS_ROOT`（env → `~/workspaces/workspace-partme-ai/jianying-headless`
→ 询问用户），然后运行：

```bash
python3 "$JIANYING_HEADLESS_ROOT/skills/yichen-jianying-edit/scripts/headless_draft.py" doctor
```

汇报：fork 检出与哈希钉扎状态、剪映版本是否符合 11.4.x 钉扎、草稿根路径、
已采集资源目录健康度。缺什么按 `jianying-setup` 的指引给出对应命令
（clone fork、装剪映、启动一次剪映建草稿）。不做任何草稿写入，不改 fork。
