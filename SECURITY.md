# FAZORA Media Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.0   | ✅        |

## Security Model

### Untrusted Data

All media files, remote streams, metadata, subtitles, and playlists are
treated as untrusted input. The application:

- Never loads arbitrarily large media files entirely into memory
- Validates file paths to prevent traversal attacks
- Constrains archive extraction destinations
- Handles malformed metadata gracefully
- Propagates native engine errors without exposing raw system errors

### Native Engine Integration

FAZORA uses native multimedia engines (libVLC, libmpv) via FFI. Security
considerations:

- Engine resources are released safely on shutdown
- Native callbacks cannot access destroyed Rust objects
- Pointer ownership and lifetimes are validated at FFI boundaries
- Panic boundaries exist around native calls

### Credentials and Secrets

- No hardcoded credentials in source code
- Build secrets (signing keys, API tokens) use GitHub environment secrets
- Secret access is restricted to protected release jobs

### File Operations

- Path traversal protection for all file operations
- Symlink traversal is not followed during recursive operations unless explicitly intended
- Recursive destructive operations have deliberate safeguards
- Cancellation and partial failure handling is implemented
- Temporary files are created securely

## Reporting a Vulnerability

To report a security vulnerability, please contact the project maintainer
via GitHub Issues. Do not disclose vulnerabilities publicly until they have
been addressed.

## Security Scanning

The project uses automated security scanning via GitHub Actions:

- `cargo audit` — vulnerability scanning of Rust dependencies
- `cargo deny` — license and dependency policy enforcement
- `actionlint` — GitHub Actions workflow linting
- `zizmor` — GitHub Actions workflow security scanning

Security scans run on every push and pull request, with a full dependency
audit scheduled weekly.

## Disclaimer

FAZORA Media is provided as-is without security warranties. Users are
responsible for validating the suitability of the software for their use case.
