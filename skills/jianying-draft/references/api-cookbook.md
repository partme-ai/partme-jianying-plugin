# pyJianYingDraft 0.3.0 API Cookbook

实测通过的片段（macOS + 剪映专业版，pyJianYingDraft 0.3.0 / Python 3.12）。

## 多轨 + 转场 + 关键帧

```python
import pyJianYingDraft as draft

folder = draft.DraftFolder(root)                     # 根目录必须存在
script = folder.create_draft("name", 1280, 720, fps=24, allow_replace=True)

vref = script.append_track(draft.TrackSpec(draft.TrackType.video))
seg = draft.VideoSegment(draft.VideoMaterial("clip.mp4"),
                         target_timerange=draft.Timerange(0, 3_000_000))
seg.add_transition(draft.TransitionType.叠化, duration=500_000)
seg.add_keyframe(draft.KeyframeProperty.uniform_scale, 0, 1.0)
seg.add_keyframe(draft.KeyframeProperty.uniform_scale, 3_000_000, 1.15)
script.add_segment(seg, vref)

seg2 = draft.VideoSegment(draft.VideoMaterial("clip.mp4"),
                          target_timerange=draft.Timerange(3_000_000, 3_000_000))
script.add_segment(seg2, vref)

tref = script.append_track(draft.TrackSpec(draft.TrackType.text))
txt = draft.TextSegment("标题", timerange=draft.Timerange(200_000, 2_000_000),
                        style=draft.TextStyle(size=8.0, bold=True,
                                              color=(1.0, 0.8, 0.2),
                                              letter_spacing=2))
script.add_segment(txt, tref)

script.dump("<root>/name/draft_content.json")
```

## 速查：命名陷阱

| 陷阱 | 正确写法 |
|---|---|
| TextSegment 时间参数 | `timerange=`（VideoSegment 是 `target_timerange=`） |
| 空轨道集合 falsy | `a.strips if hasattr(a,'strips') else a.sequences`（`or` 会走反） |
| `TransitionType` 中文名 | 枚举成员是中文（叠化/推近/拉远…），`list(TransitionType)` 查 |
| position 归一化 | 0-1 相对画面；位移配 `uniform_scale ≥ 1+2×幅度` 防露边 |
| 草稿根缺失 | 先 `Path.mkdir(parents=True, exist_ok=True)`；剪映 6+ 建议用已有草稿做模板 |
