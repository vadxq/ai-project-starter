# v0.1.0 实施与验收计划

状态：2026-10-21 的 Next / Backend 整体换新、四端契约适配及本轮本地验收已完成。两库、Web、Android API 34 / 37.1、iOS 18.6 / 27.0 均有真实运行证据；开发依赖告警与发布边界见 [本轮报告](../verification/2026-10-21-upstream-refresh.md)。本计划是唯一的跨端执行来源。

## 2026-10-21 上游换新（已完成）

用户已批准整体换新，Backend 冲突项以上游为准。新基线：Next `81e089fbcc06a237353e2107bd578d20567dfba3`，Backend `6df373d98b573e8155cd1d1931065aa16af81598`。

- [x] U-01 固定基线、同步规范与配置迁移。
- [x] U-02 导入 Backend 基座，补齐 PostgreSQL / MySQL 的 JWT、会话、Item 事务及上游响应协议。
- [x] U-03 两库真实验证，导出 OpenAPI。
- [x] U-04 导入新版 Next 组织、组件、主题与国际化，接入业务与 standalone，去除 PWA。
- [x] U-05 四端同步新契约，适配响应包装、错误与毫秒时间戳。
- [x] U-06 五端本地构建、真实联调、独立复制及文档检查。
- [x] U-07 记录实际结果与未完成项。

本次为破坏性协议升级：成功 `{code,message,data}`，错误 `{code,message,details?}`，毫秒时间戳；JSON rejection 遵循上游 400 / 413 / 415 / 422；配置使用 `APP_*`；保留双数据库、可选数据库启动、lazy pool 与上游 CORS。认证、权限和版本规则作为业务模块接入。当前约束见 [需求](requirements.md)，实现组织见 [架构](architecture.md)。

U-01–U-07 已完成；实际命令、结果、环境问题及已知依赖告警见 [本轮验证](../verification/2026-10-21-upstream-refresh.md)。本轮没有执行签名、远程部署或新增 AI 产品演练。

## 直接验证契约

- Backend：fmt、Clippy、build、迁移重复执行、离线导出；真实 HTTP / DB / Auth 验证正常 CRUD、Unicode 字节边界、空值 / 未知字段、分页、无效 UUID、缺 token / issuer / audience / 会话 / 期限、跨用户 404、并发 409、未知路由 / method、数据库中断 503。
- Web：分别 npm ci、全工程 lint / format / typecheck / production build；Playwright 使用真实 Rust 密码登录及 API；新增 / 修改 / 删除、过期 / 登出、分页、错误 / 重试、双语与主题、360 / 768 / 1440、键盘及深链刷新；Next 公共 SSR 与双语登录表单。
- Android：Wrapper、锁定依赖、devDebug / prodRelease 产物、四变体环境 / HTTP 限制、API 34 与当前稳定 Emulator、真实认证及 CRUD；语言 / 主题 / 旋转不重复写入。
- iOS：单独复制后 dev → prod → dev 各执行 XcodeGen + pod install；Debug / Release 的 API / bundle ID / Pods 一致；Swift 6 Simulator build；iOS 18 与当前稳定运行；真实认证 / CRUD、双语与主题。
- Contract：后端重新导出与根 / 各端快照一致；生成结果与仓库一致；TypeScript / Kotlin / Swift 编译；检查不能写回源码。
- 独立性：每个工程复制到隔离目录，用本端命令安装和构建，不依赖父目录配置或兄弟源码。
- Production：SPA 原生 Nginx fallback 与 API 错误语义；Next standalone Node 启动；Rust release API / migrate 二进制；Android unsigned release；iOS 无签名 Simulator。
- Performance：release API、1,000 Items、并发 5、60 秒，p95 ≤300 ms、非预期错误 0；记录机器和耗时。认证不计入 CRUD 时延。

产品完整发布仍需需求中的字段、筛选、UI 交互三项 AI 维护演练，并保留独立补丁和验证；本次基座换新未执行这些演练，不将其计为通过。签名、部署和商店上架不在本轮范围。

## 验收证据

`.ailog/runs/<任务>/` 保存原始日志和临时计划，`docs/verification` 保存可分享结果：时间、工具版本、命令、退出码、验证范围和未完成项。生成 workspace 不等于编译成功，编译成功不等于真实认证成功；生产配置检查不等于真实生产联通。验收只适用于报告列出的版本和范围。

根验证脚本必须显式传入 `quick` 或 `full`。quick 是静态/构建检查；full 包含当前目标的真实运行，backend full 缺 TEST_API_URL / TEST_DATABASE_URL / APP_AUTH__JWT_SECRET 在构建前失败。两者都不能单独代表完整跨端发布；矩阵与完整流程见 [AI 工作流](../ai-workflows.md)。

多端 SDK、模拟器和构建产物会占用较大磁盘，构建时检查资源并分阶段回收本任务缓存。缺少必要 runtime 或远端依赖不可用时，明确记录未执行项与阻碍。
