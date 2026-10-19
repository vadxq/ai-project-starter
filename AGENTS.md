# Engineering collaboration rules

Read [docs/README.md](docs/README.md), the current requirements, and the target project's `AGENTS.md` first. The execution source is [docs/v0.1.0/delivery-plan.md](docs/v0.1.0/delivery-plan.md). Current verification status lives there; dated reports are evidence for their stated revision and scope.

## Task routing

| Target  | Local instructions                     | Changelog example skill                                                |
| ------- | -------------------------------------- | ---------------------------------------------------------------------- |
| SPA     | [spa/AGENTS.md](spa/AGENTS.md)         | [spa-changelog](spa/.agents/skills/spa-changelog/SKILL.md)             |
| Next    | [next/AGENTS.md](next/AGENTS.md)       | [next-changelog](next/.agents/skills/next-changelog/SKILL.md)          |
| Backend | [backend/AGENTS.md](backend/AGENTS.md) | [backend-changelog](backend/.agents/skills/backend-changelog/SKILL.md) |
| Android | [android/AGENTS.md](android/AGENTS.md) | [android-changelog](android/.agents/skills/android-changelog/SKILL.md) |
| iOS     | [ios/AGENTS.md](ios/AGENTS.md)         | [ios-changelog](ios/.agents/skills/ios-changelog/SKILL.md)             |

Use [update-changelog](.agents/skills/update-changelog/SKILL.md) when asked to create or update the repository's `changelog.md`. Each project carries a scoped example for its own log. These skills are optional writing workflows; native verification commands remain in project READMEs and the delivery plan. For documentation-only work, validate links and changed configuration. Root sessions must explicitly read a child skill when needed.

## Engineering boundaries

- Install, generate, and build `spa`, `next`, `backend`, `android`, and `ios` independently. Do not add a root JavaScript package or workspace, pnpm, shared runtime source packages, or `.cursor/rules/`.
- Each Web project uses npm and its own lockfile. Use Gradle flavors, XcodeGen YAML, and CocoaPods for mobile environments; do not create Python installation or generation wrappers.
- Rust is the sole implementation source for business rules and authorization. Keep OpenAPI exports and project snapshots synchronized. Never edit generated code by hand.
- Preserve complete zh-CN / en support, including errors and accessibility text, and light / dark / system themes. Show only information users need in the UI.
- Prefer established libraries and pure functions. Follow platform conventions for external connectors. Use strict types and specific errors. Keep handwritten functions at most 120 lines, files at most 800 lines, nesting at most 3 levels, and positional parameters at most 3. Follow upstream signatures for platform protocols and overrides; do not rewrite generated code to fit these limits.
- Do not add abstraction layers, dependencies, or unit tests by default. Validate stable transformations, authorization, transactions, and real integrations according to risk; prefer real services.
- Run the target project's lint, typecheck, build, and smoke checks locally. Report missing tools, skipped checks, and failures accurately. Release verification covers all projects and must not be delegated to CI or manual verification.
- Do not log tokens, passwords, personal paths, or secrets. Do not put personal agent or MCP configuration in public templates.
- Do not automatically commit, push, deploy, or send external messages. Do not clean up data or caches belonging to other projects.

Keep transient plans, logs, screenshots and test environments in ignored `.ailog/runs/<task>/`; project-native test reports and caches may use ignored `.ailog` subdirectories. Share sanitized verification summaries in `docs/verification`.

Select checks according to the changed behavior and risk. Use available search/planning tools; missing optional MCP servers or personal skills must not block ordinary development. Do not substitute self-scores for command results. For delegated work, assign non-overlapping files, keep shared configuration and contracts under one owner, then integrate and verify the affected projects locally.

For each delivery, report actual changes, commands, results and unfinished work. `quick` verification is not release acceptance; `full` requires real integrations and still covers only the selected target/environment. See [AI workflows](docs/ai-workflows.md). When a plan2go file is provided, use it instead of creating a duplicate product plan.
