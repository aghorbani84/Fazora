//! Queue service tests.
//!
//! Uses Dioxus's testing render scope because `QueueService::new()`
//! creates `Signal`s via `use_signal()` which requires a component scope.

use crate::models::Track;
use crate::queue::{QueueService, QueueMode};

fn test_track(id: &str, title: &str) -> Track {
    Track {
        id: id.to_string(),
        title: title.to_string(),
        artist: "Test Artist".to_string(),
        artist_id: None,
        album: None,
        album_id: None,
        genre: None,
        year: None,
        track_number: None,
        duration_sec: Some(180),
        path: format!("/music/{}.mp3", id),
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
    }
}

#[test]
fn queue_initial_state_is_empty() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    assert!(queue.entries.read().is_empty());
    assert_eq!(queue.index.read(), &None);
    assert_eq!(queue.mode.read(), &QueueMode::Sequential);
    assert!(queue.history.read().is_empty());
    assert!(queue.current().is_none());
    assert!(queue.upcoming().is_empty());
}

#[test]
fn queue_add_tracks() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.add(test_track("t2", "Track Two"), false);
    queue.add(test_track("t3", "Track Three"), false);

    assert_eq!(queue.entries.read().len(), 3);
    assert_eq!(queue.index.read(), &None); // index not set on add
}

#[test]
fn queue_add_play_next_inserts_after_current() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.set_index(0);
    queue.add(test_track("t2", "Track Two"), true);
    queue.add(test_track("t3", "Track Three"), false);

    let entries = queue.entries.read();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0].track.id, "t1");
    assert_eq!(entries[1].track.id, "t2"); // play_next inserts at index+1
    assert_eq!(entries[2].track.id, "t3");
}

#[test]
fn queue_add_many() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    let tracks = vec![
        test_track("t1", "Track One"),
        test_track("t2", "Track Two"),
        test_track("t3", "Track Three"),
    ];
    queue.add_many(tracks);

    assert_eq!(queue.entries.read().len(), 3);
}

#[test]
fn queue_remove_updates_index() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.add(test_track("t2", "Track Two"), false);
    queue.add(test_track("t3", "Track Three"), false);
    queue.set_index(1); // current is Track Two

    // Remove track at position 0 (Track One)
    queue.remove(0);

    // Index should shift from 1 to 0
    assert_eq!(queue.index.read(), &Some(0));
    assert_eq!(queue.entries.read().len(), 2);
    assert_eq!(queue.entries.read()[0].track.id, "t2");
}

#[test]
fn queue_remove_last_element_clears_index() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.set_index(0);
    queue.remove(0);

    assert_eq!(queue.index.read(), &None);
    assert!(queue.entries.read().is_empty());
}

#[test]
fn queue_clear() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.add(test_track("t2", "Track Two"), false);
    queue.set_index(1);
    queue.history.write().push("h1".to_string());

    queue.clear();

    assert!(queue.entries.read().is_empty());
    assert_eq!(queue.index.read(), &None);
    assert!(queue.history.read().is_empty());
}

#[test]
fn queue_reorder() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.add(test_track("t2", "Track Two"), false);
    queue.add(test_track("t3", "Track Three"), false);

    // Move t3 (index 2) to position 0
    queue.reorder(2, 0);

    let entries = queue.entries.read();
    assert_eq!(entries[0].track.id, "t3");
    assert_eq!(entries[1].track.id, "t1");
    assert_eq!(entries[2].track.id, "t2");
}

#[test]
fn queue_reorder_invalid_positions_noop() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.add(test_track("t2", "Track Two"), false);

    queue.reorder(0, 5); // out of bounds
    queue.reorder(5, 0); // out of bounds

    assert_eq!(queue.entries.read().len(), 2);
    assert_eq!(queue.entries.read()[0].track.id, "t1");
    assert_eq!(queue.entries.read()[1].track.id, "t2");
}

#[test]
fn queue_save_as_playlist() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.add(test_track("t2", "Track Two"), false);

    let playlist = queue.save_as_playlist("My Playlist".to_string());

    assert_eq!(playlist.title, "My Playlist");
    assert_eq!(playlist.track_ids.len(), 2);
    assert_eq!(playlist.track_ids[0], "t1");
    assert_eq!(playlist.track_ids[1], "t2");
    assert!(!playlist.favorite);
    assert!(playlist.description.is_none());
    assert!(playlist.artwork.is_none());
}

#[test]
fn queue_move_next_cycles_to_start() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.add(test_track("t2", "Track Two"), false);
    queue.add(test_track("t3", "Track Three"), false);
    queue.set_index(2); // last track

    let next = queue.move_next();
    assert_eq!(next.unwrap().id, "t1"); // wraps to first
    assert_eq!(queue.index.read(), &Some(0));
}

#[test]
fn queue_move_next_advances() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.add(test_track("t2", "Track Two"), false);
    queue.set_index(0);

    let next = queue.move_next();
    assert_eq!(next.unwrap().id, "t2");
    assert_eq!(queue.index.read(), &Some(1));
}

#[test]
fn queue_move_previous_wraps_to_end() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.add(test_track("t2", "Track Two"), false);
    queue.add(test_track("t3", "Track Three"), false);
    queue.set_index(0); // first track

    let prev = queue.move_previous();
    assert_eq!(prev.unwrap().id, "t3"); // wraps to last
    assert_eq!(queue.index.read(), &Some(2));
}

#[test]
fn queue_move_previous_goes_back() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.add(test_track("t2", "Track Two"), false);
    queue.set_index(1);

    let prev = queue.move_previous();
    assert_eq!(prev.unwrap().id, "t1");
    assert_eq!(queue.index.read(), &Some(0));
}

#[test]
fn queue_upcoming_returns_tracks_after_current() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.add(test_track("t2", "Track Two"), false);
    queue.add(test_track("t3", "Track Three"), false);
    queue.set_index(0);

    let upcoming = queue.upcoming();
    assert_eq!(upcoming.len(), 2);
    assert_eq!(upcoming[0].id, "t2");
    assert_eq!(upcoming[1].id, "t3");
}

#[test]
fn queue_set_mode_toggle_shuffle() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    assert_eq!(queue.mode.read(), &QueueMode::Sequential);
    queue.toggle_shuffle();
    assert_eq!(queue.mode.read(), &QueueMode::Shuffle);
    queue.toggle_shuffle();
    assert_eq!(queue.mode.read(), &QueueMode::Sequential);
}

#[test]
fn queue_mode_default() {
    assert_eq!(QueueMode::default(), QueueMode::Sequential);
}

#[test]
fn queue_set_index_out_of_bounds_noop() {
    let mut scope = dioxus::testing::RenderScope::new();
    let queue = scope.create_service(|| QueueService::new());

    queue.add(test_track("t1", "Track One"), false);
    queue.set_index(5); // out of bounds

    assert_eq!(queue.index.read(), &None);
}

#[test]
fn queue_default() {
    use std::default::Default;
    let queue = QueueService::default();
    assert!(queue.entries.read().is_empty());
    assert_eq!(queue.index.read(), &None);
}
