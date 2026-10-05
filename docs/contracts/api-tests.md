# v0.1.0 API 验收契约

输入 / 输出规范见 [requirements](../v0.1.0/requirements.md)，机器规范见 [openapi.json](openapi.json)。Rust 的 DTO / 路由是导出来源，各端快照只能通过同步更新。

| 范围       | 真实验证                                                                     | 实现位置                               |
| ---------- | ---------------------------------------------------------------------------- | -------------------------------------- |
| 身份与登录 | Rust 用户名密码、JWT / refresh session，重复 me 稳定身份                     | backend/tests/support、api_integration |
| 隔离       | 两个账号的 GET / PATCH / DELETE 越权均 404                                   | api_integration                        |
| 并发       | 同版本两个 PATCH 只一个 200，另一个 409                                      | api_integration                        |
| JWT        | audience、scope、过期、篡改签名、ID token、缺 / 非法 token                   | api_integration                        |
| 校验       | 空 / Unicode 超字节 title、未知字段、null、分页上下限、非法 UUID、请求体大小 | 单元稳定转换与 api_integration         |
| CRUD       | 创建 Location / version1、读取、更新、删除 / 重复删除、时间格式              | api_integration                        |
| 路由       | 未知路由404、method405、Problem Details media type / requestId               | api_integration                        |
| 依赖与性能 | 数据库中断503、release 1,000条 / 并发5 / 60秒                                | 发布前独立记录                         |

`cargo test` 的 ignored 测试不计为通过。实际执行需要启动服务，明确设置 TEST_API_URL / JWT_SECRET；日志不输出 token。
