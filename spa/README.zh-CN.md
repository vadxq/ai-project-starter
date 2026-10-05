# Vite + React SPA

[English](README.md) | 简体中文

独立 npm 工程，Node 24.21 / npm 11。使用 React、TanStack Query、react-i18next，以及本端 Hey API 生成的 SDK。运行时只连接配置的 Rust API。

```sh
cp -n .env.example .env
npm ci
npm run dev
```

默认 `http://localhost:5173`。唯一必填客户端配置为 VITE_API_BASE_URL，是公开值。应用内用户名密码登录，调用 Rust API，无外部跳转或回调配置。

先按 backend 文档创建本地账号，例如 alice / starter-password。access token 只在内存，refresh token 存当前标签页 sessionStorage，页面刷新通过轮换恢复会话。关闭标签页需重新登录；退出或认证失效清会话和 Query cache。token 不写入 localStorage。

## 命令

```sh
npm run generate:api
npm run lint
npm run format:check
npm run typecheck
npm run build
npm run preview
```

生成输入 `openapi/openapi.json`，输出 `src/api/generated`，不可手改。复制本端时这些文件和配置全部保留，无需父目录或其他端源码。依赖锁定在 package-lock.json；js-yaml override 用兼容修复版本消除生成器传递依赖的已知漏洞。

`npm run test:e2e` 在生产静态产物上执行真实登录 / CRUD、语言 / 主题、窄屏与刷新 / 注销。需先 build 并启动 API；首次浏览器准备可用 `npx playwright install chromium`。本端 `.codex` 保存截图、trace 和报告。

语言资源位于 `src/locales`，新增语言时补齐全部 key / 复数，再更新支持列表。主题和语言是本地偏好；用户事项不翻译。生产部署使用 dist，Web server 对业务深链回退 index.html；API 代理必须先于回退匹配，不能把 API 404 变成 HTML。

生产构建前设置 VITE_API_BASE_URL，再执行 `npm run build`；这些公开配置在构建时写入浏览器代码，变更后需要重新构建。将 `dist` 部署到静态服务器即可。

`nginx.conf` 提供原生 Nginx 示例：监听8088，静态目录 `/var/www/starter/dist`，`/api/` 转发到 `http://127.0.0.1:8080`。按目标主机调整目录和 API 地址，纳入 Nginx 的 http 配置并执行 `nginx -t` 后启动。默认客户端直接访问配置的 API URL；使用同源代理时，将 VITE_API_BASE_URL 设置为站点 origin。上游暂不可达时返回网关错误，静态页面和业务深链仍可访问。
