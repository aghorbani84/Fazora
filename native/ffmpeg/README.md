# FAZORA FFmpeg native build

Pinned upstream: FFmpeg 9.0.2. Initial target: Android API 26+, ARM64-v8a and x86_64. Build shared libraries only, with external-library autodetection disabled and Android MediaCodec/JNI enabled.

This directory is the native dependency build foundation, not a functioning player by itself. The Rust C-ABI adapter, audio sink, Android video surface, desktop output adapters, lifecycle integration, and UI wiring are separate implementation steps.

Run from Ubuntu with Android SDK/NDK installed:

```sh
export ANDROID_NDK_HOME="$ANDROID_HOME/ndk/28.2.13676358"
scripts/build-ffmpeg-android.sh arm64-v8a
scripts/build-ffmpeg-android.sh x86_64
```

The script verifies the upstream PGP signature using FFmpeg's published key, checks generated GPL/nonfree/shared configuration, and retains the exact source archive, signature, license files, configure command and build log alongside outputs.

Never add external codec libraries without exact license and patent review. Do not enable x264/x265, FDK-AAC, OpenSSL, or any external library by assumption.

Every distributed build must include the exact matching FFmpeg source, local patch diff, configure/toolchain metadata, third-party notices and SBOM. Add an About/Legal notice and source link next to release downloads. LGPL compliance and patent exposure are separate; this is not legal advice.
