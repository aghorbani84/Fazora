//! Settings service — persisted application settings.

use crate::models::Playlist;
use crate::security::{read_settings_file_limited, validate_settings_path};
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

    pub fn load(&mut self) -> Result<(), crate::security::SecurityError> {
        // Read settings.json from the platform config dir if present.
        if let Some(path) = &self.path {
            let validated = validate_settings_path(path)?;
            let text = read_settings_file_limited(&validated)?;
            if let Ok(settings) = serde_json::from_str::<Settings>(&text) {
                self.settings.set(settings);
            }
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), crate::security::SecurityError> {
        if let Some(path) = &self.path {
            let validated = validate_settings_path(path)?;
            if let Some(parent) = validated.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let text = serde_json::to_string_pretty(&*self.settings.read())
                .map_err(|_| crate::security::SecurityError::InvalidInput)?;
            std::fs::write(&validated, text)
                .map_err(|_| crate::security::SecurityError::InvalidInput)?;
        }
        Ok(())
    }

    pub fn add_recent_query(&self, query: String) {
        let mut s = self.settings.write();
        let sanitized = crate::security::sanitize_query(&query);
        if sanitized.is_empty() {
            return;
        }
        s.recent_queries.retain(|q| q != &sanitized);
        s.recent_queries.insert(0, sanitized);
        s.recent_queries.truncate(crate::security::MAX_RECENT_QUERIES);
    }

    pub fn clear_recent_queries(&self) {
        let mut s = self.settings.write();
        s.recent_queries.clear();
    }
}
