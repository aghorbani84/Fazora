//! Search service tests.
//!
//! `SearchService::init()` creates `Signal`s via `use_signal()`, which must
//! run inside a component scope, so each test mounts the service through the
//! `VirtualDom` harness defined in `mod.rs`.

use crate::models::{SearchResults, Track};
use crate::search::SearchService;
use dioxus::prelude::*;

service_harness!(
    SearchHarness,
    SEARCH_SLOT,
    SearchService,
    SearchService::init
);

#[test]
fn search_initial_state() {
    let search = with_service();

    assert_eq!(search.state.read().query, "");
    assert!(search.state.read().results.tracks.is_empty());
    assert!(search.state.read().results.albums.is_empty());
    assert!(search.state.read().results.artists.is_empty());
    assert!(search.state.read().results.videos.is_empty());
    assert!(!search.state.read().loading);
}

#[test]
fn search_set_query() {
    let search = with_service();

    search.set_query("queen".to_string());
    assert_eq!(search.state.read().query, "queen");
}

#[test]
fn search_set_results() {
    let search = with_service();

    let results = SearchResults {
        tracks: vec![Track {
            id: "t1".to_string(),
            title: "Bohemian Rhapsody".to_string(),
            artist: "Queen".to_string(),
            artist_id: None,
            album: None,
            album_id: None,
            genre: None,
            year: None,
            track_number: None,
            duration_sec: Some(354),
            path: "/music/queen.mp3".to_string(),
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
        albums: vec![],
        artists: vec![],
        videos: vec![],
    };

    search.set_loading(true);
    assert!(search.state.read().loading);

    search.set_results(results.clone());
    assert!(!search.state.read().loading);
    assert_eq!(search.state.read().results.tracks.len(), 1);
    assert_eq!(
        search.state.read().results.tracks[0].title,
        "Bohemian Rhapsody"
    );
}

#[test]
fn search_set_loading() {
    let search = with_service();

    assert!(!search.state.read().loading);
    search.set_loading(true);
    assert!(search.state.read().loading);
    search.set_loading(false);
    assert!(!search.state.read().loading);
}

#[test]
fn search_results_default() {
    let results = SearchResults::default();
    assert!(results.tracks.is_empty());
    assert!(results.albums.is_empty());
    assert!(results.artists.is_empty());
    assert!(results.videos.is_empty());
}

#[test]
fn search_state_default() {
    let state = crate::search::SearchState::default();
    assert_eq!(state.query, "");
    assert!(state.results.tracks.is_empty());
    assert!(!state.loading);
}
