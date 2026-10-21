# 架构说明

## 分层

项目保留单服务、按领域组织的结构：

- `api`：HTTP 路由、中间件、统一响应与错误。
- `domain`：业务 DTO、service、repo、handler、routes。
- `platform`：配置、数据库连接池、日志与退出信号。
- `bootstrap`：从配置组装领域依赖。

当前包含 health、auth 与 items 领域；auth / items 需要数据库和 `APP_AUTH__JWT_SECRET`，缺失时返回 503，公开探针仍可运行。

## 启动与退出

`main → Settings::load → init_tracing → build_state → router → axum::serve`。

配置读取当前目录的 .env 与进程环境快照，交给 config crate 反序列化。读取配置不写进程环境，测试可显式传入两组配置。IP、端口、布尔值、正整数及日志过滤器在启动时校验。默认没有数据库连接。

配置数据库后按 URL 创建 SQLx lazy pool，首次连接失败不会丢弃连接池；后续探针继续尝试，数据库恢复后可返回就绪。Ctrl+C / SIGTERM 触发 Axum 优雅关闭，随后关闭连接池。

## 双数据库

`DatabasePool` 是 `Postgres(PgPool)` / `MySql(MySqlPool)` 枚举，URL 协议决定分支：

- `postgres://`、`postgresql://`：PostgreSQL。
- `mysql://`：MySQL。
- 未配置：只启动 HTTP。
- 协议错误或 URL 无法解析：启动失败，错误不回显凭证。

探针通过共同的 `ping` 行为执行 `SELECT 1`；业务 repo 对两种数据库分别编写 SQL，避免隐藏方言差异。MySQL 使用 TLS，无需可选 RSA 认证依赖。

`migrations/postgres/` 和 `migrations/mysql/` 保持相同版本与业务语义。显式运行 `cargo run --locked --bin migrate`，SQLx 选择对应目录、加迁移锁、记录版本与校验和。HTTP 启动不执行 DDL。MySQL DDL 可能隐式提交，迁移失败时需检查状态后修复。

## HTTP 契约

成功返回 `{code, message, data}`，失败返回 `{code, message, details?}`，code 是 HTTP 状态码字符串。

AppJson 保留 Axum 原始状态：JSON 语法错误 400、超过 body 上限 413、Content-Type 缺失/错误 415、数据结构不匹配 422。

ServiceBuilder 按 `生成请求 ID → trace → 响应传播 ID → CORS → handler` 接收请求。客户端提供的请求 ID 保留；新生成的 ID 会出现在成功、错误与 CORS 预检响应中。trace 记录方法、路径、ID、状态与耗时，不记录查询串或请求体。

`/ready` 的超时覆盖连接池获取与 SQL 查询全程。数据库未配置、不可用和超时均返回 503；详细错误进入结构化日志。
