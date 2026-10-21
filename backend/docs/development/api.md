# API 开发说明

## 当前路由

- `GET /health`
- `GET /ready`
- `POST /api/v1/auth/login`、`POST /api/v1/auth/refresh`、`POST /api/v1/auth/logout`
- `GET /api/v1/me`
- `GET /api/v1/items`、`POST /api/v1/items`
- `GET /api/v1/items/{id}`、`PATCH /api/v1/items/{id}`、`DELETE /api/v1/items/{id}`

业务协议见 [认证与 Item 测试契约](../test/api/business.md)。使用 `cargo run --locked --example export-openapi` 离线导出完整定义。

## 统一响应协议

### 成功响应

除空响应体 204 外，成功接口统一返回：

```json
{
  "code": "200",
  "message": "success",
  "data": {}
}
```

约定：

- `code` 使用字符串形式的 HTTP 状态码
- `message` 当前固定为 `success`
- `data` 承载具体响应体

### 错误响应

所有错误接口统一返回：

```json
{
  "code": "400",
  "message": "invalid json body",
  "details": {}
}
```

约定：

- `code` 使用字符串形式的 HTTP 状态码
- `message` 是错误说明
- `details` 为可选字段
- JSON 解析错误、路由未命中、方法不允许也都走这套结构
- `AppJson` 保留 Axum 的状态：语法错误 `400`、body 超限 `413`、Content-Type 错误 `415`、字段结构错误 `422`
- 所有根路由响应包含 `x-request-id`，日志使用同一 ID 关联请求

## 新增领域接口

建议按领域组织代码：

```text
src/domain/<domain>/
├── dto.rs
├── handler.rs
├── mod.rs
├── repo.rs
├── routes.rs
└── service.rs
```

推荐顺序：

1. `dto.rs`：定义请求和响应结构
2. `service.rs`：实现业务规则
3. `repo.rs`：实现数据库访问
4. `handler.rs`：连接 extractor、service 和响应
5. `routes.rs`：挂载路由
6. `src/domain/mod.rs`：暴露领域模块
7. `src/api/router.rs`：合并领域路由

数据库查询在 repo 中按 `DatabasePool::Postgres` / `DatabasePool::MySql` 分别实现。PostgreSQL 使用 `$1` 占位符，MySQL 使用 `?`，值通过 `.bind()` 传入。可将两种方言的静态 SQL 分别保存于领域内的 `sql/postgres/`、`sql/mysql/`，通过 `include_str!` 加载；动态查询使用 `sqlx::QueryBuilder`。

每次数据结构变更同时新增 `migrations/postgres/<版本>_<描述>.sql` 和 `migrations/mysql/<版本>_<描述>.sql`。相同版本保持相同业务语义；两种数据库都执行真实测试。业务表和字段需要中文注释，PostgreSQL 使用 `COMMENT ON`，MySQL 使用 `COMMENT`。

## JSON 请求

需要统一 JSON 错误时使用 `AppJson<T>`：

```rust
use crate::api::response::AppJson;

pub async fn create_item(AppJson(req): AppJson<CreateItemRequest>) {
    // ...
}
```

## 时间字段

对外时间字段如果需要 13 位毫秒时间戳，可使用：

- `serialize_timestamp_millis`
- `serialize_option_timestamp_millis`

## health / ready

- `/health`：进程存活，不依赖数据库
- `/ready`：对所选 PostgreSQL / MySQL 执行 `SELECT 1`，超时或不可用时返回 `503`，恢复后无需重启服务
