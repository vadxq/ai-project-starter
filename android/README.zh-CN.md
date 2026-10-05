# Kotlin Android

[English](README.md) | 简体中文

本目录独立使用 Gradle Wrapper。设备下限 Android14 / API34，compile SDK37.1、target37，JDK21。Compose / Material3、ViewModel / StateFlow、JWT 认证、Tink Keystore 存储、Retrofit 和生成 API 类型。

在 Android Studio 打开本目录，安装声明的 Android SDK；命令行设置 ANDROID_HOME 和 JAVA_HOME，或由 Android Studio 生成本机 local.properties。

```sh
./gradlew :app:assembleDevDebug
./gradlew :app:assembleProdRelease
```

dev/prod 是 environment flavor，与 debug/release 分开。dev 的 applicationId 为 com.example.starter.dev，prod 为 com.example.starter。公开 API 配置位于 app/build.gradle.kts；生产占位地址需替换。Release 默认无签名，不包含商店发布。

开发 Emulator 转发 API 端口：

```sh
adb reverse tcp:8080 tcp:8080
```

启动 Rust API，按 backend 文档创建 alice / starter-password，然后安装 devDebug。应用内输入用户名密码；access token 在内存，refresh token 使用 Tink / Keystore 加密保存。仅 devDebug 对 localhost / 127.0.0.1 允许 HTTP，其余变体使用 HTTPS；devRelease 的开发地址也需 HTTPS。

## 契约与验证

本地输入 `app/openapi/openapi.json`，OpenAPI Generator 在编译前生成 Kotlin；生成结果在 app/build，禁止手改。首次或显式依赖升级执行 `./gradlew :app:assembleDevDebug --write-locks` 并审查锁文件，常规构建沿用锁定版本。

```sh
./gradlew :app:openApiGenerate
./gradlew :app:lintDevDebug
./gradlew :app:assembleDevDebug :app:assembleProdRelease
./gradlew :app:connectedDevDebugAndroidTest
```

原生 strings / plurals 位于 values 与 values-zh-rCN；应用语言使用 LocaleManager。复制本端后不需要 Web 工程或根配置。新增语言补资源与 locales_config，保持错误和无障碍文案完整。
