"""jy-headless CLI：detect / generate / verify。"""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
    from jy_headless import detect, generate
else:
    from . import detect, generate


def main() -> int:
    ap = argparse.ArgumentParser(prog="jy-headless")
    sub = ap.add_subparsers(dest="cmd", required=True)

    sub.add_parser("detect", help="探测剪映草稿根与已有草稿")

    g = sub.add_parser("generate", help="按 jianying-plan/v1 计划生成原生草稿")
    g.add_argument("--plan", required=True)
    g.add_argument("--root", help="剪映草稿根目录（缺省自动探测）")
    g.add_argument("--no-replace", action="store_true", help="同名草稿已存在时报错而非替换")

    v = sub.add_parser("verify", help="校验生成的草稿 JSON 可解析")
    v.add_argument("--draft-dir", required=True)

    args = ap.parse_args()

    if args.cmd == "detect":
        print(json.dumps(detect.detect(), ensure_ascii=False, indent=2))
        return 0
    if args.cmd == "generate":
        plan = detect.load_plan(args.plan)
        out = generate.generate(plan, root=args.root, allow_replace=not args.no_replace)
        print(json.dumps({"draft_dir": str(out)}, ensure_ascii=False))
        return 0
    if args.cmd == "verify":
        d = json.loads((Path(args.draft_dir) / "draft_content.json").read_text(encoding="utf-8"))
        print(json.dumps({"ok": True, "tracks": len(d.get("tracks", [])),
                          "duration_us": d.get("duration")}, ensure_ascii=False))
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
