---
description: 把剪辑计划 JSON 生成剪映原生草稿（可在剪映中继续编辑）
argument-hint: "<plan.json 路径> [--root 草稿根] [--no-replace]"
---

1. 读取 plan.json，校验 jianying-plan/v1 契约与素材文件存在性（ffprobe 探时长）。
2. 运行 `python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jy_headless/cli.py" generate
   --plan <plan> [--root <root>] [--no-replace]`。
3. 运行 verify 子命令校验草稿 JSON，报告轨道/段落/时长。
4. 提示用户在剪映专业版开始页打开该草稿继续编辑与导出。已有同名草稿时尊重
   --no-replace 门禁，不覆盖用户可能编辑过的草稿。
