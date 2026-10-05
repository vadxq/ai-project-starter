# Android development rules

This project can be copied on its own. Read the README first, use native tools, and resolve dependencies and contracts only from this project's configuration.

- Use Android 14 / minSdk 34 and Kotlin / Compose. Keep dev/prod flavors separate from debug/release build types. Build with the Gradle Wrapper.
- Generate API types from this project's OpenAPI input; do not edit generated code by hand. Use Rust username/password login and rotating refresh tokens. Store refresh tokens in native secure storage; never persist passwords.
- Preserve complete English and Chinese support and system / light / dark themes. Put error and accessibility text in native resources. Production and Release configurations must not allow local HTTP exceptions.
- Separate business state from network and authentication connectors. Cancelled tasks and logout must not restore previous users' data. Repeated clicks must not trigger duplicate writes.
- Run targeted compilation and static checks before local Emulator tests. Record evidence separately for the minimum and current system versions. Compilation does not prove real sign-in works.
- Do not automatically commit, push, sign, or publish to a store. Keep temporary files in .codex. Do not raise the minimum device version to work around build issues.
