//! FAZORA Media library crate.
//!
//! Public surface used by tests and potential desktop/mobile embedders.
//! The binary entry point lives in `src/main.rs`.

pub use app::App;
pub use design::tokens::*;
pub use models::*;
pub use playback::PlaybackService;
pub use queue::QueueService;
pub use settings::SettingsService;

#[cfg(test)]
mod tests;
