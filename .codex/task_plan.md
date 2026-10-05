# v0.1.0 全面审查与实施计划

更新时间：2026-10-05（Asia/Shanghai）。五端实现与本地验收已完成。历史文档保存在 `.codex/archive/2026-10-04`。

## 授权和约束

- 优化需求和方案，清理旧内容，实现五个独立项目并完成本地验证。
- SPA / Next 各用 npm；无 JS workspace、pnpm、Cursor 规则。
- Android API 34+ / Kotlin；iOS 18+ / Swift；双环境使用 flavor / XcodeGen + CocoaPods。
- 全端中英文、主题、OIDC PKCE、同一 Rust API / PostgreSQL。
- 无自动提交、推送或部署。缓存和临时文件在 `.codex`。不自研 Python 工程安装或生成包装。

## 阶段和验收

1. [complete] P0 重写当前需求、架构、交付计划；验证兼容工具链，清理历史页面。
2. [complete] P1 Rust / PostgreSQL / OIDC / OpenAPI：真实集成、Clippy、Release 性能（p95 56.723ms / 8150 requests / errors 0）、真实数据库中断恢复通过。
3. [complete] P2 独立 SPA / Next：构建、lint、typecheck、3+4 项真实浏览器测试通过，me、刷新失败清理、分页和故障恢复已完成。
4. [complete] P3 Android / iOS：iOS18.6 / 27.0各2项真实测试通过；双环境和独立复制通过；Android14 / 17完整真实流程均通过，最终Espresso3.7最低系统复核通过。
5. [complete] P4 规则、模板、契约薄脚本、五端独立复制、三项维护演练已通过；三端容器构建与真实Smoke通过，最终审查通过。

## 充分性检查

- 接口：health / ready / me / items；UUID、UTC、camelCase、分页、乐观锁、RFC9457。详见 requirements.md。
- 风险：SDK 兼容、OIDC 回调和 issuer、原生依赖体积、翻译与契约漂移。
- 验证：实际构建、真实 PostgreSQL / Keycloak、浏览器与模拟器 Smoke。未执行的检查不算通过。
- sequential-thinking、ACE、shrimp 及 writing-plans / verification-before-completion skills 不可用。使用文件计划、显式分析、真实命令和报告，不声称调用不存在的能力。
- brainstorming 设计确认已满足；planning-with-files 管理过程；前端使用 Frontend Skill / Frontend Design。

## 当前发现

- 初始仅 LICENSE 被跟踪，各应用目录为空；已有验证仅覆盖文档与最小 XcodeGen / CocoaPods 双环境。
- TS7 与 ESLint 类型插件 peer 不兼容；TS6.0.3 + Hey API0.99 的 peer 范围兼容，需编译验证。

## 错误与处理

- 计划补丁先后因缺少 Begin Patch 及同一路径 Delete/Add 被拒绝，未修改文件；改用直接完整写入纠正。

## 2026-10-05 后续配置清理

执行用户要求：各端独立 .gitignore，清理部署配置与说明。详细临时任务记录见 gitignore-cleanup-plan.md；产品来源仍为 docs/v0.1.0/delivery-plan.md。

## 2026-10-05 文档语言调整

1. [complete] 收集根与五端规则、README、当前需求与交付计划；保存修改前快照。
2. [complete] 英文 README / AGENTS，新增中文 README 与语言导航。
3. [complete] Markdown / 链接 / 命令与约束一致性 / 空白验证，生成审查。

验收契约和风险见 context-scan.json。任务仅调整文档，不改变运行代码。无 plan2go；产品计划继续使用 docs/v0.1.0/delivery-plan.md。

错误与恢复：磁盘不足连续三次写入失败，按用户规则请求释放空间；检测约12GiB可用后恢复。一次补丁匹配错误被拒绝，依据原文生成补丁纠正。中文正文对照初次误计表格分隔线宽度，忽略格式差异后通过。Markdown lint默认MD013与原文长段落风格冲突，本任务仅关闭MD013。记录脚本一次stdin编码错误，改用apply_patch完成。所有交付检查已通过。

## JWT authentication refactor

1. [complete] Review context and user intent; define API, storage and acceptance contract.
2. [complete] Backend local accounts, Argon2id, JWT/session endpoints and OpenAPI.
3. [complete] SPA/Next login forms, token refresh and cache cleanup; 4 / 5 real browser tests passed.
4. [complete] Android/iOS native login and secure refresh storage; remove AppAuth. iOS 18.6 real UI tests passed; Android devDebug/prodRelease/test APK/lint passed; native UI verification blocked by host disk and system_server failure.
5. [complete] Update current requirements, architecture, plan, English/Chinese READMEs and English AGENTS.
6. [blocked] Final review and all executable checks complete. Rust Release + Smoke passed; Android native UI remains blocked by host disk / current-system crash. Full release gate is not complete.

Use docs/v0.1.0/delivery-plan.md as the product plan; local work records only in .codex. No plan2go. Preserve existing user data; tests use a new isolated local database. No commit/push.
