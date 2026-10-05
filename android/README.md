# Kotlin Android

[English](README.md) | [简体中文](README.zh-CN.md)

This directory uses its own Gradle Wrapper. The minimum device version is Android 14 / API 34, with compile SDK 37.1, target 37, and JDK 21. The stack includes Compose / Material 3, ViewModel / StateFlow, JWT authentication, Tink Keystore storage, Retrofit, and generated API types.

Open this directory in Android Studio and install the declared Android SDK. For command-line builds, set ANDROID_HOME and JAVA_HOME, or let Android Studio generate your local local.properties.

```sh
./gradlew :app:assembleDevDebug
./gradlew :app:assembleProdRelease
```

dev/prod are environment flavors, separate from debug/release build types. The dev applicationId is com.example.starter.dev; prod uses com.example.starter. Public API settings live in app/build.gradle.kts. Replace the production placeholder URLs. Release builds are unsigned by default; store publishing is outside the scope.

Forward the API port for the development Emulator:

```sh
adb reverse tcp:8080 tcp:8080
```

Start the Rust API, create alice / starter-password through the backend guide, then install devDebug. Sign in inside the app with a username and password. Access tokens stay in memory; Tink / Keystore encrypt refresh tokens. Only devDebug allows HTTP for localhost / 127.0.0.1. All other variants require HTTPS, including development URLs used by devRelease.

## Contract and verification

The local input is `app/openapi/openapi.json`. OpenAPI Generator produces Kotlin before compilation, with output in app/build; do not edit it by hand. For initial dependency locking or an explicit upgrade, run `./gradlew :app:assembleDevDebug --write-locks` and review the lockfiles. Regular builds use the locked versions.

```sh
./gradlew :app:openApiGenerate
./gradlew :app:lintDevDebug
./gradlew :app:assembleDevDebug :app:assembleProdRelease
./gradlew :app:connectedDevDebugAndroidTest
```

Native strings and plurals live in values and values-zh-rCN; app language selection uses LocaleManager. A standalone copy needs no Web project or root configuration. When adding a language, update resources and locales_config, including all error and accessibility text.
