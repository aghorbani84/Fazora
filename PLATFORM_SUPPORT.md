# Platform Support

## Supported Platforms

| Platform | Architecture | Target | Status | Backend |
|----------|-------------|--------|--------|---------|
| Linux | x86_64 | x86_64-unknown-linux-gnu | Supported | WebKitGTK |
| Linux | aarch64 | aarch64-unknown-linux-gnu | Supported | WebKitGTK |
| Windows | x86_64 | x86_64-pc-windows-msvc | Supported | WebView2 |
| Android | arm64-v8a | aarch64-linux-android | Planned | Dioxus mobile |
| Android | x86_64 | x86_64-linux-android | Planned | Dioxus mobile |

## Platform Notes

### Linux

- WebKitGTK rendering backend
- Requires `libwebkit2gtk-4.0-dev` and `libgtk-3-dev`
- MPRIS integration planned
- Desktop notifications planned
- Wayland and X11 compatible via WebKitGTK

### Windows

- WebView2 rendering backend
- MSVC toolchain required
- System media controls planned
- File associations planned
- High-DPI scaling supported via WebView2

### Android

- Dioxus mobile toolchain
- NDK r26b required
- JDK 17 required
- Storage Access Framework for file access
- MediaSession for background audio
- APK packaging not yet configured (native library builds only)

### iOS

iOS is **not currently supported**. Adding iOS support would require:

- macOS runner with Xcode
- Rust iOS targets: `aarch64-apple-ios`, `x86_64-ios-sim`
- Dioxus iOS/mobile toolchain configuration
- iOS app bundle packaging

## Platform-Specific Features

| Feature | Linux | Windows | Android |
|---------|-------|---------|---------|
| Desktop notifications | Planned | Planned | N/A |
| MPRIS | Planned | N/A | N/A |
| System media controls | Planned | Planned | MediaSession |
| File associations | Planned | Planned | SAF |
| Storage Access Framework | N/A | N/A | Planned |
| Background audio | N/A | N/A | Planned |

## Build Validation

Each platform build is validated in GitHub Actions:

- Linux: `build-linux.yml` (x86_64, aarch64)
- Windows: `build-windows.yml` (x86_64)
- Android: `build-android.yml` (arm64-v8a, x86_64)

A platform is considered supported only when its build and runtime behavior
are verified on CI.
