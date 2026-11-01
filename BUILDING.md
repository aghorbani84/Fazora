# Building FAZORA Media

## Overview

FAZORA Media uses Cargo as its build system. The application targets Linux,
Windows, and Android with platform-specific rendering backends.

## Build Commands

### Linux (WebKitGTK backend)

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Build for aarch64
cargo build --release --target aarch64-unknown-linux-gnu
```

Required system packages: `libwebkit2gtk-4.0-dev libgtk-3-dev`

### Windows (WebView2 backend)

```bash
# Debug build
cargo build

# Release build
cargo build --release
```

Required: MSVC toolchain (`rustup default stable`)

### Android (Dioxus mobile)

```bash
# Add targets
rustup target add aarch64-linux-android x86_64-linux-android

# Build native library
cargo build --release --target aarch64-linux-android
cargo build --release --target x86_64-linux-android
```

Required: Android SDK, NDK r26b, JDK 17

## Build Profiles

| Profile | Use Case |
|---------|----------|
| `debug` | Development, faster compile times |
| `release` | Distribution, optimized binary |

## Features

FAZORA uses default Dioxus features. Platform-specific backends are
selected automatically by dioxus-desktop based on the target OS.

## Reproducible Builds

To ensure reproducible builds:

1. Commit `Cargo.lock`
2. Pin the Rust toolchain version
3. Use verified native library versions
4. Record compiler and toolchain versions

## CI Builds

GitHub Actions builds all platforms on every push and pull request:

- Linux x86_64 and aarch64
- Windows x86_64
- Android arm64-v8a and x86_64

See `.github/workflows/` for workflow definitions.

## Binary Size

Release builds are optimized for size. Use `cargo build --release` for
distribution. Strip debug symbols if further size reduction is needed:

```bash
strip target/release/fazora
```
