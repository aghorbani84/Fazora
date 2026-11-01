//! Album model.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Album {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub artist_id: Option<String>,
    pub year: Option<i32>,
    pub genre: Option<String>,
    pub track_count: Option<u32>,
    pub duration_sec: Option<u32>,
    pub artwork: Option<String>,
    pub added_at: i64,
}

impl Album {
    pub fn subtitle(&self) -> String {
        match (self.year, &self.artist) {
            (Some(year), Some(artist)) => format!("{} · {}", artist, year),
            (None, Some(artist)) => artist.clone(),
            (Some(year), None) => year.to_string(),
            (None, None) => "Unknown album".to_string(),
        }
    }
}
