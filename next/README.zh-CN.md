# Next.js 应用

[English](README.md) | [简体中文](README.zh-CN.md)

以上游 nextjs-ai-starter 的 81e089fbcc06a237353e2107bd578d20567dfba3 为基础。使用 Next 16.4.2 / React 19.3.1、Tailwind 4、shadcn/ui、next-intl、next-themes 和 Zustand。公共语言页面由服务端渲染，私有 Client Components 直接请求 Rust。

## 启动

使用 Node 24.22.0 与 npm 12.2.0。本目录独立使用 npm 和自己的锁文件。

```sh
cp -n .env.example .env.local
npm ci
npm run dev
```

NEXT_PUBLIC_API_BASE_URL 为 Rust API 地址，NEXT_PUBLIC_APP_URL 为应用公开 origin。两者在构建时写入公开产物，修改后需重新构建。语言路由为 /en、/zh-CN；私有工作区为 /{locale}/items。

账号按后端指南创建。access token 仅在浏览器内存，refresh token 保存在当前标签页 sessionStorage；页面刷新通过轮换恢复会话。退出清除身份与 Query 缓存。Next 不保存服务端凭证，不连接业务数据库。

## 目录与契约

- app/：语言根布局、公共 SSR、私有客户端入口和 Metadata。
- components/：上游 UI primitives、布局、Providers、登录与 Item 工作区。
- lib/auth/：浏览器会话；lib/api/generated/：Hey API 生成结果。
- lib/i18n/、locales/：类型化语言路由与双语 ICU 文案。
- stores/：按组件树隔离并验证持久化数据的 UI 偏好；服务端数据由 TanStack Query 管理。

Rust API 使用 code/message/data 成功响应、code/message/details 错误和整数毫秒时间戳。更新本地 openapi/openapi.json 后重新生成：

```sh
npm run generate:api
npm run format:check
npm run lint
npm run check:i18n
npm run typecheck
npm run build
```

禁止手改生成文件。当前适配排除 PWA / Service Worker 与模板演示内容。

## 产物与浏览器验证

postbuild 将 public 与静态资源复制到 .next/standalone。完整复制该目录并在其中运行 node server.js；按主机设置 NODE_ENV=production、PORT、HOSTNAME。本地 npm start 也运行该 standalone 产物。

启动真实 Rust / 数据库服务并创建 alice / bob 后：

```sh
npx playwright install chromium
npm run test:e2e
```

验证真实认证和 CRUD、刷新、退出、双语 SSR、主题、语言切换、偏好、错误、响应式与键盘操作。截图、trace 和报告保存在 .ailog。npm run verify 串接本地质量检查。

## 维护

先读 [AGENTS.md](AGENTS.md)，独立复制时保留 .agents。[英文](CHANGELOG.md) / [中文](CHANGELOG.zh-CN.md) 日志同步维护。本端原生命令不依赖父目录配置。直接依赖与锁文件一起审查，npm 12 allowScripts 允许必要的 native 构建依赖。当前结果以验证报告为准，不沿用上游结果作为本项目证据。许可证：[Apache-2.0](LICENSE)。
