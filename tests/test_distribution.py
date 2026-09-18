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

    def test_engine_tiers_wireup(self) -> None:
        for rel in ("scripts/VENDOR.json", "scripts/vendor_engine.py", "scripts/jy_headless"):
            self.assertFalse((ROOT / rel).exists(), f"parallel engine must stay removed: {rel}")
        for rel in ("scripts/vendor/pyJianYingDraft/VENDOR.json",
                    "scripts/jydraft_run.py", "scripts/jydraft_check.py"):
            self.assertTrue((ROOT / rel).is_file(), f"missing engine artifact: {rel}")
        router = (ROOT / "skills/jianying-use/SKILL.md").read_text(encoding="utf-8")
        self.assertIn("pyJianYingDraft", router)
        self.assertIn("jianying-headless", router)

    def test_zcode_userconfig_is_record(self) -> None:
        d = json.loads((ROOT / ".zcode-plugin/plugin.json").read_text())
        self.assertIsInstance(d.get("userConfig"), dict)

    def test_kimi_session_start_preloads_router(self) -> None:
        d = json.loads((ROOT / "kimi.plugin.json").read_text())
        self.assertEqual(d.get("sessionStart"), {"skill": "jianying-use"})


if __name__ == "__main__":
    unittest.main()
