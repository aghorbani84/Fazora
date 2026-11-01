# Contributing to FAZORA Media

Thank you for contributing to FAZORA Media! This document outlines the
guidelines for contributing to the project.

## How to Contribute

1. **Fork the repository** on GitHub
2. **Create a feature branch** from `main`
3. **Make your changes** following the coding standards below
4. **Run tests** — `cargo test`
5. **Run clippy** — `cargo clippy --workspace --all-targets`
6. **Run fmt** — `cargo fmt --all -- --check`
7. **Open a pull request** against `main`

## Coding Standards

### Rust

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `clippy` with `-D warnings` — no clippy warnings allowed
- Format with `rustfmt` — `cargo fmt` must pass
- Write tests for new functionality
- Document public APIs with doc comments
- Prefer safe Rust; avoid `unsafe` unless absolutely necessary
- Review all `unsafe` blocks and FFI boundaries

### UI Components

- Presentation-only — no business logic in components
- Use design tokens from `src/design/css.rs` and `src/design/tokens.rs`
- Follow the component architecture in `src/components/`
- Ensure keyboard accessibility and screen reader labels
- Respect `prefers-reduced-motion`

### RTL Support

- Persian/Arabic UI uses Vazirmatn font with `dir="rtl"`
- Mirror navigation and spacing for RTL
- Media controls, timeline direction, artwork composition keep appropriate conventions

## Pull Request Process

1. Ensure all CI checks pass (format, clippy, tests, build)
2. Update documentation if implementation changes affect documented behavior
3. Add tests for new functionality
4. Link related issues in the PR description
5. Request review from maintainers

## Design System Changes

Changes to design tokens must be mirrored in both:
- `src/design/css.rs` (CSS custom properties)
- `src/design/tokens.rs` (Rust-side constants)

## Reporting Issues

Use GitHub Issues to report bugs or request features. Include:

- Steps to reproduce
- Expected vs. actual behavior
- Platform and version
- Screenshots (for UI issues)

## License

By contributing, you agree that your contributions will be licensed under
the project's license.
