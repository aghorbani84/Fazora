plugins {
    id("com.android.library") version "8.7.3"
    id("org.jetbrains.kotlin.android") version "2.0.21"
}

android {
    namespace = "com.fazora.media"
    compileSdk = 34

    defaultConfig {
        minSdk = 24
        consumerProguardFiles("consumer-rules.pro")
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }
}

dependencies {
    // The Dioxus 0.7.10-generated host app currently compiles against API 34.
    // Media3 1.4.1 is the stable release compatible with that compile SDK.
    // Keep this plugin's dependencies within the host app's AAR metadata floor.
    implementation("androidx.media3:media3-exoplayer:1.4.1")
}
