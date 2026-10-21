# Rust + Axum API

[English](README.md) | [简体中文](README.zh-CN.md)

以上游 rust-axum-starter 的 6df373d98b573e8155cd1d1931065aa16af81598 为基础，使用 Rust stable / edition 2024、Axum 0.8.9、SQLx 0.9，支持 PostgreSQL / MySQL。包含密码登录、JWT / 刷新会话、个人 Item CRUD、健康检查和离线 OpenAPI 导出。本目录可以独立复制。

## 本地启动

使用原生 PostgreSQL / MySQL，或选择上游 Compose profile：

```sh
cp -n .env.example .env
docker-compose --profile postgres up -d --wait
# 或 docker-compose --profile mysql up -d --wait
```

在 .env 配置 APP_DATABASE__URL。执行 openssl rand -hex 32 生成随机值，设置 APP_AUTH__JWT_SECRET，然后运行：

```sh
cargo run --locked --bin migrate
STARTER_USER_PASSWORD=starter-password cargo run --locked --bin create-user -- alice
STARTER_USER_PASSWORD=starter-password cargo run --locked --bin create-user -- bob
cargo run --locked
```

账号显式创建，不覆盖已有用户名。示例密码仅用于本地测试。API 启动不执行迁移。已有 PostgreSQL 迁移只移动至 migrations/postgres，版本和内容保持不变；MySQL 对应迁移位于 migrations/mysql。

## 配置

| 配置                          | 含义                                           |
| ----------------------------- | ---------------------------------------------- |
| APP_HTTP__BIND_ADDR           | 监听地址，默认 0.0.0.0:8080                    |
| APP_DATABASE__URL             | PostgreSQL / MySQL URL；空值只启动 HTTP 探针   |
| APP_DATABASE__MAX_CONNECTIONS | 正整数连接上限，默认 10                        |
| APP_DATABASE__TIMEOUT_SECS    | 获取连接 / 就绪探针总时限，默认 3 秒           |
| APP_AUTH__JWT_SECRET          | 至少 32 字节随机值，只在后端配置；认证业务必需 |
| APP_TELEMETRY__LOG_LEVEL      | tracing 过滤器，默认 info；RUST_LOG 优先       |
| APP_TELEMETRY__JSON           | JSON 日志，默认 false                          |

优先级为默认值、当前目录 .env、进程环境。配置读取不修改进程环境；非法值或已知分组中的未知字段会导致启动失败。旧 DATABASE_URL / BIND_ADDRESS / JWT_SECRET / CORS_ORIGINS 配置名不再读取。

数据库使用 lazy pool。无数据库或数据库故障时 /health 仍为 200，/ready 为 503，恢复连接后无需重启。认证依赖或密钥未配置时业务返回 503。CORS 遵循上游宽松策略。

## 接口与业务

除无响应体的 204 外，成功响应为 {code,message,data}，code 是实际 HTTP 状态码字符串；错误为 {code,message,details?}，业务 details.reason 提供稳定本地化键。Item 时间为 Unix epoch 毫秒整数。旧裸 DTO、Problem Details 与 RFC3339 协议已被替换。

JSON 语法错误 400、超限 413、Content-Type 缺失 / 错误 415、字段类型 / 缺字段 / 未知字段 422；业务校验 400。X-Request-Id 按上游规则生成与传播。

登录签发 15 分钟 HS256 JWT 和 30 天不透明 refresh token。JWT 校验 issuer、audience、exp、sub、sid 与数据库会话；数据库只存 refresh 散列。刷新在行锁事务中单次轮换，退出撤销同一会话的 access / refresh。业务请求使用 Bearer。

Item 列表必填 limit / offset；更新、删除必填 version。title 经 ASCII 裁剪后为 1–240 UTF-8 字节。所有读写限定 owner，他人资源返回 404；行锁后校验版本，冲突 409。分页计数与结果来自同一 repeatable-read 快照，两库语义一致。

## 本地验证

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked --release --bins
cargo run --locked --example export-openapi
```

真实数据库 / HTTP 测试默认 ignored。准备专用库，迁移、创建 alice / bob，并以同一密钥启动 API，再分别执行：

```sh
TEST_DATABASE_URL='<专用 PostgreSQL URL>' cargo test --locked --test database postgres_ -- --ignored
TEST_DATABASE_URL='<专用 MySQL URL>' cargo test --locked --test database mysql_ -- --ignored
TEST_API_URL=http://localhost:8080 APP_AUTH__JWT_SECRET='<API 使用的密钥>' cargo test --locked --test api_integration -- --ignored
```

本机配置代理时，HTTP 测试与性能验证需将 `NO_PROXY` 和 `no_proxy` 都设为 `localhost,127.0.0.1,::1`。

详见 [测试说明](docs/development/testing.md)。ignored 不代表通过。发布产物为 rust-axum-starter、migrate、create-user；迁移命令需要随附 migrations/ 并从对应目录结构运行。生产配置使用 HTTPS 和自己的凭证。

## 维护

阅读 [AGENTS.md](AGENTS.md)。独立复制时保留本端锁文件、语言资源、迁移、examples 与 .agents。临时记录放忽略的 .ailog。实际变更进入 [CHANGELOG.md](CHANGELOG.md)，不自动 commit / push。许可证：[Apache-2.0](LICENSE)。
