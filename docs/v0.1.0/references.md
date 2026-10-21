# v0.1.0 参考项目与换新来源

2026-10-21 已按用户批准方案整体导入两个上游的新基座，Backend 冲突项以上游为准。以下 SHA 是本轮检查远程 HEAD 后固定的来源；未来上游更新需重新审查和验证。

| 工程    | 固定来源                                                                                            | 采用内容                                                                                                               | 本项目适配                                                                           |
| ------- | --------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------ |
| Next    | [81e089f](https://github.com/vadxq/nextjs-ai-starter/tree/81e089fbcc06a237353e2107bd578d20567dfba3) | Next 16.4.2、React 19.3.1、Tailwind 4.3.3、shadcn/ui、next-themes、Zustand、类型化语言路由、React Compiler、全工程检查 | 浏览器 JWT、TanStack Query、生成式 SDK、Item 工作区、standalone；排除 PWA 和演示页面 |
| Backend | [6df373d](https://github.com/vadxq/rust-axum-starter/tree/6df373d98b573e8155cd1d1931065aa16af81598) | Axum 0.8.9、SQLx 0.9、edition 2024、stable、PostgreSQL / MySQL、APP_*、lazy pool、统一响应、请求追踪、宽松 CORS        | 两库密码认证、JWT / 轮换会话、Item 权限和版本事务、显式迁移、OpenAPI                 |

依赖版本以固定提交的 manifest / lockfile 与本项目锁文件为准。上游的测试或文档不能代替本项目验证；实际命令、结果和限制统一见 [交付计划](delivery-plan.md) 与其关联报告。

## 已确认的边界

- Backend 保留上游可选数据库启动、lazy pool、APP_* 配置和 stable 工具链。
- 成功响应采用 `{code,message,data}`，错误采用 `{code,message,details?}`，时间为整数 epoch 毫秒。业务 `details.reason` 用于客户端本地化；JSON rejection 保留 400 / 413 / 415 / 422。
- Rust 是业务和授权的唯一来源；Next 不连接业务数据库。四端使用各自的 OpenAPI 快照生成 SDK。
- Next 使用上游目录、主题和组件生态，业务请求保留在浏览器；英文 / 简体中文、系统 / 浅色 / 深色主题及 standalone 输出均保留。
- 五个工程仍独立安装和构建，无根 JavaScript workspace、跨端共享源码或个人 MCP 配置。

## 生态文档入口

- [Apple Swift OpenAPI Generator](https://github.com/apple/swift-openapi-generator)：Swift 构建期生成与 URLSession transport。
- [OpenAPI Generator Kotlin](https://github.com/OpenAPITools/openapi-generator/blob/master/docs/generators/kotlin.md)：Retrofit、协程与序列化配置。
- [Android uses-sdk](https://developer.android.com/guide/topics/manifest/uses-sdk-element)：minSdk 与 targetSdk 的不同职责。
- [Apple Xcode 系统要求](https://developer.apple.com/xcode/system-requirements)：SDK、deployment target 与 Simulator 支持范围。

这些是维护时的官方入口，不作为未执行功能或未来版本兼容性的验收证据。

## 来源与许可证

两个上游均为 Apache-2.0；本项目保留 `next/LICENSE` 和 `backend/LICENSE`。Next 的目录、组件、样式、语言路由和工具配置，以及 Backend 的 bootstrap、配置、连接池、探针、响应、中间件与开发文档来自上述固定提交，并按本项目业务调整。具体变更记录于各工程 CHANGELOG，原始导入和验证日志保存在被忽略的 `.ailog`。
