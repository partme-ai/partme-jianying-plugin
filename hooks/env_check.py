#!/usr/bin/env python3
"""SessionStart hook: report JianYing editing readiness. Advisory — always exit 0."""
from __future__ import annotations

import json
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    lines: list[str] = []

    engine = ROOT / "scripts" / "jy_headless" / "cli.py"
    vendor = ROOT / "scripts" / "VENDOR.json"
    lines.append("引擎: 就绪" if engine.is_file() and vendor.is_file()
                 else "引擎: 缺失——运行 scripts/vendor_engine.py update")
    if vendor.is_file():
        try:
            m = json.loads(vendor.read_text(encoding="utf-8"))
            lines.append(f"引擎版本: {m.get('ref')} @ {m.get('resolved_sha', '')[:12]}")
        except Exception:
            pass

    home = Path.home()
    roots = [
        home / "Movies" / "JianyingPro" / "User Data" / "Projects" / "com.lveditor.draft",
        home / "AppData" / "Local" / "JianyingPro" / "User Data" / "Projects" / "com.lveditor.draft",
    ]
    found = next((r for r in roots if r.is_dir()), None)
    lines.append(f"剪映草稿根: {found}" if found
                 else "剪映草稿根: 未找到——需安装并启动一次剪映专业版（com.lemon.lvpro）")

    ff = shutil.which("ffmpeg")
    lines.append(f"ffmpeg: {ff or '未找到（素材探测/合成需要）'}")

    try:
        sys.stdin.read()
    except Exception:
        pass

    print("剪映编辑插件环境：" + "；".join(lines))
    return 0


if __name__ == "__main__":
    try:
        json.load(sys.stdin)
    except Exception:
        pass
    sys.exit(main())
