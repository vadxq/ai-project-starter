# AI 开发工作流

本仓库是一组可独立复制的 starter。AI 从根与目标工程的 AGENTS.md 读取约束，操作方式以各端 README 为准；当前需求和验收状态见 [文档入口](README.md) 与 [交付计划](v0.1.0/delivery-plan.md)。

## 文件职责

| 文件                    | 用途                                              |
| ----------------------- | ------------------------------------------------- |
| 根 AGENTS.md            | 工程地图、跨端边界与文档入口                      |
| 各端 AGENTS.md          | 本端规则、模块边界、原生验证命令入口              |
| 根和各端 .agents/skills | 编写更新日志的示范 skill                          |
| 各端 README 与 docs     | 安装、架构、需求、契约和实际验收                  |
| changelog.md            | 对外可读的变更记录，由日志 skill 按请求创建或维护 |
| .ailog/runs/<任务>/     | Git 忽略的临时计划、工具日志、调试证据            |

模板以 AGENTS.md 为规则入口。原先的契约/发布/验证 skills、Claude 专用入口与项目 Codex 配置已移除。临时记录统一放在 .ailog；上一轮的旧资料保留在本机 .ailog/archive 和 .ailog/runs，历史验收不代表当前改动通过。

## 更新日志示例

只示范“编写更新日志”这一种能力，根目录和五个独立工程各带一份，名称按范围区分：

| 范围    | Skill                                                                     | 写入范围             |
| ------- | ------------------------------------------------------------------------- | -------------------- |
| 仓库    | [update-changelog](../.agents/skills/update-changelog/SKILL.md)           | 根目录 changelog.md  |
| SPA     | [spa-changelog](../spa/.agents/skills/spa-changelog/SKILL.md)             | spa/changelog.md     |
| Next    | [next-changelog](../next/.agents/skills/next-changelog/SKILL.md)          | next/changelog.md    |
| Backend | [backend-changelog](../backend/.agents/skills/backend-changelog/SKILL.md) | backend/changelog.md |
| Android | [android-changelog](../android/.agents/skills/android-changelog/SKILL.md) | android/changelog.md |
| iOS     | [ios-changelog](../ios/.agents/skills/ios-changelog/SKILL.md)             | ios/changelog.md     |

在对应目录请求，例如：

```text
使用 $spa-changelog，根据本次已确认的改动创建或更新 changelog.md。
```

日志不存在时，skill 创建 Changelog / Unreleased 结构；已存在时沿用其语言和格式，合并到未发布区并保留发布历史。Added、Changed、Fixed、Removed 仅按实际内容出现，不预填功能、版本、发布日期或测试结论。当前只是提供示例 skill，不预先生成产品日志。

单端复制时保留 AGENTS.md、README、隐藏的 .agents、工具配置、锁文件与本地 OpenAPI 输入。skill 不依赖父目录或兄弟工程。在根会话操作某一端时，显式读取其规则和 skill；单端任务可直接在该工程目录启动。其他客户端的自动发现能力以其实际支持为准。

## 原生验证

验证命令留在各端 README 与交付计划，日志 skill 按请求使用。完整仓库保留薄脚本：

```sh
bash scripts/verify.sh backend quick
bash scripts/verify.sh spa full
bash scripts/contracts.sh check
```

| 目标       | quick                                        | full 追加内容                                                |
| ---------- | -------------------------------------------- | ------------------------------------------------------------ |
| backend    | fmt、Clippy、普通测试、Release bins          | 真实 HTTP / PostgreSQL，构建前要求 TEST_API_URL / JWT_SECRET |
| spa / next | format、lint、typecheck、production build    | 真实 Playwright，先准备 API 与测试账号                       |
| android    | lintDevDebug、devDebug / prodRelease         | Emulator connectedDevDebugAndroidTest                        |
| ios        | dev 生成、pod install、Debug Simulator build | XCUITest；两种模式都要求 IOS_DESTINATION                     |
| contracts  | Rust 导出、快照与 Web 生成漂移检查           | 与 quick 相同                                                |

full 只覆盖当前目标和环境，完整发布仍须交付计划中的最低/当前系统矩阵、环境切换、隔离复制、生产 smoke、故障恢复与性能验证。缺依赖、失败或未执行项必须分别报告；普通 Cargo 测试中的 ignored 不计为真实集成通过。

## 变更影响

Rust DTO、路由或错误语义变化时，修改 Rust 元数据与真实测试，再运行契约生成/漂移检查和四个消费者的生成编译。认证恢复、刷新、退出或缓存变化时检查 SPA/Next 的独立实现；协议变化同时检查原生客户端。多语言、主题、无障碍、环境与依赖变化按本端要求验证。

只有规则与文档变化时，检查 skill 元数据、链接、格式、删除项和独立复制引用。更新日志仅描述已有证据，不替代验证，也不自动提交、打 tag 或发布。
