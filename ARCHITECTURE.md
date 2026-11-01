# FAZORA Media Architecture

## Overview

FAZORA Media is a premium multimedia application built with Rust and Dioxus Labs.
The architecture follows a clean separation between UI state, playback state,
library state, settings state, and native engine state.

## High-Level Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library crate root
├── app.rs               # Root Dioxus component
├── routing.rs           # Application routes (dioxus-router)
├── state/               # Global application state container
│   └── app_state.rs     # AppState aggregate
├── playback.rs          # Playback service (engine abstraction)
├── queue.rs             # Queue service (playback queue)
├── library.rs           # Library service (media indexing)
├── settings.rs          # Settings service (persistence)
├── search.rs            # Search service (global search)
├── audio.rs             # Audio service (equalizer, DSP)
├── models/              # Data models (Track, Album, Artist, etc.)
├── components/          # Dioxus UI components
│   ├── branding/        # FAZORA logo, wordmark, icons
│   ├── layout/          # App shell, sidebar, topbar, navigation
│   ├── media/           # Artwork, cards, track rows, metadata
│   ├── player/          # Mini-player, controls, timeline, volume, queue
│   ├── audio/           # Equalizer, spectrum, lyrics, device selector
│   └── primitives/      # Button, toggle, slider, input, badge, etc.
├── design/              # Design system (CSS tokens, Rust constants)
│   ├── css.rs           # CSS custom properties and component styles
│   └── tokens.rs        # Rust-side mirror of design tokens
└── views/               # Page-level views (Home, Music, Album, etc.)
```

## Layered Architecture

### Layer 1: Data Models (`src/models/`)

Pure data structures with `serde` serialization. No Dioxus dependencies.
- `Track`, `Album`, `Artist`, `Video`, `Playlist`
- `Library`, `LibraryStats` (aggregates)
- `LyricLine` (synchronised lyrics)
- Methods: `formatted_duration()`, `subtitle()`, `progress_ratio()`, etc.

### Layer 2: Services (`src/*.rs`)

UI-facing service abstractions. Each service owns `Signal` state and exposes
methods for business logic. Services never touch native engine resources
directly — they delegate to engine adapters.

- `PlaybackService` — playback state machine (8 states)
- `QueueService` — playback queue with reorder, save-as-playlist
- `LibraryService` — media scanning, metadata, artwork access
- `SettingsService` — persisted application settings (JSON)
- `SearchService` — global search state
- `AudioService` — equalizer, output devices, DSP

### Layer 3: State Container (`src/state/`)

`AppState` aggregates all services into a single unit, exposed via Dioxus
context. Views access services through `use_context::<Signal<AppState>>()`.

### Layer 4: Components (`src/components/`)

Reusable Dioxus RSX components. Presentation-only — no business logic.
- **Branding**: FAZORA monogram, wordmark, SVG icon system
- **Layout**: App shell, sidebar (collapsible), topbar, bottom navigation
- **Media**: Artwork (with fallback), album/artist/track/video cards, metadata
- **Player**: Mini-player dock, playback controls, timeline, volume, queue panel
- **Audio**: Equalizer (10 bands), spectrum canvas, lyrics view, device selector
- **Primitives**: Button, icon button, toggle, slider, input, badge, divider, tooltip, card

### Layer 5: Views (`src/views/`)

Page-level views that compose components and services. Each view corresponds
to a route defined in `routing.rs`.

## Design System

Design tokens are defined in CSS (`src/design/css.rs`) and mirrored in Rust
(`src/design/tokens.rs`) for canvas drawing and layout math.

- **Colors**: Obsidian (#0B0D10), Carbon (#15191F), Champagne Gold (#D8B56A),
  Ivory (#F4F1E9), Steel (#89919D), Teal (#78CFC1)
- **Typography**: Manrope (brand/UI), IBM Plex Mono (technical),
  Vazirmatn (Persian/Arabic)
- **Motion**: 120–240ms transitions, ease-out easing, reduced-motion support
- **Layout**: 260px sidebar, 56px topbar, 88px player dock, 360px secondary panel

## State Flow

```
User Action → View Component → Service Method → Signal Update → UI Re-render
                                                        ↑
Native Engine Event → PlaybackService::set_state() ─────┘
```

Services are the only layer that can modify signals. Components trigger
service methods; services update signals; Dioxus re-renders affected views.

## Platform Abstraction

- **Linux**: WebKitGTK backend, libVLC/libmpv via FFI, MPRIS, desktop notifications
- **Windows**: WebView2 backend, libVLC/libmpv DLLs, system media controls
- **Android**: Dioxus mobile, JNI integration, MediaSession, Storage Access Framework

The UI layer is platform-agnostic. Platform-specific code is isolated in
engine adapters and build configuration.

## Dependency Boundaries

- UI components depend only on design tokens and models
- Services depend on models and Dioxus signals
- Views depend on components and services
- No circular dependencies
- Native engine dependencies are abstracted behind service interfaces
