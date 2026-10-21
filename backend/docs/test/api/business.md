# 认证与 Item 测试契约

入口：`tests/api_integration.rs`；使用真实 HTTP API、专用 PostgreSQL / MySQL、显式创建的 alice / bob 测试账号。两库运行同组场景，不能以普通测试中 ignored 用例代替真实执行。

## 认证

- login 接收 username / password；错误密码和未知账号均返回 401。Argon2id 校验成功返回 15 分钟 JWT 与 30 天 refresh token。
- refresh 接收 refreshToken；数据库只存哈希。行锁保证并发轮换只有一次成功，旧 token 重放返回 401。
- logout 接收 refreshToken，返回空 204；同会话 access / refresh 均撤销。
- 私有路由校验签名、issuer、audience、期限和持久化会话，未认证返回 401。缺数据库或密钥返回 503。
- 业务错误为 `{code,message,details?}`，`details.reason` 为稳定本地化键；不返回密码或 token。

## Item

- 认证用户只能访问自己的 Item；不存在和其他用户的 ID 一律 404。
- 创建输入 title / completed；title 去除首尾 ASCII 空格、TAB、CR、LF 后必须为 1–240 个 UTF-8 字节。
- PATCH 必须携带正整数 version，至少提供 title / completed 中一项；省略不修改，显式 null 和未知字段拒绝。DELETE 通过 query 携带 version。
- 修改 / 删除先锁定所属行，再校验版本；过期 version 返回 409，修改成功 version 加一。
- 分页显式传入 limit 1–100、offset ≥0；总数和结果在同一 repeatable-read 快照读取。
- 创建返回 201 包装，读取 / 修改返回 200 包装，删除返回空 204；createdAt / updatedAt 为整数 epoch 毫秒。

## 请求边界与实际验证

JSON 语法错误 400、超过默认 2 MB body 上限 413、Content-Type 错误 415、字段结构错误 422；业务语义错误 400。所有响应携带 x-request-id。

真实测试覆盖登录、JWT 边界、刷新重放 / 并发、撤销、权限隔离、CRUD、版本冲突、分页和 JSON 边界。执行方式见 [本地测试](../../development/testing.md)。旧 PostgreSQL 两个已应用迁移仅移动目录，版本与内容不变；升级验证必须保留原账号和 Item。
