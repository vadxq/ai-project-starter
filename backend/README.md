# Rust API

[English](README.md) | [简体中文](README.zh-CN.md)

Rust 1.99 / Axum / SQLx / PostgreSQL 16. Provides username/password login, JWT access tokens, rotating refresh sessions, personal Item CRUD, `/health`, and `/ready`. This directory is standalone.

## Setup and running

Install and start native PostgreSQL 16. Create a dedicated user and database through a database administrator connection:

```sh
createuser --pwprompt starter
createdb --owner=starter starter
cp -n .env.example .env
```

Edit DATABASE_URL, BIND_ADDRESS, CORS_ORIGINS, and JWT_SECRET in `.env`. Generate a random signing secret with `openssl rand -hex 32` and set JWT_SECRET to that value. The example placeholder is rejected. If port 5432 is in use, run this project's database on a separate port.

```sh
cargo run --locked --bin migrate
STARTER_USER_PASSWORD=starter-password cargo run --locked --bin create-user -- alice
STARTER_USER_PASSWORD=starter-password cargo run --locked --bin create-user -- bob
cargo run --locked
```

Accounts are created explicitly; no accounts are seeded automatically. The example password above is for local development. The create-user command requires a unique username of 1–64 ASCII letters, digits, dots, underscores, or hyphens, and an 8–128 byte password. It does not overwrite existing accounts.

| Setting      | Description                                                         |
| ------------ | ------------------------------------------------------------------- |
| DATABASE_URL | Required PostgreSQL URL; server-side only                           |
| BIND_ADDRESS | Required listening address, such as 127.0.0.1:8080                  |
| JWT_SECRET   | Required random signing secret, at least 32 bytes; server-side only |
| CORS_ORIGINS | Required comma-separated list of exact Web origins                  |
| RUST_LOG     | tracing filter, such as starter_api=info                            |

Run migrations explicitly before starting the API. Existing identities and Items are preserved by the additive local-auth migration. Old external-provider identities do not acquire passwords automatically; create local accounts explicitly. No external identity provider is needed.

## Authentication

`POST /api/v1/auth/login` accepts `{username,password}` and returns `{accessToken,refreshToken,tokenType,expiresIn,user}`. The user has `id` and `username`. Passwords use Argon2id. The access token is a 15-minute HS256 JWT with issuer, audience, expiry, user, and session claims.

`POST /api/v1/auth/refresh` accepts `{refreshToken}` and returns a rotated token pair. Refresh tokens are random opaque values; only SHA-256 hashes are stored in PostgreSQL. Sessions expire after 30 days. A refresh token can be used once; clients must serialize refresh requests.

`POST /api/v1/auth/logout` accepts `{refreshToken}` and revokes that session. Both its access JWT and refresh token become unusable. Private endpoints require `Authorization: Bearer <accessToken>`. Use HTTPS in production. This is direct password authentication with JWT, not a full OAuth 2.0 authorization server.

## Verification and contract

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release --locked --bins
cargo run --locked --example export-openapi
```

Export reads DTOs and routes without connecting to services. Item title is 1–240 UTF-8 bytes after ASCII trimming. Lists require limit 1–100 and offset ≥0. PATCH / DELETE require version. Operations are scoped to the owner; other users' resources return 404, and stale versions return 409. Errors use RFC9457 Problem Details with stable code and requestId.

Start PostgreSQL and the API with alice/bob accounts, then run:

```sh
TEST_API_URL=http://localhost:8080 JWT_SECRET='<same secret as the running API>' \
cargo test --test api_integration -- --ignored
```

Tests use real password verification, JWTs, PostgreSQL, and HTTP. They cover invalid credentials, signature / issuer / audience / expiry, refresh rotation, logout, CRUD, owner isolation, and version conflicts. Regular cargo test leaves these service-dependent tests ignored; that does not mean they passed.

Production binaries are `target/release/migrate`, `target/release/create-user`, and `target/release/starter-api`. Configure server settings, migrate, create accounts with your own passwords, then start the API. Never ship the local example password or expose JWT_SECRET to clients.
