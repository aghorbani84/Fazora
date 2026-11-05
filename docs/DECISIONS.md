# FAZORA — Owner Decision Record

Date: 2026-11-05
Authority: Owner instruction in ChatGPT conversation: “خودت تصمیم بگیر و بهترین کارو بکن”
Applies to: Phase 0 decisions and subsequent engineering work. This file records engineering defaults; it does not replace legal advice or repository administrator settings.

## 1. Project license — retain MIT

**Decision: Keep the existing MIT license.**

Rationale:
- The repository already contains an MIT LICENSE and GitHub metadata reports MIT.
- Changing to proprietary terms would be a disruptive legal/product decision and could conflict with existing public contributions or reuse.
- The lowest-risk consistent action is to retain the existing license and correct README/documentation claims that say “Proprietary”.

Implementation constraints:
- Do not remove or rewrite the MIT LICENSE.
- Add third-party notices for all bundled dependencies.
- Review every native media dependency's exact license and build configuration before bundling.
- MIT does not eliminate third-party obligations or patent considerations.

Classification: [Fact] current LICENSE and repository metadata are MIT; [Opinion] retaining the established license is the least disruptive option. This is not legal advice.

## 2. Loudness normalization — ReplayGain 2.0 profile

**Decision: Use ReplayGain 2.0 as the default normalization profile, with a -18 LUFS reference target.**

Product behavior:
- Provide Off / Track / Album modes.
- Default mode: Album when album gain metadata is available; otherwise Track.
- Preserve album dynamics and track-to-track consistency as intended by the selected mode.
- Provide preamp and clipping-prevention controls.
- Do not silently normalize the audio stream if metadata is absent; expose the configured fallback clearly.
- Treat ReplayGain tags and scanned loudness as metadata inputs, not as proof of true-peak safety. Apply peak-aware clipping prevention.

Rationale:
- Fazora is a general music player, not a broadcast loudness-compliance tool.
- EBU R128's -23 LUFS reference is primarily a broadcast delivery context and is not the best default target for a consumer music library.
- ReplayGain is more directly aligned with per-track/album playback gain management.

Classification: [Opinion] this is a product choice. Exact loudness and peak handling must be tested with reference audio before release.

## 3. Equalizer band centers — use mathematical octave centers

**Decision: Use the exact ten centers 31.25, 62.5, 125, 250, 500, 1,000, 2,000, 4,000, 8,000 and 16,000 Hz.**

- Store and calculate using the exact numeric center frequencies.
- Display labels as 31.25, 62.5, 125, 250, 500, 1k, 2k, 4k, 8k and 16k Hz; do not calculate coefficients from rounded display strings.
- Validate each filter's coefficients and pole stability for every supported sample rate.
- Do not freeze Q/bandwidth semantics until the DSP implementation and tests define them explicitly.

Rationale:
- The first two centers then follow the same octave progression as the remaining eight bands.
- Exact center values avoid mixing rounded labels with coefficient-generation values.

Classification: [Opinion] chosen design consistency; frequency values are exact arithmetic.

## 4. Media engine — no premature selection

**Decision: Do not select or bundle FFmpeg, libVLC or libmpv yet.**

Run a documented comparison before Phase 5. Include platform support, Android hardware decode, audio output latency, gapless playback, package size, build reproducibility, security cadence, LGPL/GPL obligations and patent-sensitive options. Keep any platform-native adapter behind a stable internal interface.

## 5. Existing pull requests

**Decision: Do not merge or close PRs automatically.**

Rebase and evaluate each candidate diff against the current main and the approved phase branch. Keep useful work only after resolving conflicts and validating the exact final SHA.

## 6. Execution order

1. Finish Phase 0 documentation and obtain green checks on the exact Phase 0 PR head.
2. Merge Phase 0 only after its required checks pass.
3. Start Phase 1 on a separate branch from the updated main.
4. Do not merge Phase 1 until all required CI checks pass on its exact final SHA.
5. Never push directly to main.

This decision record authorizes the team to use the above defaults in engineering planning. It does not authorize skipping tests, license review, security review or phase exit criteria.
