//! Playback service — the bridge between UI state and the native engine.
//!
//! This struct is the UI-facing abstraction of playback. A real engine
//! adapter (libVLC / libmpv / platform backend) drives these signals; the
//! UI never touches native playback resources directly.

use crate::models::Track;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum RepeatMode {
    #[default]
    Off,
    One,
    All,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PlaybackState {
    Idle,
    Loading { track: Track },
    Playing { track: Track },
    Paused { track: Track },
    Buffering { track: Track },
    Seeking { track: Track, to_sec: f64 },
    Ended { track: Track },
    Error { track: Option<Track>, message: String },
}

impl PlaybackState {
    pub fn current_track(&self) -> Option<&Track> {
        match self {
            PlaybackState::Idle => None,
            PlaybackState::Loading { track }
            | PlaybackState::Playing { track }
            | PlaybackState::Paused { track }
            | PlaybackState::Buffering { track }
            | PlaybackState::Seeking { track, .. }
            | PlaybackState::Ended { track } => Some(track),
            PlaybackState::Error { track, .. } => track.as_ref(),
        }
    }

    pub fn is_playing(&self) -> bool {
        matches!(self, PlaybackState::Playing { .. })
    }

    pub fn is_error(&self) -> bool {
        matches!(self, PlaybackState::Error { .. })
    }
}

/// UI-facing playback service. Initialise with `PlaybackService::init()` inside
/// a component scope, then provide it via context.
#[derive(Clone)]
pub struct PlaybackService {
    pub state: Signal<PlaybackState>,
    pub position_sec: Signal<f64>,
    pub duration_sec: Signal<f64>,
    pub volume: Signal<f32>,
    pub muted: Signal<bool>,
    pub repeat: Signal<RepeatMode>,
    pub shuffle: Signal<bool>,
    pub engine_available: Signal<bool>,
}

impl PlaybackService {
    pub fn init() -> Self {
        Self {
            state: use_signal(|| PlaybackState::Idle),
            position_sec: use_signal(|| 0.0),
            duration_sec: use_signal(|| 0.0),
            volume: use_signal(|| 1.0),
            muted: use_signal(|| false),
            repeat: use_signal(|| RepeatMode::All),
            shuffle: use_signal(|| false),
            engine_available: use_signal(|| true),
        }
    }

    /// Called by the native engine adapter when state changes externally.
    pub fn set_state(&self, state: PlaybackState) {
        self.state.set(state);
    }

    pub fn set_position(&self, sec: f64) {
        self.position_sec.set(sec);
    }

    pub fn set_duration(&self, sec: f64) {
        self.duration_sec.set(sec);
    }

    pub fn load(&self, track: Track) {
        self.state.set(PlaybackState::Loading {
            track: track.clone(),
        });
    }

    pub fn play_toggle(&self) {
        match &*self.state.read() {
            PlaybackState::Paused { track }
            | PlaybackState::Ended { track } => {
                self.state.set(PlaybackState::Playing {
                    track: track.clone(),
                });
            }
            PlaybackState::Playing { track } => {
                self.state.set(PlaybackState::Paused {
                    track: track.clone(),
                });
            }
            _ => {}
        }
    }

    pub fn pause(&self) {
        if let PlaybackState::Playing { track } = &*self.state.read() {
            self.state.set(PlaybackState::Paused {
                track: track.clone(),
            });
        }
    }

    pub fn seek(&self, to_sec: f64) {
        if let Some(track) = self.state.current_track().cloned() {
            self.state.set(PlaybackState::Seeking {
                track,
                to_sec,
            });
        }
    }

    pub fn set_volume(&self, volume: f32) {
        self.volume.set(volume.clamp(0.0, 1.0));
        self.muted.set(false);
    }

    pub fn toggle_mute(&self) {
        self.muted.set(!self.muted.get());
    }

    pub fn stop(&self) {
        self.state.set(PlaybackState::Idle);
    }

    pub fn set_repeat(&self, mode: RepeatMode) {
        self.repeat.set(mode);
    }

    pub fn toggle_shuffle(&self) {
        self.shuffle.set(!self.shuffle.get());
    }

    pub fn current_track(&self) -> Option<Track> {
        self.state.read().current_track().cloned()
    }

    pub fn is_playing(&self) -> bool {
        self.state.read().is_playing()
    }

    pub fn progress_ratio(&self) -> f64 {
        let pos = self.position_sec.get();
        let dur = self.duration_sec.get();
        if dur > 0.0 {
            (pos / dur).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}
