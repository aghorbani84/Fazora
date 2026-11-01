# Fazora

FAZORA Media — a premium multimedia application for Linux, Windows and Android.
Built with **Rust + Dioxus Labs**.

> Sound. Motion. Precision.

## Status

The repository contains the full UI/UX implementation: design system, component
library, desktop layouts, music and video library screens, Now Playing interface,
queue/playlist engine, lyrics, equalizer, settings, responsive states, loading
and error states, and accessibility documentation. See `docs/` for the design
deliverables.

## Tech stack

- **Language:** Rust (edition 2021)
- **Framework:** Dioxus 0.5 (stable) + dioxus-desktop
- **Routing:** dioxus-router
- **Platforms:** Linux (WebKitGTK), Windows (WebView2), Android (Dioxus mobile/later)

## Build (desktop)

Requires the Rust toolchain (1.78+) and the desktop webview dependencies:

### Linux

```bash
sudo apt-get install libwebkit2gtk-4.0-dev libgtk-3-dev
cargo build --release
```

### Windows

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

## Repository layout

```
src/            Application source (services, models, components, views, design)
docs/           Design deliverables (DESIGN_SYSTEM, UI_UX guidelines,
                RESPONSIVE design, ACCESSIBILITY)
assets/         Brand assets, icons, artwork placeholders
```

## Design tokens

Brand colors (source of truth: `src/design/tokens.css`):

- Obsidian `#0B0D10` — foundation
- Carbon `#15191F` — panels / navigation
- Champagne Gold `#D8B56A` — focus, selected, active
- Ivory `#F4F1E9` — primary typography
- Steel `#89919D` — secondary information
- Teal `#78CFC1` — playback progress / positive status

## License

Proprietary — FAZORA Media.