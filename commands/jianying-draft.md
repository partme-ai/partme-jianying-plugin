---
description: 把 jy14 计划 JSON 构建并发布为剪映原生草稿（可在剪映中继续编辑）
argument-hint: "<plan.json 路径> [--publish | --verify-only]"
---

1. 解析 `$JIANYING_HEADLESS_ROOT`（env → `~/workspaces/workspace-partme-ai/jianying-headless`
   → 询问用户），先跑 doctor 预检。
2. 读取 plan.json，校验 `jy14-headless-plan/v1` 契约：主视频轨第一且连续、
   fps ∈ {24,25,30,50,60}、素材 `source` 存在并用 ffprobe 实测时长回填。
3. `headless_draft.py build --plan <plan> --out WORK/build`，随后
   `verify-build --build WORK/build --report WORK/vb.json`。
4. `--verify-only` 到此为止；否则确认剪映已完全关闭，`publish --build WORK/build
   --audit WORK/audit`（中断用 resume-publish）。
5. 报告草稿名与轨道/段数；同名冲突按 `jianying-recover` 处理，不覆盖用户草稿。
