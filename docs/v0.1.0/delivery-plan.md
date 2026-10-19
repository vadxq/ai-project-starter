# v0.1.0 实施与验收计划

状态：P0–P4 基础实现已通过历史验收；JWT 登录代码及本次 AI 工作流调整已完成；移动端真实运行矩阵仍未完整验收，完整跨端发布门禁尚未通过。更新：2026-10-19。本计划是当前唯一的跨端实施计划；各次证据见 [文档入口](../README.md)。

## 阶段与原子任务

| 阶段 | 任务                                                                                                    | 输出与依赖                                         |
| ---- | ------------------------------------------------------------------------------------------------------- | -------------------------------------------------- |
| P0   | T-01 审查约束和参考；T-02 清理文档；T-03 验证工具链 / 生成器                                            | 当前需求、架构、兼容版本、生成编译证据             |
| P1   | T-04 后端配置 / 日志；T-05 迁移；T-06 本地账号与 JWT；T-07 CRUD；T-08 导出契约；T-09 集成测试           | 真实 PostgreSQL / JWT、完整错误 / 隔离 / 版本行为  |
| P2   | T-10 SPA 基础；T-11 SPA 业务；T-12 Next SSR / locale；T-13 Next 业务；T-14 Web 验证                     | 两个独立工程、锁文件、真实登录 CRUD 和双语 / 主题  |
| P3   | T-15 Android 双环境；T-16 Android 业务；T-17 iOS 双环境；T-18 iOS 业务；T-19 原生验证                   | 原生 SDK、JWT、CRUD、多语言，最低 / 当前系统 Smoke |
| P4   | T-20 AI 规则 / 模板；T-21 契约与验证脚本；T-22 生产输出；T-23 隔离复制；T-24 AI 维护演练；T-25 最终验收 | 本地可复现证据、当前版本完整报告                   |

按依赖顺序推进，各端构建可以独立执行。全部完成前不得把版本标记为发布通过。

## 直接验证契约

- Backend：fmt、Clippy、build、迁移重复执行、离线导出；真实 HTTP / DB / Auth 验证正常 CRUD、Unicode 字节边界、空值 / 未知字段、分页、无效 UUID、缺 token / issuer / audience / 会话 / 期限、跨用户 404、并发 409、未知路由 / method、数据库中断 503。
- Web：分别 npm ci、全工程 lint / format / typecheck / production build；Playwright 使用真实 Rust 密码登录及 API；新增 / 修改 / 删除、过期 / 登出、分页、错误 / 重试、双语与主题、360 / 768 / 1440、键盘及深链刷新；Next 公共 SSR 与双语登录表单。
- Android：Wrapper、锁定依赖、devDebug / prodRelease 产物、四变体环境 / HTTP 限制、API 34 与当前稳定 Emulator、真实认证及 CRUD；语言 / 主题 / 旋转不重复写入。
- iOS：单独复制后 dev → prod → dev 各执行 XcodeGen + pod install；Debug / Release 的 API / bundle ID / Pods 一致；Swift 6 Simulator build；iOS 18 与当前稳定运行；真实认证 / CRUD、双语与主题。
- Contract：后端重新导出与根 / 各端快照一致；生成结果与仓库一致；TypeScript / Kotlin / Swift 编译；检查不能写回源码。
- 独立性：每个工程复制到隔离目录，用本端命令安装和构建，不依赖父目录配置或兄弟源码。
- Production：SPA 原生 Nginx fallback 与 API 错误语义；Next standalone Node 启动；Rust release API / migrate 二进制；Android unsigned release；iOS 无签名 Simulator。
- Performance：release API、1,000 Items、并发 5、60 秒，p95 ≤300 ms、非预期错误 0；记录机器和耗时。认证不计入 CRUD 时延。
- AI：字段、筛选、UI 交互三项演练分别保留补丁和验证；演练不悄悄扩大首版范围。

## 验收证据

