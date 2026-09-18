"""Vendored pyJianYingDraft engine smoke tests."""
import json
import shutil
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
VENDOR = ROOT / "scripts" / "vendor"
sys.path.insert(0, str(VENDOR))

import pyJianYingDraft as draft  # noqa: E402


class VendorEngineTests(unittest.TestCase):
    def test_imports_from_vendor_not_site_packages(self) -> None:
        self.assertIn("vendor", Path(draft.__file__).as_posix())

    def test_minimal_text_draft_roundtrip(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            store = Path(tmp) / "store"
            store.mkdir()
            script = draft.DraftFolder(str(store)).create_draft(
                "unit-demo", 1920, 1080, fps=30
            )
            t = script.append_track(draft.TrackSpec(draft.TrackType.text))
            seg = draft.TextSegment(
                "单元测试字幕",
                timerange=draft.Timerange(100_000, 1_000_000),
                style=draft.TextStyle(size=8, align=1),
                border=draft.TextBorder(color=(0.0, 0.0, 0.0)),
                clip_settings=draft.ClipSettings(transform_y=-0.78),
            )
            script.add_segment(seg, t)
            out = store / "unit-demo" / "draft_content.json"
            script.dump(str(out))
            self.assertTrue(out.is_file())
            self.assertTrue((store / "unit-demo" / "draft_meta_info.json").is_file())
            d = json.loads(out.read_text(encoding="utf-8"))
            self.assertEqual(d["duration"], 1_100_000)  # segment end = start + duration
            self.assertEqual([t["type"] for t in d["tracks"]], ["text"])
            content = json.loads(d["materials"]["texts"][0]["content"])
            self.assertEqual(content["styles"][0]["range"], [0, 6])  # UTF-16 units

    @unittest.skipUnless(shutil.which("ffmpeg"), "ffmpeg needed to synthesize media")
    def test_media_segment_with_transition(self) -> None:
        import subprocess

        with tempfile.TemporaryDirectory() as tmp:
            tmp = Path(tmp)
            media = tmp / "a.mp4"
            subprocess.run(
                ["ffmpeg", "-v", "error", "-f", "lavfi",
                 "-i", "color=c=blue:s=320x240:d=1", "-y", str(media)],
                check=True,
            )
            store = tmp / "store"
            store.mkdir()
            script = draft.DraftFolder(str(store)).create_draft("media-demo", 1280, 720)
            v = script.append_track(draft.TrackSpec(draft.TrackType.video))
            seg = draft.VideoSegment(str(media), target_timerange=draft.Timerange(0, 1_000_000))
            seg.add_transition(draft.TransitionType.叠化, duration=300_000)
            script.add_segment(seg, v)
            out = store / "media-demo" / "draft_content.json"
            script.dump(str(out))
            d = json.loads(out.read_text(encoding="utf-8"))
            self.assertEqual(d["materials"]["transitions"][0]["name"], "叠化")
            self.assertEqual(d["materials"]["transitions"][0]["effect_id"], "322577")


if __name__ == "__main__":
    unittest.main()
