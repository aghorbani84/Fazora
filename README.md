# FAZORA Media

Premium multimedia player for Linux, Windows, Android, and iOS.
Built with **Rust + Dioxus Labs**.

> **Sound. Motion. Precision.**

## Project Status

| Component | Status |
|-----------|--------|
| UI/UX Design System | Complete |
| Component Library | Complete |
| Service Layer | Complete |
| Desktop Views (Home, Music, Video, etc.) | Complete |
| Android Views | Planned |
| CI/CD (GitHub Actions) | Configured |
| Automated Tests | Partial (service layer + models) |
| Linux Build | Configured |
| Windows Build | Configured |
| Android Build | APK for ARM64 and x86_64 |
| iOS Build | Unsigned ARM64 .app bundle |

## Features

- **Premium design system** — Obsidian/Carbon/Champagne Gold color palette, Manrope typography, IBM Plex Mono for technical data, Vazirmatn for Persian/Arabic
- **Application shell** — Collapsible sidebar, topbar, main content area, persistent player dock, optional secondary panel
- **Music library** — Album/artist/track views, grid and list displays, search, filter, sort
- **Video library** — Thumbnail grid, continue watching, favorites, metadata
- **Now Playing** — Expanded player with artwork focus, waveform seekbar, queue and lyrics panels
- **Persistent mini-player** — Dock at bottom with artwork, title, controls, progress, volume
- **Queue management** — Add, remove, reorder, save as playlist, shuffle, repeat
- **Global search** — Tracks, artists, albums, videos, playlists
- **Equalizer (Soundstage)** — 10-band EQ, presets, preamp, DSP controls
- **Lyrics** — Synchronised lyrics with current-line highlighting and auto-scroll
- **Video player** — Immersive playback with subtitles, audio tracks, speed, aspect ratio
- **Settings** — General, appearance, audio, video, library, playback, advanced, about
- **Responsive design** — Desktop shell vs. mobile bottom navigation
- **Accessibility** — Keyboard navigation, screen reader labels, focus indicators, reduced motion
- **GitHub Actions CI/CD** — Format, clippy, tests, Linux/Windows/Android/iOS builds, security scans, automated releases

## Supported Platforms

| Platform | Target | Backend / Package |
|----------|--------|-------------------|
| Linux | x86_64-unknown-linux-gnu | WebKitGTK; TAR.GZ release package |
| Windows | x86_64-pc-windows-msvc | WebView2; ZIP release package |
| Android | aarch64-linux-android, x86_64-linux-android | Dioxus mobile; signed APKs |
| iOS | aarch64-apple-ios | Dioxus mobile; unsigned .app bundle ZIP |

## Installation

### From Source

Requires Rust 1.78+ and the appropriate system dependencies.

#### Linux

```bash
# Install system dependencies
sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev

# Build
cargo build --release

# Run
cargo run --release
```

#### Windows

```powershell
# Build with MSVC toolchain
cargo build --release

# Run
cargo run --release
```

#### Android

Install Android SDK command-line tools, Android NDK r28c or newer, JDK 17, Rust, and the Dioxus CLI 0.7.10. Then build a real APK (not just a Rust shared library):

```bash
rustup target add aarch64-linux-android
cargo install dioxus-cli --version 0.7.10 --locked
dx bundle --android --release --package-types apk --target aarch64-linux-android
```

The Android GitHub Actions workflow builds installable APKs for `arm64-v8a` and `x86_64`, verifies their signatures, and uploads each APK with a SHA-256 checksum. CI uses a temporary signing key, so APKs from separate workflow runs have different signing identities; uninstall an older CI build before installing a new one. For production updates, configure a persistent private signing key in GitHub Actions secrets.

#### iOS

An iOS ARM64 app bundle is built on macOS with Xcode and Dioxus CLI 0.7.10:

```bash
rustup target add aarch64-apple-ios
cargo install dioxus-cli --version 0.7.10 --locked
dx bundle --ios --release --package-types app --target aarch64-apple-ios
```

The CI and Release workflows publish an unsigned .app bundle ZIP with a SHA-256 checksum. It is not directly installable on a physical iPhone or publishable to the App Store; Apple code signing and provisioning are required for distribution.

### GitHub Releases

Push a version tag matching `v*` (for example, `v0.1.0`) after the release workflow is merged into the default branch. GitHub Actions builds all platform packages and publishes them together in the repository's Releases page. Each package includes a SHA-256 checksum.

Release assets:
- Windows x86_64: ZIP
- Linux x86_64: TAR.GZ (requires compatible WebKitGTK/GTK system libraries)
- Android ARM64 and x86_64: APK
- iOS ARM64: unsigned .app bundle ZIP

## Development Setup

```bash
# Clone the repository
git clone https://github.com/aghorbani84/Fazora.git
cd Fazora

# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Format code
cargo fmt

# Run clippy
cargo clippy --workspace --all-targets

# Run tests
cargo test

# Build
cargo build --release
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test module
cargo test --test fazora_tests models
cargo test --test fazora_tests playback
cargo test --test fazora_tests queue
cargo test --test fazora_tests library
cargo test --test fazora_tests settings
cargo test --test fazora_tests search
cargo test --test fazora_tests audio
```

See [TESTING.md](./TESTING.md) for details.

## Documentation

| Document | Description |
|----------|-------------|
| [ARCHITECTURE.md](./ARCHITECTURE.md) | System architecture and component design |
| [SECURITY.md](./SECURITY.md) | Security model and audit findings |
| [CONTRIBUTING.md](./CONTRIBUTING.md) | Contribution guidelines |
| [CHANGELOG.md](./CHANGELOG.md) | Release history |
| [INSTALLATION.md](./INSTALLATION.md) | Installation instructions |
| [BUILDING.md](./BUILDING.md) | Build instructions per platform |
| [DEVELOPMENT.md](./DEVELOPMENT.md) | Development workflow |
| [TESTING.md](./TESTING.md) | Testing guide |
| [PLATFORM_SUPPORT.md](./PLATFORM_SUPPORT.md) | Platform compatibility matrix |
| [ANDROID_STORAGE.md](./ANDROID_STORAGE.md) | Android storage access |
| [FILE_OPERATIONS.md](./FILE_OPERATIONS.md) | File operation security |
| [docs/DESIGN_SYSTEM.md](./docs/DESIGN_SYSTEM.md) | Design tokens and brand identity |
| [docs/UI_UX_GUIDELINES.md](./docs/UI_UX_GUIDELINES.md) | UI/UX guidelines |
| [docs/RESPONSIVE_DESIGN.md](./docs/RESPONSIVE_DESIGN.md) | Responsive design specifications |
| [docs/ACCESSIBILITY.md](./docs/ACCESSIBILITY.md) | Accessibility requirements |

## Security Model

- All media files and remote streams are treated as untrusted
- No hardcoded credentials; secrets managed via GitHub environment secrets
- Native engine resources released safely on shutdown
- Path traversal and symlink escape protection for file operations
- See [SECURITY.md](./SECURITY.md) for the full security policy

## Known Limitations

- Rust toolchain required for building (not available in all sandboxes)
- Native multimedia engines (libVLC, libmpv) integration pending validation
- Android media scanning and native playback integrations still require device-level validation; CI confirms packaging and APK signature, not full playback functionality
- iOS release bundle is unsigned and requires Apple signing/provisioning for device installation or App Store distribution

## License

Proprietary — FAZORA Media. See [LICENSE](./LICENSE) for details.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for contribution guidelines.
