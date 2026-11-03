//! Playback service tests.
//!
//! `PlaybackService::init()` creates `Signal`s via `use_signal()`, which must
//! run inside a component scope, so each test mounts the service through the
//! `VirtualDom` harness defined in `mod.rs`.

use crate::models::Track;
use crate::playback::{PlaybackService, PlaybackState, RepeatMode};
use dioxus::prelude::*;

service_harness!(
    PlaybackHarness,
    PLAYBACK_SLOT,
    PlaybackService,
    PlaybackService::init
);

fn test_track() -> Track {
    Track {
        id: "t1".to_string(),
        title: "Test Track".to_string(),
        artist: "Test Artist".to_string(),
        artist_id: None,
        album: None,
        album_id: None,
        genre: None,
        year: None,
        track_number: None,
        duration_sec: Some(180),
        path: "/music/test.mp3".to_string(),
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
fn playback_initial_state_is_idle() {
    let playback = with_service();

    assert!(matches!(playback.state.read().as_ref(), PlaybackState::Idle));
    assert!(!playback.is_playing());
    assert!(!playback.state.read().is_error());
    assert_eq!(playback.position_sec.get(), 0.0);
    assert_eq!(playback.duration_sec.get(), 0.0);
    assert_eq!(playback.volume.get(), 1.0);
    assert!(!playback.muted.get());
    assert_eq!(playback.repeat.get(), RepeatMode::All);
    assert!(!playback.shuffle.get());
    assert!(playback.engine_available.get());
}

#[test]
fn playback_load_sets_loading_state() {
    let playback = with_service();

    let track = test_track();
    playback.load(track.clone());

    assert!(matches!(
        playback.state.read().as_ref(),
        PlaybackState::Loading { ref t } if t.id == "t1"
    ));
    assert_eq!(playback.current_track().unwrap().id, "t1");
}

#[test]
fn playback_play_toggle_from_idle_does_nothing() {
    let playback = with_service();

    playback.play_toggle();

    assert!(matches!(playback.state.read().as_ref(), PlaybackState::Idle));
    assert!(!playback.is_playing());
}

#[test]
fn playback_play_toggle_from_paused_starts_playing() {
    let playback = with_service();

    let track = test_track();
    playback.set_state(PlaybackState::Paused {
        track: track.clone(),
    });
    playback.play_toggle();

    assert!(matches!(
        playback.state.read().as_ref(),
        PlaybackState::Playing { ref t } if t.id == "t1"
    ));
    assert!(playback.is_playing());
}

#[test]
fn playback_play_toggle_from_playing_pauses() {
    let playback = with_service();

    let track = test_track();
    playback.set_state(PlaybackState::Playing {
        track: track.clone(),
    });
    playback.play_toggle();

    assert!(matches!(
        playback.state.read().as_ref(),
        PlaybackState::Paused { ref t } if t.id == "t1"
    ));
    assert!(!playback.is_playing());
}

#[test]
fn playback_play_toggle_from_ended_restarts() {
    let playback = with_service();

    let track = test_track();
    playback.set_state(PlaybackState::Ended {
        track: track.clone(),
    });
    playback.play_toggle();

    assert!(matches!(
        playback.state.read().as_ref(),
        PlaybackState::Playing { ref t } if t.id == "t1"
    ));
    assert!(playback.is_playing());
}

#[test]
fn playback_pause_from_playing_sets_paused() {
    let playback = with_service();

    let track = test_track();
    playback.set_state(PlaybackState::Playing {
        track: track.clone(),
    });
    playback.pause();

    assert!(matches!(
        playback.state.read().as_ref(),
        PlaybackState::Paused { ref t } if t.id == "t1"
    ));
    assert!(!playback.is_playing());
}

#[test]
fn playback_pause_from_idle_does_nothing() {
    let playback = with_service();

    playback.pause();

    assert!(matches!(playback.state.read().as_ref(), PlaybackState::Idle));
}

#[test]
fn playback_seek_sets_seeking_state() {
    let playback = with_service();

    let track = test_track();
    playback.set_state(PlaybackState::Playing {
        track: track.clone(),
    });
    playback.seek(45.5);

    assert!(matches!(
        playback.state.read().as_ref(),
        PlaybackState::Seeking { ref t, to_sec } if t.id == "t1" && *to_sec == 45.5
    ));
}

#[test]
fn playback_seek_without_track_does_nothing() {
    let playback = with_service();

    playback.seek(45.5);

    assert!(matches!(playback.state.read().as_ref(), PlaybackState::Idle));
}

#[test]
fn playback_set_volume_clamps() {
    let playback = with_service();

    playback.set_volume(1.5);
    assert_eq!(playback.volume.get(), 1.0);
    assert!(!playback.muted.get());

    playback.set_volume(-0.5);
    assert_eq!(playback.volume.get(), 0.0);

    playback.set_volume(0.5);
    assert_eq!(playback.volume.get(), 0.5);
}

#[test]
fn playback_toggle_mute() {
    let playback = with_service();

    assert!(!playback.muted.get());
    playback.toggle_mute();
    assert!(playback.muted.get());
    playback.toggle_mute();
    assert!(!playback.muted.get());
}

#[test]
fn playback_stop_sets_idle() {
    let playback = with_service();

    let track = test_track();
    playback.set_state(PlaybackState::Playing {
        track: track.clone(),
    });
    playback.stop();

    assert!(matches!(playback.state.read().as_ref(), PlaybackState::Idle));
}

#[test]
fn playback_set_repeat_cycles() {
    let playback = with_service();

    assert_eq!(playback.repeat.get(), RepeatMode::All);
    playback.set_repeat(RepeatMode::One);
    assert_eq!(playback.repeat.get(), RepeatMode::One);
    playback.set_repeat(RepeatMode::Off);
    assert_eq!(playback.repeat.get(), RepeatMode::Off);
}

#[test]
fn playback_toggle_shuffle() {
    let playback = with_service();

    assert!(!playback.shuffle.get());
    playback.toggle_shuffle();
    assert!(playback.shuffle.get());
    playback.toggle_shuffle();
    assert!(!playback.shuffle.get());
}

#[test]
fn playback_progress_ratio() {
    let playback = with_service();

    // No duration set -> ratio is 0
    assert_eq!(playback.progress_ratio(), 0.0);

    playback.set_duration(100.0);
    playback.set_position(25.0);
    assert_eq!(playback.progress_ratio(), 0.25);

    playback.set_position(150.0);
    assert_eq!(playback.progress_ratio(), 1.0);

    playback.set_position(-10.0);
    assert_eq!(playback.progress_ratio(), 0.0);
}

#[test]
fn playback_state_is_playing() {
    assert!(PlaybackState::Playing {
        track: test_track()
    }
    .is_playing());
    assert!(!PlaybackState::Paused {
        track: test_track()
    }
    .is_playing());
    assert!(!PlaybackState::Idle.is_playing());
    assert!(!PlaybackState::Loading {
        track: test_track()
    }
    .is_playing());
    assert!(!PlaybackState::Buffering {
        track: test_track()
    }
    .is_playing());
    assert!(!PlaybackState::Seeking {
        track: test_track(),
        to_sec: 0.0
    }
    .is_playing());
    assert!(!PlaybackState::Ended {
        track: test_track()
    }
    .is_playing());
}

#[test]
fn playback_state_is_error() {
    assert!(PlaybackState::Error {
        track: None,
        message: "test error".to_string()
    }
    .is_error());
    assert!(!PlaybackState::Idle.is_error());
    assert!(!PlaybackState::Playing {
        track: test_track()
    }
    .is_error());
}

#[test]
fn playback_state_current_track() {
    let track = test_track();

    assert!(PlaybackState::Idle.current_track().is_none());
    assert_eq!(
        PlaybackState::Loading {
            track: track.clone()
        }
        .current_track()
        .unwrap()
        .id,
        "t1"
    );
    assert_eq!(
        PlaybackState::Playing {
            track: track.clone()
        }
        .current_track()
        .unwrap()
        .id,
        "t1"
    );
    assert_eq!(
        PlaybackState::Paused {
            track: track.clone()
        }
        .current_track()
        .unwrap()
        .id,
        "t1"
    );

    // Error with no track
    assert!(PlaybackState::Error {
        track: None,
        message: "err".to_string()
    }
    .current_track()
    .is_none());

    // Error with track
    assert_eq!(
        PlaybackState::Error {
            track: Some(track.clone()),
            message: "err".to_string()
        }
        .current_track()
        .unwrap()
        .id,
        "t1"
    );
}

#[test]
fn repeat_mode_default() {
    assert_eq!(RepeatMode::default(), RepeatMode::All);
}

#[test]
fn repeat_mode_variants() {
    assert_eq!(RepeatMode::Off as u32, 0);
    assert_eq!(RepeatMode::One as u32, 1);
    assert_eq!(RepeatMode::All as u32, 2);
}
