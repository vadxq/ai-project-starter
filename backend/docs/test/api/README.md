# HTTP 公共行为契约

测试入口：tests/router.rs。探针为公开 GET 请求，不改变业务数据。

| 场景                        | 状态 | 响应约束                          |
| --------------------------- | ---- | --------------------------------- |
| GET /health                 | 200  | code="200"，data.status="ok"      |
| GET /ready，未配置数据库    | 503  | message="database not configured" |
| 未知路由                    | 404  | message="route not found"         |
| 已知路由使用错误方法        | 405  | message="method not allowed"      |
| JSON 语法错误               | 400  | 统一错误 JSON                     |
| JSON 超过 body 上限         | 413  | 统一错误 JSON                     |
| JSON Content-Type 缺失/错误 | 415  | 统一错误 JSON                     |
| JSON 字段类型或必填字段错误 | 422  | 统一错误 JSON                     |

错误 JSON 的 code 与 HTTP 状态一致，JSON rejection 的 details.source_status 也一致。成功响应保留 code、message、data 结构。

所有根路由响应，包括 404、405、503 和 CORS 预检，均包含 x-request-id。客户端已有 ID 保留；缺失时生成 UUID。请求日志包含同一 ID、方法、路径、状态与耗时，不记录请求体或 URL 查询串。
