//! Library service — media scanning, metadata and artwork access.
//!
//! Backed by a real metadata indexer on disk; the UI observes it through
//! signals and resources.

use crate::models::{Library, LibraryStats};
use crate::security::validate_media_dir;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ScanStatus {
    Idle,
    #[default]
    Scanning,
    Indexing,
    Finished,
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaDirectory {
    pub path: PathBuf,
    pub included: bool,
}

#[derive(Clone, PartialEq)]
pub struct LibraryService {
    pub library: Signal<Library>,
    pub stats: Signal<LibraryStats>,
    pub scan_status: Signal<ScanStatus>,
    pub media_dirs: Signal<Vec<MediaDirectory>>,
    pub indexed: Signal<bool>,
}

impl LibraryService {
    pub fn init() -> Self {
        Self {
            library: use_signal(Library::default),
            stats: use_signal(LibraryStats::default),
            scan_status: use_signal(|| ScanStatus::Idle),
            media_dirs: use_signal(|| Vec::new()),
            indexed: use_signal(|| false),
        }
    }

    pub fn track_count(&self) -> usize {
        self.library.read().tracks.len()
    }

    pub fn add_media_dir(&self, path: PathBuf) -> Result<(), crate::security::SecurityError> {
        let canonical = validate_media_dir(&path)?;
        let mut media_dirs = self.media_dirs;
        media_dirs.write().push(MediaDirectory {
            path: canonical,
            included: true,
        });
        Ok(())
    }

    pub fn remove_media_dir(&self, path: &PathBuf) -> Result<(), crate::security::SecurityError> {
        let canonical = validate_media_dir(path)?;
        let mut media_dirs = self.media_dirs;
        media_dirs.write().retain(|d| &d.path != &canonical);
        Ok(())
    }

    pub fn request_scan(&self) {
        let mut scan_status = self.scan_status;
        scan_status.set(ScanStatus::Scanning);
        // engine adapter performs the scan and calls `set_library` when done
    }

    pub fn set_library(&self, library: Library) {
        let stats = LibraryStats {
            track_count: library.tracks.len(),
            album_count: library.albums.len(),
            artist_count: library.artists.len(),
            video_count: library.videos.len(),
            playlist_count: library.playlists.len(),
            total_duration_sec: library
                .tracks
                .iter()
                .filter_map(|track| track.duration_sec)
                .chain(library.videos.iter().filter_map(|video| video.duration_sec))
                .map(u64::from)
                .sum(),
            indexed_dirs: self.media_dirs.read().len(),
            last_scan: Some(chrono::Utc::now().timestamp_millis()),
        };
        let mut stats_signal = self.stats;
        stats_signal.set(stats);
        let mut library_signal = self.library;
        library_signal.set(library);
        let mut scan_status = self.scan_status;
        scan_status.set(ScanStatus::Finished);
        let mut indexed = self.indexed;
        indexed.set(true);
    }

    pub fn set_scan_error(&self, message: String) {
        let mut scan_status = self.scan_status;
        scan_status.set(ScanStatus::Error { message });
    }
}
