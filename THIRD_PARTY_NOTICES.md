# Third-Party Notices

This plugin does not bundle third-party application binaries, proprietary SDKs,
credentials, generated media, or NC-licensed third-party source code. It ships
guidance (skills, commands, hooks) plus its own Apache-2.0 Rust CLI (`cli/`).

## Built-in engines

**Vendored pyJianYingDraft (`scripts/vendor/pyJianYingDraft`, primary engine).**
Vendored verbatim from
[GuanYixuan/pyJianYingDraft](https://github.com/GuanYixuan/pyJianYingDraft)
(Apache-2.0; the LICENSE text ships inside the vendored directory), pinned by
per-file SHA-256 in its `VENDOR.json` — the distribution validator fails on any
drift. It is used via `scripts/jydraft_run.py`, which prepends the vendored
package to `sys.path` so the pinned copy always wins over any pip install.

**jycut (`cli/`, fast path).** Original code (Apache-2.0) synthesizing
interoperability facts from the same project's assets plus:

- [renezander030/capcut-cli](https://github.com/renezander030/capcut-cli) (MIT):
  format/write-discipline facts (mirror files, store registration, editor
  guard, render_index conventions). No code copied.
- `cli/assets/` skeleton assets and transition metadata come from
  GuanYixuan/pyJianYingDraft; see `cli/assets/ASSETS-PROVENANCE.md`.

**Optional pro tier.** For native MP4 export, existing-draft editing and ASR
ledger the skills can drive the user's own checkout of
[partme-ai/jianying-headless](https://github.com/partme-ai/jianying-headless)
via `JIANYING_HEADLESS_ROOT` — a fork of
[mcncarl/jianying-headless](https://github.com/mcncarl/jianying-headless)
(Personal Learning and Non-Commercial License). The checkout is driven as-is:
never modified, never re-vendored, never redistributed; contract facts only
were referenced.

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

