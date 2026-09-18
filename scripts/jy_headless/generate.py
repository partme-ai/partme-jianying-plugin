"""Plan → 原生剪映草稿生成（基于 pyJianYingDraft 0.3.0，Apache-2.0 上游）。"""
from __future__ import annotations

from pathlib import Path

import pyJianYingDraft as jy
from pyJianYingDraft import Timerange, TrackSpec, TrackType

from .detect import draft_roots

_US = 1_000_000


def _root(root: str | None) -> Path:
    if root:
        return Path(root)
    roots = draft_roots()
    if not roots:
        raise SystemExit("未找到剪映草稿根目录——确认剪映专业版已安装并启动过，"
                         "或用 --root / JY_DRAFT_ROOT 指定")
    return roots[0]


def generate(plan: dict, root: str | None = None, allow_replace: bool = True) -> Path:
    """按 jianying-plan/v1 计划生成原生剪映草稿，返回草稿目录。"""
    dr = plan["draft"]
    folder = jy.DraftFolder(str(_root(root)))
    script = folder.create_draft(dr["name"], dr.get("width", 1280), dr.get("height", 720),
                                 fps=dr.get("fps", 24), allow_replace=allow_replace)

    for track in plan.get("tracks", []):
        ttype = {"video": TrackType.video, "text": TrackType.text,
                 "audio": TrackType.audio}[track["type"]]
        ref = script.append_track(TrackSpec(ttype, name=track.get("name")))
        for clip in track.get("clips", []):
            if track["type"] == "video":
                seg = jy.VideoSegment(
                    jy.VideoMaterial(clip["material"]),
                    target_timerange=Timerange(start=int(clip["start_us"]),
                                               duration=int(clip["duration_us"])),
                    volume=float(clip.get("volume", 1.0)),
                    speed=float(clip.get("speed", 1.0)) or None,
                )
            elif track["type"] == "audio":
                seg = jy.AudioSegment(
                    jy.AudioMaterial(clip["material"]),
                    target_timerange=Timerange(start=int(clip["start_us"]),
                                               duration=int(clip["duration_us"])),
                    volume=float(clip.get("volume", 1.0)),
                )
            else:
                style = jy.TextStyle(size=float(clip.get("size", 8.0)),
                                     color=tuple(clip.get("color", (1.0, 1.0, 1.0))))
                seg = jy.TextSegment(clip["text"],
                                     timerange=Timerange(start=int(clip["start_us"]),
                                                         duration=int(clip["duration_us"])),
                                     style=style)
            script.add_segment(seg, ref)

    out_dir = Path(_root(root)) / dr["name"]
    out_dir.mkdir(parents=True, exist_ok=True)
    script.dump(str(out_dir / "draft_content.json"))
    return out_dir
