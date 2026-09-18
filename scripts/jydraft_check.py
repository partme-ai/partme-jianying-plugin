#!/usr/bin/env python3
"""Advisory environment check for the vendored pyJianYingDraft engine.

Always exits 0; prints one JSON object with actionable status lines.
"""
from __future__ import annotations

import importlib
import json
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VENDOR = ROOT / "scripts" / "vendor" / "pyJianYingDraft"


def check_import() -> str:
    try:
        sys.path.insert(0, str(ROOT / "scripts" / "vendor"))
        mod = importlib.import_module("pyJianYingDraft")
        from_file = getattr(mod, "__file__", "")
        if "vendor" not in from_file:
            return f"shadowed by pip install at {from_file} — vendor entry lost"
        return f"ok v{getattr(mod, '__version__', '?')} (vendored)"
    except Exception as exc:  # noqa: BLE001 - advisory report
        return f"broken: {exc}"


def check_mediainfo() -> str:
    try:
        importlib.import_module("pymediainfo")
    except Exception:
        return "missing — pip install pymediainfo"
    try:
        from pymediainfo import MediaInfo

        if MediaInfo.can_parse():
            return "ok"
        return "MediaInfo library not parseable — macOS: brew install mediainfo; Windows: install MediaInfo and add to PATH"
    except Exception as exc:  # noqa: BLE001
        return f"broken: {exc}"


def main() -> int:
    home = Path.home()
    roots = [
        home / "Movies/JianyingPro/User Data/Projects/com.lveditor.draft",
        home / "Movies/CapCut/User Data/Projects/com.lveditor.draft",
        home / "AppData/Local/JianyingPro/User Data/Projects/com.lveditor.draft",
    ]
    report = {
        "engine": check_import(),
        "media_probe": check_mediainfo(),
        "ffprobe": shutil.which("ffprobe") or "missing (素材实测需要)",
        "ffmpeg": shutil.which("ffmpeg") or "missing",
        "draft_roots": [{"path": str(r), "exists": r.is_dir()} for r in roots],
        "rust_cli": str(ROOT / "cli" / "target" / "release" / "jycut"),
        "fork_pro_tier": {
            "env": bool(Path(p).is_dir()) if (p := __import__("os").environ.get("JIANYING_HEADLESS_ROOT")) else False,
            "hint": "仅原生导出/已有草稿编辑/ASR 记账需要；普通生成不需要",
        },
    }
    print(json.dumps(report, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    sys.exit(main())
