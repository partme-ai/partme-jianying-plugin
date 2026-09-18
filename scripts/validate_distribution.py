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
    base = version.split("+")[0]
    for label, m in (("zcode", zcode), ("kimi", kimi)):
        if m.get("name") != PLUGIN_ID or m.get("version") not in {version, base}:
            fail(f"{label} manifest name/version must match")
    entry = market["plugins"][0]
    if entry.get("name") != PLUGIN_ID or entry.get("version") not in {version, base}:
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

    # fork 直连完整性：禁止任何 vendored 引擎路径残留；路由必须指向 fork 入口脚本
    stale = []
    skip_dirs = {".git", "node_modules", ".venv", "__pycache__", "jy_headless", "target"}
    for path in ROOT.rglob("*"):
        if not path.is_file() or skip_dirs.intersection(path.parts):
            continue
        if path.name in {"validate_distribution.py", "test_distribution.py", "AGENTS.md"}:
            continue
        try:
            text = path.read_text(encoding="utf-8", errors="ignore")
        except OSError:
            continue
        for pattern in ("scripts/jy_headless", "VENDOR.json", "vendor_engine"):
            if pattern in text:
                stale.append(f"{path.relative_to(ROOT)}: {pattern}")
                break
    if stale:
        fail("stale vendored-engine references: " + "; ".join(sorted(set(stale))))

    router = (ROOT / "skills/jianying-use/SKILL.md").read_text(encoding="utf-8")
    if "skills/yichen-jianying-edit/scripts/headless_draft.py" not in router:
        fail("router must reference the fork entry script headless_draft.py")

    if "SessionStart" not in load_json("hooks/hooks.json").get("hooks", {}):
        fail("hooks.json missing SessionStart")

    print(f"validated {PLUGIN_ID} {version}: "
          f"{len(list((ROOT / 'skills').iterdir()))} skills, fork-direct, hooks wired")
    return 0


if __name__ == "__main__":
    sys.exit(main())
