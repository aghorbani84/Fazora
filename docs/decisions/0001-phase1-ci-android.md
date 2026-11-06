# ADR 0001: Phase 1 Android CI and APK pipeline

Date: 2026-11-06
Status: Proposed on branch `ci/android-apk`
Scope: CI/CD only; no UI, scanner, playback, permissions, or product-feature changes.

## Context and hard constraints

- Dioxus is pinned to 0.7.10.
- Its published manifest sets Rust MSRV to 1.83.0; the application previously declared 1.78.
- The project must emit a real APK for `aarch64-linux-android` and `x86_64-linux-android`, signed with an ephemeral debug key for testing.
- CI must build on every push, verify signatures/checksums, and attempt an x86_64 emulator install/launch smoke test.
- No production signing secret, tag, release, or main-branch mutation is allowed in this phase.

## Options and weighted evaluation

Weights: correctness/safety 0.30, maintainability 0.20, performance 0.20, reversibility 0.15, effort 0.15.

| Option | Correctness | Maintainability | Performance | Reversibility | Effort | Weighted |
|---|---:|---:|---:|---:|---:|---:|
| A. GitHub-hosted Ubuntu 24.04 + KVM + x86_64 emulator | 9 | 9 | 8 | 9 | 8 | 8.65 |
| B. External hosted runner with nested virtualization | 8 | 6 | 9 | 6 | 5 | 7.05 |
| C. Static APK/signature checks only; no emulator | 5 | 8 | 4 | 9 | 10 | 6.75 |

Sensitivity test: each weight varied individually by +/-20%, then weights renormalized. Option A remains the winner in all ten cases. Confidence: medium-high before actual runner execution; KVM availability is verified at runtime and the job fails rather than silently skipping the emulator.

## Decision

1. Pin Rust to stable 1.99.1; declare MSRV 1.83.0 to match Dioxus 0.7.10.
2. Commit Cargo.lock and use locked builds.
3. Set release profile to size-first: `opt-level = "z"`, full `lto = true`, `codegen-units = 1`, `strip = true`. This intentionally trades longer compile/link time for smaller release artifacts.
4. Keep `panic = "unwind"` explicitly (the Rust default) so panic diagnostics remain available to the smoke test and crash logs; do not optimize away unwind diagnostics with `panic = "abort"`.
5. Build APKs with Dioxus CLI 0.7.10 using the source-verified flags `dx bundle --android --release --package-types apk --target <triple> --out-dir dist --cargo-args=--locked`.
6. Install Android platform 35 and Build Tools 35.0.0, NDK r28c and JDK 17 explicitly. Do not rely on runner-image preinstalls.
7. Use GitHub-hosted Ubuntu 24.04 with a runtime `/dev/kvm` and `kvm-ok` gate. If unavailable, fail the smoke job and move it to a runner with documented KVM exposure; do not report emulator success.
8. Sign each CI APK with a temporary debug key. Different runs will not be update-compatible; uninstall an older CI build before installing a new run's APK. Persistent signing remains a later owner-controlled gate.

## Verified Dioxus 0.7.10 CLI surface

Source: https://github.com/DioxusLabs/dioxus/tree/v0.7.10/packages/cli/src
- `packages/cli/src/cli/bundle.rs`: `--package-types`, `--out-dir`; Android supports `PackageType::Apk` and `PackageType::Aab`.
- `packages/cli/src/cli/target.rs`: `--android`, `--release`, `--target`, `--cargo-args`.
- `packages/cli/src/build/request.rs`: `cargo_args` are shell-word parsed and appended to Cargo arguments; `--locked` is passed to Cargo.
- Android uses the CLI's Android bundle path; this phase does not add Kotlin source or modify the Android manifest.

## Action pins

Full commit SHAs were resolved through the GitHub commit API before use:
- checkout: `actions/checkout@08c6903cd8c0fde910a37f88322edcfb5dd907a8`
- setup Android: `android-actions/setup-android@9fc6c4e9069bf8d3d10b2204b1fb8f6ef7065407`
- setup NDK: `nttld/setup-ndk@afb4c9964b521afb97c864b7d40b11e6911bd410`
- setup Java: `actions/setup-java@c5195efecf7bdfc987ee8bae7a71cb8b11521c00`
- upload artifact: `actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02`
- download artifact: `actions/download-artifact@d3f86a106a0bac45b974a628896c90dbdf5c8093`
- Android emulator runner: `ReactiveCircus/android-emulator-runner@a421e43855164a8197daf9d8d40fe71c6996bb0d`

## Test target correction

The test modules are compiled into the auto-discovered library target via `src/lib.rs` and `#[cfg(test)] mod tests;`. No `[[test]]` target named `fazora_tests` exists. The correct commands are `cargo test --locked --workspace` or module-filtered `cargo test --locked --lib tests::<module>`.

## Reversal and risks

- Rust pin/MSRV: change `rust-toolchain.toml` and `Cargo.toml`; low reversal cost.
- Release profile: remove or tune profile keys; medium build-time/size trade-off.
- Emulator: replace runner label/action while keeping smoke script; low source impact.
- APK signing: debug key is intentionally ephemeral. Production update compatibility requires a persistent private key and explicit approval.
- The Dioxus release archive SHA-256 currently reused from the existing repository workflow must be revalidated against the upstream release checksum asset before treating the CLI download as independently verified. The binary version and URL are pinned; checksum provenance is a remaining review item.

## Evidence labels

[Fact] Dioxus 0.7.10 CLI source accepts the documented platform, target, package type, output directory, and extra Cargo arguments. Rust 1.99.1 is the official stable release published 2026-10-15. GitHub documents hardware acceleration for Android SDK tools on hosted Linux runners.

[Theory] The x86_64 APK will install and remain alive for 60 seconds on API 35; only the exact workflow run can establish this.

[Opinion] Full LTO and size optimization are suitable for a sideloadable test APK despite longer CI build times. Ephemeral debug signing is safer than storing a long-lived private key in Phase 1.
