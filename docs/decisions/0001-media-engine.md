# ADR 0001: FFmpeg native media engine

Date: 2026-11-05
Status: Accepted by explicit owner approval

## Decision

Use FFmpeg 9.0.2 as the shared demux/decode core behind a narrow C ABI. Start with Android API 26+ (ARM64-v8a and x86_64 CI); add platform audio sinks and video surfaces independently. Use MediaCodec acceleration when available and software decoding as fallback.

The build must use shared libraries and omit GPL, version-3, and nonfree components. Disable external-library autodetection. Do not enable x264/x265, FDK-AAC, OpenSSL, or any other external codec library without a separate license/patent review.

## Weighted options

Weights from AGENTS.md: correctness/safety 0.30, maintainability 0.20, performance 0.20, reversibility 0.15, effort 0.15. Scores are estimates, not measurements.

| Option | Correctness | Maintainability | Performance | Reversibility | Effort | Weighted |
|---|---:|---:|---:|---:|---:|---:|
| FFmpeg shared + C ABI + platform adapters | 9 | 8 | 9 | 7 | 5 | 7.95 |
| Android Media3 only | 7 | 8 | 9 | 8 | 8 | 7.80 |
| libVLC / libmpv | 8 | 7 | 8 | 6 | 5 | 7.10 |
| State-only placeholder | 1 | 8 | 1 | 10 | 10 | 4.30 |

FFmpeg is selected both on the evaluation and because the owner explicitly approved this dependency. Scores are not benchmarks.

## License gates

- Verify the upstream FFmpeg release signature and retain exact matching source.
- Build shared libraries only; do not pass --enable-gpl, --enable-version3, or --enable-nonfree.
- Disable external autodetection and audit the linked dependency graph.
- Retain configure command, build logs, upstream license files, and local patch diff.
- Publish corresponding source and notices with every distributed binary; add an About/Legal notice and source link.
- Produce an SBOM and per-release third-party license inventory.
- LGPL compliance does not resolve codec patent exposure; obtain specialist review before distribution.

## Security and runtime constraints

Treat media files, tags, artwork, subtitles and decoder output as untrusted. Bound metadata, dimensions, frame/packet queues and concurrency. Decoder work must be cancellable and shut down deterministically. Real-time audio callbacks must not allocate, block, or perform file/network I/O. Do not claim universal codec support; publish a tested codec/profile/container/device matrix.

## Reversal cost

Medium. The narrow C ABI isolates FFmpeg from UI/domain code, but replacing the core still requires native builds and codec regression tests.

## Evidence

- Fact: FFmpeg's official download page lists 9.0.2 as the latest stable 9.0 point release on 2026-11-05.
- Fact: FFmpeg's default license is LGPL; GPL components change the resulting license, and nonfree combinations are not redistributable.
- Fact: FFmpeg's LGPL checklist recommends dynamic linking, matching source, build instructions and notices.
- Theory: A shared FFmpeg core with platform output/render adapters is the most maintainable cross-platform design for FAZORA.
- Opinion: Prefer a shared core over platform-only decoders to preserve one demux/decode implementation.

## Sources

- https://ffmpeg.org/download.html
- https://ffmpeg.org/legal.html
- https://github.com/FFmpeg/FFmpeg/blob/master/LICENSE.md
- https://github.com/FFmpeg/FFmpeg/blob/master/configure
- https://dioxuslabs.com/learn/0.7/guides/platforms/mobile/
