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
    Loading {
        track: Track,
    },
    Playing {
        track: Track,
    },
    Paused {
        track: Track,
    },
    Buffering {
        track: Track,
    },
    Seeking {
        track: Track,
        to_sec: f64,
    },
    Ended {
        track: Track,
    },
    Error {
        track: Option<Track>,
        message: String,
    },
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

#[derive(Clone, PartialEq)]
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

    pub fn set_state(&self, value: PlaybackState) {
        let mut signal = self.state;
        signal.set(value);
    }

    pub fn set_position(&self, sec: f64) {
        let mut signal = self.position_sec;
        signal.set(sec);
    }

    pub fn set_duration(&self, sec: f64) {
        let mut signal = self.duration_sec;
        signal.set(sec);
    }

    pub fn load(&self, track: Track) {
        self.set_state(PlaybackState::Loading { track });
    }

    pub fn play_toggle(&self) {
        let current = self.state.read().clone();
        let next = match current {
            PlaybackState::Paused { track } | PlaybackState::Ended { track } => {
                Some(PlaybackState::Playing { track })
            }
            PlaybackState::Playing { track } => Some(PlaybackState::Paused { track }),
            _ => None,
        };
        if let Some(next) = next {
            self.set_state(next);
        }
    }

    pub fn pause(&self) {
        let current = self.state.read().clone();
        if let PlaybackState::Playing { track } = current {
            self.set_state(PlaybackState::Paused { track });
        }
    }

    pub fn seek(&self, to_sec: f64) {
        if let Some(track) = self.current_track() {
            self.set_state(PlaybackState::Seeking {
                track,
                to_sec: to_sec.max(0.0),
            });
        }
    }

    pub fn set_volume(&self, volume: f32) {
        let mut volume_signal = self.volume;
        volume_signal.set(volume.clamp(0.0, 1.0));
        let mut muted_signal = self.muted;
        muted_signal.set(false);
    }

    pub fn toggle_mute(&self) {
        let mut muted = self.muted;
        let next = !*muted.read();
        muted.set(next);
    }

    pub fn stop(&self) {
        self.set_state(PlaybackState::Idle);
    }

    pub fn set_repeat(&self, mode: RepeatMode) {
        let mut repeat = self.repeat;
        repeat.set(mode);
    }

    pub fn toggle_shuffle(&self) {
        let mut shuffle = self.shuffle;
        let next = !*shuffle.read();
        shuffle.set(next);
    }

    pub fn current_track(&self) -> Option<Track> {
        self.state.read().current_track().cloned()
    }

    pub fn is_playing(&self) -> bool {
        self.state.read().is_playing()
    }

    pub fn progress_ratio(&self) -> f64 {
        let pos = *self.position_sec.read();
        let dur = *self.duration_sec.read();
        if dur > 0.0 {
            (pos / dur).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}
