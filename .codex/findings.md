# 当前发现

- 用户完整实施授权生效；历史暂停已退出当前 docs。iOS 仍使用 XcodeGen + pod install 两步。
- 用户已释放空间，实测 24 GiB，首次 Cargo 检查后 23 GiB。
- Node24 / TS6 与 Hey API peer 兼容；TS7 当前 ESLint 插件不兼容。Web 编译仍待执行。
- 后端选成熟 jsonwebtoken / reqwest / SQLx / utoipa / rust-i18n；不自建认证账号或密码系统。
- 首次 Cargo check 通过，实际 lock 包含 Axum0.8.9、SQLx0.8.6、utoipa5.5；IndexMap 警告已修。
- 开发 realm 禁用 password grant，测试也通过真实登录表单 + PKCE code exchange。
- 本机 5432 已有数据库；本任务使用 .codex 下新 PostgreSQL16、55432。Keycloak26.8 使用 JDK21 和 8081。

## 原生与容器最终验证

- Android真实Keycloak发token成功但AppAuth仍报告认证异常，需读取协议错误分类；编译通过不能证明此链路有效。
- 新建Colima Ubuntu24.04镜像缺失systemd-resolved但保留悬空resolv.conf；通过本机HTTP代理访问registry有效，Docker daemon需单独配置代理。
- 已通过记录见docs/verification/2026-10-05-v0.1.0.md；Android运行和容器是剩余主要门禁。
- Android auth_error为AppAuth code9 Invalid ID Token：上游IdToken.validate独立强制issuer HTTPS。现使用官方setSkipIssuerHttpsCheck，限定devDebug且issuer为localhost/127.0.0.1 HTTP；生产/Release仍要求HTTPS。保留nonce、issuer值、audience、期限验证，取消协程不保存回调状态。

## 忽略规则与原生部署清理

根忽略规则需要拆分到五端；iOS workspace 中只有 SwiftPM Package.resolved 保留提交。SPA Nginx 的旧网络 DNS 与上游地址需要同步调整为本机 API。后端保留 dev/realm.json，使用原生 Keycloak 导入，不改变认证契约。仓库大部分源码尚未跟踪，因此 git diff 不能代替未跟踪文件审查。

## 文档语言调整

根与五端 README、AGENTS 当前均为中文。六组 README 需要双语入口；六份 AGENTS 保持英文模板。docs 中现有需求和验收记录保留中文。全仓大部分文件未跟踪，使用 .codex/doc-language/before 快照比较实际改动。指定 MCP / verification skill 未提供。

英文README已标识链接中的需求和验收材料为中文；中文首页链接五端中文说明。AGENTS保留平台工具链、授权、生成约束、多语言、平台下限和本地验证。全部临时证据及本次npm缓存位于.codex。

## JWT refactor

The previous Android failure was AppAuth ID-token issuer HTTPS enforcement in local devDebug; repaired previously. OIDC exists in all clients, backend discovery/JWKS, configs, contracts and real tests. User now authorizes removing it. Recommended design is direct username/password login with JWT and rotating refresh sessions; this is not advertised as a full OAuth2 authorization server. Existing migrations remain immutable; additive local credential/session tables preserve identities/items. No user database deletion.
