# Android Storage Access

## Overview

FAZORA Media on Android uses the Storage Access Framework (SAF) for file
access, consistent with Android's scoped storage model. The application does
not assume unrestricted filesystem access.

## Storage Access Framework

### URI Permissions

FAZORA requests persistent URI permissions for media directories:

```kotlin
// Conceptual SAF usage
val intent = Intent(Intent.ACTION_OPEN_DOCUMENT_TREE)
startActivityForResult(intent, REQUEST_CODE)
// On result:
contentResolver.takePersistableUriPermission(
    uri,
    Intent.FLAG_GATE_READ_URI_PERMISSION
)
```

### Permission Revocation

The application handles permission revocation gracefully:

- If a URI permission is revoked, the corresponding media directory is
  marked as inaccessible
- Users are prompted to re-grant access when needed
- No crash or data loss occurs on revocation

### Document Provider Failures

- Document provider unavailability is handled with an error state
- Users can select an alternative directory
- Previously indexed media remains available until re-indexing is attempted

## Scoped Storage

FAZORA complies with Android scoped storage:

- Access is limited to app-specific directories and user-selected directories
- No access to shared storage without explicit user action
- Media files are accessed via content URIs, not file paths
- No `REQUEST_INSTALL_PACKAGES` or broad storage permissions

## Testing

Android storage tests use isolated test fixtures and never modify real user
documents. Tests cover:

- SAF URI permission grant and persistence
- Permission revocation handling
- Document provider failure scenarios
- Scoped storage compliance

See `src/tests/` for the test structure. Android-specific tests require
the Android emulator or a physical device.
