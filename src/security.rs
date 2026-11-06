//! Security utilities for FAZORA Media.
//!
//! Provides path validation, canonicalization, and input sanitization
//! to defend against path traversal, symlink attacks, and untrusted input.

use std::path::{Path, PathBuf};

/// Maximum size for the settings JSON file (1 MiB).
pub const MAX_SETTINGS_FILE_SIZE: u64 = 1_048_576;

/// Maximum length for a user-provided search query.
pub const MAX_QUERY_LENGTH: usize = 1_000;

/// Maximum number of recent queries to retain.
pub const MAX_RECENT_QUERIES: usize = 10;

/// Allowed media directory root prefixes for path validation.
/// Paths outside these roots are rejected to prevent access to
/// sensitive system directories (e.g., /etc, /var, /root).
#[allow(dead_code, reason = "P3 platform media-directory adapter will use this")]
#[cfg(target_os = "linux")]
pub const ALLOWED_MEDIA_ROOTS: [&str; 5] = ["/home", "/mnt", "/media", "/run/media", "/tmp"];

#[allow(dead_code, reason = "consumed by the later platform media-directory adapter")]
#[cfg(target_os = "android")]
pub const ALLOWED_MEDIA_ROOTS: [&str; 4] = [
    "/storage/emulated/0",
    "/sdcard",
    "/data/user/0",
    "/data/data",
];

#[allow(dead_code, reason = "consumed by the later platform media-directory adapter")]
#[cfg(target_os = "macos")]
pub const ALLOWED_MEDIA_ROOTS: [&str; 3] = ["/Users", "/Volumes", "/tmp"];

#[allow(dead_code, reason = "consumed by the later platform media-directory adapter")]
#[cfg(target_os = "ios")]
pub const ALLOWED_MEDIA_ROOTS: [&str; 2] = ["/var/mobile/Media", "/private/var/mobile/Media"];

#[allow(dead_code, reason = "consumed by the later platform media-directory adapter")]
#[cfg(target_os = "windows")]
pub const ALLOWED_MEDIA_ROOTS: [&str; 3] = ["C:", "D:", "E:"];

/// Validate and canonicalize a media directory path.
///
/// Returns an error if the path is:
/// - Not absolute
/// - Contains `..` components (path traversal)
/// - Does not resolve to an existing directory
/// - Resolves outside an allowed root prefix
/// - Is a symlink pointing outside an allowed root
#[allow(dead_code, reason = "P3 media-directory permission layer will use this")]
pub fn validate_media_dir(path: &Path) -> Result<PathBuf, SecurityError> {
    // Reject relative paths
    if !path.is_absolute() {
        return Err(SecurityError::RelativePath);
    }

    // Reject path traversal components
    for component in path.components() {
        if matches!(component, std::path::Component::ParentDir) {
            return Err(SecurityError::PathTraversal);
        }
    }

    // Canonicalize to resolve symlinks and `..` references
    let canonical = path
        .canonicalize()
        .map_err(|_| SecurityError::PathNotFound)?;

    // Reject if canonical path is not a directory
    if !canonical.is_dir() {
        return Err(SecurityError::NotADirectory);
    }

    // Reject paths outside allowed roots
    let allowed = ALLOWED_MEDIA_ROOTS
        .iter()
        .any(|root| canonical.starts_with(Path::new(root)));

    if !allowed {
        return Err(SecurityError::PathOutsideAllowedRoot);
    }

    Ok(canonical)
}

/// Validate a settings file path before reading or writing.
///
/// Ensures the path is absolute and canonicalizes it to prevent
/// symlink-based attacks.
pub fn validate_settings_path(path: &Path) -> Result<PathBuf, SecurityError> {
    if !path.is_absolute() {
        return Err(SecurityError::RelativePath);
    }

    // Canonicalize if the file already exists; otherwise validate the parent
    if path.exists() {
        return path.canonicalize().map_err(|_| SecurityError::PathNotFound);
    }

    if let Some(parent) = path.parent() {
        if parent.exists() {
            let canonical_parent = parent
                .canonicalize()
                .map_err(|_| SecurityError::PathNotFound)?;
            return Ok(canonical_parent.join(path.file_name().unwrap_or_default()));
        }
    }

    Err(SecurityError::PathNotFound)
}

/// Sanitize a user-provided search query.
///
/// Truncates to MAX_QUERY_LENGTH and strips null bytes.
pub fn sanitize_query(query: &str) -> String {
    let trimmed = query.trim();
    let no_nulls: String = trimmed.chars().filter(|c| *c != '\0').collect();
    let truncated: String = no_nulls.chars().take(MAX_QUERY_LENGTH).collect();
    truncated
}

/// Sanitize a recent query string for storage.
#[allow(dead_code, reason = "P4 search-history integration will use this")]
pub fn sanitize_recent_query(query: &str) -> String {
    sanitize_query(query)
}

/// Security errors for path and input validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SecurityError {
    RelativePath,
    PathTraversal,
    PathNotFound,
    NotADirectory,
    PathOutsideAllowedRoot,
    FileTooLarge,
    InvalidInput,
}

impl std::fmt::Display for SecurityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SecurityError::RelativePath => write!(f, "path must be absolute"),
            SecurityError::PathTraversal => write!(f, "path traversal detected"),
            SecurityError::PathNotFound => write!(f, "path not found"),
            SecurityError::NotADirectory => write!(f, "path is not a directory"),
            SecurityError::PathOutsideAllowedRoot => {
                write!(f, "path is outside allowed media roots")
            }
            SecurityError::FileTooLarge => write!(f, "file exceeds maximum size"),
            SecurityError::InvalidInput => write!(f, "invalid input"),
        }
    }
}

impl std::error::Error for SecurityError {}

/// Read a settings file with a size limit to prevent denial of service
/// via excessively large files.
pub fn read_settings_file_limited(path: &Path) -> Result<String, SecurityError> {
    let metadata = std::fs::metadata(path).map_err(|_| SecurityError::PathNotFound)?;
    if metadata.len() > MAX_SETTINGS_FILE_SIZE {
        return Err(SecurityError::FileTooLarge);
    }
    std::fs::read_to_string(path).map_err(|_| SecurityError::InvalidInput)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_query_truncates() {
        let long = "a".repeat(2000);
        let result = sanitize_query(&long);
        assert_eq!(result.len(), MAX_QUERY_LENGTH);
    }

    #[test]
    fn test_sanitize_query_removes_nulls() {
        let result = sanitize_query("hello\0world");
        assert_eq!(result, "helloworld");
    }

    #[test]
    fn test_validate_media_dir_rejects_relative() {
        let result = validate_media_dir(Path::new("relative/path"));
        assert!(matches!(result, Err(SecurityError::RelativePath)));
    }

    #[test]
    fn test_validate_media_dir_rejects_traversal() {
        let result = validate_media_dir(Path::new("/home/../../etc"));
        assert!(matches!(result, Err(SecurityError::PathTraversal)));
    }
}
