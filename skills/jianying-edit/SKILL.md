---
name: jianying-edit
description: "Turn a jy14-headless-plan/v1 into a native, fully editable JianYing Pro draft via the jianying-headless engine: build -> verify-build -> publish -> verify. Also the edit-plan (口播 keeps/protect) compile path and copy-based editing of existing drafts. Non-destructive; draft name uniqueness enforced."
---

# JianYing Edit（计划 → 原生草稿）

引擎在 fork 检出内（见 `jianying-harness` 的命令面与环境）。你设计计划；
引擎编译时间映射并无界面构建原生草稿。所有命令以绝对路径调用：

```bash
HD="$JIANYING_HEADLESS_ROOT/skills/yichen-jianying-edit/scripts/headless_draft.py"
```

## 两条入口

- **直接写计划**（多轨视频/字幕/音频/滤镜/特效）→ 写 `jy14-headless-plan/v1`
  （字段手册见 [references/plan-format.md](references/plan-format.md)，
  亦可读 fork 内 `references/plan-format.md` 原文）→ `build` 链。
- **口播精剪**（keeps/protect/subtitles）→ 写 `jianying-edit-plan/v1` →
  `edit_plan.py compile` → `from-compiled` → `build` 链（见 `jianying-narration`）。

## 工作流（直接写计划）

1. **探测**：素材 ffprobe 实测时长（`jianying-inspect`）；`$HD doctor`
   确认草稿根与运行时。计划里的 `duration_us` 用实测值，不用设计值。
2. **写计划**：主视频轨必须是 `tracks[0]` 且首段从 0 起连续无黑场（间隙放
   画中画轨）；fps ∈ {24,25,30,50,60}；草稿名为可见单层目录名。
3. **build**：`$HD build --plan WORK/plan.json --out WORK/build`（纯产物，
   不写剪映）。
4. **verify-build**：`$HD verify-build --build WORK/build [--report WORK/verify-build.json]`。
5. **publish**（剪映完全关闭后）：`$HD publish --build WORK/build --audit WORK/audit`。
   中断后用 `resume-publish`（幂等）续做。
6. **verify**：`$HD verify --build WORK/build`（已发布草稿的回读核验）。
   一步到位可用 `$HD create --plan WORK/plan.json --work WORK/run`（= build+publish）。
7. **交付**：用户在剪映开始页打开草稿检查画面/字幕/音量/切口。

## 改已有草稿（独立副本）

`$HD edit inspect --draft <草稿目录> --out WORK/inspect.json` 先看结构；
再按 fork 的 `edit-existing-macos.md` 用 `edit build/publish` 链在**副本**上改，
原草稿永不被触碰。复合片段（嵌套时间线）走 `edit` 入口，不走主 `build`。

## 原生导出

`$HD export --build WORK/build --out WORK/export` 产出 `render.mp4`
（`--bitrate` 默认 4M、`--timeout` 默认 600s）。仅吃**已验证 build 快照**；
会员特效被隔离校验拦下时如实报告，改用普通资源或让用户在剪映内导出。

## Never do

- Never 引用不存在的素材，或用计划时长代替 ffprobe 实测值。
- Never 覆盖用户可能编辑过的同名草稿；publish 冲突时换名或走 resume-publish。
- Never 绕过引擎的哈希钉扎与资源目录校验（改 fork、伪造 pin、下载未采集资源）。
- Never 把会员标记当授权；商用许可边界见 `jianying-setup`。
