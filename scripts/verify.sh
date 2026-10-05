#!/usr/bin/env bash
set -euo pipefail

# 只串接原生命令；每个工程仍能用自己的 README 独立执行。
root="$(cd "$(dirname "$0")/.." && pwd)"
target="${1:?Use backend, spa, next, android, ios or contracts}"
mkdir -p "$root/.codex"

case "$target" in
  backend)
    cd "$root/backend"
    export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/.codex/target}"
    cargo fmt --check
    cargo clippy --locked --all-targets -- -D warnings
    cargo test --locked
    cargo build --locked --release --bins
    # 真实依赖必须由调用者提供，未提供时不会伪装成已执行。
    if [[ -n "${TEST_API_URL:-}" && -n "${JWT_SECRET:-}" ]]; then
      cargo test --locked --test api_integration -- --ignored
    else
      printf '%s\n' 'Integration tests skipped: set TEST_API_URL and JWT_SECRET; create alice/bob accounts first.'
    fi
    ;;
  spa|next)
    cd "$root/$target"
    npm run format:check
    npm run lint
    npm run typecheck
    npm run build
    npm run test:e2e
    ;;
  android)
    cd "$root/android"
    ./gradlew :app:lintDevDebug :app:assembleDevDebug :app:assembleProdRelease
    ./gradlew :app:connectedDevDebugAndroidTest
    ;;
  ios)
    cd "$root/ios"
    : "${IOS_DESTINATION:?Set IOS_DESTINATION to platform=iOS Simulator,id=<device UUID>}"
    xcodegen generate -s project.yml
    pod install
    xcodebuild -workspace Starter.xcworkspace -scheme Starter -configuration Debug \
      -destination "$IOS_DESTINATION" -derivedDataPath "$root/.codex/ios-derived" \
      -clonedSourcePackagesDirPath "$root/.codex/swift-packages" \
      -skipPackagePluginValidation -parallel-testing-enabled NO CODE_SIGN_IDENTITY=- test
    ;;
  contracts)
    "$root/scripts/contracts.sh" check
    ;;
  *) printf '%s\n' 'Use backend, spa, next, android, ios or contracts.' >&2; exit 2 ;;
esac
