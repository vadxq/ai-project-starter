# Vite + React SPA

[English](README.md) | [简体中文](README.zh-CN.md)

A standalone npm project using Node 24.21 / npm 11, React, TanStack Query, react-i18next, and a locally generated Hey API SDK. At runtime, it connects only to the configured Rust API.

```sh
cp -n .env.example .env
npm ci
npm run dev
```

The default URL is `http://localhost:5173`. VITE_API_BASE_URL is the only required public client setting. Sign in inside the app with a username and password. Rust handles authentication without external redirects or callback configuration.

Create local accounts through the backend guide, for example alice / starter-password. Access tokens stay in memory; refresh tokens live in the current tab's sessionStorage and rotate when restoring a page reload. Closing the tab requires sign-in again. Logout or invalid authentication clears the session and Query cache. Tokens are never written to localStorage.

## Commands

```sh
npm run generate:api
npm run lint
npm run format:check
npm run typecheck
npm run build
npm run preview
```

Generation reads `openapi/openapi.json` and writes `src/api/generated`; do not edit the output by hand. Keep these files and configuration when copying this project. No parent directory or sibling source is required. Dependencies are locked in package-lock.json. The js-yaml override selects a compatible patched version to address a known vulnerability in the generator's transitive dependencies.

`npm run test:e2e` tests real sign-in and CRUD, languages and themes, narrow screens, page refresh, and logout against the production static build. Build first and start the API. Install the browser initially with `npx playwright install chromium`. Screenshots, traces, and reports are saved in this project's `.codex` directory.

Language resources live in `src/locales`. When adding a language, supply all keys and plural forms, then update the supported language list. Theme and language are local preferences; user Items are not translated. Deploy dist in production and configure the Web server to fall back to index.html for application deep links. Match API proxy routes before this fallback so API 404 responses never become HTML.

Set VITE_API_BASE_URL before running `npm run build` for production. This public setting is embedded in browser code at build time, so changes require a rebuild. Deploy `dist` to a static server.

`nginx.conf` is a native Nginx example: it listens on port 8088, serves `/var/www/starter/dist`, and proxies `/api/` to `http://127.0.0.1:8080`. Adjust the directory and API address for your host, include it in Nginx's http configuration, run `nginx -t`, then start the server. By default, the client accesses the configured API URL directly. For a same-origin proxy, set VITE_API_BASE_URL to the site origin. If the upstream is unavailable, API requests return a gateway error while static pages and application deep links remain accessible.
