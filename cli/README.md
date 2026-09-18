# jycut — 本地剪映/CapCut 草稿 CLI

`jycut` 是 partme-jianying-plugin **内置的 Rust 本地引擎**：把一份
`jycut-plan/v1` JSON 计划构建成**剪映专业版可直接打开的原生草稿**（多轨视频/
文字/音频、转场、关键帧），构建产物可校验、可发布进草稿库。无账号、无上传、
无 Python 环境——单二进制，确定性输出。

## 三源整合与许可

| 来源 | 许可 | 吸收了什么 |
|---|---|---|
| [GuanYixuan/pyJianYingDraft](https://github.com/GuanYixuan/pyJianYingDraft) | Apache-2.0 | 最小草稿骨架两件资产 + 转场资源元数据（`assets/`，含出处说明）；segment/文本样式格式事实 |
| [renezander030/capcut-cli](https://github.com/renezander030/capcut-cli) | MIT | 双镜像写盘（draft_content+draft_info）、root_meta_info 注册、编辑器运行守卫、render_index 约定、伴生材料集 |
| [partme-ai/jianying-headless](https://github.com/partme-ai/jianying-headless) | 个人学习与非商业 | **仅契约/工作流事实**（build→verify→publish 门禁、主轨连续规则）；零代码零资产 |

## 构建

```bash
cd cli && cargo build --release   # 产物 target/release/jycut
cargo test                        # 3 集成测试
```

## 命令

```bash
jycut doctor                          # 草稿根/运行中的编辑器/ffprobe 预检
jycut probe media.mp4                 # 实测时长/分辨率/流
jycut build plan.json --out draft-v1  # 计划 → 草稿目录（自带 verify 门禁）
jycut verify draft-v1                 # 结构 lint（引用完整/主轨连续/时长一致）
jycut inspect draft-v1                # 轨道/段数摘要
jycut publish draft-v1 [--root DIR]   # 拷入草稿库 + 注册 root_meta_info.json
                                      #   剪映运行中拒绝（--force 覆盖）
```

发布后剪映开始页即出现该草稿；精修与最终导出在剪映内完成。

## 计划契约 `jycut-plan/v1`

时间一律微秒；相对素材路径按计划文件所在目录解析。

```json
{
  "schema": "jycut-plan/v1",
  "name": "草稿名",
  "canvas": {"width": 1920, "height": 1080, "fps": 30},
  "tracks": [
    {"type": "video", "segments": [
      {"start_us": 0, "duration_us": 4000000, "source": "shot.mp4",
       "source_start_us": 0, "speed": 1.0, "volume": 1.0,
       "scale": 1.0, "x": 0.0, "y": 0.0, "rotation": 0.0, "opacity": 1.0,
       "transition_out": {"name": "叠化", "duration_us": 500000},
       "keyframes": {"scale": [{"at_us": 0, "value": 1.0}, {"at_us": 4000000, "value": 1.12}]}}
    ]},
    {"type": "audio", "segments": [
      {"start_us": 0, "duration_us": 4000000, "source": "narration.mp3", "volume": 0.8}
    ]},
    {"type": "text", "segments": [
      {"start_us": 200000, "duration_us": 1500000, "text": "字幕", "size": 8,
       "color": "#FFFFFF", "border_color": "#000000", "border_width": 4}
    ]}
  ]
}
```

校验规则（违规即报错）：**主视频轨必须是第一轨且首段从 0 起连续**（黑场放
画中画轨）；同轨段递增不重叠；fps ∈ {24,25,30,50,60}；speed 0.1-8；
volume 0-4；转场仅限视频轨且不能落在末段，名称限内置非 VIP 目录
（`assets/transitions.json`：叠化/闪黑/闪白/模糊/百叶窗/水墨/翻页/叠化扭曲）；
关键帧线性、每通道 ≥2 点、首点 `at_us=0`，且要求 `speed=1 && source_start_us=0`。

## MVP 边界（如实）

- 不含：蒙版、滤镜/特效轨、画中画的多视频轨已支持但样式仅基础 transform、
  原生 MP4 导出（导出在剪映内由用户完成——NC 引擎的导出能力依赖其私有管
  线，本 CLI 不复刻）。
- 文本 content 的 `range` 按 UTF-16 code units 计算（与 app 一致，CJK 已测）。
- 路线图：SRT→字幕轨、模板参考、滤镜/特效目录扩充（Apache 元数据源充足）。
