---
name: jianying-draft
description: "pyJianYingDraft API cookbook for the vendored engine: DraftFolder/ScriptFile/TrackSpec/segments constructors with exact signatures, transitions (453-name TransitionType), keyframes, masks, styled text (TextStyle/TextBorder/TextShadow/TextBackground), animations, filters/effects, SRT import. The reference every generation script is written from."
license: Apache-2.0
---

# JianYing Draft（pyJianYingDraft API 手册）

剪映草稿 = JSON（`draft_content.json` + `draft_meta_info.json`），本引擎
（插件 vendored，Apache-2.0）从内嵌骨架从零生成，无需用户模板。以下签名
全部按 vendor 版源码核实。

## 骨架流程

```python
from pyJianYingDraft import DraftFolder, TrackSpec, TrackType
import os
os.makedirs("store", exist_ok=True)          # 根目录必须先存在
script = DraftFolder("store").create_draft("demo", 1920, 1080, fps=30,
                                           allow_replace=False)
v = script.append_track(TrackSpec(TrackType.video))   # → TrackRef
# ... add_segment ...
script.dump("store/demo/draft_content.json")          # dump 需显式路径
```

- `create_draft` 同名默认报错；`allow_replace=True` 需用户明确同意。
- `duplicate_as_template(template_name, new_name)` 改已有草稿 = 独立副本。
- 时间单位全库微秒：`Timerange(start, duration)`。

## 段落构造器（核实签名）

```python
VideoSegment(material_or_path, target_timerange=Timerange, *,
             source_timerange=None,   # 截取源内区间
             speed=None, volume=1.0, change_pitch=False,
             clip_settings=ClipSettings(...))   # 越界 raise ValueError
AudioSegment(material_or_path, target_timerange=, *, volume=1.0, speed=...)
TextSegment(text, timerange=Timerange, style=TextStyle(...),
            border=TextBorder(...), background=TextBackground(...),
            shadow=TextShadow(...), clip_settings=ClipSettings(...))
```

- `TextSegment` 的时间参数叫 **`timerange`**（VideoSegment 叫
  `target_timerange`）。
- 变速代数：`source_timerange` 与 `speed` 同时给 → 覆盖 target 时长；素材
  越界直接 ValueError——先 ffprobe 实测。
- `ClipSettings(transform_x=, transform_y=, scale_x=, scale_y=, rotation=, alpha=)`，
  坐标以半幅计，字幕惯例 `transform_y=-0.8`。

## 文本样式

```python
TextStyle(size=8.0, bold=False, italic=False, underline=False,
          color=(1.0, 1.0, 1.0),   # RGB 浮点 0..1
          align=0,                  # 0 左 1 中 2 右
          auto_wrapping=True, max_line_width=0.82)
TextBorder(color=(0,0,0), width=40.0)      # width 默认 40（描边粗细）
TextShadow(...); TextBackground(color="#RRGGBB", style=1, ...)
```

## 段上能力（链式）

| 能力 | 调用 | 备注 |
|---|---|---|
| 转场 | `seg.add_transition(TransitionType.叠化, duration=500_000)` | 453 个中文名枚举；作用段尾；目录纪律见 `jianying-transitions` |
| 关键帧 | `seg.add_keyframe(KeyframeProperty.scale_x, 0, 1.0)` | 属性：position_x/y、scale_x/y、rotation、alpha、saturation、contrast、brightness；音频版 `add_keyframe(time_offset, volume)`；线性插值 |
| 蒙版 | `seg.add_mask(MaskType.圆形, center_x=, center_y=, size=)` | 6 形：线性/镜面/圆形/矩形/爱心/星形 |
| 滤镜 | `seg.add_filter(FilterType.<名>, intensity=100.0)` | 1052 目录 |
| 画面特效 | `seg.add_effect(VideoSceneEffectType.<名> \| VideoCharacterEffectType.<名>)` | 1097+240 目录 |
| 音频特效 | `seg.add_effect(AudioSceneEffectType.<名>)` | 场景音 85 |
| 入场/出场/循环动画 | `seg.add_animation(IntroType.\|OutroType.\|GroupAnimationType.<名>)` | 文本版 TextIntro/TextOutro/TextLoopAnim；**淡入淡出用动画，不用 alpha 关键帧**（渲染被忽略） |
| 花字 | `TextSegment.add_effect(effect_id, resource_id)` | 需要 effect_id/resource_id 元数据 |

## SRT 字幕一键导入

```python
script.import_srt("subs.srt", "字幕轨名",
                  text_style=TextStyle(size=5, align=1, auto_wrapping=True),
                  clip_settings=ClipSettings(transform_y=-0.8),
                  style_reference=可选样式基准段)
```

## 常见坑（实测）

- 空集合 falsy：`editor.strips or editor.sequences` 类 fallback 会走反。
- `AudioSegment` 的素材不能含视频轨（ValueError：音频素材不应包含视频轨道）。
- `allow_replace=False`（默认）下同名 create/duplicate 直接失败——这是
  非破坏纪律，不要图省事开 replace。
- 剪映大版本升级可能改草稿内部结构；生成后冷重开回读一次最稳。
