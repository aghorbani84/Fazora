//! Video model.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Video {
    pub id: String,
    pub title: String,
    pub path: String,
    pub thumbnail: Option<String>,
    pub duration_sec: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub watched_position_sec: Option<u32>,
    pub watched: bool,
    pub favorite: bool,
    pub added_at: i64,
    pub codec: Option<String>,
    pub fps: Option<f64>,
}

impl Video {
    pub fn formatted_duration(&self) -> Option<String> {
        let secs = self.duration_sec?;

        let minutes = secs / 60;
        let seconds = secs % 60;
        Some(format!("{}:{:02}", minutes, seconds))
    }

    pub fn progress_ratio(&self) -> f64 {
        match (self.watched_position_sec, self.duration_sec) {
            (Some(pos), Some(dur)) if dur > 0 => pos as f64 / dur as f64,
            _ => 0.0,
        }
    }

    pub fn resolution_label(&self) -> Option<String> {
        match (self.width, self.height) {
            (Some(w), Some(h)) => Some(format!("{}x{}", w, h)),
            _ => None,
        }
    }
}
