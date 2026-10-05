# AI Project Starter

[English](README.md) | [简体中文](README.zh-CN.md)

Five standalone projects to choose from: a Vite + React SPA, Next.js, Rust + Axum, Kotlin Android, and Swift iOS. The examples include username/password sign-in with JWT, personal Item CRUD, English and Simplified Chinese, and system / light / dark themes. Requirements and plans live in [docs](docs/README.md) (in Chinese).

| Project | Project guide                          | Prerequisites                              |
| ------- | -------------------------------------- | ------------------------------------------ |
| SPA     | [spa/README.md](spa/README.md)         | Node 24, npm                               |
| Next    | [next/README.md](next/README.md)       | Node 24, npm                               |
| Backend | [backend/README.md](backend/README.md) | Rust 1.99, PostgreSQL 16                   |
| Android | [android/README.md](android/README.md) | JDK 21, Android SDK; Android 14+ device    |
| iOS     | [ios/README.md](ios/README.md)         | Xcode, XcodeGen, CocoaPods; iOS 18+ device |

Each directory can be copied on its own and has its own configuration, `.gitignore`, and lockfiles. The Web projects use npm independently. There is no root JavaScript package, shared JavaScript workspace, or source dependency between projects.

## Getting started locally

Start in `backend`, run native PostgreSQL, copy `.env.example` to `.env`, and configure database settings and a random JWT_SECRET. Run migrations, create accounts with `create-user`, then run `cargo run`. Local demo accounts alice / bob with starter-password must be created explicitly as described in the backend guide; use your own passwords in production.

Choose SPA or Next, enter its directory, copy the example configuration, and run `npm ci` followed by `npm run dev`. Default ports are SPA 5173, Next 3000, API 8080. Each project only needs its own toolchain and access to the configured services.

For the iOS development environment:

```sh
cd ios
xcodegen generate -s project.yml
pod install
```

For production, replace the first command with `xcodegen generate -s project.prod.yml`, then run `pod install` as usual. Open the generated `Starter.xcworkspace`. Configure production URLs in the project's YAML files.

Android uses its own Gradle Wrapper: `./gradlew :app:assembleDevDebug` or `./gradlew :app:assembleProdRelease`. See [mobile environments](docs/v0.1.0/mobile-environments.md) (in Chinese) for network and environment details.

## Development and verification

AI agents should start with [AGENTS.md](AGENTS.md) and the target project's rules, then follow the [delivery plan](docs/v0.1.0/delivery-plan.md). The API contract comes from Rust DTOs and routes. Clients generate code from their local OpenAPI snapshots; do not edit generated code by hand.

The root `scripts` directory provides helpers for the complete repository. Each project's native build, check, and test commands remain its standalone entry points. Real authentication and database tests require running local services. Signing, production deployment, and store submission are outside the first release's scope.

Examples: `bash scripts/verify.sh spa`, `bash scripts/verify.sh backend`, or `bash scripts/contracts.sh check`. Web tests require a real API. Android tests require an Emulator and `adb reverse`. For iOS, set `IOS_DESTINATION='platform=iOS Simulator,id=<UUID>'`. The scripts do not install system tools or change personal configuration.

All five projects in v0.1.0 have been implemented and verified locally. See the [verification record](docs/verification/2026-10-05-v0.1.0.md) (in Chinese) for the actual commands, results, and scope. Configure production service URLs, signing, and deployment for your own project.

Inspired by [vadxq/nextjs-ai-starter](https://github.com/vadxq/nextjs-ai-starter) and [vadxq/rust-axum-starter](https://github.com/vadxq/rust-axum-starter). Design choices and pinned reference commits are documented in the [reference analysis](docs/v0.1.0/references.md) (in Chinese). Licensed under Apache-2.0.
