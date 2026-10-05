# Engineering collaboration rules

Read `docs/README.md`, the current requirements, and the target project's `AGENTS.md` first. The delivery plan is `docs/v0.1.0/delivery-plan.md`. Keep temporary reasoning, logs, and evidence in `.codex`; they are not a second source of product requirements.

- Install, generate, and build `spa`, `next`, `backend`, `android`, and `ios` independently. Do not add a root JavaScript package or workspace, pnpm, shared runtime source packages, or `.cursor/rules/`.
- Each Web project uses npm and its own lockfile. Use Gradle flavors, XcodeGen YAML, and CocoaPods for mobile environments; do not create Python installation or generation wrappers.
- Rust is the sole implementation source for business rules and authorization. Keep OpenAPI exports and project snapshots synchronized. Never edit generated code by hand.
- Preserve complete zh-CN / en support, including errors and accessibility text, and light / dark / system themes. Show only information users need in the UI.
- Prefer established libraries and pure functions. Follow platform conventions for external connectors. Use strict types and specific errors. Keep handwritten functions at most 120 lines, files at most 800 lines, nesting at most 3 levels, and positional parameters at most 3. Follow upstream signatures for platform protocols and overrides; do not rewrite generated code to fit these limits.
- Do not add abstraction layers, dependencies, or unit tests by default. Validate stable transformations, authorization, transactions, and real integrations according to risk; prefer real services.
- Run the target project's lint, typecheck, build, and smoke checks locally. Report missing tools, skipped checks, and failures accurately. Release verification covers all projects and must not be delegated to CI or manual verification.
- Do not log tokens, passwords, personal paths, or secrets. Do not put personal agent or MCP configuration in public templates.
- Do not automatically commit, push, deploy, or send external messages. Do not clean up data or caches belonging to other projects.

For each delivery, report actual changes, commands and results, and unfinished work. When a plan2go file is provided, use it as the execution source and keep it synchronized instead of creating a duplicate plan.
