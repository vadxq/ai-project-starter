# v0.1.0 需求设计

状态：基础实现已通过历史验收；当前 JWT 登录改造正在验收。更新日期：2026-10-05。接口细节以本文为准，实现组织见 [架构](architecture.md)，验收见 [交付计划](delivery-plan.md)。

## 目标与使用流程

提供可分别复制使用的 SPA、Next、Rust backend、Android、iOS 工程。通过真实登录、当前身份和最小 Item CRUD，验证全栈开发、契约演进、多语言和本地自动验收。AI 用于辅助开发，产品运行不依赖模型或 AI API key。

开发者选择所需工程，按本端 README 安装工具与依赖，配置 API，启动后端与 PostgreSQL，创建本地账号，通过应用内用户名密码登录，管理个人 Item。跨端共用协议和业务服务，各端自行管理源码、配置、依赖、语言资源与构建。

## 功能需求

| 编号   | 必须交付           | 验收要点                                                                                                                                         |
| ------ | ------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| REQ-01 | 五个独立工程       | 单独复制后能安装、生成与构建；不读取兄弟工程或根工具配置；各端独立 .gitignore；无根 JS package / workspace；SPA、Next 分别用 npm 和独立 lockfile |
| REQ-02 | Vite + React SPA   | 严格 TypeScript、路由、真实登录、Item CRUD、深链刷新、加载 / 空 / 错误 / 重试；生产静态部署和 API 转发示例                                       |
| REQ-03 | Next.js App Router | 多语言公共 SSR / Metadata；私有业务由 Client Component 调 Rust；登出清缓存；独立 Node 生产产物                                                   |
| REQ-04 | Rust + Axum API    | PostgreSQL、显式迁移、健康 / 就绪、身份与 CRUD、输入和故障处理、结构化日志、fmt / Clippy / build / 真实集成验证                                  |
| REQ-05 | Kotlin Android     | Compose / Material 3、ViewModel / StateFlow、应用内密码登录与 JWT、CRUD；minSdk 34；dev/prod flavor；生命周期不重复提交                          |
| REQ-06 | Swift iOS          | SwiftUI / Observation / async/await、密码登录、Keychain、CRUD；iOS 18.0；两份 XcodeGen 入口生成后 pod install                                    |
| REQ-07 | 统一契约           | Rust 导出 OpenAPI；各客户端保存本地快照并用成熟生成器生成类型；覆盖错误 / 空值 / 分页；漂移检查只读且失败退出非零                                |
| REQ-08 | 真实认证与隔离     | Rust 用户名密码登录，Argon2id；JWT 校验 issuer / audience / 签名 / 期限 / 会话，轮换 refresh token；他人资源 404；登出清本端身份和数据           |
| REQ-09 | AI 开发上下文      | 根与各端 AGENTS.md、Claude 薄入口、需求 / 计划 / 契约模板；原生命令；无私有 MCP / 模型 / 个人凭证依赖                                            |
| REQ-10 | 可操作文档与脚本   | scripts 仅做契约同步和本地验证等薄封装；原生工具准备环境；不覆盖个人配置；本地证据可追溯；不以 CI 替代本地验收                                   |
| REQ-11 | 全端多语言与主题   | 完整 zh-CN / en；语言偏好持久化、初始系统匹配、未支持时 en；浅色 / 深色 / 系统；错误和无障碍文案也翻译                                           |

## 接口契约

传输为 JSON，字段 camelCase，UUID 标识，时间为 UTC RFC3339 毫秒格式（Z）。业务请求使用 Bearer access token；成功返回 DTO，不套额外 envelope。未知 JSON 字段、错误字段类型、缺必填值均为 400。

| 方法   | 路径                   | 请求 / 成功结果                                                                        |
| ------ | ---------------------- | -------------------------------------------------------------------------------------- |
| GET    | `/health`              | 200，进程存活，不探测依赖                                                              |
| GET    | `/ready`               | 200，数据库可查询；依赖故障 503                                                        |
| POST   | `/api/v1/auth/login`   | `{username,password}`；200 `{accessToken,refreshToken,tokenType,expiresIn,user}`       |
| POST   | `/api/v1/auth/refresh` | `{refreshToken}`；200 轮换后的 token 对与 user                                         |
| POST   | `/api/v1/auth/logout`  | `{refreshToken}`；204，撤销当前会话                                                    |
| GET    | `/api/v1/me`           | 200，`id`、`username`                                                                  |
| GET    | `/api/v1/items`        | 必填 `limit` 1–100、`offset` ≥0；200 `{items,total,limit,offset}`；createdAt / id 降序 |
| POST   | `/api/v1/items`        | 必填 `title`、`completed`；201 Item，并返回 Location                                   |
| GET    | `/api/v1/items/{id}`   | 200 Item；非法 UUID 400                                                                |
| PATCH  | `/api/v1/items/{id}`   | 必填正整数 `version`，至少提供 title / completed 之一；200 更新后的 Item               |
| DELETE | `/api/v1/items/{id}`   | 必填正整数 query `version`；204，无响应体                                              |

