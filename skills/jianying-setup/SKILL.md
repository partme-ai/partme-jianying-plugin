---
name: jianying-setup
description: "Diagnose and fix the JianYing editing environment: app installation, draft root discovery, engine vendor state, ffmpeg presence, and per-OS guidance (macOS/Windows). Advisory only."
---

# JianYing Setup

## Checks (in order)

1. **剪映专业版 installation** — macOS: `/Applications/剪映专业版.app`
   (bundle id `com.lemon.lvpro`); Windows: official installer layout under
   `%LOCALAPPDATA%\JianyingPro`. Not installed → download from the official
   剪映 website and launch once.
2. **Draft root** — `jy_headless detect` prints roots and existing drafts.
   Missing root → the app has never created a draft; launch 剪映 and create any
   draft once. Custom root → set `JY_DRAFT_ROOT`.
3. **Engine** — `scripts/jy_headless/` must be vendored
   (`scripts/vendor_engine.py check`); if drifted, `vendor_engine.py update`
   re-pins from partme-ai/jy-headless v0.1.0.
4. **ffmpeg/ffprobe** — needed for material duration probing and media prep;
   not bundled by this plugin.

## Version sensitivity

The engine targets current 剪映专业版 draft formats and is validated against a
pinned app version (see `docs/VERIFICATION.md` in the engine repo). A major
剪映 app update may change draft internals — if generated drafts fail to open
after an app upgrade, report the app version and stop regenerating.

## Never do

- Never edit or delete existing user drafts to "fix" the environment.
- Never install 剪映 from unofficial mirrors.
- Never work around a draft-format mismatch by hand-editing generated JSON.
