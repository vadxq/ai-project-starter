# Next.js App Router

[English](README.md) | [简体中文](README.zh-CN.md)

A standalone npm project using Node 24 / npm 11, Next 16, and React 19. Public locale pages are rendered on the server; Client Components request private Items from Rust. Next does not connect to the business database or store user tokens.

```sh
cp -n .env.example .env.local
npm ci
npm run dev
```

The default URL is `http://localhost:3000`. Locale paths are `/en` and `/zh-CN`; the application page is `/{locale}/items`. NEXT_PUBLIC_API_BASE_URL is the only required public setting. The login form calls Rust directly; there is no authentication callback route.

Create local accounts through the backend guide, for example alice / starter-password. Access tokens stay in browser memory; refresh tokens live in the current tab's sessionStorage. Page reloads restore the session through refresh rotation. Closing the tab requires sign-in again. Logout or invalid authentication clears Query cache. The Next server does not store credentials.

```sh
npm run generate:api
npm run lint
npm run format:check
npm run typecheck
npm run build
npm run start
```

The build produces `.next/standalone`. This project's postbuild step copies static assets, and start runs that output directly. Public environment variables are embedded in browser code at build time; changes to production settings require a rebuild.

Set NEXT_PUBLIC_API_BASE_URL before running `npm run build` for production. Deploy the entire `.next/standalone` directory and run `NODE_ENV=production PORT=3000 HOSTNAME=127.0.0.1 node server.js` from that directory. The local postbuild step already includes static assets. Configure the listening address, port, and reverse proxy for your host.

API input and generation configuration live in this project's `openapi` directory and `openapi-ts.config.ts`. The SDK is in `src/api/generated`; do not edit it by hand. Language resources use locally maintained next-intl ICU messages. The locale layout updates translations without clearing root authentication state when switching languages.

After starting the real API and building the project, `npm run test:e2e` checks real password login and CRUD, both languages and themes, refresh, logout, responsive pages, and SSR. Initially, run `npx playwright install chromium`. Results are saved in this project's `.codex` directory.
