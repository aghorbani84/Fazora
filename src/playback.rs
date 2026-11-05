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
    Off,
    One,
    #[default]
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

    pub fn is_active(&self) -> bool {
        matches!(
            self,
            PlaybackState::Playing { .. } | PlaybackState::Buffering { .. }
        )
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
    /// True only after a platform playback adapter reports successful readiness.
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
            // No native decoder/output adapter is wired at initialization.
            // The platform adapter must explicitly set this after it is ready.
            engine_available: use_signal(|| false),
        }
    }

    pub fn set_engine_available(&self, available: bool) {
        let mut signal = self.engine_available;
        signal.set(available);
    }

    pub fn set_state(&self, value: PlaybackState) {
        let mut signal = self.state;
        signal.set(value);
    }

    pub fn set_position(&self, sec: f64) {
        let mut signal = self.position_sec;
        signal.set(sec.max(0.0));
    }

    pub fn set_duration(&self, sec: f64) {
        let mut signal = self.duration_sec;
        signal.set(sec.max(0.0));
    }

    /// Initialize the Android engine only after an explicit user gesture.
    /// This keeps JNI/class-loader failures from preventing the app shell from launching.
    pub fn initialize_engine(&self) -> Result<(), String> {
        #[cfg(target_os = "android")]
        {
            crate::audio::android::initialize_global()?;
            self.set_engine_available(true);
            return Ok(());
        }
        #[cfg(not(target_os = "android"))]
        {
            Err("Native audio playback is not available on this platform".to_string())
        }
    }

    pub fn open_audio_picker(&self) -> Result<(), String> {
        self.initialize_engine()?;
        #[cfg(target_os = "android")]
        {
            return crate::audio::android::open_audio_picker();
        }
        #[cfg(not(target_os = "android"))]
        {
            Err("Audio file selection is only available on Android".to_string())
        }
    }

    pub fn load(&self, track: Track) {
        #[cfg(target_os = "android")]
        {
            if !*self.engine_available.read() {
                self.set_state(PlaybackState::Error {
                    track: Some(track),
                    message: "Audio engine is not initialized".to_string(),
                });
                return;
            }
            if let Err(message) = crate::audio::android::set_media_uri(&track.path) {
                self.set_state(PlaybackState::Error {
                    track: Some(track),
                    message,
                });
                return;
            }
        }
        self.set_state(PlaybackState::Loading { track });
    }

    pub fn play_track(&self, track: Track) {
        self.load(track);
        if matches!(&*self.state.read(), PlaybackState::Loading { .. }) {
            self.play_toggle();
        }
    }

    pub fn play_toggle(&self) {
        #[cfg(target_os = "android")]
        {
            if !*self.engine_available.read() {
                return;
            }
            let current = self.state.read().clone();
            let Some(track) = current.current_track().cloned() else {
                return;
            };

            if current.is_active() {
                match crate::audio::android::pause() {
                    Ok(()) => self.set_state(PlaybackState::Paused { track }),
                    Err(message) => self.set_state(PlaybackState::Error {
                        track: Some(track),
                        message,
                    }),
                }
            } else {
                match crate::audio::android::play() {
                    Ok(()) => self.set_state(PlaybackState::Loading { track }),
                    Err(message) => self.set_state(PlaybackState::Error {
                        track: Some(track),
                        message,
                    }),
                }
            }
        }

        #[cfg(all(not(target_os = "android"), test))]
        {
            if !*self.engine_available.read() {
                return;
            }
            let current = self.state.read().clone();
            let next = match current {
                PlaybackState::Paused { track } | PlaybackState::Ended { track } => {
                    Some(PlaybackState::Playing { track })
                }
                PlaybackState::Playing { track } | PlaybackState::Buffering { track } => {
                    Some(PlaybackState::Paused { track })
                }
                _ => None,
            };
            if let Some(next) = next {
                self.set_state(next);
            }
        }
    }

    pub fn pause(&self) {
        #[cfg(target_os = "android")]
        {
            if !*self.engine_available.read() {
                return;
            }
            let current = self.state.read().clone();
            if let Some(track) = current.current_track().cloned() {
                match crate::audio::android::pause() {
                    Ok(()) => self.set_state(PlaybackState::Paused { track }),
                    Err(message) => self.set_state(PlaybackState::Error {
                        track: Some(track),
                        message,
                    }),
                }
            }
        }

        #[cfg(all(not(target_os = "android"), test))]
        {
            if !*self.engine_available.read() {
                return;
            }
            let current = self.state.read().clone();
            if let PlaybackState::Playing { track } = current {
                self.set_state(PlaybackState::Paused { track });
            }
        }
    }

    pub fn seek(&self, to_sec: f64) {
        #[cfg(target_os = "android")]
        {
            if !*self.engine_available.read() || !to_sec.is_finite() {
                return;
            }
            if let Some(track) = self.current_track() {
                let target = to_sec.max(0.0);
                match crate::audio::android::seek_to_ms((target * 1000.0).round() as u64) {
                    Ok(()) => {
                        self.set_position(target);
                        self.set_state(PlaybackState::Seeking {
                            track,
                            to_sec: target,
                        });
                    }
                    Err(message) => self.set_state(PlaybackState::Error {
                        track: Some(track),
                        message,
                    }),
                }
            }
        }

        #[cfg(all(not(target_os = "android"), test))]
        {
            if !*self.engine_available.read() || !to_sec.is_finite() {
                return;
            }
            if let Some(track) = self.current_track() {
                self.set_state(PlaybackState::Seeking {
                    track,
                    to_sec: to_sec.max(0.0),
                });
            }
        }
    }

    pub fn set_volume(&self, volume: f32) {
        let volume = volume.clamp(0.0, 1.0);
        #[cfg(target_os = "android")]
        if *self.engine_available.read() {
            if let Err(message) = crate::audio::android::set_volume(volume) {
                if let Some(track) = self.current_track() {
                    self.set_state(PlaybackState::Error {
                        track: Some(track),
                        message,
                    });
                }
                return;
            }
        }
        let mut volume_signal = self.volume;
        volume_signal.set(volume);
        let mut muted_signal = self.muted;
        muted_signal.set(false);
    }

    pub fn toggle_mute(&self) {
        let next = !*self.muted.read();
        #[cfg(target_os = "android")]
        if *self.engine_available.read() {
            let target_volume = if next { 0.0 } else { *self.volume.read() };
            if let Err(message) = crate::audio::android::set_volume(target_volume) {
                if let Some(track) = self.current_track() {
                    self.set_state(PlaybackState::Error {
                        track: Some(track),
                        message,
                    });
                }
                return;
            }
        }
        let mut muted = self.muted;
        muted.set(next);
    }

    pub fn stop(&self) {
        #[cfg(target_os = "android")]
        if *self.engine_available.read() {
            if let Err(message) = crate::audio::android::stop() {
                self.set_state(PlaybackState::Error {
                    track: self.current_track(),
                    message,
                });
                return;
            }
        }
        self.set_state(PlaybackState::Idle);
        self.set_position(0.0);
    }

    /// Poll the native player and SAF picker; call from the app's UI task.
    pub fn poll_native(&self) {
        #[cfg(target_os = "android")]
        {
            if !*self.engine_available.read() {
                return;
            }

            match crate::audio::android::take_picked_uri() {
                Ok(Some(uri)) => {
                    let title = uri
                        .rsplit('/')
                        .next()
                        .unwrap_or("Unknown track")
                        .split('?')
                        .next()
                        .unwrap_or("Unknown track")
                        .to_string();
                    let track = Track {
                        id: uuid::Uuid::new_v4().to_string(),
                        title,
                        artist: "Unknown artist".to_string(),
                        artist_id: None,
                        album: None,
                        album_id: None,
                        genre: None,
                        year: None,
                        track_number: None,
                        duration_sec: None,
                        path: uri,
                        artwork: None,
                        play_count: 0,
                        rating: 0,
                        added_at: chrono::Utc::now().timestamp_millis(),
                        last_played: None,
                        bitrate: None,
                        sample_rate: None,
                        channels: None,
                        lyrics_synced: None,
                        lyrics: None,
                    };
                    self.play_track(track);
                }
                Ok(None) => {}
                Err(message) => self.set_state(PlaybackState::Error {
                    track: self.current_track(),
                    message,
                }),
            }

            if let Ok(snapshot) = crate::audio::android::snapshot() {
                if !snapshot.engine_available {
                    self.set_engine_available(false);
                    return;
                }
                let mut volume_signal = self.volume;
                volume_signal.set(snapshot.volume.clamp(0.0, 1.0));
                self.set_position(snapshot.position_ms.max(0) as f64 / 1000.0);
                self.set_duration(snapshot.duration_ms.max(0) as f64 / 1000.0);
                if let Some(message) = snapshot.error {
                    self.set_state(PlaybackState::Error {
                        track: self.current_track(),
                        message,
                    });
                } else if let Some(track) = self.current_track() {
                    let next = if snapshot.is_playing {
                        Some(PlaybackState::Playing { track: track.clone() })
                    } else {
                        match snapshot.playback_state {
                            2 if snapshot.play_when_ready => {
                                Some(PlaybackState::Buffering { track: track.clone() })
                            }
                            2 => Some(PlaybackState::Loading { track: track.clone() }),
                            3 => Some(PlaybackState::Paused { track: track.clone() }),
                            4 => Some(PlaybackState::Ended { track: track.clone() }),
                            _ => None,
                        }
                    };
                    if let Some(next) = next {
                        if *self.state.read() != next {
                            self.set_state(next);
                        }
                    }
                }
            }
        }
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
