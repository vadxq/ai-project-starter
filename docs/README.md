# 全栈脚手架文档

当前范围和接口以 [需求](v0.1.0/requirements.md) 为准，实施与验收状态统一见 [交付计划](v0.1.0/delivery-plan.md)。2026-10-21 上游换新的实际命令、结果和已知依赖告警见 [验证报告](verification/2026-10-21-upstream-refresh.md)。

| 文档                                                                                                         | 用途                                      |
| ------------------------------------------------------------------------------------------------------------ | ----------------------------------------- |
| [需求](v0.1.0/requirements.md)                                                                               | 首版范围、接口约束与成功标准              |
| [架构](v0.1.0/architecture.md)                                                                               | 模块边界、认证、生成链路、工具选择        |
| [AI 工作流](ai-workflows.md)                                                                                 | AGENTS、更新日志示例 skill 与本地验证模式 |
| [实施与验收](v0.1.0/delivery-plan.md)                                                                        | 阶段、依赖与发布门禁                      |
| [移动双环境](v0.1.0/mobile-environments.md)                                                                  | 两端开发 / 生产配置及原生命令             |
| [参考分析](v0.1.0/references.md)                                                                             | 两个参考仓库与官方依据                    |
| [API 验收契约](contracts/api-tests.md)                                                                       | 跨端接口验证范围与 OpenAPI 入口           |
| [需求模板](templates/requirement.md) / [计划模板](templates/plan.md) / [API 模板](templates/api-contract.md) | 新任务的需求与验收记录                    |

各工程 README 提供独立安装和原生命令，AGENTS.md 定义开发约束；根文档维护跨端需求、架构和验收。临时计划、日志和原始证据保存在 Git 忽略的 `.ailog/runs/<任务>/`，可分享结果保存在 `docs/verification`。
