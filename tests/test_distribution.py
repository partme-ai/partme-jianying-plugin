"""Distribution contract tests for partme-jianying-plugin."""
import importlib.util
import json
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))

spec = importlib.util.spec_from_file_location("validate_distribution", ROOT / "scripts" / "validate_distribution.py")
vd = importlib.util.module_from_spec(spec)
spec.loader.exec_module(vd)


class DistributionTests(unittest.TestCase):
    def test_validator_accepts_distribution(self) -> None:
        self.assertEqual(vd.main(), 0)

    def test_manifest_identity(self) -> None:
        d = json.loads((ROOT / ".codex-plugin/plugin.json").read_text())
        self.assertEqual(d["name"], "jianying-edit")
        self.assertEqual(d["repository"], "https://github.com/partme-ai/partme-jianying-plugin")

    def test_vendor_manifest_intact(self) -> None:
        m = json.loads((ROOT / "scripts/VENDOR.json").read_text())
        self.assertEqual(m["repo"], "https://github.com/partme-ai/jy-headless.git")
        self.assertEqual(m["ref"], "v0.1.0")

    def test_kimi_session_start_preloads_router(self) -> None:
        d = json.loads((ROOT / "kimi.plugin.json").read_text())
        self.assertEqual(d.get("sessionStart"), {"skill": "jianying-use"})


if __name__ == "__main__":
    unittest.main()
