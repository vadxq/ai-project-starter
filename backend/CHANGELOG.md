# 更新记录

## 未发布

### 变更

- 以上游 `6df373d98b573e8155cd1d1931065aa16af81598` 重建后端基础设施，采用 Axum 0.8.9、SQLx 0.9、APP_* 配置、lazy pool、请求追踪与 stable 工具链。
- **破坏性变更**：成功响应改为 code/message/data，错误改为 code/message/details，时间改为 epoch 毫秒整数；客户端须更新契约和数据解析。
- 将认证和 Item CRUD 适配至 PostgreSQL / MySQL，两库保持刷新轮换、撤销、用户隔离和版本事务语义。
- 配置改用 APP_DATABASE__URL、APP_HTTP__BIND_ADDR、APP_AUTH__JWT_SECRET；旧配置名不再读取。迁移显式执行，PostgreSQL 已应用迁移保持校验和。
- 新响应通过 OpenAPI 离线导出；两库迁移、真实认证 / CRUD、故障恢复和跨端本地验证已通过，实际范围与告警记录于工程交付报告。
