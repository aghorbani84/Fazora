# ADR 0001 — Android audio playback engine

Date: 2026-11-05
Status: Accepted for Phase 1 Android implementation; desktop/iOS adapters remain separate decisions.

## Context

[Fact] The current `PlaybackService` only mutates Dioxus signals. It has no decoder, audio sink, Android media session, or SAF/MediaStore integration. `engine_available` was initialized to `true`, which incorrectly advertised a working engine.

[Fact] The application targets Android API 35 and currently builds with Dioxus 0.7.10. Dioxus documents Android native API access through JNI. AndroidX Media3 is Google's maintained media playback stack; the official release page lists 1.11.1 as stable (checked 2026-11-05).

## Options

Scoring is an engineering estimate, not a benchmark. Weights: correctness/safety 0.30, maintainability 0.20, performance 0.20, reversibility 0.15, implementation effort 0.15. Scores 0–10; higher is better.

| Option | Correctness | Maintainability | Performance | Reversibility | Effort | Weighted |
|---|---:|---:|---:|---:|---:|---:|
| AndroidX Media3 ExoPlayer + MediaSession | 9 | 9 | 9 | 8 | 7 | 8.55 |
| Android platform MediaPlayer | 7 | 6 | 8 | 8 | 9 | 7.45 |
| FFmpeg custom decoder/output pipeline | 8 | 5 | 8 | 5 | 3 | 6.20 |
| Do nothing | 0 | 10 | 0 | 10 | 10 | 5.00 |

Sensitivity: the Media3 option remains the leading choice under +/-20% perturbations of individual weights using these estimates. No measured benchmark is claimed.

## Decision

[Opinion] Use AndroidX Media3 ExoPlayer 1.11.1 (stable, not alpha/beta) as the Android playback backend for Phase 1. Use MediaSession for system integration and a foreground playback service only when background playback is implemented. Keep a Rust-facing platform-neutral command/event interface; isolate Android implementation behind JNI/Kotlin.

[Fact] Dioxus 0.7 documentation describes direct JNI access for Android. Official Android Media3 documentation shows the ExoPlayer dependency and current stable version:
- https://developer.android.com/jetpack/androidx/releases/media3
- https://developer.android.com/media/media3/exoplayer/hello-world
- https://dioxuslabs.com/learn/0.7/guides/platforms/mobile/

## Scope and constraints

Phase 1 minimum:
1. Android document picker using Storage Access Framework; retain URI permission when supported.
2. Play local audio from content:// URI without converting it to an unsafe filesystem path.
3. Real play/pause/stop/seek/volume and authoritative position/duration events.
4. Queue next/previous and end-of-track handling.
5. UI must show unavailable/error states honestly until the native engine reports readiness.
6. Tests for state transitions and Android adapter; CI build is not a substitute for device playback testing.

Out of scope for this ADR: video, FFmpeg integration, DSP/equalizer processing, gapless guarantees, cloud streaming, release publication, and persistent signing credentials.

## Risks / open validation

- [Theory] Media3 should provide a more maintainable Android playback path than a custom FFmpeg output stack; validate on API 26+ and the target HONOR device.
- Native Kotlin/Gradle integration into the Dioxus-generated Android project must be prototyped before adding the dependency broadly.
- Check transitive dependency licenses and package-size impact in CI.
- Never treat a successful APK build as proof of audio output.


## Native bridge implementation note

[Fact] Dioxus 0.7.10 ships the `manganis::ffi` macro and documents Kotlin FFI. The official v0.7.10 geolocation-native-plugin example uses a Gradle library module under a source folder and an `extern "Kotlin"` block. FAZORA follows that documented shape with a small Media3 wrapper. Numeric command values cross JNI as strings and are parsed on the Kotlin side to avoid a reported numeric-argument code-generation defect in earlier Manganis 0.7.x releases.

[Risk] A Dioxus issue reports Kotlin class lookup/classloader failures with `manganis::ffi` in 0.7.9 and 0.8.0-alpha.0. This is not proof that 0.7.10 fails, but it requires an actual Android launch test before treating the bridge as operational. The bridge is not marked ready merely because it compiles.

[Fact] The first adapter implementation exposes initialization, content/file URI loading, play, pause, stop, seek, volume, state snapshot, and release. SAF selection, persistent URI permission handling, MediaSession/background service, and UI integration remain separate work.
