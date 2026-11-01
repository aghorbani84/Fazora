//! Aggregate state holder.

use crate::{
    audio::AudioService, library::LibraryService, playback::PlaybackService,
    queue::QueueService, search::SearchService, settings::SettingsService,
};
use dioxus::prelude::*;

/// Root application state. Every service lives here; views reach them through
/// `use_context::<Signal<AppState>>()`.
#[derive(Clone)]
pub struct AppState {
    pub playback: PlaybackService,
    pub queue: QueueService,
    pub library: LibraryService,
    pub settings: SettingsService,
    pub audio: AudioService,
    pub search: SearchService,
}

impl AppState {
    pub fn new() -> Signal<Self> {
        use_signal(|| Self {
            playback: PlaybackService::init(),
            queue: QueueService::new(),
            library: LibraryService::init(),
            settings: SettingsService::init(),
            audio: AudioService::init(),
            search: SearchService::init(),
        })
    }
}
