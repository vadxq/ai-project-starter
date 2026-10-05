# AI Project Starter

[English](README.md) | 简体中文

五个独立工程，按需选择：Vite + React SPA、Next.js、Rust + Axum、Kotlin Android、Swift iOS。示例提供 用户名密码与 JWT 登录、个人事项 CRUD、中英文和系统 / 浅色 / 深色主题。需求与计划统一放在 [docs](docs/README.md)。

| 工程    | 本端说明                                           | 基础要求                                 |
| ------- | -------------------------------------------------- | ---------------------------------------- |
| SPA     | [spa/README.zh-CN.md](spa/README.zh-CN.md)         | Node 24、npm                             |
| Next    | [next/README.zh-CN.md](next/README.zh-CN.md)       | Node 24、npm                             |
| Backend | [backend/README.zh-CN.md](backend/README.zh-CN.md) | Rust 1.99、PostgreSQL 16                 |
| Android | [android/README.zh-CN.md](android/README.zh-CN.md) | JDK 21、Android SDK；设备 Android 14+    |
| iOS     | [ios/README.zh-CN.md](ios/README.zh-CN.md)         | Xcode、XcodeGen、CocoaPods；设备 iOS 18+ |

各目录可单独复制，使用本端配置、`.gitignore` 和锁文件。Web 分别运行 npm，没有根 JS package、共享 JS workspace 或跨端源码依赖。

## 本地开始

先进入 backend，按本端说明启动原生 PostgreSQL，复制 `.env.example` 为 `.env`，配置连接信息与随机 JWT_SECRET。显式迁移，用 `create-user` 创建本地账号后执行 `cargo run`。示例 alice / bob 和密码 starter-password 需按 backend 文档手动创建；生产使用实际密码。

选择 SPA 或 Next，进入相应目录复制配置、执行 `npm ci`、`npm run dev`。默认端口：SPA 5173、Next 3000、API 8080。每个端只需要自己的工具链与可访问的服务。

iOS 开发环境：

```sh
cd ios
xcodegen generate -s project.yml
pod install
```

生产环境将第一条生成命令换成 `xcodegen generate -s project.prod.yml`，仍使用 `pod install`。打开生成的 `Starter.xcworkspace`。生产地址需要在本端 YAML 中配置。

Android 使用本端 Gradle Wrapper：`./gradlew :app:assembleDevDebug` 或 `./gradlew :app:assembleProdRelease`。详细网络和环境说明见 [移动双环境](docs/v0.1.0/mobile-environments.md)。

## 开发与验收

AI 从 [AGENTS.md](AGENTS.md) 和目标工程规则开始，按 [实施计划](docs/v0.1.0/delivery-plan.md) 工作。契约来自 Rust DTO / 路由，客户端使用本端 OpenAPI 快照生成；不能手改生成代码。

根 scripts 只提供完整仓库内的辅助操作。各端原生的构建、检查和测试命令仍是独立使用入口。真实认证 / 数据库测试需要启动本地服务；签名、生产部署和商店上架不包含在首版中。

例如 `bash scripts/verify.sh spa`、`bash scripts/verify.sh backend` 或 `bash scripts/contracts.sh check`。Web 测试需要实际 API；Android 需要 Emulator 和 adb reverse；iOS 指定 `IOS_DESTINATION='platform=iOS Simulator,id=<UUID>'`。脚本不安装系统工具或更改个人配置。

v0.1.0 已完成五端实现和本地验收，具体命令、结果与适用范围见 [验收记录](docs/verification/2026-10-05-v0.1.0.md)。生产服务地址、签名和部署由实际项目配置。

参考 [vadxq/nextjs-ai-starter](https://github.com/vadxq/nextjs-ai-starter) 与 [vadxq/rust-axum-starter](https://github.com/vadxq/rust-axum-starter)，取舍与固定参考提交见 [参考分析](docs/v0.1.0/references.md)。许可证为 Apache-2.0。
