---
name: jianying-draft
description: "Generate JianYing Pro drafts directly with pyJianYingDraft: DraftFolder/ScriptFile/TrackSpec/VideoSegment/TextSegment/AudioSegment API cookbook, draft_content.json principles, version sensitivity, and offline draft generation. Use when the edit-plan abstraction does not fit and raw library control is needed."
---

# JianYing Draft（pyJianYingDraft 直接生成）

当剪辑计划抽象（`jianying-edit`）不够用时，直接用
[pyJianYingDraft](https://github.com/GuvaI/pyJianYingDraft) 0.3.0（Apache-2.0）
生成草稿。原理：剪映草稿是 JSON（`draft_content.json` + `draft_meta_info.json`），
其余文件打开剪映后会自动补全。

> **fork 直连语境**：`jianying-headless` 引擎本身就是这一层库的封装。
> 优先走 `jianying-edit` 的计划链（build/publish 有校验与审计）；本页的
> 直接库调用仅用于 fork 计划字段确实覆盖不到的实验场景，且同样遵守
> "独立副本、不碰已有草稿"纪律。直接库调用绕过引擎的哈希钉扎校验——
> 交付前必须在剪映里人工验收，冷重开回读。

## 安装

```bash
pip install pyJianYingDraft pymediainfo
```

## 最小示例

```python
import pyJianYingDraft as draft

folder = draft.DraftFolder("<剪映草稿根目录>")
script = folder.create_draft("demo", 1280, 720, fps=24, allow_replace=True)

vref = script.append_track(draft.TrackSpec(draft.TrackType.video))
seg = draft.VideoSegment(
    draft.VideoMaterial("clip.mp4"),
    target_timerange=draft.Timerange(start=0, duration=3_000_000),
    volume=0.8,
)
script.add_segment(seg, vref)

tref = script.append_track(draft.TrackSpec(draft.TrackType.text))
txt = draft.TextSegment("标题", timerange=draft.Timerange(start=200_000, duration=2_000_000),
                        style=draft.TextStyle(size=8.0, bold=True))
script.add_segment(txt, tref)

script.dump("<草稿根>/demo/draft_content.json")
```

## API 速查

| 类 | 要点 |
|---|---|
| `DraftFolder(root)` | 草稿根；根目录必须已存在（先建目录） |
| `create_draft(name, w, h, fps, allow_replace)` | 返回 ScriptFile；allow_replace 控制同名 |
| `TrackSpec(TrackType.video/text/audio, name)` | 配 `append_track` 返回 TrackRef |
| `VideoMaterial/AudioMaterial(path)` | 本地素材（会被 ffprobe/probe 读取） |
| `VideoSegment(material, target_timerange, volume, speed)` | `target_timerange=Timerange(start,duration)` 微秒 |
| `TextSegment(text, timerange, style=TextStyle(...))` | **timerange 是位置/关键字参数，不是 target_timerange** |
| `seg.add_transition(TransitionType.叠化, duration=500_000)` | 转场（段尾生效） |
| `seg.add_keyframe(KeyframeProperty.uniform_scale, us, value)` | 关键帧 |
| `TextStyle(size, color, bold, italic, align, letter_spacing)` | 全字段样式 |

## 版本敏感

- pyJianYingDraft 0.3.0：类名为 camelCase（`DraftFolder`/`ScriptFile`），
  轨道操作是 `append_track(TrackSpec)` + `add_segment(seg, track_ref)`。
- 剪映大版本更新可能改变草稿内部结构——生成前确认引擎声明的兼容版本区间。
- 新建草稿无需模板；修改已有草稿需先校验来源并做独立副本。

## 常见坑

- `TextSegment` 的样式参数叫 `timerange`（VideoSegment 叫 `target_timerange`）。
- 空集合是 falsy：`editor.strips or editor.sequences` 这类 fallback 会走反。
- 世界坐标：position 归一化 0-1；放大超 1.15 会露边/变软。
- 转场作用于**段尾**，且与首尾帧素材一样吃画面时间。
