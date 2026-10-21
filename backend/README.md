# Rust + Axum API

[English](README.md) | [简体中文](README.zh-CN.md)

Based on upstream rust-axum-starter commit 6df373d98b573e8155cd1d1931065aa16af81598. Uses Rust stable / edition 2024, Axum 0.8.9, SQLx 0.9, PostgreSQL or MySQL. Includes password login, rotating JWT sessions, owner-scoped Item CRUD, health probes and offline OpenAPI export. This directory is standalone.

## Run locally

Use a native PostgreSQL / MySQL installation, or start one of the upstream Compose profiles:

```sh
cp -n .env.example .env
docker-compose --profile postgres up -d --wait
# Alternative: docker-compose --profile mysql up -d --wait
```

Set APP_DATABASE__URL in .env for your database. Set APP_AUTH__JWT_SECRET to a random value generated with openssl rand -hex 32. Then:

```sh
cargo run --locked --bin migrate
STARTER_USER_PASSWORD=starter-password cargo run --locked --bin create-user -- alice
STARTER_USER_PASSWORD=starter-password cargo run --locked --bin create-user -- bob
cargo run --locked
```

Accounts are created explicitly and existing usernames are never overwritten. The example password is for local testing. The API does not migrate automatically. To retain existing PostgreSQL data, keep applied migration files unchanged; the same versions now live under migrations/postgres. MySQL has equivalent migrations under migrations/mysql.

## Configuration

| Setting                       | Meaning                                                             |
| ----------------------------- | ------------------------------------------------------------------- |
| APP_HTTP__BIND_ADDR           | Listen address, default 0.0.0.0:8080                                |
| APP_DATABASE__URL             | PostgreSQL / MySQL URL; empty means probes-only HTTP startup        |
| APP_DATABASE__MAX_CONNECTIONS | Positive connection limit, default 10                               |
| APP_DATABASE__TIMEOUT_SECS    | Pool / readiness timeout, default 3 seconds                         |
| APP_AUTH__JWT_SECRET          | At least 32 random bytes; backend only; required for authentication |
| APP_TELEMETRY__LOG_LEVEL      | tracing filter, default info; RUST_LOG takes precedence             |
| APP_TELEMETRY__JSON           | JSON logging, default false                                         |

Configuration precedence is defaults, then local .env, then process environment. Invalid values and unknown fields in known groups fail startup. Configuration loading does not mutate the process environment. Legacy DATABASE_URL / BIND_ADDRESS / JWT_SECRET / CORS_ORIGINS names are no longer used.

Database pools connect lazily. Without a database or during an outage, /health remains 200 and /ready is 503. Readiness recovers when the database returns. Authentication returns 503 when its database or signing key is not configured. CORS follows the upstream permissive implementation.

## API contract

Except empty 204 responses, success uses {code,message,data}; code matches the HTTP status as a string. Errors use {code,message,details?}. Business details.reason is a stable localization key. Item timestamps are integer Unix epoch milliseconds. This replaces the previous bare DTO / Problem Details / RFC3339 contract.

JSON syntax errors return 400, oversized bodies 413, missing or invalid Content-Type 415, and JSON type / missing-field / unknown-field errors 422. Business validation is 400. Request IDs are generated or propagated through X-Request-Id.

POST /api/v1/auth/login accepts username/password. Refresh rotates a 30-day opaque token atomically; only its hash is stored. A 15-minute HS256 JWT validates issuer, audience, expiry, subject and session. Logout revokes both credentials for that session. Business requests use Bearer access tokens.

Items require limit / offset for listing and version for updates / deletes. Titles contain 1–240 UTF-8 bytes after ASCII trimming. Other owners' resources return 404. Transactions lock the owned row before version checks; stale versions return 409. Both databases enforce the same semantics.

## Local verification

```sh
cargo fmt --all -- --check
cargo clippy --locked --all-targets --all-features -- -D warnings
cargo test --locked
cargo build --locked --release --bins
cargo run --locked --example export-openapi
```

Real database / HTTP tests are ignored by default. Prepare a dedicated database, migrate and create alice / bob, start the API with the same signing key, then run each database separately:

```sh
TEST_DATABASE_URL='<dedicated PostgreSQL URL>' cargo test --locked --test database postgres_ -- --ignored
TEST_DATABASE_URL='<dedicated MySQL URL>' cargo test --locked --test database mysql_ -- --ignored
TEST_API_URL=http://localhost:8080 APP_AUTH__JWT_SECRET='<running API key>' cargo test --locked --test api_integration -- --ignored
```

When local proxy variables are configured, set both `NO_PROXY` and `no_proxy` to `localhost,127.0.0.1,::1` for HTTP tests and benchmarks.

See [testing](docs/development/testing.md). Tests use real database transactions, HTTP and passwords. An ignored test is not a passed test.

Release output includes rust-axum-starter, migrate and create-user. Ship migrations/ alongside the migration command and run it from this directory structure. Use HTTPS and your own production credentials.

## Maintenance

Read [AGENTS.md](AGENTS.md). Keep this directory's lockfile, locales, migrations, examples and .agents when copying it. Temporary evidence belongs in ignored .ailog. Update [CHANGELOG.md](CHANGELOG.md) with actual changes; do not automatically commit or push. License: [Apache-2.0](LICENSE).
