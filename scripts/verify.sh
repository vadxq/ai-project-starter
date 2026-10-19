#!/usr/bin/env bash
set -euo pipefail

# 只串接原生命令；每个工程仍能用自己的 README 独立执行。
root="$(cd "$(dirname "$0")/.." && pwd)"
target="${1:?Use backend, spa, next, android, ios or contracts}"
mode="${2:?Specify quick or full}"
if [[ "$#" -ne 2 || ! "$mode" =~ ^(quick|full)$ ]]; then
  printf '%s\n' 'Usage: bash scripts/verify.sh <backend|spa|next|android|ios|contracts> <quick|full>' >&2
  exit 2
fi
# 在构建前检查真实验证条件，避免缺依赖时产生成功结果。
if [[ "$target" == backend && "$mode" == full ]]; then
  : "${TEST_API_URL:?Full backend verification requires TEST_API_URL}"
  : "${JWT_SECRET:?Full backend verification requires JWT_SECRET matching the running API}"
fi
mkdir -p "$root/.ailog"

case "$target" in
  backend)
    cd "$root/backend"
    export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/.ailog/target}"
    cargo fmt --check
    cargo clippy --locked --all-targets -- -D warnings
    cargo test --locked
    cargo build --locked --release --bins
    if [[ "$mode" == full ]]; then
      cargo test --locked --test api_integration -- --ignored
    else
      printf '%s\n' 'Quick checks passed; real API/database integration was not run. This is not release acceptance.'
    fi
    ;;
  spa|next)
    cd "$root/$target"
    npm run format:check
    npm run lint
    npm run typecheck
    npm run build
    if [[ "$mode" == full ]]; then npm run test:e2e; fi
    ;;
  android)
    cd "$root/android"
    ./gradlew :app:lintDevDebug :app:assembleDevDebug :app:assembleProdRelease
    if [[ "$mode" == full ]]; then ./gradlew :app:connectedDevDebugAndroidTest; fi
    ;;
  ios)
    cd "$root/ios"
    : "${IOS_DESTINATION:?Set IOS_DESTINATION to platform=iOS Simulator,id=<device UUID>}"
    xcodegen generate -s project.yml
    pod install
    action=build
    if [[ "$mode" == full ]]; then action=test; fi
    xcodebuild -workspace Starter.xcworkspace -scheme Starter -configuration Debug \
      -destination "$IOS_DESTINATION" -derivedDataPath "$root/.ailog/ios-derived" \
      -clonedSourcePackagesDirPath "$root/.ailog/swift-packages" \
      -skipPackagePluginValidation -parallel-testing-enabled NO CODE_SIGN_IDENTITY=- "$action"
    ;;
  contracts)
    "$root/scripts/contracts.sh" check
    ;;
  *) printf '%s\n' 'Use backend, spa, next, android, ios or contracts.' >&2; exit 2 ;;
esac
printf 'Verification passed: target=%s mode=%s\n' "$target" "$mode"
