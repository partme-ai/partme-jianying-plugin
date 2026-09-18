# Third-Party Notices

This plugin does not bundle third-party application binaries, proprietary SDKs,
credentials, generated media, or NC-licensed third-party source code. It ships
guidance (skills, commands, hooks) plus its own Apache-2.0 Rust CLI (`cli/`).

## Built-in CLI engine (`cli/`) — jycut

`jycut` is original code (Apache-2.0) synthesizing interoperability facts and
permissively-licensed assets from three projects:

- [GuanYixuan/pyJianYingDraft](https://github.com/GuanYixuan/pyJianYingDraft)
  (Apache-2.0): the two minimal skeleton assets under `cli/assets/` and the
  curated transition resource metadata in `cli/assets/transitions.json` are
  taken from this project; see `cli/assets/ASSETS-PROVENANCE.md`.
- [renezander030/capcut-cli](https://github.com/renezander030/capcut-cli) (MIT):
  format/write-discipline facts (mirror files, store registration, editor
  guard, render_index conventions). No code copied.
- [partme-ai/jianying-headless](https://github.com/partme-ai/jianying-headless)
  (Personal Learning and Non-Commercial): plan-contract and workflow facts
  only; NO code or assets taken.

## Engine dependency (external, optional, unmodified)

For advanced capabilities (native MP4 export, existing-draft editing, ASR
ledger) the skills can drive the user's own checkout of
[partme-ai/jianying-headless](https://github.com/partme-ai/jianying-headless)
via `JIANYING_HEADLESS_ROOT`. This plugin drives that checkout as-is: it never
modifies, re-vendors, or redistributes engine code, and it never bypasses the
checkout's hash pins. That checkout is a fork of
[mcncarl/jianying-headless](https://github.com/mcncarl/jianying-headless),
distributed under a Personal Learning and Non-Commercial License — usage of
the engine inherits that boundary.

## Methodology references (no code or text copied)

- [mcncarl/yichen-skills](https://github.com/mcncarl/yichen-skills)
  (yichen-jianying-edit, Personal Learning and Non-Commercial License) —
  design reference only; no source code or reference text copied.

## Interoperability references

JianYing, 剪映, CapCut, and com.lemon.lvpro identify interoperability targets
and remain trademarks of their respective owners. This plugin is not an
official 剪映 SDK; it generates drafts for the locally installed app and never
distributes the app, its resources, or account data. Membership-gated effects
are treated as an authorization boundary, not an obstacle.

