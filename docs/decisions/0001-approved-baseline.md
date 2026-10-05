# ADR-0001：v0.1.0 已确认基线

状态：接受。确认日期：2026-10-04；整理日期：2026-10-05。

| 决策 | 已确认内容                                                                                                             |
| ---- | ---------------------------------------------------------------------------------------------------------------------- |
| D-01 | 同仓五个独立工程，各自安装 / 配置 / 构建及维护 .gitignore；Web 各用 npm，无 JS workspace / pnpm                        |
| D-02 | 登录、当前用户、最小 Item CRUD                                                                                         |
| D-03 | 用户名密码 + JWT / refresh session；Rust 统一认证与业务（2026-10-05 用户要求替换）                                     |
| D-04 | Next 公共 SSR，私有 Client Components 请求 Rust                                                                        |
| D-05 | Android 14 / API 34 起；iOS 18.0 起；编译工具与设备下限分开                                                            |
| D-06 | 所有端多语言，首批 zh-CN / en；浅色 / 深色 / 系统主题，简洁原生交互                                                    |
| D-07 | 静态 Web 与原生 Nginx 示例、Next standalone Node 产物、Rust release 二进制、Android APK、iOS Simulator；不含部署和上架 |
| D-08 | Android dev/prod flavors；iOS 两份 XcodeGen YAML + 同一 Podfile，原生命令切换环境                                      |

各端独立带来配置与语言资源的少量重复，这是用户明确选择的维护边界。API 使用生成类型保持一致，根脚本仅辅助完整仓库维护。无 `.cursor/rules/`，不提供自研 Python 环境或工程生成包装。

2026-10-05 用户恢复完整实施。旧的构建暂停说明已退出当前文档。实现中调整兼容 patch / 生成器和内部文件组织由本地验证决定，产品范围变化仍须同步需求与验收。
