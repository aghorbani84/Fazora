# ADR 0001: Phase 1 Android CI and APK pipeline

Date: 2026-11-06
Status: Proposed on branch `ci/android-apk`
Scope: CI/CD only; no UI, scanner, playback, permissions, or product-feature changes.

## Context and hard constraints

- Dioxus is pinned to 0.7.10.
- Its published manifest sets Rust MSRV to 1.83.0; the application previously declared 1.78.
- The project must emit a real APK for `aarch64-linux-android` and `x86_64-linux-android`, signed with an ephemeral debug key for testing.
- CI must build on every push, verify signatures/checksums, and attempt an x86_64 Android 15 16 KiB page-size emulator install/launch smoke test.
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
3. Verify APK ZIP alignment with `zipalign -P 16 -f 4` and inspect each packaged `.so` with NDK `llvm-objdump` to require ELF LOAD alignment of at least `2**14`. This is required for Android 15+ 16 KiB page-size compatibility. Set release profile to size-first: `opt-level = "z"`, full `lto = true`, `codegen-units = 1`, `strip = true`. This intentionally trades longer compile/link time for smaller release artifacts.
4. Keep `panic = "unwind"` explicitly (the Rust default) so panic diagnostics remain available to the smoke test and crash logs; do not optimize away unwind diagnostics with `panic = "abort"`.
5. Build APKs with Dioxus CLI 0.7.10 using the source-verified flags `dx bundle --android --release --package-types apk --target <triple> --out-dir dist --cargo-args=--locked`.
6. The current Ubuntu 24.04 runner image already lists Android platform 35, Build Tools 35.0.0, command-line tools and platform-tools; it lists NDK 27.3.13750724 and 29.0.14206865, not r28c. The workflow explicitly installs platform 35, Build Tools 35.0.0, NDK r28c and JDK 17 so the build does not depend on image drift. NDK r28+ is required here because Android 15+ 16 KiB page-size compatibility requires 16 KiB ELF LOAD alignment; NDK r28 provides this by default.
7. The emulator smoke uses the documented `google_apis_ps16k` x86_64 system image and asserts `adb shell getconf PAGE_SIZE` equals 16384 before installation. GitHub's Android emulator runner documents hardware acceleration for larger Linux runners. Standard `ubuntu-24.04` KVM availability is not assumed: the workflow checks `/dev/kvm` and `kvm-ok`, uses hardware acceleration when available, and attempts software emulation otherwise. If software emulation times out, use a larger GitHub runner with KVM or a self-hosted Linux runner with KVM; report the smoke gate as failed until it passes.
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

[Fact] Dioxus 0.7.10 CLI source accepts the documented platform, target, package type, output directory, and extra Cargo arguments. Android's official guidance requires `zipalign -P 16` for APKs containing shared libraries and states NDK r28+ emits 16 KiB ELF alignment by default. Rust 1.99.1 is the official stable release published 2026-10-15. The current Ubuntu 24.04 runner image manifest lists Android platform 35, Build Tools 35.0.0, and NDK 27.3.13750724 / 29.0.14206865. The emulator action documentation limits documented hardware acceleration to larger Linux runners.

[Theory] The x86_64 APK will install and remain alive for 60 seconds on API 35; only the exact workflow run can establish this.

[Opinion] Full LTO and size optimization are suitable for a sideloadable test APK despite longer CI build times. Ephemeral debug signing is safer than storing a long-lived private key in Phase 1.
