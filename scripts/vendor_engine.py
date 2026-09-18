#!/usr/bin/env python3
"""Vendor the jy-headless engine from partme-ai/jy-headless at a pinned ref.

Copies jy_headless/*.py into scripts/jy_headless/ and records a VENDOR.json
manifest (ref, resolved sha, per-file SHA-256). `check` verifies the digests.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REPO = "https://github.com/partme-ai/jy-headless.git"
REF = "v0.1.0"
MANIFEST = ROOT / "scripts" / "VENDOR.json"
ENGINE_FILES = ("__init__.py", "cli.py", "detect.py", "generate.py")


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def fetch(ref: str, workdir: Path) -> Path:
    checkout = workdir / "checkout"
    subprocess.run(["git", "init", "--quiet", str(checkout)], check=True)
    subprocess.run(["git", "-C", str(checkout), "remote", "add", "origin", REPO], check=True)
    subprocess.run(["git", "-C", str(checkout), "fetch", "--quiet", "--depth", "1",
                    "origin", ref], check=True)
    subprocess.run(["git", "-C", str(checkout), "checkout", "--quiet", "--detach",
                    "FETCH_HEAD"], check=True)
    return checkout


def write_manifest(ref: str, sha: str) -> None:
    files = {}
    engine_dir = ROOT / "scripts" / "jy_headless"
    for name in ENGINE_FILES:
        files[name] = sha256((engine_dir / name).read_bytes())
    MANIFEST.write_text(json.dumps(
        {"repo": REPO, "ref": ref, "resolved_sha": sha, "files": files},
        ensure_ascii=False, indent=2) + "\n", encoding="utf-8")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("command", choices=("update", "check"))
    ap.add_argument("--ref", default=REF)
    ap.add_argument("--offline", action="store_true")
    args = ap.parse_args()

    if args.command == "check":
        if not MANIFEST.is_file():
            print("FAIL: VENDOR.json missing")
            return 1
        manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
        engine_dir = ROOT / "scripts" / "jy_headless"
        for name, digest in manifest["files"].items():
            f = engine_dir / name
            if not f.is_file():
                print(f"FAIL: vendored engine file missing: {name}")
                return 1
            if sha256(f.read_bytes()) != digest:
                print(f"FAIL: vendored engine file drifted: {name}")
                return 1
        print("vendor engine check: all files match the manifest (offline)")
        return 0

    if args.offline:
        print("update 需要网络；--offline 只支持 check")
        return 1
    with tempfile.TemporaryDirectory(prefix="jy-engine-vendor-") as tmp:
        checkout = fetch(args.ref, Path(tmp))
        resolved = subprocess.run(["git", "-C", str(checkout), "rev-parse", "HEAD"],
                                  check=True, capture_output=True, text=True).stdout.strip()
        target = ROOT / "scripts" / "jy_headless"
        target.parent.mkdir(parents=True, exist_ok=True)
        if target.exists():
            import shutil
            shutil.rmtree(target)
        target.mkdir()
        for name in ENGINE_FILES:
            src = checkout / "jy_headless" / name
            (target / name).write_bytes(src.read_bytes())
        write_manifest(args.ref, resolved)
        print(f"vendored engine {args.ref} @ {resolved[:12]}: {len(ENGINE_FILES)} files")
    return 0


if __name__ == "__main__":
    sys.exit(main())
