# 计划格式 `jy14-headless-plan/v1`（字段手册）

引擎 `jianying-headless` 的输入契约。本文为按引擎校验器（`validate_plan`）
归纳的速查；权威原文在 fork 检出内 `references/plan-format.md`（个人学习许可，
勿复制进本插件）。时间一律**微秒整数**；坐标为归一化值。

## 顶层

| 字段 | 类型 | 必填 | 说明 |
|---|---|---|---|
| schema | string | ✅ | 恒为 `jy14-headless-plan/v1` |
| name | string | ✅ | 草稿名：可见单层目录名（禁 `/ \ .` 前缀等，<200 字节），草稿根内唯一 |
| canvas | object | ✅ | `width`/`height`（16..8192）、`fps` ∈ {24,25,30,50,60} |
| tracks | array | ✅ | 轨道数组；**主视频轨必须是第一轨** |

## 轨道

`{type, name, segments[]}`；type ∈ `video | audio | text | filter | effect`；
每轨至少一段；filter 与 effect 各最多一轨。主视频轨首段 `start_us=0` 且段间
连续（相邻差 ≤1µs）——黑场/间隙放画中画 video 轨。

## 段字段

公共：`start_us`（时间轴起点，同轨递增不重叠）、`duration_us`、`source`（本地
素材路径）、`source_start_us`（源内入点）、`source_duration_us`（可选）、
`speed`（0.1-8）、`volume`（0-4）、`keyframes`。

- **video** 额外：`scale`、`x`、`y`、`rotation`、`opacity`(0-1)、`mask`、
  `transition_out`
- **text**：`text`、`size`、`x`、`y`（底部字幕默认 -0.78）、`color`(#RRGGBB)、
  `border_color`、`border_width`、`opacity`、`keyframes`、`text_effect`
- **filter**：`name`（已采集目录内）、`strength`（默认 1）
- **effect**：`name`（已采集目录内）、`params`（按目录声明的参数键）
- 可用资源名读 fork 内 `engine/native-resource-catalog.json`（哈希钉扎，
  未采集名字直接报错，不要猜名字）

## keyframes（按通道字典）

```json
{"keyframes": {"scale": [{"at_us": 0, "value": 1.0},
                          {"at_us": 4000000, "value": 1.12}]}}
```

- 通道：video=`scale/x/y/rotation/opacity`；text=`x/y/scale/rotation`；
  audio=`volume`。值域：x/y ±5、scale 0.01-10、rotation ±360、opacity 0-1、
  volume 0-4。
- 线性插值；每通道 ≥2 点、`at_us` 严格递增、**首点必须 at_us=0**；
  同段若有同名字段，其值须等于首点值。
- 带关键帧的段 **`speed` 必须 1 且 `source_start_us` 必须 0**（裁剪/变速源的
  时间映射需另行验证，引擎直接拒绝）。

## mask（video 段）

`{shape, width, height, x, y, rotation, feather, invert, round_corner}`；
shape ∈ `circle | rectangle | line | mirror | star | heart`（line 是半平面
掩码，width/height 无效）。width 默认 0.28、height 默认 0.5（范围 0.001-5）；
feather/round_corner 0-1；round_corner 仅 rectangle 有效；invert 布尔。

## transition_out（仅主视频轨）

`{name: "dissolve", duration_us, edge_policy}`。当前只支持叠化；必须落在
**有后继段**的段上（末段不能加）；`edge_policy` ∈ `require-handles`（要求素材
有转场手柄）/ `repeat-edge`（重复边缘帧）；居中转场要求时间轴帧数为偶数。

## 约束速记

- 同轨段递增不重叠；主轨无缝。
- 转场、黑场、画中画：先想清楚"哪一轨承担"，主轨永远连续。
- 计划值是设计意图，**放置前一律用 ffprobe 实测时长回填** `duration_us`。
