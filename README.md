# FAZORA Media

Premium multimedia player for Linux, Windows, and Android.
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
| CI/CD (GitHub Actions) | Complete |
| Automated Tests | Partial (service layer + models) |
| Linux Build | Configured |
| Windows Build | Configured |
| Android Build | Configured |

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
- **GitHub Actions CI/CD** — Format, clippy, tests, Linux/Windows/Android builds, security scans, releases

## Supported Platforms

| Platform | Target | Backend |
|----------|--------|---------|
| Linux | x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu | WebKitGTK |
| Windows | x86_64-pc-windows-msvc | WebView2 |
| Android | aarch64-linux-android, x86_64-linux-android | Dioxus mobile |

## Installation

### From Source

Requires Rust 1.78+ and the appropriate system dependencies.

#### Linux

```bash
# Install system dependencies
sudo apt-get install libwebkit2gtk-4.0-dev libgtk-3-dev

# Build
cargo build --release

# Run
cargo run --release
```

#### Windows

```bash
# Build with MSVC toolchain
cargo build --release

# Run
cargo run --release
```

#### Android

```bash
# Install Android SDK, NDK (r26b), and JDK 17
# Add Rust Android targets
rustup target add aarch64-linux-android x86_64-linux-android

# Build
cargo build --release --target aarch64-linux-android
```

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
| [docs/UI_UX_GUIDELINES.md](./docs/UI_UX_GUIDELINES.md) | UI/UX design guidelines |
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
- Android APK packaging requires Dioxus mobile toolchain configuration
- iOS is not currently a target platform

## License

Proprietary — FAZORA Media. See [LICENSE](./LICENSE) for details.

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for contribution guidelines.
