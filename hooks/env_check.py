#!/usr/bin/env python3
"""SessionStart hook: report JianYing editing readiness. Advisory — always exit 0."""
from __future__ import annotations

import json
import os
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def main() -> int:
    lines: list[str] = []

    env = os.environ.get("JIANYING_HEADLESS_ROOT", "")
    default = Path.home() / "workspaces" / "workspace-partme-ai" / "jianying-headless"
    if env and Path(env).is_dir():
        lines.append(f"fork 检出: {env}")
    elif default.is_dir():
        lines.append(f"fork 检出: {default}（建议 export JIANYING_HEADLESS_ROOT 指向它）")
    else:
        lines.append("fork 检出: 未找到——git clone https://github.com/partme-ai/jianying-headless"
                     " 并 export JIANYING_HEADLESS_ROOT=<绝对路径>（插件不带引擎，直连用户 fork）")

    lines.append("预检命令: python3 \"$JIANYING_HEADLESS_ROOT/skills/yichen-jianying-edit/"
                 "scripts/headless_draft.py\" doctor")

    home = Path.home()
    roots = [
        home / "Movies" / "JianyingPro" / "User Data" / "Projects" / "com.lemon.lvpro",
        home / "Movies" / "JianyingPro" / "User Data" / "Projects" / "com.lveditor.draft",
        home / "AppData" / "Local" / "JianyingPro" / "User Data" / "Projects" / "com.lemon.lvpro",
    ]
    found = next((r for r in roots if r.is_dir()), None)
    lines.append(f"剪映草稿根: {found}" if found
                 else "剪映草稿根: 未找到——需安装并启动一次剪映专业版（11.4.x 钉扎，com.lemon.lvpro）")

    ff = shutil.which("ffmpeg")
    lines.append(f"ffmpeg: {ff or '未找到（素材探测需要）'}")

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
