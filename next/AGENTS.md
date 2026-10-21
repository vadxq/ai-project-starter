# Repository Guidelines

## Project Scope

This is a general-purpose Next.js starter for AI-assisted development. Use npm, TypeScript, React, shadcn/ui, and Tailwind CSS. English is the default application language, with Simplified Chinese support. Do not create Git commits or push changes without explicit authorization.

## Structure and Responsibilities

- `app/`: routes, layouts, and metadata; localized pages live in `app/[locale]/`.
- `components/ui/`: shadcn primitives; prefer CLI updates and avoid redundant wrappers.
- `components/layout/` and `components/providers/`: shared layouts and client boundaries.
- `stores/`: Zustand store factories; never share mutable state across server requests.
- `lib/auth/`: browser password login, serialized refresh rotation and logout. Access tokens remain in memory; refresh tokens use sessionStorage. Keep these modules in the client graph.
- `lib/api/generated/`: Hey API output from this project's OpenAPI snapshot. Run npm run generate:api; never edit generated files manually. Unwrap data and map upstream errors in the handwritten connectors.
- `components/items/`: private Item UI. Rust owns authorization and transactions; Next does not connect to the business database or keep server-side credentials.
- `lib/i18n/` and `locales/`: internationalization configuration and bilingual messages.
- `tests/e2e/`: critical browser flows; `README.md` and `README.zh-CN.md`: matching project guides.
- `CHANGELOG.md` and `CHANGELOG.zh-CN.md`: matching English and Chinese change records.

## Coding Conventions

- Default to Server Components; introduce client boundaries where interaction is needed.
- Use PascalCase for custom React components; preserve official shadcn filenames.
- Enable TypeScript strict mode, avoid `any`, and type public interfaces explicitly.
- Prefer pure functions, established libraries, and direct implementations. Avoid abstractions, dependencies, and experimental options without a concrete need.
- Keep user-facing copy in translation files, with matching keys and interpolation arguments across both languages.
- Let next-themes manage themes, next-intl manage locales, and Zustand manage necessary shared client state.
- This adaptation excludes PWA and service workers. Preserve standalone Node output, including public and static assets.
- Keep interfaces concise. Include only functional actions, valid links, and substantiated claims.

## Local Verification

Run `npm run format:check`, `npm run lint`, `npm run check:i18n`, `npm run typecheck`, `npm run build`, and `npm run test:e2e`. The combined entry point is `npm run verify`.

Browser tests use production builds and the real Rust API. Cover password login, refresh, logout, CRUD, both languages, themes, preference persistence, locale switching, responsive layouts, and public SSR. Keep temporary screenshots and reports in ignored .ailog. Run verification locally without depending on CI.

## Changelog Requirements

- Every feature addition or modification must include a changelog update in the same change. Record user-visible bug fixes and removals as well.
- Update both `CHANGELOG.md` and `CHANGELOG.zh-CN.md`, keeping their entries equivalent.
- Add entries under `Unreleased` / `未发布`, grouped as Added, Changed, Fixed, or Removed. Include only categories with actual changes.
- Describe the affected feature and resulting behavior. Include compatibility impact, migration steps, or configuration changes when applicable.
- Write concise, meaningful entries rather than copying commit messages or implementation logs. Do not invent releases or release dates.
- Do not mark a feature complete until its changelog entries and relevant documentation are updated.

## Dependencies and Delivery

- Keep `package-lock.json` versioned; do not introduce other package managers or Git hook tooling.
- Check peer compatibility when upgrading dependencies; do not force unsupported combinations.
- Allow installation scripts only for necessary dependencies.
- Update bilingual documentation and report changes, actual verification results, and known limitations.
- Do not add editor or agent state, process logs, or temporary files to the repository.
- Consult the installed framework documentation in `node_modules/next/dist/docs/` before changing framework APIs. Automatic agent rule writing is disabled so development servers do not rewrite this file.
