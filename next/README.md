# Next.js application

[English](README.md) | [简体中文](README.zh-CN.md)

Based on upstream nextjs-ai-starter commit 81e089fbcc06a237353e2107bd578d20567dfba3. Next 16.4.2 / React 19.3.1, Tailwind 4, shadcn/ui, next-intl, next-themes and Zustand. Public locale pages render on the server; private Client Components call Rust directly.

## Start

Use Node 24.22.0 and npm 12.2.0. This is an independent npm project with its own lockfile.

```sh
cp -n .env.example .env.local
npm ci
npm run dev
```

Set NEXT_PUBLIC_API_BASE_URL to your Rust API and NEXT_PUBLIC_APP_URL to the public application origin. These public values are embedded during builds; rebuild after changing them. Routes are /en and /zh-CN, with private Items at /{locale}/items.

Create accounts through the backend guide. Access tokens remain in browser memory and refresh tokens in the current tab's sessionStorage. Reload restores the session through refresh rotation. Logout clears credentials and Query caches. Next does not store server-side credentials or connect to the business database.

## Structure and contract

- app/: localized root layouts, public SSR, private client entry points and metadata.
- components/: upstream UI primitives, layouts, providers, sign-in and Item workspace.
- lib/auth/: browser session connector; lib/api/generated/: Hey API output.
- lib/i18n/ and locales/: typed locale routing and bilingual ICU messages.
- stores/: request-isolated, validated UI preferences; TanStack Query owns server data.

The Rust API uses code/message/data success responses, code/message/details errors and integer millisecond timestamps. Keep the local openapi/openapi.json input and regenerate after contract updates:

```sh
npm run generate:api
npm run format:check
npm run lint
npm run check:i18n
npm run typecheck
npm run build
```

Do not edit generated code manually. This adaptation excludes PWA / service workers and starter demo content.

## Production and browser verification

The postbuild step includes public and static assets in .next/standalone. Copy that complete directory and run node server.js inside it; set NODE_ENV=production, PORT and HOSTNAME for your host. npm start runs this same standalone output locally.

With real Rust / database services and alice / bob accounts running:

```sh
npx playwright install chromium
npm run test:e2e
```

Tests cover real authentication and CRUD, reload, logout, bilingual SSR, themes, locale switching, preferences, errors, responsive layouts and keyboard navigation. Temporary screenshots, traces and reports are saved in .ailog. npm run verify runs the local quality pipeline.

## Maintenance

Read [AGENTS.md](AGENTS.md). Keep .agents when copying the project. Change records are [English](CHANGELOG.md) and [Chinese](CHANGELOG.zh-CN.md). Native npm commands do not require parent configuration. Direct dependencies and lockfile are reviewed together; npm 12 allowScripts permits the required native build dependencies. Actual results belong in verification reports; upstream results are not this application's evidence. License: [Apache-2.0](LICENSE).
