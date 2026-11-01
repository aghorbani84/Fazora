# Testing

## Test Suite Overview

FAZORA Media has automated tests covering:

- **Data models** — Track, Album, Artist, Video, Playlist, Library
- **Playback service** — State transitions, play/pause, seek, volume, mute, repeat, shuffle
- **Queue service** — Add, remove, reorder, save-as-playlist, move next/previous
- **Library service** — Media directories, scan status, set_library
- **Settings service** — Recent queries, defaults
- **Search service** — Query, results, loading state
- **Audio service** — Equalizer bands, presets, preamp, devices

## Running Tests

```bash
# Run all tests
cargo test

# Run all tests with output
cargo test -- --nocapture

# Run a specific test binary
cargo test --test fazora_tests

# Run a specific test module
cargo test --test fazora_tests models
cargo test --test fazora_tests playback
cargo test --test fazora_tests queue
cargo test --test fazora_tests library
cargo test --test fazora_tests settings
cargo test --test fazora_tests search
cargo test --test fazora_tests audio

# Run a specific test function
cargo test test_track_formatted_duration
```

## Test Structure

Tests are located in `src/tests/`:

| File | Description |
|------|-------------|
| `mod.rs` | Module declarations |
| `models.rs` | Data model tests (pure logic) |
| `playback.rs` | Playback service tests |
| `queue.rs` | Queue service tests |
| `library.rs` | Library service tests |
| `settings.rs` | Settings service tests |
| `search.rs` | Search service tests |
| `audio.rs` | Audio service tests |

## Writing Tests

### Model Tests

Model tests are pure logic and don't require Dioxus:

```rust
#[test]
fn track_formatted_duration() {
    let track = Track { duration_sec: Some(125), .. };
    assert_eq!(track.formatted_duration(), Some("2:05".to_string()));
}
```

### Service Tests

Service tests use Dioxus's testing render scope because services create
`Signal`s via `use_signal()`:

```rust
#[test]
fn playback_play_toggle() {
    let mut scope = dioxus::testing::RenderScope::new();
    let playback = scope.create_service(|| PlaybackService::init());

    playback.set_state(PlaybackState::Paused { track: track.clone() });
    playback.play_toggle();

    assert!(playback.is_playing());
}
```

### Adding New Tests

1. Add the test to the appropriate file in `src/tests/`
2. Run `cargo test` to verify
3. Ensure `cargo clippy` passes with `-D warnings`

## CI Testing

GitHub Actions runs `cargo test --workspace` on every push and pull request.
Test results are available in the CI workflow run artifacts.

## Coverage

There is no enforced coverage threshold, but all new functionality should
include tests. Focus on:

- State transitions
- Edge cases (empty collections, invalid input)
- Error handling
- Boundary conditions
