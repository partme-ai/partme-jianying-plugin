"""JianYing draft-root detection and plan-to-draft generation.

macOS:   ~/Movies/JianyingPro/User Data/Projects/com.lveditor.draft
Windows: %USERPROFILE%\\AppData\\Local\\JianyingPro\\User Data\\Projects\\com.lveditor.draft
"""
from __future__ import annotations

import json
import os
from pathlib import Path


def draft_roots() -> list[Path]:
    """All plausible JianYing draft roots on this machine, existing ones only."""
    home = Path.home()
    candidates = [
        home / "Movies" / "JianyingPro" / "User Data" / "Projects" / "com.lveditor.draft",
        home / "AppData" / "Local" / "JianyingPro" / "User Data" / "Projects" / "com.lveditor.draft",
    ]
    extra = os.environ.get("JY_DRAFT_ROOT")
    if extra:
        candidates.insert(0, Path(extra))
    return [p for p in candidates if p.is_dir()]


def detect() -> dict:
    roots = draft_roots()
    drafts: list[str] = []
    if roots:
        try:
            drafts = sorted(p.name for p in roots[0].iterdir() if p.is_dir())[:20]
        except OSError:
            pass
    return {"roots": [str(r) for r in roots],
            "primary": str(roots[0]) if roots else None,
            "existing_drafts": drafts,
            "note": "剪映专业版需已安装且至少启动过一次以生成草稿根目录" if not roots else ""}


def load_plan(path: str | Path) -> dict:
    plan = json.loads(Path(path).read_text(encoding="utf-8"))
    if plan.get("schema") != "jianying-plan/v1":
        raise ValueError(f"plan schema 必须是 jianying-plan/v1，得到 {plan.get('schema')!r}")
    if not plan.get("draft", {}).get("name"):
        raise ValueError("plan.draft.name 必填")
    for track in plan.get("tracks", []):
        if track.get("type") not in ("video", "text", "audio"):
            raise ValueError(f"未知轨道类型: {track.get('type')}")
    return plan
