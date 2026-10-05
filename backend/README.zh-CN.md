# Rust API

[English](README.md) | 简体中文

Rust 1.99 / Axum / SQLx / PostgreSQL 16。提供用户名密码登录、JWT access token、可轮换的 refresh session、个人 Item CRUD、`/health` 和 `/ready`。本目录可独立复制。

## 安装与运行

安装并启动原生 PostgreSQL 16，通过数据库管理员连接创建专用账号和数据库：

```sh
createuser --pwprompt starter
createdb --owner=starter starter
cp -n .env.example .env
```

配置 `.env` 中的 DATABASE_URL、BIND_ADDRESS、CORS_ORIGINS 和 JWT_SECRET。用 `openssl rand -hex 32` 生成随机签名密钥，填入 JWT_SECRET；示例占位值会被拒绝。已有服务占用 5432 时，为本工程使用独立端口。

```sh
cargo run --locked --bin migrate
STARTER_USER_PASSWORD=starter-password cargo run --locked --bin create-user -- alice
STARTER_USER_PASSWORD=starter-password cargo run --locked --bin create-user -- bob
cargo run --locked
```

账号必须显式创建，不自动注入默认账号。上述密码只用于本地开发。create-user 要求唯一用户名：1–64 个 ASCII 字母、数字、点、下划线或连字符；密码为 8–128 UTF-8 字节。不会覆盖已有账号。

| 配置         | 说明                                         |
| ------------ | -------------------------------------------- |
| DATABASE_URL | 必填 PostgreSQL URL，仅服务端使用            |
| BIND_ADDRESS | 必填监听地址，例如 127.0.0.1:8080            |
| JWT_SECRET   | 必填随机签名密钥，至少 32 字节，仅服务端使用 |
| CORS_ORIGINS | 必填逗号分隔的精确 Web origin                |
| RUST_LOG     | tracing filter，例如 starter_api=info        |

启动 API 前显式迁移。追加的本地认证迁移保留已有 identity 和 Item；原外部身份不会自动获得密码，需要显式创建本地账号。不再需要外部认证服务。

## 认证接口

`POST /api/v1/auth/login` 接收 `{username,password}`，返回 `{accessToken,refreshToken,tokenType,expiresIn,user}`；user 包含 `id` 和 `username`。密码使用 Argon2id。access token 为有效期 15 分钟的 HS256 JWT，校验 issuer、audience、期限、用户和会话。

`POST /api/v1/auth/refresh` 接收 `{refreshToken}`，返回轮换后的 token 对。refresh token 是随机不透明值，数据库只保存 SHA-256 散列；会话有效期 30 天。每个 refresh token 只能使用一次，客户端必须串行刷新。

`POST /api/v1/auth/logout` 接收 `{refreshToken}`，撤销当前会话，access JWT 和 refresh token 同时失效。私有接口使用 `Authorization: Bearer <accessToken>`。生产必须使用 HTTPS。这是账号密码加 JWT 认证，不是完整的 OAuth 2.0 授权服务器。

## 验证与契约

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release --locked --bins
cargo run --locked --example export-openapi
```

导出只读 DTO / 路由，不连接服务。Item title 经 ASCII 首尾裁剪后为 1–240 UTF-8 字节；列表必填 limit 1–100、offset ≥0；PATCH / DELETE 必填 version。读写受 owner 限制，他人资源返回 404，旧版本返回 409。错误为 RFC9457 Problem Details，包含稳定 code 和 requestId。

启动真实 PostgreSQL 和 API，创建 alice / bob 后执行：

```sh
TEST_API_URL=http://localhost:8080 JWT_SECRET='<same secret as the running API>' \
cargo test --test api_integration -- --ignored
```

测试覆盖真实密码验证、JWT、数据库和 HTTP：错误凭证、签名 / issuer / audience / 期限、刷新轮换、退出、CRUD、用户隔离和版本冲突。普通 cargo test 的 ignored 不代表这些测试通过。

生产产物为 `target/release/migrate`、`target/release/create-user` 和 `target/release/starter-api`。配置服务端环境、迁移、用实际密码创建账号，再启动 API。生产不用本地示例密码，JWT_SECRET 不进入客户端。
