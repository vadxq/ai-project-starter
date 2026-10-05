# AI 维护演练

2026-10-05 在独立副本中完成，主工程保持已确认的 v0.1.0 功能范围。三个补丁均已用 `git apply --check` 对当前工程检查，可以直接审查具体修改。

| 演练     | 补丁                         | 实际改动                                                                         | 验证                                                                                           |
| -------- | ---------------------------- | -------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- |
| 新增字段 | [field.patch](field.patch)   | Item 增加 titleBytes；PostgreSQL 用 octet_length 计算；Rust DTO 自动进入 OpenAPI | Release 编译；真实 PKCE / 数据库集成2项通过，中文标题验证12字节；SPA重新生成SDK并typecheck通过 |
| 新增筛选 | [filter.patch](filter.patch) | 列表增加可选 completed；total 与 rows 使用同一过滤条件和 owner 边界              | Release 编译；真实集成3项通过，含true / false / 非法值；SPA重新生成SDK并typecheck通过          |
| 调整交互 | [ui.patch](ui.patch)         | 新增框按 Escape 清除草稿和本地校验状态                                           | SPA production build；真实登录后 Playwright 交互测试1项通过                                    |

字段与筛选演练未写入主契约，UI交互演练未加入主页面。相应代码仅保留在可审查补丁和本机 `.codex/exercises` / 隔离副本内。原始结果见 `.codex/logs/exercise-field-final-tests.log`、`exercise-filter-tests.log`、`exercise-ui-test.log`，以及各自编译和SDK日志。

演练发现多个同名 Cargo 工程复用 target 目录时可能命中另一副本的包产物；切换演练副本后使用 `cargo clean --release -p starter-api` 只清该包，再构建和运行对应测试，依赖缓存可继续复用。最终字段演练日志为2项、筛选为3项，避免把错误副本的通过结果计入验收。
