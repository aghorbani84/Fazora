# Development

## Setup

```bash
# Clone the repository
git clone https://github.com/aghorbani84/Fazora.git
cd Fazora

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
cargo --version
rustc --version
```

## Daily Workflow

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace --all-targets

# Run tests
cargo test

# Build
cargo build --release

# Run application
cargo run --release
```

## Project Structure

```
src/
├── main.rs              # Binary entry point
├── lib.rs               # Library crate root
├── app.rs               # Root component
├── routing.rs           # Route definitions
├── state/               # Global state container
├── playback.rs          # Playback service
├── queue.rs             # Queue service
├── library.rs           # Library service
├── settings.rs          # Settings service
├── search.rs            # Search service
├── audio.rs             # Audio service
├── models/              # Data models
├── components/          # UI components
│   ├── branding/        # Logo, icons
│   ├── layout/          # Shell, sidebar, navigation
│   ├── media/           # Cards, artwork
│   ├── player/          # Player components
│   ├── audio/           # Equalizer, lyrics
│   └── primitives/      # Button, input, etc.
├── design/              # Design system
│   ├── css.rs           # CSS tokens
│   └── tokens.rs        # Rust token constants
└── views/               # Page views
```

## Adding a New Component

1. Create the component file in the appropriate `src/components/` subdirectory
2. Export it from the subdirectory's `mod.rs`
3. Add `pub mod component_name;` to `src/components/mod.rs`
4. Use it in views with full path: `crate::components::...::ComponentName {}`

## Adding a New View

1. Create the view file in `src/views/`
2. Export it from `src/views/mod.rs`
3. Add a route variant to `src/routing.rs`
4. Add a match arm to `resolve_component()` in `routing.rs`

## Adding a New Service

1. Create the service file in `src/`
2. Implement the service with `Signal` state
3. Add the service to `AppState` in `src/state/app_state.rs`
4. Export from `src/lib.rs` if needed externally

## Design Tokens

Design tokens are defined in two places:

- `src/design/css.rs` — CSS custom properties (used in RSX styles)
- `src/design/tokens.rs` — Rust constants (used in canvas/layout math)

Changes must be mirrored in both files.

## Testing During Development

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Watch for changes (requires cargo-watch)
cargo watch -x test
```

## Debugging

```bash
# Enable debug logging
FAZORA_LOG=debug cargo run --release

# Enable Dioxus logging
RUST_LOG=dioxus=debug cargo run --release
```

## Commit Guidelines

- Use conventional commits: `feat:`, `fix:`, `docs:`, `refactor:`, `test:`
- Reference issues in commit messages
- Keep commits atomic and focused
