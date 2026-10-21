# 本地测试

## 默认验证

```bash
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked --release --bins
cargo audit
```

默认测试不需要数据库：

- `tests/config.rs`：配置优先级、无全局环境修改、类型边界、URL 选择和凭证保护。
- `tests/router.rs`：路由、统一响应、JSON 拒绝状态、请求 ID、CORS 预检。
- `tests/db_sql.rs`：两种占位符的稳定 SQL 片段转换。

## 真实数据库测试

`tests/database.rs` 使用 `#[ignore]` 和 `#[serial]`，要求显式的 `TEST_DATABASE_URL` 指向专用测试库。测试不会自动删除或重建数据库，只执行 migration、查询迁移记录并在连接中创建临时表。

测试覆盖迁移首次执行/重复执行、唯一键冲突识别、连接池耗尽时探针超时、正常 ready 和连接池关闭后的 503。

先启动需要的开发数据库，再创建独立测试库。PostgreSQL：

```bash
docker-compose --profile postgres up -d --wait
docker-compose exec -T postgres createdb -U starter starter_test
TEST_DATABASE_URL=postgres://starter:starter@localhost:5432/starter_test \
  cargo test --locked --test database postgres_ -- --ignored
```

MySQL：

```bash
docker-compose --profile mysql up -d --wait
docker-compose exec -T mysql sh -c 'MYSQL_PWD="$MYSQL_ROOT_PASSWORD" mysql -u root -e "CREATE DATABASE starter_test CHARACTER SET utf8mb4; GRANT ALL ON starter_test.* TO starter;"'
TEST_DATABASE_URL='mysql://starter:starter@localhost:3306/starter_test?ssl-mode=required' \
  cargo test --locked --test database mysql_ -- --ignored
```

每种数据库分别运行对应过滤器；不要把同一个 URL 交给两种测试。测试库初始化命令只需执行一次，重复验证可直接运行 Cargo。

业务 HTTP 验证需先迁移、创建 alice / bob 并启动 API，然后设置 TEST_API_URL 与 APP_AUTH__JWT_SECRET，运行 cargo test --locked --test api_integration -- --ignored。本机配置代理时为 localhost / 127.0.0.1 设置 NO_PROXY / no_proxy，确保连接本地真实服务。并发刷新、会话撤销、owner 隔离、版本冲突、整数毫秒时间和 JSON rejection 必须分别在两库运行。

MySQL TLS 示例只要求加密；部署时按证书配置 `ssl-mode=verify-identity` 及 `ssl-ca`。两套 SQL 需在同一组业务场景下分别验证。

## 真实 HTTP 与恢复

启动服务后检查：

```bash
curl -i http://localhost:8080/health
curl -i http://localhost:8080/ready
curl -i -H 'x-request-id: local-smoke' http://localhost:8080/missing
```

验证请求 ID 与统一 JSON。对专用开发实例执行停止/启动，确认服务进程持续运行，`/health` 始终 200，`/ready` 从 503 恢复到 200。两种数据库都要执行。

## 验证级别

按影响选择 Level 0–4，具体入口见 AGENTS.md。交付记录实际命令、数据库版本、结果和限制。临时日志与独立实例数据放 `.ailog/`，禁止依赖人工验证或远程 CI 代替本地证据。
