# Third-Party Notices

This plugin does not bundle third-party application binaries, proprietary SDKs,
credentials, generated media, or third-party source code. It ships guidance
(skills, commands, hooks) only; the editing engine is an external checkout.

## Engine dependency (external, unmodified)

Draft generation is driven through the user's own checkout of
[partme-ai/jianying-headless](https://github.com/partme-ai/jianying-headless),
located via `JIANYING_HEADLESS_ROOT`. This plugin drives that checkout as-is:
it never modifies, re-vendors, or redistributes engine code, and it never
bypasses the checkout's hash pins. The engine is built on
[pyJianYingDraft](https://github.com/GuvaI/pyJianYingDraft) 0.3.0 (Apache-2.0)
as a runtime dependency of the checkout itself.

Users should be aware that `partme-ai/jianying-headless` is a fork of
[mcncarl/jianying-headless](https://github.com/mcncarl/jianying-headless),
which is distributed under a Personal Learning and Non-Commercial License —
usage of the engine inherits that boundary.

## Methodology references (no code or text copied)

- [mcncarl/jianying-headless](https://github.com/mcncarl/jianying-headless)
  (Personal Learning and Non-Commercial License) — design reference only.
- [mcncarl/yichen-skills](https://github.com/mcncarl/yichen-skills)
  (yichen-jianying-edit, Personal Learning and Non-Commercial License) —
  design reference only; no source code or reference text copied.

## Interoperability references

JianYing, 剪映, CapCut, and com.lemon.lvpro identify interoperability targets
and remain trademarks of their respective owners. This plugin is not an
official 剪映 SDK; it generates drafts for the locally installed app and never
distributes the app, its resources, or account data. Membership-gated effects
are treated as an authorization boundary, not an obstacle.
