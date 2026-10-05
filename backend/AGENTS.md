# Backend development rules

This project can be copied on its own. `src/api` handles HTTP and OpenAPI; `src/domain` handles DTOs and transaction rules; `src/platform` handles configuration, Argon2id / JWT authentication, and connections. PostgreSQL is the only business database. Do not duplicate authorization rules in clients.

- Use Rust 1.99, edition 2024, and Cargo.lock. Explain the need for new dependencies before adding them.
- Add explicit migrations for schema changes. Never migrate automatically on API startup. Do not automatically commit or push.
- Scope every Item read and write to its owner. Unknown IDs and IDs belonging to other users both return 404. Use version for updates and deletes.
- Keep utoipa and integration test contracts synchronized when adding routes. Do not edit generated code by hand.
- Do not print tokens, passwords, database URLs, or user input bodies.
- Keep handwritten functions at most 120 lines, files at most 800 lines, and positional parameters at most 3. Prefer simple functions and specific error types.
- Verify with `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, and `cargo build --release`. Prepare real services for integration tests as described in the README.
- Export with `cargo run --example export-openapi`; no service connection is required.
