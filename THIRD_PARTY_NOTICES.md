# Third-Party Notices

This plugin does not bundle third-party application binaries, proprietary SDKs, credentials, generated media, or vendor source code beyond the vendored engine declared below.

## Vendored engine (verbatim, content-pinned)

`scripts/jy_headless/` is vendored verbatim from
[partme-ai/jy-headless](https://github.com/partme-ai/jy-headless) v0.1.0
(Apache-2.0), pinned by per-file SHA-256 in `scripts/VENDOR.json`; the check
gate fails on any drift. The engine is built on
[pyJianYingDraft](https://github.com/GuvaI/pyJianYingDraft) 0.3.0 (Apache-2.0),
which is a runtime dependency installed separately.

## Methodology references (no code or text copied)

The edit-plan → native-draft workflow design was informed by:

- [mcncarl/jianying-headless](https://github.com/mcncarl/jianying-headless)
  (private preview, Personal Learning and Non-Commercial License) — used as a
  design reference only; no source code copied or distributed.
- [mcncarl/yichen-skills](https://github.com/mcncarl/yichen-skills)
  (yichen-jianying-edit, Personal Learning and Non-Commercial License) — same
  status.

## Interoperability references

JianYing, 剪映, CapCut, and com.lemon.lvpro identify interoperability targets
and remain trademarks of their respective owners. This plugin is not an
official 剪映 SDK; it generates drafts for the locally installed app and never
distributes the app, its resources, or account data.
