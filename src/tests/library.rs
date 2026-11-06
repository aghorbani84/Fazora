//! Library service tests.
//!
//! `LibraryService::init()` creates `Signal`s via `use_signal()`, which must
//! run inside a component scope, so each test mounts the service through the
//! `VirtualDom` harness defined in `mod.rs`.

use crate::library_service::{LibraryService, MediaDirectory, ScanStatus};
use crate::models::{Album, Artist, Library, LibraryStats, Track, Video};
use dioxus::prelude::*;
use std::path::PathBuf;

service_harness!(
    LibraryHarness,
    LIBRARY_SLOT,
    LibraryService,
    LibraryService::init
);

#[test]
fn library_initial_state() {
    let library = with_service();

    assert_eq!(library.track_count(), 0);
    assert_eq!(&*library.scan_status.read(), &ScanStatus::Idle);
    assert!(library.media_dirs.read().is_empty());
    assert!(!*library.indexed.read());
    assert!(library.library.read().tracks.is_empty());
}

#[test]
fn library_add_remove_media_dir() {
    let library = with_service();

    let root = tempfile::tempdir().expect("create temporary media root");
    let dir1 = root.path().join("music");
    let dir2 = root.path().join("video");
    std::fs::create_dir(&dir1).expect("create music test directory");
    std::fs::create_dir(&dir2).expect("create video test directory");

    library.add_media_dir(dir1.clone()).expect("add music test directory");
    library.add_media_dir(dir2.clone()).expect("add video test directory");

    assert_eq!(library.media_dirs.read().len(), 2);
    assert!(library.media_dirs.read()[0].included);
    assert_eq!(library.media_dirs.read()[0].path, dir1);

    library.remove_media_dir(&dir1).expect("remove music test directory");
    assert_eq!(library.media_dirs.read().len(), 1);
    assert_eq!(library.media_dirs.read()[0].path, dir2);

    library.remove_media_dir(&dir2).expect("remove video test directory");
    assert!(library.media_dirs.read().is_empty());
}

#[test]
fn library_request_scan_sets_scanning() {
    let library = with_service();

    assert_eq!(&*library.scan_status.read(), &ScanStatus::Idle);
    library.request_scan();
    assert_eq!(&*library.scan_status.read(), &ScanStatus::Scanning);
}

#[test]
fn library_set_library_updates_stats_and_status() {
    let library_service = with_service();

    let lib = Library {
        tracks: vec![Track {
            id: "t1".to_string(),
            title: "Track One".to_string(),
            artist: "Artist One".to_string(),
            artist_id: None,
            album: None,
            album_id: None,
            genre: None,
            year: None,
            track_number: None,
            duration_sec: Some(180),
            path: "/music/t1.mp3".to_string(),
            artwork: None,
            play_count: 0,
            rating: 0,
            added_at: 0,
            last_played: None,
            bitrate: None,
            sample_rate: None,
            channels: None,
            lyrics_synced: None,
            lyrics: None,
        }],
        albums: vec![Album {
            id: "a1".to_string(),
            title: "Album One".to_string(),
            artist: "Artist One".to_string(),
            artist_id: None,
            year: None,
            genre: None,
            track_count: None,
            duration_sec: None,
            artwork: None,
            added_at: 0,
        }],
        artists: vec![Artist {
            id: "ar1".to_string(),
            name: "Artist One".to_string(),
            album_count: None,
            track_count: None,
            artwork: None,
            genre: None,
            added_at: 0,
        }],
        videos: vec![Video {
            id: "v1".to_string(),
            title: "Video One".to_string(),
            path: "/video/v1.mp4".to_string(),
            thumbnail: None,
            duration_sec: Some(120),
            width: Some(1920),
            height: Some(1080),
            watched_position_sec: None,
            watched: false,
            favorite: false,
            added_at: 0,
            codec: None,
            fps: None,
        }],
        playlists: vec![],
        stats: LibraryStats::default(),
    };

    library_service.set_library(lib.clone());

    assert_eq!(library_service.track_count(), 1);
    assert_eq!(library_service.library.read().tracks.len(), 1);
    assert_eq!(library_service.library.read().albums.len(), 1);
    assert_eq!(library_service.library.read().artists.len(), 1);
    assert_eq!(library_service.library.read().videos.len(), 1);
    assert_eq!(library_service.stats.read().track_count, 1);
    assert_eq!(library_service.stats.read().album_count, 1);
    assert_eq!(library_service.stats.read().artist_count, 1);
    assert_eq!(library_service.stats.read().video_count, 1);
    assert_eq!(library_service.stats.read().total_duration_sec, 180 + 120);
    assert!(*library_service.indexed.read());
    assert_eq!(&*library_service.scan_status.read(), &ScanStatus::Finished);
}

#[test]
fn library_set_scan_error() {
    let library = with_service();

    library.set_scan_error("Permission denied".to_string());

    assert_eq!(
        &*library.scan_status.read(),
        &ScanStatus::Error {
            message: "Permission denied".to_string()
        }
    );
}

#[test]
fn library_stats_default() {
    let stats = LibraryStats::default();
    assert_eq!(stats.track_count, 0);
    assert_eq!(stats.total_duration_sec, 0);
    assert!(stats.last_scan.is_none());
}

#[test]
fn media_directory_creation() {
    let dir = MediaDirectory {
        path: PathBuf::from("/music"),
        included: true,
    };

    assert_eq!(dir.path, PathBuf::from("/music"));
    assert!(dir.included);
}
