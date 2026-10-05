# Swift iOS

[English](README.md) | 简体中文

设备下限 iOS18.0，Swift6。先安装 Xcode、XcodeGen2.45+ 和 CocoaPods1.16。独立复制本目录后，开发环境只需：

```sh
xcodegen generate -s project.yml
pod install
```

生产环境：

```sh
xcodegen generate -s project.prod.yml
pod install
```

两种入口生成同一个 Starter.xcodeproj / Starter.xcworkspace。打开 workspace；切换环境重新执行对应的两条命令。公共结构在 project.base.yml，公开 API / bundle ID 在入口 YAML，Pods 共用一个锁文件。生产域名是待替换配置。

目前没有 CocoaPods 第三方依赖，保留 Podfile 与两步 workspace 生成入口。官方 Swift OpenAPI package 在 Xcode 构建时解析和生成，无额外 Python / 生成器步骤；Pods 的部署下限仍为 18。

## 构建与验证

```sh
xcodebuild -workspace Starter.xcworkspace -scheme Starter \
  -configuration Debug -destination 'generic/platform=iOS Simulator' \
  -skipPackagePluginValidation CODE_SIGNING_ALLOWED=NO build
```

使用 destination 选择 Simulator；Swift 生成器需要在 macOS 主机编译执行，因此不要全局设置 SDKROOT=iphonesimulator。首次包解析需要网络。上述命令验证编译；模拟器运行及 XCUITest 应将 `CODE_SIGNING_ALLOWED=NO` 替换为 `CODE_SIGN_IDENTITY=-`，使用本地 ad hoc 签名，使 Keychain 能正常工作。真机签名由本机配置。

`Starter.xcworkspace/xcshareddata/swiftpm/Package.resolved` 保留原生 SwiftPM 锁文件；Pods 使用 `Podfile.lock`。两者都随本目录复制，环境生成不会清除它们。

开发 Simulator 可访问 localhost:8080。启动 Rust API，按 backend 文档创建 alice / starter-password，应用内用户名密码登录。仅 dev Debug 允许本地 HTTP；Release 与 prod Debug 使用严格 HTTPS 配置，环境与编译优化仍独立。

语言在 App/Localizable.xcstrings，初始按系统选择中英文；应用内偏好持久化。SwiftUI / Observation 管理状态，AuthSession 执行密码登录 / 刷新 / 退出，Keychain 保存 refresh token。

本地 API 输入 `Packages/APIClient/Sources/APIClient/openapi.json`，通过官方 build plugin 生成 Swift 类型，不手改输出。无需仓库其他目录。Tests 包含本地 XCUITest；完整运行以 iOS18 和当前系统的 Simulator 结果为准。
