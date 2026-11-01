# File Operations

## Security Principles

FAZORA treats all file operations as potentially unsafe. The following
safeguards are implemented:

### Path Traversal Protection

- All user-supplied paths are validated before use
- Path components are normalized and checked for `..` sequences
- Absolute paths outside allowed directories are rejected

### Symlink Handling

- Symbolic links are not followed during recursive operations unless
  explicitly intended and safely implemented
- Symlink escapes are detected and blocked

### Recursive Operations

- Recursive deletion, copy, and move have deliberate safeguards
- Cancellation is supported at each step
- Partial failures are handled gracefully without data loss

### Archive Extraction

- Destination paths are constrained to prevent ZIP Slip attacks
- Resource consumption is bounded (file count, total size)
- Special files (device files, fifos, symlinks) are handled safely

### Temporary Files

- Temporary files are created with secure permissions
- Temporary files are cleaned up after use
- Race conditions are avoided via atomic operations where possible

## Platform-Specific Behavior

### Linux

- Follows XDG Base Directory specification for config and data
- File permissions use standard Unix mode bits
- Symlinks are supported but not followed in recursive operations

### Windows

- Handles long paths (MAX_PATH > 260)
- Case-insensitive path comparison
- File locking behavior respected (locked files cannot be modified)
- UNC paths supported where applicable

### Android

- Uses Storage Access Framework (SAF) for file access
- No unrestricted filesystem access
- URI permissions are persisted across sessions
- Scoped storage compliance

## Operations Covered

| Operation | Safeguards |
|-----------|------------|
| List directories | Path validation, permission checks |
| File metadata | Read-only, no modification |
| File creation | Path validation, secure temp files |
| Directory creation | Path validation, parent existence |
| Copy | Path traversal protection, partial failure handling |
| Move | Path traversal protection, cross-filesystem support |
| Rename | Path validation, collision handling |
| Delete | Recursive safeguards, cancellation |
| Overwrite | Confirmation required, backup where applicable |
| Archive extraction | ZIP Slip protection, resource bounds |

## Testing

File operation tests use isolated test fixtures in temporary directories.
Real user documents are never modified during automated tests.

Test coverage includes:

- Normal operations (create, read, update, delete)
- Error conditions (permission denied, disk full, path not found)
- Security edge cases (path traversal, symlink escapes)
- Platform-specific behavior (Windows long paths, Linux permissions)
