---
name: jianying-edit
description: "Core workflow: turn a requirement or shot plan into a native JianYing draft by writing a pyJianYingDraft script and running it in the vendored engine (jydraft_run.py). Multi-track video/text/audio, transitions, keyframes, styled text; deterministic, non-destructive, verified before delivery."
license: Apache-2.0
---

# JianYing Edit（需求 → 原生草稿，核心工作流）

引擎 = 插件内置的 vendored pyJianYingDraft（Apache-2.0）。你写生成脚本；
库负责落盘合法草稿。运行器把 vendor 包前置到 `sys.path`：

```bash
python3 "${CLAUDE_PLUGIN_ROOT}/scripts/jydraft_run.py" gen.py
```

## 工作流

1. **预检**：`jydraft_check.py`（见 `jianying-setup`）。缺 MediaInfo 探测时
   如实报告，不要跳过实测。
2. **实测素材**：ffprobe 每个素材的真实时长/分辨率（`jianying-inspect`）。
   `duration_us` 用实测值；生成模型产物会漂移。
3. **设计**：轨道分层（主视频轨承载连续叙事，黑场/间隙放画中画轨）、字幕
   对齐实测语音、音频分层音量基线（能力细节见 subtitles/audio/motion/
   transitions 各技能）。
4. **写脚本**：按 `jianying-draft` 的 API 速查写 gen.py。纪律：
   `DraftFolder` 的根目录必须先存在；`create_draft` 同名默认报错
   （`allow_replace` 需用户明确同意）；`dump()` 用绝对路径。
5. **运行**：`jydraft_run.py gen.py`。报错就是契约错误——修脚本，不要绕。
6. **核验**：读回 `draft_content.json`（tracks/segments 数、transition/
   keyframe 条目、duration = 各段末端最大值）。
7. **交付**：草稿名 + 轨道摘要。用户在剪映开始页打开检查画面/字幕/音量/
   切口后自行导出。

## 最小骨架（可直接抄）

```python
from pyJianYingDraft import (DraftFolder, TrackSpec, TrackType, VideoSegment,
                             TextSegment, Timerange, TextStyle)
import os
os.makedirs("<工作目录>/store", exist_ok=True)
script = DraftFolder("<工作目录>/store").create_draft("demo", 1920, 1080, fps=30)
v = script.append_track(TrackSpec(TrackType.video))
seg = VideoSegment("shot.mp4", target_timerange=Timerange(0, 3_000_000))
script.add_segment(seg, v)
script.dump("<工作目录>/store/demo/draft_content.json")
```

## 迭代与恢复

- 被否决的剪辑 = 改脚本重跑；同名用新草稿名（加 `-v2` 日期后缀），旧稿留对比。
- 用户在剪映里编辑过的草稿**永不覆盖**（`allow_replace` 门禁）。
- 素材被移动：新路径回填脚本重跑，或在剪映里手动重链。

## Never do

- Never 引用不存在的素材，或用设计时长冒充实测时长。
- Never 覆盖用户可能编辑过的同名草稿。
- Never 声称已自动化导出；Never 写死 pip 安装的 pyJianYingDraft（必须经
  jydraft_run 引导 vendor 版，保证钉扎与可复现）。
