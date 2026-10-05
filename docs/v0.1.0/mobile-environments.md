# v0.1.0 移动端开发与生产环境

状态：**已实现并通过本地验收**。2026-10-05 整理。Android / iOS 同时支持开发与生产环境；iOS 直接使用 XcodeGen 和 CocoaPods 的标准命令准备工程。需求对应 REQ-05 / REQ-06，验收以 [交付计划](delivery-plan.md) 为准。

## 1. 统一环境约定

| 项目      | 开发环境 dev                            | 生产环境 prod                  |
| --------- | --------------------------------------- | ------------------------------ |
| API 地址  | 开发 API 的明确配置值                   | 生产 HTTPS API 的明确配置值    |
| 应用标识  | 独立 dev bundle ID / applicationId      | 正式 bundle ID / applicationId |
| 应用名称  | 带 Dev 标识，便于同时安装辨认           | 正式名称                       |
| 本地 HTTP | 仅允许开发环境的 Debug 使用明确开发例外 | 所有编译模式均使用 HTTPS       |

dev / prod 决定连接的业务环境，Debug / Release 决定编译优化和调试行为，分别管理。生成或构建时选定环境，应用启动不读取隐藏的环境状态文件，不依赖开发者本机的 Python 配置程序。

公开 API 地址 和应用标识保存在本端版本化配置中。数据库密码、JWT_SECRET、签名私钥和个人证书不进入客户端配置。工程生成与依赖安装不需要签名；真机运行和 Archive 使用本机已配置的合法签名。

## 2. iOS 两条命令

在 `ios` 目录执行。前提是新机器已经安装适用版本的 Xcode、XcodeGen 和 CocoaPods；应用本身与依赖版本由本端文件声明。

开发环境：

```sh
xcodegen generate -s project.yml
pod install
```

生产环境：

```sh
xcodegen generate -s project.prod.yml
pod install
```

XcodeGen 生成 `Starter.xcodeproj`，CocoaPods 集成依赖并生成 `Starter.xcworkspace`。随后打开 workspace 开发和构建。切换环境时重新执行对应的两条命令，覆盖同一份生成工程，不维护两套容易混用的 Xcode 文件。

### 工程来源

```text
ios/
├── project.yml              # 开发环境入口
├── project.prod.yml         # 生产环境入口
├── project.base.yml         # XcodeGen include 的共同项目结构
├── Podfile                  # 固定 project / target 名称，普通第三方依赖
├── Podfile.lock             # 提交依赖锁文件，新机器沿用相同 Pods 版本
├── App/                     # 两个环境使用同一份 Swift / SwiftUI 源码
├── Packages/APIClient/      # 本地契约、官方 Swift OpenAPI package / plugin
├── Tests/
├── README.md
└── AGENTS.md
```

- `project.yml` / `project.prod.yml` 都 include `project.base.yml`；公共文件声明源码、targets、schemes、iOS 18.0、Swift 6 language mode 和依赖关系。
- 两个入口分别在应用 target 的 build settings 中声明本环境公开值，对 Debug / Release 均生效；dev 的 Release 仍连接开发环境，prod 的 Debug 仍连接生产环境。
- 工程名固定为 `Starter`，应用 target 也固定为 `Starter`；Podfile 显式声明 `project 'Starter.xcodeproj'` 和 `target 'Starter'`，两种入口都能直接 `pod install`。
- API 地址等公开值直接写在两个入口 YAML 的 target settings 中，公共 Info.plist 使用 build setting 占位符供构建时展开；不需要为切换环境额外生成配置文件。
- CocoaPods 按标准方式管理 Debug / Release 的依赖 xcconfig，环境值使用 target 的显式 build settings。每次重建工程后执行 pod install 恢复 Pods 集成，避免自定义 base configuration 覆盖 Pods 配置。
- 当前无 CocoaPods 第三方依赖，保留原生 Podfile / workspace 两步入口。已有 Apple Swift OpenAPI 工具使用 Xcode 原生 Swift Package 接入与官方 build plugin，在 Xcode 构建时处理；生成 workspace 不需要额外的手动 package 命令或代码生成包装。
- 两套环境的 Pods 依赖保持相同，使用同一个 Podfile.lock；依赖升级另行显式进行，不在切换环境时执行 pod update。
- `.xcodeproj`、workspace 工程内容、Pods 和编译缓存为生成产物。仅保留 workspace 内原生 `xcshareddata/swiftpm/Package.resolved` 锁文件；工程来源是 YAML、源码、Podfile / 锁文件和本端配置。

两条命令负责生成并准备对应环境的 workspace。首次 Swift Package 解析与应用编译由 Xcode 正常完成，不将工程生成等同于应用构建或签名验收。

## 3. Android 原生环境选择

在 app 的 Gradle 配置中建立 `environment` flavor dimension，包含 `dev` / `prod` 两个 product flavor。每个 flavor 管理本环境 applicationId、应用名称、API 地址 与必要资源；debug / release 为独立 build type。

使用 `app/src/dev` 和 `app/src/prod` 保存本环境资源 / manifest；公共业务与 UI 保存在 main。公开配置通过 Gradle BuildConfig / resValue 显式注入，设备运行时不会按本机变量猜测环境。

在 Android Studio 直接选择 Build Variant；命令行在 `android` 目录使用 Gradle Wrapper：

```sh
./gradlew :app:assembleDevDebug
./gradlew :app:assembleProdRelease
```

开发和生产版本使用不同标识，可同时安装。prodRelease 的签名从本机签名配置注入，密钥不提交；商店发布继续在首版范围之外。开发环境 Debug 才启用明确的本地 HTTP 例外，prodDebug 与全部 Release 变体均关闭。

## 4. 新机器与切换验收

- 从单独复制的 ios 工程执行开发两条命令和生产两条命令，分别生成可打开的同名 workspace，不执行自定义工程生成程序。
- dev → prod → dev 重复生成与 pod install 后，环境标识、API、bundle ID、Info.plist 和 Pods 配置均符合当前入口，不残留上一环境值。
- Debug / Release 分别检查环境值，证明环境选择与编译模式分离；开发和生产应用可同时安装。
- Android 执行 devDebug / prodRelease 生成与 Smoke，并检查其余变体的配置和网络例外。
- 最低平台仍为 Android 14 / API 34、iOS 18.0，多语言仍按本端资源完整覆盖。
- 本地执行实际编译、依赖安装、系统认证与开发服务联调。生产公开值需要替换为真实服务地址；未提供生产服务时只验收配置、网络限制和产物，不将占位域名记为真实生产联通。

`scripts` 仅保留必要的跨端契约和验证操作，优先调用标准 CLI。移动端工程生成、环境选择和依赖安装使用 XcodeGen、CocoaPods、Gradle 自身能力。