Item 字段：`id`、`title`、`completed`、`version`、`createdAt`、`updatedAt`。title 仅裁剪首尾 ASCII 空格 / TAB / CR / LF，裁剪后 1–240 UTF-8 字节；不把字符数当字节数。version 初始 1，成功更新加 1。PATCH 不允许 null；显式 null 与缺字段分别处理。

列表和写入始终受当前 owner 限制；不存在或属于他人的 ID 都返回 404。更新 / 删除在事务中锁定本人资源，再检查 version，过期版本返回 409；失败不改变数据。重复删除为 404。分页计数和结果使用同一数据库快照。

错误使用 `application/problem+json`（RFC9457）：`type`、`title`、`status`、`code`、`detail`、`instance`、`requestId`，可附 `errors: [{field,code}]`。code 为稳定机器值。状态涵盖 400 非法输入、401 无效认证、403 权限不足、404、405、409 版本冲突、413 请求过大、503 依赖故障、500 内部错误。认证失败使用 WWW-Authenticate；不返回 SQL、token 或内部异常正文。所有响应包含 X-Request-Id。

## 认证与语言体验

Rust 管理本地账号，用户名为 1–64 个 ASCII 字母、数字、点、下划线或连字符，创建时密码为 8–128 UTF-8 字节，使用 Argon2id 散列入库。账号通过 create-user 显式创建，首版不增加注册或找回密码。登录返回 user.id / username、15 分钟 HS256 access JWT 和 30 天不透明 refresh token；JWT_SECRET 至少 32 字节且仅在后端配置。JWT 固定校验 issuer / audience / exp / sub / sid；业务请求只接受 Bearer access token。这是直接密码认证，不是完整 OAuth 2.0 授权服务器。

Web access token 仅驻留内存，refresh token 存当前标签页 sessionStorage，刷新页面通过 refresh 接口轮换恢复，关闭标签页需重新登录。移动端 refresh token 使用 Keystore / Keychain，密码不持久化。数据库只保存 refresh token 的 SHA-256 散列；原子轮换，同一个旧 token 并发刷新只允许一次成功。客户端串行刷新；非幂等业务写入不自动重试。退出撤销当前会话，access JWT 与 refresh token 均失效，并清除本端身份和数据，不影响其他设备会话。

所有 UI、校验、错误、主题 / 语言选项和无障碍标签进入本端语言资源。数字、日期、复数按 locale 格式化，用户 Item title 保持原文。Android 使用原生资源和应用语言；iOS 使用 String Catalog。Next locale 路由涵盖公共和业务页面，登录表单随 locale 翻译。Rust 返回稳定 code 及按 Accept-Language 选择的安全文案。

UI 为简洁任务列表，显示登录、个人内容、操作与可行动错误。Web 在 360 / 768 / 1440 px 可使用、无正文横向滚动，键盘焦点可见；原生支持字体缩放和无障碍标签。提交时禁用重复操作，列表修改失败保留输入并可重试。

## 非功能与交付

- 依赖通过本端锁文件和工具链声明固定；手写代码严格类型、短函数、简单层级，生成代码保留上游风格。
- 后端配置缺失 / 初始化失败立即明确报错；外部读取有限重试并 warning，运行期数据库故障为 503。日志不记录 token、密码、完整用户输入。
- 性能目标：本地 release API，1,000 条 Item、并发 5、60 秒，CRUD p95 ≤300 ms、非预期错误为 0；记录硬件与原始结果。
- 验证最低系统和当前稳定系统；最低平台不随编译 SDK 升级自动上调。
- 交付 SPA 静态产物与原生 Nginx 配置示例、Next standalone Node 产物、Rust release 二进制、Android APK 和未签名 release 产物、iOS Simulator 构建。开发依赖使用原生 PostgreSQL。生产 URL 是待配置公开值，真实生产联调需可用服务，不能以占位域名宣称通过。
- 发布前完成三项 AI 维护演练：增加字段、增加筛选参数、修改页面交互；每项有独立补丁 / 真实验证证据，演练后恢复首版契约或明确记录纳入的范围。

不包含产品 AI、复杂 RBAC、支付 / 上传 / 推送、离线同步、Next 私有 SSR / BFF、PWA、跨端共享 UI、脚手架 CLI、CI/CD、云部署或商店签名上架。无自动 Git commit / push。