`.ailog/runs/<任务>/` 保存新原始日志和临时计划，`docs/verification` 保存可分享结果：时间、工具版本、命令、退出码、验证范围和未完成项。历史报告中的旧路径与已撤下配置保留历史含义。生成 workspace 不等于编译成功，编译成功不等于真实认证成功；生产配置检查不等于真实生产联通。

根验证脚本必须显式传入 `quick` 或 `full`。quick 是静态/构建检查；full 包含当前目标的真实运行，backend full 缺 TEST_API_URL / JWT_SECRET 在构建前失败。两者都不能单独代表完整跨端发布；矩阵与完整流程见 [AI 工作流](../ai-workflows.md)。

各项通过状态记录在 [本地验收记录](../verification/2026-10-05-v0.1.0.md)。多端 SDK、模拟器和构建产物会占用较大磁盘，构建时检查资源并分阶段回收本任务缓存。若本机缺少必要 runtime 或远端依赖不可用，先完成可执行部分，再明确记录阻碍，不能虚构完整通过。

## 当前认证改造任务

- A-01 Rust 本地账号、Argon2id、JWT、轮换刷新与退出撤销；追加迁移保留原数据。
- A-02 OpenAPI 同步和各端生成；移除外部认证依赖和回调配置。
- A-03 SPA / Next 双语登录表单、恢复、失效、缓存；真实浏览器验证。
- A-04 Android / iOS 原生登录、安全存储、双环境；本地构建和可用模拟器验证。
- A-05 英文 README / AGENTS 与中文文档同步，记录本次验收证据。

认证验收覆盖错误密码与未知账号一致401、字段400、JWT签名/issuer/audience/期限、旧refresh重放与并发轮换、退出后access/refresh失效、用户隔离与CRUD、刷新页面、移动恢复、双语/主题和重复操作。历史登录验收不能替代本次结果，当前结果见 [JWT 改造验收](../verification/2026-10-05-jwt-auth.md)。

当前 A-01 / A-02 / A-03 / A-05 已完成；A-04 的两端原生实现、Android 构建 / lint 与 iOS 18.6 完整运行已通过，Android 真实运行及当前系统矩阵仍未完成。Rust Release 与真实认证 Smoke 已通过。

## 2026-10-19 架构与 AI 工作流调整

- W-01 显式 quick/full 验证、真实集成前置条件与失败退出。
- W-02 统一 JSON rejection，认证请求超限返回 413；同步 Rust 契约、客户端快照和生成结果。
- W-03 根/各端规则和第一轮 AI 流程配置；最终示例范围以本节后续精简调整为准。
- W-04 过程文件从交付树移除并归档到 `.ailog`；新日志/报告/缓存统一到 `.ailog`，补充双语入口与当前状态说明。
- W-05 本地定向验证与本次证据汇总；本次改动不自动补齐或关闭 A-04 的历史未完成矩阵。

W-01–W-05 已完成。Rust full、两端 Web 静态/构建和真实浏览器、Android lint/Debug/unsigned Release、iOS Simulator build、契约和 AI 配置静态检查通过。实际命令与范围见 [本次验收报告](../verification/2026-10-19-ai-workflows.md)。A-04 的完整移动运行矩阵仍未完成。

## 更新日志示例精简

- C-01 删除原契约/发布/验证 skills；根与五端仅提供创建/更新 changelog.md 的示例。
- C-02 删除全部 CLAUDE.md 和仓库 .codex 配置目录；临时记录继续使用 .ailog。
- C-03 同步 AGENTS、双语 README、需求与架构；执行 skill、文档和独立引用定向检查。

C-01–C-03 已完成。六份日志示例通过 skill 校验；33 份 Markdown 格式/lint、128 个本地链接、五端独立引用与两端 ESLint 检查通过。根和各端旧配置目录已移除，历史临时资料保留在忽略的 .ailog。本轮仅调整示例、规则和工具忽略配置，没有重跑应用构建或完整发布。使用方式见 [AI 工作流](../ai-workflows.md)。
