# AI 协作开发

## 入口

先读 AGENTS.md，再按改动定位 domain、API、platform 及相关测试。架构入口是 docs/architecture.md，接口规则是 docs/development/api.md。避免把整仓库内容无差别放入上下文。

## 开发闭环

1. 确定接口输入、输出、错误、状态变化、数据库差异和验证方式。
2. 非平凡行为在 docs/test/<domain>/ 中建立或更新契约。
3. 按领域小步实现；基础设施优先采用成熟 crate。
4. 新增 SQL 同时维护 PostgreSQL / MySQL 版本，数据值通过 bind 传入；动态查询使用 SQLx QueryBuilder。
5. 执行影响范围内的本地验证，检查 git --no-pager diff，再记录实际证据。

配置测试使用 Settings::from_sources，禁止测试修改全局环境。数据库测试显式传入 TEST_DATABASE_URL，连接专用测试库。未配置、连接失败、超时和恢复都要有明确行为。

## 依赖更新

工具链保持 stable，可使用 rustup update stable 更新本机。更新前查看官方 registry 与 release notes；逐项升级 Cargo.toml 的直接依赖，再执行 cargo update 更新锁文件。跨主版本或 0.x 次版本需验证 API、feature、MSRV 与运行时变化。

验证使用 --locked。运行 cargo audit，追踪存在问题的依赖链并优先移除不需要的 feature。传递依赖受上游约束时记录原因，不通过虚假版本声明掩盖限制。

## 交付证据

记录行为变化、配置项、两套迁移路径、实际验证命令及结果。说明无法执行的检查。工作记录与 review-report 放 .ailog；可复用指南和契约放 docs。禁止自动 commit / push。
