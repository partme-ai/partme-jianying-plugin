---
name: jianying-use
description: "Route JianYing (剪映专业版) editing requests: capability preflight first (native app, draft root, vendored engine), then edit-plan to native draft generation, or environment setup. Drafts open as fully editable native JianYing projects."
---

# JianYing Edit Router

Use this as the entry point. Before any draft generation, run the capability
preflight (step 0) — the workflow needs 剪映专业版 installed, a draft root that
exists, and the vendored engine; missing pieces must surface as actionable
guidance, never as a raw traceback.

## Step 0 — capability preflight (always first)

```bash
python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jy_headless/cli.py" detect
```

Read the output and act on it:

- `roots: []` → 剪映专业版未安装或未启动过。Guidance: install 剪映专业版
  (macOS /Applications/剪映专业版.app, Windows 官网安装包), launch it once and
  create/open any draft so the draft root is created, then re-run. Windows
  users can also set `JY_DRAFT_ROOT` to a custom draft root.
- Drafts listed → ready. The primary root is where generated drafts land.
- macOS requires 剪映专业版 (com.lemon.lvpro); Windows path follows the
  official installer layout.

## Routing

- **Edit-plan → native draft** (from a shot table / timeline JSON: video,
  text/subtitle, audio tracks) → `jianying-edit`.
- **Environment / install problems** → `jianying-setup`.
- The generated draft opens in 剪映 as a fully editable native project —
  fine-tuning, effects, and final export happen in the app.

## Never do

- Never modify or delete an existing user draft; generation only creates new
  drafts (allow_replace is opt-in).
- Never claim native MP4 export is automated: the deliverable is the draft;
  export happens in 剪映 by the user (automated export is a future engine
  capability).
- Never invent materials: every `material` path in the plan must exist and be
  a real media file the user provided or our pipeline generated.
