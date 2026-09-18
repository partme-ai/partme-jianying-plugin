#!/usr/bin/env python3
"""Distribution validator for partme-jianying-plugin."""
from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PLUGIN_ID = "jianying-edit"
REPOSITORY = "https://github.com/partme-ai/partme-jianying-plugin"
LEGAL = ("LICENSE", "NOTICE", "PRIVACY.md", "README.md", "README.zh-CN.md",
         "TERMS.md", "THIRD_PARTY_NOTICES.md")


def fail(msg: str) -> None:
    print(f"FAIL: {msg}")
    sys.exit(1)


def load_json(rel: str) -> dict:
    try:
        return json.loads((ROOT / rel).read_text(encoding="utf-8"))
    except Exception as exc:
        fail(f"{rel}: invalid JSON ({exc})")
        raise


def main() -> int:
    codex = load_json(".codex-plugin/plugin.json")
    zcode = load_json(".zcode-plugin/plugin.json")
    kimi = load_json("kimi.plugin.json")
    market = load_json(".agents/plugins/marketplace.json")

    if codex.get("name") != PLUGIN_ID or not re.fullmatch(r"[a-z0-9]+(-[a-z0-9]+)*", PLUGIN_ID):
        fail(f"codex manifest name must be '{PLUGIN_ID}'")
    version = codex.get("version", "")
    if not re.match(r"\d+\.\d+\.\d+", version):
        fail(f"version not semver: {version}")
    if codex.get("repository") != REPOSITORY:
        fail("codex manifest repository mismatch")
    for label, m in (("zcode", zcode), ("kimi", kimi)):
        if m.get("name") != PLUGIN_ID or m.get("version") != version:
            fail(f"{label} manifest name/version must match")
    entry = market["plugins"][0]
    if entry.get("name") != PLUGIN_ID or entry.get("version") != version:
        fail("marketplace entry name/version must match")

    for legal in LEGAL:
        if not (ROOT / legal).is_file():
            fail(f"missing legal file: {legal}")

    for skill_dir in sorted(p for p in (ROOT / "skills").iterdir() if p.is_dir()):
        md = skill_dir / "SKILL.md"
        if not md.is_file():
            fail(f"skills/{skill_dir.name}: directory without SKILL.md")
        text = md.read_text(encoding="utf-8")
        m = re.match(r"^---\n(.+?)\n---\n", text, re.DOTALL)
        if not m:
            fail(f"skills/{skill_dir.name}: missing frontmatter")
        block = m.group(1)
        if re.findall(r"^name: (\S+)$", block, re.MULTILINE) != [skill_dir.name]:
            fail(f"skills/{skill_dir.name}: frontmatter name must equal directory name")
        if len(re.findall(r"^description: ", block, re.MULTILINE)) != 1:
            fail(f"skills/{skill_dir.name}: exactly one description key required")

    # 引擎 vendor 完整性
    vendor_manifest = load_json("scripts/VENDOR.json")
    engine_dir = ROOT / "scripts" / "jy_headless"
    import hashlib
    for name, digest in vendor_manifest["files"].items():
        f = engine_dir / name
        if not f.is_file():
            fail(f"vendored engine file missing: {name}")
        if hashlib.sha256(f.read_bytes()).hexdigest() != digest:
            fail(f"vendored engine file drifted: {name}")

    if "SessionStart" not in load_json("hooks/hooks.json").get("hooks", {}):
        fail("hooks.json missing SessionStart")

    print(f"validated {PLUGIN_ID} {version}: "
          f"{len(list((ROOT / 'skills').iterdir()))} skills, engine vendored, hooks wired")
    return 0


if __name__ == "__main__":
    sys.exit(main())
