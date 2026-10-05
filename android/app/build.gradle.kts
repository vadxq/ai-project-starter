import org.openapitools.generator.gradle.plugin.tasks.GenerateTask

plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.plugin.compose")
    id("org.jetbrains.kotlin.plugin.serialization")
    id("org.openapi.generator")
}

android {
    namespace = "com.example.starter"
    compileSdk { version = release(37) { minorApiLevel = 1 } }
    defaultConfig {
        applicationId = "com.example.starter"
        minSdk = 34
        targetSdk = 37
        versionCode = 1
        versionName = "0.1.0"
        testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }
    flavorDimensions += "environment"
    productFlavors {
        create("dev") {
            dimension = "environment"
            applicationIdSuffix = ".dev"
            resValue("string", "app_name", "Items Dev")
            buildConfigField("String", "API_BASE_URL", "\"http://localhost:8080/\"")
        }
        create("prod") {
            dimension = "environment"
            resValue("string", "app_name", "Items")
            buildConfigField("String", "API_BASE_URL", "\"https://api.example.com/\"")
        }
    }
    buildTypes { release { isMinifyEnabled = true; isShrinkResources = true; proguardFiles(getDefaultProguardFile("proguard-android-optimize.txt"), "proguard-rules.pro") } }
    buildFeatures { compose = true; buildConfig = true; resValues = true }
    compileOptions { sourceCompatibility = JavaVersion.VERSION_21; targetCompatibility = JavaVersion.VERSION_21 }
    sourceSets["main"].kotlin.directories.add("build/generated/openapi/src/main/kotlin")
    packaging { resources.excludes += "/META-INF/{AL2.0,LGPL2.1}" }
}

kotlin { jvmToolchain(21) }

openApiGenerate {
    generatorName.set("kotlin")
    library.set("jvm-retrofit2")
    inputSpec.set(file("openapi/openapi.json").absolutePath)
    outputDir.set(layout.buildDirectory.dir("generated/openapi").get().asFile.absolutePath)
    apiPackage.set("com.example.starter.generated.api")
    modelPackage.set("com.example.starter.generated.model")
    typeMappings.set(mapOf("UUID" to "kotlin.String"))
    configOptions.set(mapOf("serializationLibrary" to "kotlinx_serialization", "useCoroutines" to "true", "useResponseAsReturnType" to "true", "dateLibrary" to "string", "omitGradleWrapper" to "true"))
    globalProperties.set(mapOf("apis" to "", "models" to "", "supportingFiles" to "CollectionFormats.kt", "apiTests" to "false", "modelTests" to "false", "apiDocs" to "false", "modelDocs" to "false"))
}
tasks.named("preBuild") { dependsOn(tasks.named<GenerateTask>("openApiGenerate")) }

dependencyLocking { lockAllConfigurations() }
dependencies {
    implementation(platform("androidx.compose:compose-bom:2026.09.00"))
    implementation("androidx.activity:activity-compose:1.12.4")
    implementation("androidx.compose.material3:material3")
    implementation("androidx.compose.ui:ui-tooling-preview")
    implementation("androidx.lifecycle:lifecycle-viewmodel-compose:2.10.0")
    implementation("androidx.lifecycle:lifecycle-runtime-compose:2.10.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.10.2")
    implementation("org.jetbrains.kotlinx:kotlinx-serialization-json:1.9.0")
    implementation("com.squareup.retrofit2:retrofit:3.0.0")
    implementation("com.squareup.retrofit2:converter-kotlinx-serialization:3.0.0")
    implementation("com.squareup.okhttp3:okhttp:4.12.0")
    implementation("com.google.crypto.tink:tink-android:1.18.0")
    androidTestImplementation(platform("androidx.compose:compose-bom:2026.09.00"))
    androidTestImplementation("androidx.test.ext:junit:1.3.0")
    androidTestImplementation("androidx.test:runner:1.7.0")
    // 与 runner 对齐，避免 Compose 传递引入的旧 Espresso 在新系统调用已移除的输入 API。
    androidTestImplementation("androidx.test.espresso:espresso-core:3.7.0")
    androidTestImplementation("androidx.test.uiautomator:uiautomator:2.3.0")
    androidTestImplementation("androidx.compose.ui:ui-test-junit4")
    debugImplementation("androidx.compose.ui:ui-test-manifest")
}
