# Swift iOS

[English](README.md) | [简体中文](README.zh-CN.md)

The minimum device version is iOS 18.0, using Swift 6. Install Xcode, XcodeGen 2.45+, and CocoaPods 1.16 first. After copying this directory on its own, generate the development environment with:

```sh
xcodegen generate -s project.yml
pod install
```

For production:

```sh
xcodegen generate -s project.prod.yml
pod install
```

Both entry points generate the same Starter.xcodeproj / Starter.xcworkspace. Open the workspace. To switch environments, rerun the corresponding two commands. Shared structure lives in project.base.yml; public API and bundle ID settings live in the entry YAML files. Pods share one lockfile. Replace the production placeholder domains.

There are currently no CocoaPods dependencies. Podfile and the two-step workspace generation entry remain available. The official Swift OpenAPI package resolves and generates code during the Xcode build without extra Python or generator steps. The Pods deployment target remains 18.

## Build and verification

```sh
xcodebuild -workspace Starter.xcworkspace -scheme Starter \
  -configuration Debug -destination 'generic/platform=iOS Simulator' \
  -skipPackagePluginValidation CODE_SIGNING_ALLOWED=NO build
```

Select a Simulator with destination. The Swift generator must compile and run on the macOS host, so do not set SDKROOT=iphonesimulator globally. Initial package resolution requires network access. The command above verifies compilation. For Simulator execution and XCUITest, replace `CODE_SIGNING_ALLOWED=NO` with `CODE_SIGN_IDENTITY=-` to use local ad hoc signing so Keychain works correctly. Configure physical-device signing locally.

`Starter.xcworkspace/xcshareddata/swiftpm/Package.resolved` retains the native SwiftPM lockfile; Pods use `Podfile.lock`. Copy both with this directory. Environment generation preserves them.

The development Simulator can access localhost:8080. Start Rust, create alice / starter-password through the backend guide, and sign in inside the app. Only dev Debug allows local HTTP. Release and prod Debug require HTTPS. Environment selection remains separate from build optimization.

Languages are defined in App/Localizable.xcstrings. English or Chinese is initially selected from the system language; in-app preferences persist. SwiftUI / Observation manage state, AuthSession handles password login, refresh, and logout, and Keychain stores the refresh token.

The local API input is `Packages/APIClient/Sources/APIClient/openapi.json`. The official build plugin generates Swift types; do not edit the output by hand. No other repository directory is required. Tests includes local XCUITest coverage; full verification requires Simulator results on iOS 18 and the current system version.
