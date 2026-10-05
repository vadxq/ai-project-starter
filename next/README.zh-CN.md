# Next.js App Router

[English](README.md) | 简体中文

独立 npm 工程，Node24 / npm11、Next16、React19。公共 locale 页面由服务器渲染，私有事项由 Client Components 请求 Rust。Next 不连接业务数据库或存储用户 token。

```sh
cp -n .env.example .env.local
npm ci
npm run dev
```

默认 `http://localhost:3000`；locale 路径为 `/en`、`/zh-CN`，业务页面为 `/{locale}/items`。唯一必填公开配置为 NEXT_PUBLIC_API_BASE_URL。登录表单直接调用 Rust API，没有认证回调路由。

先按 backend 文档创建本地账号，例如 alice / starter-password。access token 只在内存，refresh token 存当前标签页 sessionStorage。页面刷新通过轮换恢复会话，关闭标签页需重新登录；退出或认证失效清 Query cache。Next 服务端不保存用户凭证。

```sh
npm run generate:api
npm run lint
npm run format:check
npm run typecheck
npm run build
npm run start
```

构建生成 `.next/standalone`，本端 postbuild 复制静态资源；start 直接运行该产物。公开环境变量在构建时写入浏览器代码，生产环境变更需要重新 build。

生产构建前设置 NEXT_PUBLIC_API_BASE_URL，再执行 `npm run build`。部署完整 `.next/standalone` 目录，在该目录运行 `NODE_ENV=production PORT=3000 HOSTNAME=127.0.0.1 node server.js`；本端 postbuild 已包含静态资源。按目标主机配置监听地址、端口和反向代理。

API 输入和生成配置在本端 `openapi` / `openapi-ts.config.ts`；SDK 在 `src/api/generated`，不能手改。语言资源使用 next-intl ICU 消息，本端维护；locale layout 更新翻译，根认证状态不随切换语言清除。

启动真实 API 并 build 后，`npm run test:e2e` 检查真实密码登录 / CRUD、两种语言 / 主题、刷新、注销、响应式页面和 SSR。首次需要 `npx playwright install chromium`。结果放在本端 `.codex`。
