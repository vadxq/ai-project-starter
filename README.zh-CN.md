# AI Project Starter

[English](README.md) | 简体中文

五个独立工程，按需选择：Vite + React SPA、Next.js、Rust + Axum、Kotlin Android、Swift iOS。示例提供 用户名密码与 JWT 登录、个人事项 CRUD、中英文和系统 / 浅色 / 深色主题。需求与计划统一放在 [docs](docs/README.md)。

| 工程    | 本端说明                                           | 基础要求                                 |
| ------- | -------------------------------------------------- | ---------------------------------------- |
| SPA     | [spa/README.zh-CN.md](spa/README.zh-CN.md)         | Node 24、npm                             |
| Next    | [next/README.zh-CN.md](next/README.zh-CN.md)       | Node 24、npm                             |
| Backend | [backend/README.zh-CN.md](backend/README.zh-CN.md) | Rust stable、PostgreSQL / MySQL          |
| Android | [android/README.zh-CN.md](android/README.zh-CN.md) | JDK 21、Android SDK；设备 Android 14+    |
| iOS     | [ios/README.zh-CN.md](ios/README.zh-CN.md)         | Xcode、XcodeGen、CocoaPods；设备 iOS 18+ |

各目录可单独复制，使用本端配置、`.gitignore` 和锁文件。Web 分别运行 npm，没有根 JS package、共享 JS workspace 或跨端源码依赖。

## 本地开始

先进入 backend，按本端说明启动 PostgreSQL 或 MySQL，复制 `.env.example` 为 `.env`，配置连接信息与随机 APP_AUTH__JWT_SECRET。显式迁移，用 `create-user` 创建本地账号后执行 `cargo run`。示例 alice / bob 和密码 starter-password 需按 backend 文档手动创建；生产使用实际密码。

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

例如 `bash scripts/verify.sh spa quick`、`bash scripts/verify.sh backend full` 或 `bash scripts/contracts.sh check`。必须显式指定模式：quick 执行静态/构建检查，full 追加本端真实集成；backend full 构建前要求 TEST_API_URL / TEST_DATABASE_URL / APP_AUTH__JWT_SECRET。Web 需要实际 API；Android 需要 Emulator 和 adb reverse；iOS 指定 `IOS_DESTINATION='platform=iOS Simulator,id=<UUID>'`。单个 full 结果不替代完整发布矩阵。

Next / Backend 已按 2026-10-21 上游基线换新，四端新契约适配完成；两库、Web 与最低 / 当前安装移动系统的本地验证通过。范围与已知依赖告警见 [本轮报告](docs/verification/2026-10-21-upstream-refresh.md)，当前状态以 [交付计划](docs/v0.1.0/delivery-plan.md) 为准。生产服务地址、签名和部署由实际项目配置。

## AI 开发

每个工程携带 AGENTS.md 和本地 `.agents/skills/<project>-changelog` 示例。Next 维护 `CHANGELOG.md` / `CHANGELOG.zh-CN.md`，Backend 使用 `CHANGELOG.md`，其余示例与根 [update-changelog](.agents/skills/update-changelog/SKILL.md) 使用 `changelog.md`。日志不存在时初始化，存在时合并已确认的实际变更。验证继续使用各端 README 和交付计划中的原生命令。使用方式与范围见 [AI 工作流](docs/ai-workflows.md)。

临时计划、日志、测试报告和任务缓存统一写入 Git 忽略的 `.ailog`。模板不依赖个人模型、凭证或 MCP 安装。

参考 [vadxq/nextjs-ai-starter](https://github.com/vadxq/nextjs-ai-starter) 与 [vadxq/rust-axum-starter](https://github.com/vadxq/rust-axum-starter)，取舍与固定参考提交见 [参考分析](docs/v0.1.0/references.md)。许可证为 Apache-2.0。
