# FAZORA Engineering Model & Phase 0 Audit

> Audit baseline: `main` at `d62698c5a8462fc3daa023ad35dab6c29f0e51e9` (2026-11-05).
> Scope: architecture, source inventory, dependency/toolchain model, workflows, security, UI/design documentation, numerical budgets, and implementation sequencing.
> This document is a planning baseline. Proposed budgets are targets, not measured performance results.

## 1. Evidence and confidence convention

Every engineering statement is classified:

- **[Fact]** directly visible in repository source/configuration or confirmed by a completed CI run.
- **[Theory]** engineering inference that needs implementation or measurement to validate.
- **[Opinion]** recommended design or product decision.

External dependency facts must be rechecked at the time a dependency is changed. In particular, Dioxus 0.7.10 package metadata reports Rust MSRV 1.83.0; the repository declares 1.78. See [Dioxus 0.7.10 on docs.rs](https://docs.rs/crate/dioxus/0.7.10).

## 2. Repository baseline

### 2.1 Structure

[Fact] The repository currently has a root Rust package named `fazora` and an empty `[workspace]` declaration. It is not yet a multi-crate workspace. Application code lives in `src/`; the root contains platform documentation and GitHub Actions workflows.

[Fact] `Cargo.toml` pins Dioxus crates to 0.7.10 but does not declare a `Cargo.lock` in the audited main tree. No `rust-toolchain.toml` was present in the audited tree.

[Opinion] Keep the current root package as the Dioxus presentation/application crate during the first CI stabilization. Split domain crates only when their boundaries and tests are specified; avoid a large mechanical workspace migration in the same PR as CI repair.

### 2.2 Source inventory and responsibilities

| Area | Files / responsibility | Current assessment |
|---|---|---|
| Application composition | `src/main.rs`, `src/lib.rs`, `src/app.rs`, `src/state/app_state.rs` | Dioxus root, service initialization, context wiring |
| Routing and shell | `src/routing.rs`, `src/components/layout/*` | Route and responsive-shell UI; platform behavior needs runtime validation |
| Design system | `src/design/css.rs`, `src/design/tokens.rs` | Rust-embedded CSS/tokens; docs currently refer to a non-existent `tokens.css` |
| Media models | `src/models/*` | Track, album, artist, video, playlist and library data structures |
| Library | `src/library.rs`, `src/views/library.rs`, `src/views/video_library.rs` | Directory configuration and signal state exist; scanner/indexer is not implemented |
| Playback and queue | `src/playback.rs`, `src/queue.rs`, `src/views/now_playing.rs`, `src/views/queue.rs` | In-memory state/queue logic; no native decoder/output backend is wired |
| Audio/DSP | `src/audio.rs`, `src/components/audio/*`, `src/views/equalizer.rs` | EQ settings/UI exist; no DSP processing pipeline or output-device backend |
| Search | `src/search.rs`, `src/views/search*.rs` | Query/result state exists; no index or search execution engine |
| Security | `src/security.rs`, `SECURITY.md`, `FILE_OPERATIONS.md` | Input/path helpers exist; platform-specific authorization and race resistance need work |
| Settings | `src/settings.rs`, `src/views/settings.rs` | Model and serialization helpers exist; persistence path is not initialized in service construction |
| UI components | `src/components/{audio,branding,layout,media,player,primitives}/*` | Broad component inventory exists; each control still needs behavior/accessibility verification |
| Screens | `src/views/*` | Broad navigation surface exists; several screens depend on placeholder or empty data |
| Tests | `src/tests/*` plus inline unit tests | Unit tests exist; coverage does not establish real media playback or device integration |

### 2.3 Confirmed implementation gaps

- [Fact] `LibraryService::request_scan` only sets `ScanStatus::Scanning`; its comment delegates work to an engine adapter that is not present in this repository.
- [Fact] `PlaybackService::play_toggle` changes the in-memory state between Playing and Paused. No decoder, audio sink, media session, or native playback engine is connected.
- [Fact] `PlaybackService::engine_available` initializes to `true` despite no playback engine being wired. This can cause the UI to advertise availability incorrectly.
- [Fact] `AudioService` stores EQ/device/crossfade settings. `save_preset` returns an empty placeholder playlist; there is no DSP graph.
- [Fact] The spectrum component has no FFT/audio-sample data pipeline.
- [Fact] Search stores a sanitized query and accepts externally supplied results; it does not search or index the library.
- [Fact] Lyrics presentation has no authoritative playback-clock synchronization source.
- [Fact] Video player UI exists, but no native video decoder/render surface is connected.
- [Fact] Several home sections and quick actions are presentation-level placeholders; the product must hide data-driven sections when no real data exists.
- [Theory] Treat the product as a UI prototype with service abstractions, not as a functional multimedia player, until native media integration and device-level tests pass.

## 3. Toolchain and dependency constraints

### 3.1 MSRV mismatch

- [Fact] `Cargo.toml` declares `rust-version = "1.78"`.
- [Fact] Dioxus 0.7.10 package metadata declares Rust 1.83.0 as its minimum supported compiler.
- [Fact] The PR #34 CI run for head SHA `2b726536cea9c16bc88f875426b8ab7e879f0257` failed its explicit Rust 1.78 MSRV job.
- [Opinion] Set the declared MSRV no lower than the actual dependency floor after verifying every resolved dependency and target. Pin an exact stable toolchain in `rust-toolchain.toml`; keep a separate MSRV job at the declared minimum. Do not claim MSRV support merely because the default stable build succeeds.

### 3.2 Lockfile and reproducibility

- [Fact] `Cargo.lock` was absent from the audited `main` tree.
- [Fact] The current PR #34 workflows attempt to generate a lockfile when absent. The Windows CI job failed at that bootstrap step, before compilation.
- [Opinion] Commit `Cargo.lock` for this application binary, use `--locked` in all build/test/security jobs, and remove runtime lockfile generation from CI. The lockfile must be generated and validated on a supported stable toolchain before CI is made deterministic.

### 3.3 Dependency policy

- [Fact] `deny.toml` has an explicit allowlist and narrow exceptions for selected MPL-2.0 transitive crates.
- [Fact] The current security workflow includes cargo-audit, cargo-deny, actionlint, and zizmor.
- [Opinion] Keep exceptions version-specific, documented, and reviewed on dependency updates. Do not widen the license allowlist to make a failing check disappear.
- [Fact] The current dependency manifest has no dedicated media decoder, audio output, Android JNI/SAF/MediaStore, or DSP crate dependencies.
- [Opinion] Select the media engine only after platform support, LGPL/GPL obligations, patent exposure, binary size, hardware acceleration, security update cadence, and Android/iOS viability have been evaluated.

## 4. CI/CD baseline observed on 2026-11-05

The following observations refer to PR #34 head SHA `2b726536cea9c16bc88f875426b8ab7e879f0257`, not to `main`. Workflow state can change after this snapshot.

| Check / run | Observed result | Evidence / interpretation |
|---|---|---|
| Linux build | Success | Run 41983674486 |
| Windows build | Success on the later PR run | Run 41983674505; earlier run failed during lockfile bootstrap |
| Workspace tests | Success | Run 41983667956; `cargo test --workspace --locked` |
| Rustfmt | Success | Run 41983667956 |
| Documentation build | Success | Run 41983667956 |
| cargo deny + cargo audit | Success in completed run | Run 41983667956 |
| Clippy with warnings denied | Failure | Run 41983667956 |
| Rust 1.78 MSRV | Failure | Run 41983667956 |
| actionlint | Failure in completed security run | Run 41983663959 |
| zizmor | Failure in completed security run | Run 41983663959 |
| Android x86_64 APK | Success in one job | Run 41983674513; this is packaging/signing, not playback validation |
| Android ARM64 APK | In progress at snapshot | Run 41983674513 |
| Android emulator smoke | In progress at snapshot | Run 41983674487 |

[Fact] PR #34 is an open draft PR targeting main. Other open UI/native-media PRs also target main from older base SHAs. They must be rebased/compared and independently validated before their changes are treated as integrated.

[Opinion] Do not merge or close older PRs by assumption. Compare their file changes against the current main and the Phase 1 branch; retain useful changes, resolve conflicts, and prevent duplicate or conflicting implementations.

## 5. Documentation consistency findings

- [Fact] `docs/DESIGN_SYSTEM.md` references `src/design/tokens.css`, but the audited source contains `src/design/css.rs` and `src/design/tokens.rs`.
- [Fact] Existing responsive docs specify 768px and 1024px breakpoints; the requested product model specifies width classes derived from 360dp × φⁿ (approximately 360, 582, 942, 1525dp). These are competing specifications and must be reconciled.
- [Fact] Accessibility docs state a 48dp minimum target but also describe 44×44dp icon buttons. The stricter 48dp minimum should govern touch targets.
- [Fact] README claims a Proprietary license while `LICENSE` contains the MIT License. GitHub repository metadata also identifies MIT.
- [Opinion] Resolve the license conflict before bundling any third-party media engine or shipping third-party notices. Do not silently change the project license.
- [Fact] README’s targeted test commands refer to `cargo test --test fazora_tests`; no corresponding explicit integration-test target exists in the audited tree. The source tests are organized under `src/tests` and module tests.
- [Opinion] Replace stale commands with commands verified against the actual target layout.

## 6. Security and platform risks

### 6.1 File access

- [Fact] `validate_media_dir` uses canonical paths and a fixed allowlist. Android allowlist entries include private app-data paths and do not implement Android Storage Access Framework grants.
- [Theory] A path allowlist alone is not a complete Android media permission model. Android should use persisted URI grants through SAF and MediaStore queries where appropriate; avoid treating user-visible media access as ordinary filesystem traversal.
- [Fact] `validate_settings_path` validates a non-existent target by canonicalizing its parent. If the parent directory does not yet exist, validation returns PathNotFound; `save` creates the parent only after validation. This ordering can prevent first-run settings creation.
- [Theory] Metadata-check-then-read file access can be subject to TOCTOU changes. Open the file once, validate the opened handle/metadata, and enforce a bounded read.
- [Opinion] Add platform-specific storage adapters; do not weaken path validation globally to make Android access work.

### 6.2 Untrusted media and network inputs

- [Opinion] Treat filenames, tags, embedded artwork, subtitle/lyrics payloads, playlists, remote URLs, and decoder output as untrusted. Bound file sizes, metadata lengths, artwork dimensions, queue length, parser recursion, and concurrent decode jobs.
- [Opinion] Do not execute external media-provided paths, shell fragments, scripts, or playlist directives. Restrict network protocols and redirects explicitly when remote streams are introduced.
- [Theory] Decoder crashes and malformed metadata are likely higher-risk attack surfaces than the current UI state layer; isolate decoder work where platform constraints allow.

## 7. Target platform matrix

| Tier | Platforms | Required capability |
|---|---|---|
| T1 | Android API 26+, Linux x86_64, Windows x86_64 | Supported production target |
| T2 | Android tablet/foldable, macOS, Linux/Windows ARM64 | Follow-on support after T1 stability |
| T3 | iOS build-only | Build/package validation only; no installability claim without signing/provisioning |
| Out of scope | Android TV, Android Auto, Web | No implementation commitment |

## 8. Proposed architecture boundaries

[Opinion] Keep the Dioxus application as a thin presentation/composition layer. Domain and media crates must not depend on Dioxus.

```text
fazora/
├── Cargo.toml                 # workspace root + application package
├── Cargo.lock
├── rust-toolchain.toml
├── src/                       # Dioxus application, routing, screens
├── crates/
│   ├── fazora-core/           # IDs, domain types, errors, contracts
│   ├── fazora-library/        # scanning/indexing abstractions and models
│   ├── fazora-search/         # indexing/query engine
│   ├── fazora-media/          # decoder/player abstraction
│   ├── fazora-audio/          # output, DSP, EQ and loudness
│   ├── fazora-platform/       # cross-platform contracts
│   └── fazora-android/        # JNI, SAF, MediaStore, Android lifecycle
├── docs/
│   ├── ENGINEERING.md
│   ├── ENGINE.md
│   └── THIRD_PARTY_NOTICES.md
└── .github/workflows/
```

[Theory] The split reduces coupling and improves testability if crate APIs remain narrow. Prematurely splitting every UI component into a crate would increase build and maintenance overhead without clear value.

## 9. Module contract required before implementation

Every new or materially changed module must document:

1. Purpose and non-goals.
2. Inputs, outputs, errors, invariants, ownership and lifetime rules.
3. Numeric parameters, units, bounds, defaults and configuration source.
4. Algorithm and complexity: time and space Big-O, including worst-case behavior.
5. Capacity and resource model: RAM, CPU, disk, bandwidth, wakeups and battery where relevant.
6. Concurrency model, cancellation, backpressure and shutdown semantics.
7. Security boundary and treatment of untrusted input.
8. Tests and measurable acceptance criteria.
9. Evidence labels: Fact / Theory / Opinion.
10. Failure modes, recovery behavior and observability.

No module is considered complete merely because its UI renders or its types compile.

## 10. Numerical budgets and validation plan

All values below are **[Opinion] proposed engineering targets**, not measurements. They must be benchmarked on representative desktop and Android devices before being promoted to product guarantees.

### 10.1 UI and rendering

| Metric | Proposed budget | Validation |
|---|---:|---|
| Frame interval at 60Hz | 16.67ms | Frame timeline / Perfetto |
| Frame interval at 90Hz | 11.11ms | Frame timeline / Perfetto |
| Frame interval at 120Hz | 8.33ms | Frame timeline / Perfetto |
| App-owned scroll work | ≤5ms/frame at 60Hz | Perfetto trace |
| Seek interaction state update | ≤0.5ms | Instrumented benchmark |
| Animation update work | ≤2ms/frame | Perfetto trace |
| Long-list rendering | O(V), V = visible rows + overscan | 50k/100k item benchmark |
| Artwork loading | Lazy, bounded concurrency, cache budget below | Memory and jank trace |

Avoid expensive blur/backdrop effects in scrolling surfaces until measured on mid-range Android. Do not introduce a different Dioxus renderer without a reproducible benchmark showing material benefit.

### 10.2 Audio pipeline

| Parameter | Proposed value / bound | Calculation or rationale |
|---|---:|---|
| Sample rate | 48,000 frames/s baseline | [Opinion] baseline format; negotiate device format |
| Decode chunk | 1,024 frames | [Opinion] scheduling unit |
| Output callback block | 256 frames | [Opinion] low-latency target |
| Callback period | 256 / 48,000 = 5.333ms | [Fact] arithmetic |
| Two-period buffer | 10.667ms | [Fact] arithmetic |
| Ring buffer | 4,096 frames | [Opinion] power-of-two capacity; 4 decode chunks |
| Callback CPU | ≤25% of callback period (≤1.333ms at 256/48k) | [Fact] arithmetic from target |
| Underruns | 0 per hour in acceptance test | [Opinion] target |
| Callback behavior | No allocation, blocking locks, filesystem or network I/O | [Opinion] real-time safety rule |

Latency model: `L = N / f_s`, where N is frames and `f_s` is sample rate. At 256 frames and 48kHz, L = 5.333ms. End-to-end output latency also includes device/OS buffering and must be measured; this equation is not an end-to-end latency claim.

### 10.3 Memory

| Component | Proposed budget |
|---|---:|
| Metadata/index overhead | ≤2KiB per track average |
| 100,000-track metadata/index budget | ≤195.3MiB |
| Decoded artwork LRU | ≤64MiB |
| Runtime and working structures | ≤64MiB |
| Headroom | ≥60MiB |
| Initial total RAM ceiling | ≤384MiB |

[Fact] A decoded 256×256 RGBA image requires 256 × 256 × 4 = 262,144 bytes (256KiB), excluding allocator and cache overhead.

[Theory] The 2KiB/track target may be too low if the in-memory index retains full strings, path allocations, multiple sort keys, lyrics, or waveform data. Benchmark real metadata and store large fields separately.

### 10.4 Startup and search

| Metric | Proposed target | Test population |
|---|---:|---|
| Cold startup | <2s | Mid-range Android and reference desktop |
| Warm startup | <500ms | Same devices |
| Search p99 | ≤16ms | Desktop, 50k tracks |
| Search p99 | ≤30ms | Mid-range Android, 50k tracks |
| Search memory | Included in metadata/index budget | 50k and 100k track datasets |

Measure from process start to interactive first frame, not just Rust initialization. Search p99 must include query normalization and result ranking, but not artwork decoding.

### 10.5 Battery and package size

- [Opinion] Measure screen-off background playback wakeups per second and mAh/hour using Perfetto and `dumpsys batterystats`; define an idle/background target only after establishing a baseline on named devices.
- [Fact] No APK-size delta or media-engine binary-size baseline has been measured in Phase 0.
- [Opinion] Record per-ABI APK size before and after native engine integration. Report compressed APK size, installed size, native library size, and ABI-specific delta separately.

### 10.6 Equalizer and loudness

- [Fact] Current EQ band centers are 32, 64, 125, 250, 500, 1k, 2k, 4k, 8k, 16kHz.
- [Opinion] Requested mathematical centers use `f_k = 31.25 × 2^k Hz`, k = 0…9: 31.25, 62.5, 125, 250, 500, 1k, 2k, 4k, 8k, 16kHz. Decide whether to change the first two display centers or preserve conventional rounded labels.
- [Opinion] Use RBJ peaking biquads as a candidate implementation, but calculate coefficients from sample rate, center frequency, gain and Q; clamp invalid parameters and test pole stability at all supported sample rates.
- [Theory] A nominal Q around √2 is not universally equivalent to a one-octave bandwidth for every digital peaking-EQ interpretation. Verify the chosen bandwidth/Q convention and bilinear frequency warping rather than hard-coding a claim.
- [Opinion] Loudness normalization requires a product decision before implementation: ReplayGain 2.0 reference -18 LUFS or EBU R128 -23 LUFS. These targets are not interchangeable; expose the selected mode and allow user control.

## 11. Golden Ratio design model

The golden ratio `φ = (1 + √5) / 2 ≈ 1.6180339887` is a **design heuristic**, not evidence that a layout is more beautiful, usable, or accessible.

### 11.1 Candidate dimensions

| Use | Candidate | Status |
|---|---:|---|
| Spacing progression | Fibonacci-like 4, 8, 12, 20, 32, 52 | Heuristic; preserve established 4dp rhythm where more usable |
| Window classes | 360, 582, 942, 1525dp | 360 × φⁿ rounded; validate against actual device classes |
| Main/secondary split | 61.8% / 38.2% | Candidate for expanded layouts |
| Artwork ratio | 1:1 media artwork; φ-guided surrounding composition | Artwork itself must not be distorted |
| Touch target | ≥48dp | Functional accessibility constraint overrides φ |
| Body line height | ≥1.5 | Accessibility/readability constraint overrides φ |
| Video frame | 16:9 | Media standard overrides φ |

### 11.2 Breakpoint arithmetic and deviations

Using 360dp × φⁿ:
- n=0: 360dp
- n=1: 582.49dp
- n=2: 942.49dp
- n=3: 1525.03dp

Rounded proposal: 360 / 582 / 942 / 1525dp.

Compared with current docs:
- 582dp is 18dp (3.0%) below a 600dp Material-style reference.
- 942dp is 102dp (12.14%) above an 840dp reference.
- Existing 768/1024 breakpoints differ materially and must not coexist as competing truths.

### 11.3 Functional overrides

| Constraint | Functional override | Classification |
|---|---|---|
| Touch target ≥48dp | Do not shrink to match a ratio | Fact: accessibility requirement in product spec |
| WCAG contrast | Choose contrast-compliant color even if ratio palette shifts | Fact: compliance constraint |
| RTL | Mirror navigation/layout direction without reversing media timeline semantics | Opinion backed by interaction convention; validate |
| Foldable hinge/safe area | Respect display features and safe insets | Functional platform constraint |
| Empty queue | Hide player dock or render explicit idle state | Product correctness |
| Small windows | Collapse panels before forcing a 61.8/38.2 split | Functional usability |

### 11.4 Color contrast baseline

Ratios below are calculated for the stated solid foreground/background pairs only. Alpha overlays, disabled states, focus rings, gradients and actual rendered surfaces require separate verification.

| Foreground | #0B0D10 | #15191F |
|---|---:|---:|
| Ivory #F4F1E9 | 17.24:1 | 15.63:1 |
| Secondary #AEB4BC | 9.32:1 | 8.44:1 |
| Steel #89919D | 6.12:1 | 5.54:1 |
| Gold #D8B56A | 9.95:1 | 9.02:1 |
| Teal #78CFC1 | 10.65:1 | 9.65:1 |

[Opinion] Validate the design through a structured heuristic review first, then a small usability test (ideally 5–8 users) measuring task completion, navigation errors, readability, and preference. Do not claim φ causes aesthetic improvement without evidence.

## 12. Responsive interaction model

[Opinion] Candidate width classes:

| Class | Width | Layout |
|---|---:|---|
| Compact | <582dp | Five-destination bottom navigation; one primary pane |
| Medium | 582–941dp | Navigation rail (candidate 89dp); one pane plus optional overlay |
| Expanded | 942–1524dp | Sidebar (candidate 233dp); main/secondary split near 61.8/38.2 |
| Large | ≥1525dp | Three-pane workspace where content density justifies it |

Rules:
- Respect Android safe areas, cutouts, gesture insets and dynamic viewport height.
- Compact bottom navigation: Home, Music, Videos, Library, Settings.
- Do not show a player dock when the queue is empty; show a deliberate idle/empty state if needed.
- Expanded and large panes must collapse before content becomes cramped.
- Support keyboard navigation, media keys, screen readers, focus restoration and reduced motion.
- Support RTL Persian layout without mirroring media progress direction or physical transport semantics.
- Virtualize long lists; lazy-load artwork; cap concurrent decoding and cache memory.
- Foldables need posture/hinge-aware layout testing; width alone is insufficient.
- Evaluate Dioxus renderer behavior and native integration per platform. Do not assume CSS behavior is identical across WebView, desktop and mobile renderers.

## 13. Media engine and licensing gate

[Opinion] Before Phase 5, compare candidate engines on:
- Supported codecs/containers and Android hardware decode.
- Audio output latency, gapless playback, seeking and track switching.
- Video surface integration and subtitle handling.
- ABI/architecture support, build reproducibility and package-size delta.
- LGPL/GPL obligations, dynamic-linking implications, source-offer/notices obligations and patent exposure.
- Security update cadence and parser hardening.

[Opinion] If FFmpeg is selected, keep the build LGPL-compatible, exclude GPL/nonfree components unless explicitly approved, use dynamic linking where required by the selected distribution model, and ship notices/source-offer materials as applicable. Do not enable x264/x265 or patent-sensitive options without a documented user decision and specialist review. This document is not legal advice.

## 14. Phase roadmap and exit criteria

| Phase | Scope | Exit criteria |
|---|---|---|
| 0 — Audit & Engineering Model | Repository audit, architecture, numerical budgets, design model, risk/decision register | This document reviewed; decisions captured; no product implementation changes |
| 1 — CI/CD foundation | Stable toolchain/MSRV, lockfile, fmt, clippy, tests, docs, deny/audit, Linux/Windows/Android packaging, emulator smoke, release hardening | Required checks green on exact PR head SHA; reproducible APKs; evidence attached |
| 2 — UI foundation | Responsive shell, navigation, design tokens, real empty/loading/error states, RTL/accessibility | Screenshots and interaction tests on Android and desktop |
| 3 — Library | SAF/MediaStore and desktop scanning, metadata extraction, persistent index, bounded artwork cache | Real media indexed, permission/revocation tests, large-library benchmark |
| 4 — Search | Incremental index, query normalization, ranking, filters, cancellation | p99 targets met on 50k/100k dataset |
| 5 — Audio/DSP | Engine decision, audio output, decode, EQ, loudness, gapless/crossfade | Device playback, latency/underrun tests, license notices and DSP tests |
| 6 — Video | Decode/render surface, subtitles, audio tracks, seek/speed | Device-level playback and resource-lifecycle tests |
| 7 — System integration | Android background playback/notification/media session; desktop media keys/MPRIS; lifecycle | OS integration tests and battery profiling |
| 8 — Hardening/release | Fuzzing, crash recovery, accessibility, performance, signing, SBOM/notices | Release checklist, reproducible evidence and rollback plan |

## 15. Phase 1 CI/CD design requirements (planning only)

- Pin Rust stable in `rust-toolchain.toml`; declare and test a verified MSRV.
- Commit `Cargo.lock`; use `--locked` everywhere.
- Required CI: fmt, clippy, unit/integration tests, docs, dependency policy, vulnerability audit, Linux and Windows build.
- Android: ARM64 + x86_64 APK, verify package metadata and signature; persistent release signing only from protected secrets, ephemeral debug signing for fork-safe PR builds.
- Emulator smoke test: install, launch, navigate Home/Library/Search/Now Playing, capture screenshots, verify crash-free startup. This is not proof of real playback.
- Release workflow: publish only from protected tags after required checks; checksums, SBOM, third-party notices, provenance where available.
- Nightly: dependency/security scan and selected platform smoke checks.
- Dependabot: Rust dependencies and GitHub Actions.
- Pin third-party Actions to full commit SHAs; least-privilege permissions; no secrets in fork PRs; concurrency cancellation for superseded PR runs; bounded timeouts and caches.
- Manual GitHub configuration: branch protection/required checks, environment approvals, signing secrets and release permissions. CI cannot safely self-authorize these repository administration settings.

## 16. Decision register — owner approval required

| Decision | Options | Current status |
|---|---|---|
| Project license | Keep MIT and correct README; or choose proprietary terms and replace LICENSE with owner-approved terms | **Open — do not alter license or ship bundled engine yet** |
| Loudness normalization | ReplayGain 2.0 (-18 LUFS reference) or EBU R128 (-23 LUFS reference), plus user-adjustable disable/track/album mode | **Open — no implementation default** |
| Media engine | FFmpeg / libVLC / libmpv / platform-native combination | **Open — requires technical and licensing comparison** |
| EQ low-frequency labels | Conventional 32/64Hz or mathematical 31.25/62.5Hz | **Open — decide before DSP API freeze** |
| Existing PRs | Rebase/retain/supersede based on diff and CI evidence | **Open — no PR closed or merged by this audit** |

## 17. Phase 0 limitations and completion boundary

- [Fact] This audit inspected the repository tree, key service/model/security modules, design documentation, workflow definitions and current PR #34 workflow results.
- [Fact] This is not a line-by-line proof of every visual component on every renderer or device.
- [Fact] No Android device was connected for physical-device testing during this audit.
- [Fact] No media engine was built, no playback latency was measured, and no battery/RAM benchmark was run.
- [Fact] Proposed numeric budgets and design ratios are not measured acceptance results.
- [Opinion] Phase 0 is ready for owner review once the decision register is answered. Phase 1 implementation must not begin until explicit approval.
