# Changelog

All notable changes to FAZORA Media are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Complete UI/UX design system with brand tokens (obsidian, carbon, gold, ivory, steel, teal)
- FAZORA geometric F monogram and wordmark components
- Inline SVG icon system (30+ symbols)
- Component library: primitives, layout, media, player, audio components
- Service layer: playback, queue, library, settings, search, audio services
- Data models: Track, Album, Artist, Video, Playlist, Library aggregates
- Views: Home, Music, Album, Artist, Playlists, Video library, Video player, Now Playing, Queue, Search, Settings, Equalizer
- Application shell with collapsible sidebar, topbar, player dock, secondary panel
- Responsive design: desktop shell vs. mobile bottom navigation
- GitHub Actions CI/CD: format, clippy, tests, Linux/Windows/Android builds, security scans, releases
- Automated tests for models and service layer
- Documentation: DESIGN_SYSTEM, UI_UX_GUIDELINES, RESPONSIVE_DESIGN, ACCESSIBILITY

### Changed

- Fixed Cargo.toml default feature declaration (removed non-existent "webview2" feature)
- Fixed Cargo.toml dependency versions (removed incorrect optional sys crate versions)

## [0.1.0] - 2026-11-01

### Added

- Initial project setup
- Cargo.toml with Dioxus 0.5 dependencies
- Basic application entry point

## [0.0.1] - 2026-11-01

### Added

- Initial commit
