//! Aggregate state holder.

use crate::{
    audio::AudioService, library_service::LibraryService, playback::PlaybackService, queue::QueueService,
    search::SearchService, settings::SettingsService,
};

/// Root application state. Every service lives here; views reach them through
/// `use_context::<Signal<AppState>>()`.
#[derive(Clone)]
pub struct AppState {
    pub playback: PlaybackService,
    pub queue: QueueService,
    pub library: LibraryService,
    #[allow(dead_code, reason = "reserved for the later settings UI integration")]
    pub settings: SettingsService,
    pub audio: AudioService,
    #[allow(dead_code, reason = "reserved for the P4 search engine integration")]
    pub search: SearchService,
}

impl AppState {
    pub fn new(
        playback: PlaybackService,
        queue: QueueService,
        library: LibraryService,
        settings: SettingsService,
        audio: AudioService,
        search: SearchService,
    ) -> Self {
        Self {
            playback,
            queue,
            library,
            settings,
            audio,
            search,
        }
    }
}
