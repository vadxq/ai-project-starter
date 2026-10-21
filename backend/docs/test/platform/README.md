# 配置与双数据库契约

## 范围

测试：tests/config.rs、tests/database.rs；数据库测试显式提供 TEST_DATABASE_URL。

配置优先级为默认值 < 当前目录 .env < 进程环境；不修改全局环境。缺失 .env 合法，损坏文件报错。默认数据库未配置，进程中的空 URL 可禁用 .env 中的数据库。

## 输入输出

- HTTP 地址解析为 SocketAddr；连接数、超时必须为正整数。
- 布尔值、日志过滤器与 http / database / auth / telemetry 分组内的未知字段错误时启动失败；其他工具的 APP_* 分组忽略。
- postgres / postgresql URL 创建 PgPool；mysql URL 创建 MySqlPool。
- URL 解析失败必须包含配置键名，不回显密码。
- lazy pool 创建不要求服务器已经启动；后续探针可重新尝试。

## SQL 与迁移

PostgreSQL 和 MySQL 分别读取 migrations/postgres、migrations/mysql。相同版本必须代表相同业务变更。迁移成功写入 SQLx 历史，重复执行不重复应用；已执行文件禁止修改。

每种数据库当前有三条迁移：初始化 identities / items、添加 accounts / auth_sessions、补充 schema 注释。`SELECT 1` 只用于就绪探针。数据库测试显式执行全部迁移，并通过临时表验证唯一键错误识别，不删除已有库或业务表。

| 场景                  | 预期                             | 验证                     |
| --------------------- | -------------------------------- | ------------------------ |
| 默认配置              | 无数据库、HTTP 可启动            | config 默认测试          |
| .env 与进程同名键     | 进程优先                         | config 默认测试          |
| 非法配置或 URL        | 启动报错且密码不泄露             | config 默认测试          |
| 两种 URL              | 创建对应 lazy pool               | config 默认测试          |
| 首次/重复迁移         | 三条迁移成功，重复执行不新增记录 | 两种数据库 ignored 测试  |
| 重复主键              | is_unique_violation 为 true      | 两种数据库 ignored 测试  |
| 连接池耗尽            | ready 在设定总时限返回 503       | 两种数据库 ignored 测试  |
| 正常数据库/连接池关闭 | ready 分别为 200/503             | 两种数据库 ignored 测试  |
| 数据库停止后恢复      | HTTP 持续存活，ready 503 → 200   | 两种数据库真实进程 smoke |
