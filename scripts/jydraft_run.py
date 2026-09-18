#!/usr/bin/env python3
"""Run a user generation script inside the vendored pyJianYingDraft environment.

Usage:
    python3 "$CLAUDE_PLUGIN_ROOT/scripts/jydraft_run.py" SCRIPT.py [args...]

The vendored Apache-2.0 pyJianYingDraft (scripts/vendor/pyJianYingDraft,
hash-pinned in its VENDOR.json) is prepended to sys.path; then SCRIPT runs
as __main__. Media probing needs pymediainfo + the MediaInfo library
(`pip install pymediainfo`; macOS `brew install mediainfo`).
"""
from __future__ import annotations

import runpy
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
# sys.path needs the PARENT of the package directory
VENDOR_PARENT = ROOT / "scripts" / "vendor"


def main() -> int:
    if len(sys.argv) < 2:
        print("usage: jydraft_run.py SCRIPT.py [args...]", file=sys.stderr)
        return 1
    script = Path(sys.argv[1]).resolve()
    if not script.is_file():
        print(f"script not found: {script}", file=sys.stderr)
        return 1
    if not (VENDOR_PARENT / "pyJianYingDraft" / "VENDOR.json").is_file():
        print("vendored pyJianYingDraft missing — re-provision scripts/vendor/",
              file=sys.stderr)
        return 1
    sys.path.insert(0, str(VENDOR_PARENT))
    sys.argv = [str(script)] + sys.argv[2:]
    runpy.run_path(str(script), run_name="__main__")
    return 0


if __name__ == "__main__":
    sys.exit(main())
