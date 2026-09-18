---
name: jianying-setup
description: "Diagnose and fix the JianYing automation environment: fork checkout (partme-ai/jianying-headless), pinned 剪映专业版 11.4.x runtime, draft root, ASR executor (YICHEN_ASR_EXECUTOR), and ffmpeg. Per-OS guidance, advisory only."
---

# JianYing Setup（环境诊断与准备）

## 检查清单（按序）

1. **fork 检出**：`JIANYING_HEADLESS_ROOT` 指向
   partme-ai/jianying-headless 的本地检出（用户自己的 fork，零改动使用）。
   未设置时按默认路径探测，都没有则引导：
   ```bash
   git clone https://github.com/partme-ai/jianying-headless.git
   export JIANYING_HEADLESS_ROOT=<检出路径>
   ```
2. **剪映专业版 11.4.x**：引擎按精确 runtime profile 钉死（11.4.0/11.4.2，
   校验 bundle ID `com.lemon.lvpro`、深度签名与 Team ID）。升级到其他版本会被
   引擎拒绝——这是保护机制，不是 bug；不要改常量绕过。
3. **ASR 执行器**（仅口播精剪需要）：默认找用户目录的
   `scripts/transcribe.py`，或 `YICHEN_ASR_EXECUTOR` 指定绝对路径。
   执行前确认兼容 asr_once.py 的参数约定并有服务授权。
4. **ffmpeg/ffprobe**：素材探测与合成需要；macOS `brew install ffmpeg`。

## 硬性边界（引擎自身的许可与设计）

- 引擎按「私有源码预览」发布：Personal Learning and Non-Commercial。
  商用需按原作者渠道取得授权；本插件不代为声明许可。
- 官方程序库、账号资料、真实素材与效果资源不随引擎分发，本插件也不打包。
- 特效资源只能来自本机已合法取得的匹配缓存（逐文件 hash 校验）；
  缓存缺失时停止，不自动下载、不伪造授权身份。

## Never do

- Never 改引擎的固定版本、hash 常量或签名检查来"通过"预检。
- Never 从非官方镜像安装剪映。
- Never 自动充值、切换 ASR 服务商或重复提交付费请求。
