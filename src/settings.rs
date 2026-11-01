//! Settings service — persisted application settings.

use crate::models::Playlist;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Settings {
    pub theme: Theme,
    pub language: String,
    pub ui_density: String, // "compact" | "comfortable" | "spacious"
    pub animations: bool,
    pub scan_on_startup: bool,
    pub autoplay: bool,
    pub repeat_mode: String,
    pub shuffle: bool,
    pub playback_speed: f32,
    pub output_device: Option<String>,
    pub equalizer_enabled: bool,
    pub crossfade_sec: f32,
    pub gapless: bool,
    pub media_dirs: Vec<String>,
    pub recent_queries: Vec<String>,
    pub window_maximised: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            theme: Theme::Dark,
            language: "en".to_string(),
            ui_density: "comfortable".to_string(),
            animations: true,
            scan_on_startup: true,
            autoplay: true,
            repeat_mode: "all".to_string(),
            shuffle: false,
            playback_speed: 1.0,
            output_device: None,
            equalizer_enabled: false,
            crossfade_sec: 0.0,
            gapless: true,
            media_dirs: Vec::new(),
            recent_queries: Vec::new(),
            window_maximised: false,
        }
    }
}

#[derive(Clone)]
pub struct SettingsService {
    pub settings: Signal<Settings>,
    pub path: Option<std::path::PathBuf>,
}

impl SettingsService {
    pub fn init() -> Self {
        Self {
            settings: use_signal(Settings::default),
            path: None,
        }
    }

    pub fn load(&mut self) {
        // Read settings.json from the platform config dir if present.
        if let Some(path) = &self.path {
            if let Ok(text) = std::fs::read_to_string(path) {
                if let Ok(settings) = serde_json::from_str::<Settings>(&text) {
                    self.settings.set(settings);
                }
            }
        }
    }

    pub fn save(&self) {
        if let Some(path) = &self.path {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if let Ok(text) = serde_json::to_string_pretty(&*self.settings.read()) {
                let _ = std::fs::write(path, text);
            }
        }
    }

    pub fn add_recent_query(&self, query: String) {
        self.settings.write().with(|s| {
            s.recent_queries.retain(|q| q != &query);
            s.recent_queries.insert(0, query);
            s.recent_queries.truncate(10);
        });
    }

    pub fn clear_recent_queries(&self) {
        self.settings.write().with(|s| s.recent_queries.clear());
    }
}
