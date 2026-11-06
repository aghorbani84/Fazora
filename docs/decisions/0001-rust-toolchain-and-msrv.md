# ADR 0001: Rust toolchain and MSRV

Date: 2026-11-06
Status: Proposed on `upgrade/phase-1-rust-stability-2026-11-06`

## Context

FAZORA uses Dioxus 0.7.10. Its published manifest declares Rust 1.83.0, while the application previously declared Rust 1.78. The repository had no tracked `Cargo.lock` and ignored the root lockfile. CI installed an unpinned stable toolchain, so builds could change as the stable channel advanced.

## Options and weighted evaluation

Hard filters: stable Rust only; preserve Dioxus 0.7.10 (the current stable line); support Android, Linux, Windows and iOS; do not alter licensing or signing.

Weights: correctness/safety 0.30, maintainability 0.20, performance 0.20, reversibility 0.15, effort 0.15.

| Option | Correctness | Maintainability | Performance | Reversibility | Effort | Weighted |
|---|---:|---:|---:|---:|---:|---:|
| Keep current floating stable + MSRV 1.78 | 3 | 4 | 7 | 9 | 10 | 5.65 |
| Pin current stable for CI; declare/test MSRV 1.83 | 9 | 9 | 9 | 9 | 7 | 8.70 |
| Pin every workflow to Rust 1.83 | 7 | 7 | 7 | 9 | 8 | 7.40 |
| Do nothing | 4 | 4 | 6 | 10 | 10 | 6.10 |

## Decision

Use Rust 1.99.1 as the reproducible project toolchain and set the declared minimum supported Rust version to 1.83.0, matching the published Dioxus 0.7.10 manifest. Add a dedicated CI job that checks the project on Rust 1.83.0. Keep Dioxus 0.7.10; do not move to the 0.8 alpha line.

This is a toolchain policy, not proof that the complete dependency graph already builds on 1.83. The MSRV job must pass before the claim is accepted. If a transitive dependency has raised its MSRV, constrain or update that dependency only after inspecting the resolved graph and compatibility.

## Sensitivity and confidence

The selected option remains the highest-scoring choice under individual +/-20% weight changes with renormalization; it is not a close call under the stated scores. Confidence: medium until exact-SHA CI completes.

## Reversal and consequences

Reversal cost is low: change `rust-toolchain.toml`, the package `rust-version`, and CI toolchain settings in one reviewed commit. A higher project MSRV excludes older developer toolchains; pinning the build toolchain avoids silent compiler drift. This does not pin third-party dependencies; a tracked Cargo.lock is still required for reproducible dependency resolution.

## Evidence

- Dioxus 0.7.10 manifest: https://docs.rs/crate/dioxus/0.7.10/source/Cargo.toml
- Rust 1.99.1 release announcement: https://blog.rust-lang.org/releases/
- Rust release notes: https://doc.rust-lang.org/releases.html

Labels: [Fact] published Dioxus manifest MSRV and Rust release history; [Opinion] toolchain selection and scoring; [Theory] the exact dependency graph will remain compatible with Rust 1.83 until verified by CI.
