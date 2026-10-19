# Next development rules

Manage this project independently with npm, using its own README, package.json / package-lock.json, tool configuration, and OpenAPI input. Do not introduce a root workspace, pnpm, or sibling project source.

- src/auth calls Rust password login and serializes refresh rotation. Access tokens stay in memory; refresh tokens use sessionStorage. src/features contains the Item UI. Hey API generates src/api/generated; do not edit it by hand.
- Public locale pages and metadata use Server Components. Private UI uses Client Components calling Rust. Keep browser authentication and API modules in the client execution graph; do not add private SSR, a BFF, Next database access or server-side token storage without changing the architecture decision.
- After changing the Rust contract, explicitly update this project's snapshot and run npm run generate:api. Standalone builds must not access the parent directory.
- Keep next-intl ICU resources and locale routing complete in English and Chinese, including errors, forms, plurals, and accessibility text. Preserve system / light / dark themes.
- Rust authorizes private data. Clear caches on logout; never write tokens to localStorage.
- Keep the UI concise and semantic, usable at 360 / 768 / 1440 viewports and with a keyboard. Do not put prompts or project introductions in the product UI.
- Run npm run lint, npm run format:check, npm run typecheck, and npm run build. Once real services are running, run npm run test:e2e.
- Do not automatically commit or push. Keep temporary evidence in .ailog and report unfinished checks.

Read [README.md](README.md) for native verification commands and `openapi/openapi.json` for the local contract. Use [next-changelog](.agents/skills/next-changelog/SKILL.md) when asked to create or update this project's `changelog.md`. A standalone copy needs no parent instructions. Item title limits are UTF-8 bytes after ASCII trimming; Rust enforces authorization and version conflicts.
