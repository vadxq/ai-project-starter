# v0.1.0 参考项目分析

调研日期：2026-10-04。信息来源为 GitHub API 返回的固定 commit、该 commit 的实际文件，以及已读取的生态官方说明；以下不把 README 的宣传或 TODO 当作已实现能力。

## 1. Next.js 参考项目

项目：[vadxq/nextjs-ai-starter](https://github.com/vadxq/nextjs-ai-starter)。固定快照：`2e462db0e32fc74e563f14b19c04c1b3f50ea2cb`。许可证：Apache-2.0。

| 来源                                                                                                                      | 已核实事实                                                         | 对本方案的影响                                                 |
| ------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ | -------------------------------------------------------------- |
| [package.json](https://github.com/vadxq/nextjs-ai-starter/blob/2e462db0e32fc74e563f14b19c04c1b3f50ea2cb/package.json)     | Next ^16.2.4、React ^19.2.5、TypeScript ^6.0.3、Tailwind ^4.2.4    | 推荐相同主版本系列，实施时独立验证并锁定                       |
| 同一 package.json                                                                                                         | SWR ^2.4.1、Zustand ^5.0.13、next-intl ^4.11.0、Serwist ^9.5.11    | 体现生态方案；首版按需求取舍，不全部预装                       |
| [README.md](https://github.com/vadxq/nextjs-ai-starter/blob/2e462db0e32fc74e563f14b19c04c1b3f50ea2cb/README.md)           | App Router、主题、国际化、PWA、UI 与工程指南                       | 复用模块组织思路和工程说明方式                                 |
| [AGENTS.md](https://github.com/vadxq/nextjs-ai-starter/blob/2e462db0e32fc74e563f14b19c04c1b3f50ea2cb/AGENTS.md)           | 已有 AI 开发规则，涉及目录、命令与代码约定                         | 本项目采用根/分端 AGENTS 与公共文档，Claude 入口仅引用统一来源 |
| [lib/http](https://github.com/vadxq/nextjs-ai-starter/tree/2e462db0e32fc74e563f14b19c04c1b3f50ea2cb/lib/http)             | SWR hooks、自研 fetch 与服务端缓存辅助层                           | 采用成熟 typed client，避免继续扩张自研 HTTP 框架              |
| [next.config.ts](https://github.com/vadxq/nextjs-ai-starter/blob/2e462db0e32fc74e563f14b19c04c1b3f50ea2cb/next.config.ts) | React Compiler、Cache Components 开关、Serwist、实验配置、图片优化 | 首版只启用经过本项目验证且确有用途的配置                       |

### Next.js 值得保留的部分

- Next.js App Router 与 Server / Client 边界意识。
- React + TypeScript strict、Tailwind、按需 shadcn/ui、主题和清晰模块目录。
- 使用根 `AGENTS.md` 维护统一工程约定的习惯。
- HTTP 错误、生产构建与文档化配置的关注点。

### Next.js 的首版调整

- 数据获取统一到 TanStack Query；API 类型来自 OpenAPI，不依赖人工填写泛型来声称类型安全。
- 按用户确认要求首版完整支持 zh-CN / en；保留参考项目的 next-intl / locale 路由思路，SPA 使用 react-i18next，原生端使用平台本地化资源。
- 首版不内置 PWA / Service Worker，避免引入离线与私有数据缓存问题。
- 不直接继承液态玻璃、iOS 26 风格或实验动画配置；各端采用简洁的必要界面。
- Rust 承载业务数据库和授权，Next 不再加入 Prisma 与第二套账号数据库。
- 按用户要求，SPA 与 Next 各自保留依赖、锁文件、API 适配和语言资源，可以单独复制使用；类型统一来源于 API 契约，各端分别生成。

### 必须纠正的能力判断

README 中的 AI、数据库、NextAuth / OAuth 仍位于 TODO。固定快照没有相应实现，因此该项目不是现成的登录、数据库或产品 AI 后端。

README 仍写 TypeScript 5.9 等版本，而 package.json 已升级到 6.0.3。依赖判断以实际清单为准。

`lint` 入口目前调用 staged 检查，还存在 `npm lint:lint-staged` 的调用形式；不能直接作为本项目质量门禁。AGENTS 文档说明尚未内置自动测试 runner，依赖手动体验检查；本项目要求本地自动浏览器验证。

## 2. Rust / Axum 参考项目

项目：[vadxq/rust-axum-starter](https://github.com/vadxq/rust-axum-starter)。固定快照：`c38a68bc75d655e2c556e5331d05ed9cd08923a9`。许可证：Apache-2.0。

| 来源                                                                                                                                                 | 已核实事实                                                          | 对本方案的影响                                                  |
| ---------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- | --------------------------------------------------------------- |
| [Cargo.toml](https://github.com/vadxq/rust-axum-starter/blob/c38a68bc75d655e2c556e5331d05ed9cd08923a9/Cargo.toml)                                    | Axum 0.8.8、SQLx 0.8.6、Tokio 1.50.0、MSRV 1.92、edition 2021       | 推荐相同主版本族；新项目 edition 与工具链在 P0 验证             |
| [架构说明](https://github.com/vadxq/rust-axum-starter/blob/c38a68bc75d655e2c556e5331d05ed9cd08923a9/docs/architecture.md)                            | `api / domain / platform`，main → config → tracing → state → router | 直接采用清晰分层，不提前扩张复杂 DDD                            |
| [README.md](https://github.com/vadxq/rust-axum-starter/blob/c38a68bc75d655e2c556e5331d05ed9cd08923a9/README.md)                                      | `/health`、`/ready`、可选 PostgreSQL、结构化日志、CORS、request ID  | 保留探针区分与基础设施边界                                      |
| [API 测试契约](https://github.com/vadxq/rust-axum-starter/blob/c38a68bc75d655e2c556e5331d05ed9cd08923a9/docs/development/ai-coding-test-contract.md) | 实现前明确请求、响应、数据变化与测试矩阵                            | 扩展为全仓契约模板，集中到根 docs                               |
| [测试说明](https://github.com/vadxq/rust-axum-starter/blob/c38a68bc75d655e2c556e5331d05ed9cd08923a9/docs/development/testing.md)                     | 纯转换测试与真实 Router oneshot，数据库测试独立执行                 | 保留定向测试，增加本版本真实 DB/Auth 的发布门禁                 |
| [response.rs](https://github.com/vadxq/rust-axum-starter/blob/c38a68bc75d655e2c556e5331d05ed9cd08923a9/src/api/response.rs)                          | 成功包装、JSON rejection 转换、毫秒时间戳工具                       | 保留统一错误思想，协议改为 DTO JSON + Problem Details + RFC3339 |
| [初始化 migration](https://github.com/vadxq/rust-axum-starter/blob/c38a68bc75d655e2c556e5331d05ed9cd08923a9/migrations/20260709_000001_init.sql)     | 只有占位注释，没有业务表                                            | identities、items 与约束是本版本新增工作                        |

### Rust 值得保留的部分

- 单服务、低层级、按业务域添加模块。
- Axum / Tokio / SQLx / tracing 生态，JSON extractor 错误统一处理。
- 进程存活和业务就绪分别检查。
- API 测试契约先于实现，并验证实际数据变化。

### Rust 的首版调整

- 参考允许 DB 失败仍启动 HTTP；本方案有真实持久化业务，启动时必需依赖初始化失败将中止。运行期故障仍按 health / ready 区分。
- 参考使用浮动 `stable`；本项目固定实际 Rust 版本与锁文件。
- 参考响应以 HTTP 状态码字符串作为 code；本项目用标准 HTTP 语义及稳定业务错误码。
- 增加 OpenAPI、身份映射、CRUD、用户隔离与乐观并发控制；这些是新实现，不能宣称直接复制即可运行。
- 后端迁移、配置和原生 PostgreSQL 启动说明在 backend 内提供独立入口；全仓文档统一归入 docs，根脚本仅调用本端命令完成可选组合操作。

## 3. 其他已读取来源

| 来源                                                                                                                                                  | 已核实内容                                                                                     | 使用边界                                                       |
| ----------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| [npm Vite metadata](https://registry.npmjs.org/vite/latest)                                                                                           | 本次返回 8.3.2；Node engines 为 `^20.19.0` 或 `>=22.12.0`                                      | 推荐 Vite 8.x + Node 24.x；该 URL 会变化，不作为未来锁版本来源 |
| [Apple Swift OpenAPI Generator README](https://github.com/apple/swift-openapi-generator/blob/main/README.md)                                          | 支持 OpenAPI 3.0/3.1；构建期生成；URLSession transport；generated runtime 支持 iOS 13+         | 满足 iOS 18 下限；不代表项目 Swift API 已编译                  |
| [OpenAPI Generator Kotlin 文档](https://github.com/OpenAPITools/openapi-generator/blob/master/docs/generators/kotlin.md)                              | 支持 jvm-retrofit2、kotlinx_serialization、useCoroutines 等配置                                | 先做生成编译验证；模板文档里的历史库版本不直接继承             |
| [官方 OpenAI 文档：AGENTS.md](https://learn.chatgpt.com/docs/agent-configuration/agents-md)                                                           | 全局与项目规则发现、根到当前工作目录的层级、子目录覆盖                                         | 设计分端规则与正确启动目录；不假设其他 agent 工具自动兼容      |
| [Android uses-sdk](https://developer.android.com/guide/topics/manifest/uses-sdk-element)                                                              | minSdk 是安装下限；targetSdk 决定相应平台行为 / 兼容模式                                       | 分别维护运行下限与当前稳定 target，不能混为一项                |
| [Android 13 平台能力](https://developer.android.com/about/versions/13/features)                                                                       | API 33 的系统 Photo Picker、应用语言等能力                                                     | 解释较现代 Android 基线；不把这些能力全部加入首版              |
| [Android 14 平台能力](https://developer.android.com/about/versions/14/features)                                                                       | API 34、平台 Credential Manager、200% 非线性字体缩放；Credential Manager 另有 Jetpack 向下支持 | 推荐 API 34 是维护策略，不是 Compose / 登录库的强制最低版本    |
| [Apple Observation 迁移说明](https://developer.apple.com/documentation/swiftui/migrating-from-the-observable-object-protocol-to-the-observable-macro) | 正文和官方 JSON 确认 SwiftUI Observation 从 iOS 17 起支持                                      | iOS 18 可采用现代状态管理，不必为它提高到最新系统              |
| [Apple Xcode SDK / 系统要求](https://developer.apple.com/xcode/system-requirements)                                                                   | 编译 SDK、deployment target、设备 / Simulator 支持与 Swift mode 分开列出                       | 使用当前稳定工具链仍可选择 iOS 18 下限；具体组合落地验证       |

读取 Codex 规则说明时，`developers.openai.com/codex/guides/agents-md` 实际重定向到上述官方页面；这里引用最终读取地址。OpenAPI 工具文档为调研日的滚动说明，实施时另行锁发布版本。

原参考中的认证、UI、任务工具等未固定发布版本的建议属于待实施选型，不能从本文推断它们已经在本仓库完成兼容验证。

## 4. 复用方式与许可证

两个参考项目以及本仓库目前均为 Apache-2.0。优先复用成熟依赖、目录与流程模式；需要复制具体文件时，在实施记录里标记来源 commit 与改动。

保留上游许可证、原始版权与适用 NOTICE，修改复制文件时记录修改说明。当前尚未复制应用源文件，不需要为“仅参考设计”伪造代码来源清单。

不把参考项目中的个人本机路径、日志、账号、MCP 配置或手工验证要求直接带入通用模板。

## 5. 调研限制

本次读取了元信息、完整文件树和重点源码 / 文档，没有在本地安装、构建或运行两个参考项目。因此，可以确认文件体现的能力与依赖声明，不能确认它们当前完整 build、PWA、缓存和实际部署行为均通过。

方案已获用户确认，2026-10-05 已恢复完整实现和本地验证。参考分析用于解释选择；各端当前工具版本与实际结果以架构文档和验收记录为准，不将上游能力直接视为本项目已通过的能力。
