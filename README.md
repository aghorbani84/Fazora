# FAZORA Media

> **Sound. Motion. Precision.** — a multimedia player UI for desktop and mobile, built with Rust and Dioxus.

[![CI](https://github.com/aghorbani84/Fazora/actions/workflows/ci.yml/badge.svg)](https://github.com/aghorbani84/Fazora/actions/workflows/ci.yml)
[![Latest release](https://img.shields.io/github/v/release/aghorbani84/Fazora)](https://github.com/aghorbani84/Fazora/releases)
![Rust](https://img.shields.io/badge/Rust-1.78%2B-DEA584?logo=rust&logoColor=black)
![Dioxus](https://img.shields.io/badge/Dioxus-0.7-FF4444)
![Status](https://img.shields.io/badge/status-in%20development-orange)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue)](LICENSE)

## Overview

FAZORA is a music and video player application written in Rust with [Dioxus](https://dioxuslabs.com/), targeting Linux, Windows, Android and iOS from one codebase. The repository contains a complete design system, a component library, the application shell and views, and the state/service layer (playback, queue, library, search, audio settings) with unit tests.

> [!NOTE]
> **Project status:** the interface and service layer are implemented, but a **native media engine (decoding / audio output / video rendering) is not integrated yet** — the video surface is a placeholder and library scanning is not wired to disk. Treat current builds as a UI preview, not a working media player. See [Known limitations](#known-limitations).

## Features

**Implemented in the UI and state layer**

- **Design system** — Obsidian / Carbon / Champagne Gold palette, Manrope and IBM Plex Mono typography, Vazirmatn for Persian/Arabic text
- **App shell** — collapsible sidebar, top bar, content area, persistent mini-player dock; bottom navigation on narrow screens
- **Views** — Home, Music library, Album, Artist, Video library, Video player, Now Playing, Queue, Playlists, Search and results, Equalizer, Settings
- **Playback state** — play/pause, seek, volume and mute, repeat modes, shuffle
- **Queue** — add, remove, reorder, shuffle and save as playlist
- **Equalizer model** — 10-band EQ with presets (Flat, Bass Boost, Vocal, Classical, Electronic, Jazz, Rock, Custom), preamp, crossfade and gapless settings
- **Now Playing** — artwork-focused layout with queue and lyrics panels
- **Settings** — categorized settings persisted to disk with path validation
- **Security helpers** — media-directory validation with path-traversal and symlink-escape protection

**Not yet implemented** — real audio/video decoding and output, on-disk media scanning, and synced lyrics data.

## Tech Stack

| Area | Technology |
| --- | --- |
| Language | Rust (edition 2021, MSRV 1.78) |
| UI framework | Dioxus 0.7 (desktop + mobile, router) |
| Desktop renderer | WebKitGTK (Linux) · WebView2 (Windows) |
| Async / utilities | Tokio, Serde, Chrono, UUID, tracing |
| CI/CD | GitHub Actions — fmt, clippy, check, tests, per-platform builds, security scans, tagged releases |

## Getting Started

### Prerequisites

- Rust 1.78 or newer
- **Linux:** `libwebkit2gtk-4.1-dev` and `libgtk-3-dev`
- **Windows:** MSVC toolchain (WebView2 is preinstalled on Windows 10/11)

### Run on desktop

```bash
git clone https://github.com/aghorbani84/Fazora.git
cd Fazora

# Linux only
sudo apt-get install libwebkit2gtk-4.1-dev libgtk-3-dev

cargo run --release
```

### Mobile builds

Mobile builds use the Dioxus CLI 0.7.10:

```bash
cargo install dioxus-cli --version 0.7.10 --locked

# Android (needs Android SDK, NDK r28c+, JDK 17)
rustup target add aarch64-linux-android
dx bundle --android --release --package-types apk --target aarch64-linux-android

# iOS (needs macOS + Xcode) — produces an unsigned .app
rustup target add aarch64-apple-ios
dx bundle --ios --release --package-types app --target aarch64-apple-ios
```

Pre-built Android APKs (arm64-v8a and x86_64, with SHA-256 checksums) are attached to [GitHub Releases](https://github.com/aghorbani84/Fazora/releases). CI signs them with a temporary key, so uninstall an older build before installing a newer one.

### Development

```bash
cargo fmt --all
cargo clippy --workspace --all-targets
cargo test                 # all unit tests
cargo test playback        # filter, e.g. models, queue, library, settings, search, audio
```

More detail: [BUILDING.md](BUILDING.md) · [DEVELOPMENT.md](DEVELOPMENT.md) · [TESTING.md](TESTING.md) · [PLATFORM_SUPPORT.md](PLATFORM_SUPPORT.md)

## Project Structure

```text
Fazora/
├── src/
│   ├── main.rs, app.rs, routing.rs
│   ├── design/          # Design tokens and generated CSS
│   ├── components/      # Primitives, layout, media cards, player, audio, branding
│   ├── views/           # Screens (home, music, video, now playing, settings…)
│   ├── models/          # Track, album, artist, video, playlist, library
│   ├── state/           # Global app state
│   ├── playback.rs, queue.rs, library.rs, search.rs, audio.rs, settings.rs
│   ├── security.rs      # Path validation helpers
│   └── tests/           # Unit tests per service
├── assets/              # Icon sprite
├── docs/                # Design system, UI/UX, accessibility, decisions
└── .github/workflows/   # CI, platform builds, security, release
```

## Known Limitations

- Native multimedia engine integration (e.g. libmpv / libVLC) is pending — no media is decoded or played yet
- Library scanning and metadata indexing are not connected to the filesystem
- Android/iOS builds confirm packaging only, not playback
- The iOS bundle is unsigned and needs Apple signing/provisioning to install on a device

## Documentation

[ARCHITECTURE.md](ARCHITECTURE.md) · [SECURITY.md](SECURITY.md) · [CHANGELOG.md](CHANGELOG.md) · [CONTRIBUTING.md](CONTRIBUTING.md) · [docs/DESIGN_SYSTEM.md](docs/DESIGN_SYSTEM.md) · [docs/ACCESSIBILITY.md](docs/ACCESSIBILITY.md)

## License

Released under the [MIT License](LICENSE).

## Author

Built by [@aghorbani84](https://github.com/aghorbani84).
