# Repository Guidelines

## Project Map

This project is a single-service Rust API built with Axum, Tokio, and SQLx, supporting PostgreSQL and MySQL.

- `src/main.rs`: HTTP startup and graceful shutdown.
- `src/bootstrap.rs`: AppState assembly.
- `src/api/`: Routing, middleware, unified responses, and error handling.
- `src/domain/`: Health, username/password authentication, and owner-scoped Item CRUD.
- `src/platform/`: Configuration, database connections, and telemetry.
- `src/bin/migrate.rs`: Explicit database migrations.
- `migrations/postgres/` and `migrations/mysql/`: Migrations for each SQL dialect.
- `tests/`: Configuration, HTTP, and real database integration tests.
- `docs/`: Architecture, development guides, and test contracts.

## Native Commands

Use the floating `stable` toolchain and Rust edition 2024. Run Cargo commands directly:

```bash
cargo run --locked
cargo run --locked --bin migrate
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked --release --bins
cargo audit
```

Start a database with `docker-compose --profile postgres up -d --wait` or `docker-compose --profile mysql up -d --wait`. Use `docker compose` when the plugin is available. Preserve data volumes when stopping services.

The migration command requires `APP_DATABASE__URL`. HTTP startup does not run migrations.

## Development Order and Boundaries

Add business domains in this order: `dto.rs`, `service.rs`, `repo.rs`, `handler.rs`, `routes.rs`, and `mod.rs`; then register them in the root router. Keep domain logic together and prefer established crates.

In `repo.rs`, branch on `DatabasePool` to use strongly typed SQLx queries. Provide PostgreSQL and MySQL implementations for every new query and migration. Keep business semantics, migration versions, and test scenarios aligned across both databases.

- PostgreSQL uses `$1`, `$2`, and subsequent numbered placeholders.
- MySQL uses `?` placeholders.
- Bind values with `.bind()`; use `sqlx::QueryBuilder` for dynamic queries.
- Document PostgreSQL tables and columns with `COMMENT ON`. Use `COMMENT`, InnoDB, and utf8mb4 for MySQL tables.
- Never modify an applied migration; add a new version. Each database maintains its own migration history.
- Authentication uses Argon2id, 15-minute JWTs, and rotating 30-day refresh sessions. Only refresh hashes are stored. Logout revokes access and refresh tokens for that session. Keep the same behavior in PostgreSQL and MySQL.
- Item reads and writes are owner-scoped. Unknown and other users' IDs return 404; stale versions return 409. Lock the owned row before checking version. Keep list counts and rows in one repeatable-read snapshot.
- Responses follow the upstream code/message/data and code/message/details contracts, with integer epoch-millisecond timestamps. Export OpenAPI offline with cargo run --locked --example export-openapi. Never edit generated clients manually.

## AI Development Workflow

Identify the affected scope and read existing implementations and contracts before changing code. For non-trivial interface or behavior changes, first update `docs/test/<domain>/` according to `docs/development/ai-coding-test-contract.md`.

Choose verification according to risk and record actual results:

1. Level 0: Documentation, diff, and formatting checks.
2. Level 1: Configuration parsing and stable data transformation tests.
3. Level 2: Router, extractor, and middleware integration tests.
4. Level 3: Migration, query, transaction, and error contracts against both real databases.
5. Level 4: Real HTTP processes, database recovery, and release builds.

Mark database tests with `#[ignore]` and `#[serial]`, require an explicit `TEST_DATABASE_URL`, and connect only to dedicated test databases. See `docs/development/testing.md` for commands.

Before delivery, rerun applicable verification and describe behavior changes, configuration and migration changes, and the commands actually executed. Explicitly report checks that could not run; never fabricate results. Store temporary files, logs, and test instance data under ignored `.ailog/`.

## Changelog Requirements

Whenever adding or modifying a feature, you MUST update the root `CHANGELOG.md` in the same change. Create the file if it does not exist. Do not defer this update to a later task or release.

Each entry MUST describe what changed and its relevant impact, including affected behavior, public APIs, configuration, database schemas or migrations, dependencies, and compatibility or required upgrade steps where applicable. Explicitly identify breaking changes and any action users must take.

Before delivery, verify that the changelog matches the implementation and test results.

## Coding and Git

Use standard rustfmt formatting, snake_case for modules and functions, and PascalCase for types. Write code comments and project documentation in Chinese; keep `AGENTS.md` in English. Prefer pure functions and strict types, and do not modify global environment variables. This crate forbids unsafe code.

Use structured log fields. Never log passwords, complete connection strings, or sensitive request bodies. Preserve useful error context internally and return unified JSON responses externally.

Do not automatically run git commit or push. When a commit is requested, use Conventional Commits with a Chinese summary. PR descriptions should list behavior, route, migration, and APP_* changes with verification evidence. Put detailed designs in docs and keep the README concise.
