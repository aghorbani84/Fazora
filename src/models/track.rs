//! Track model.

use serde::{Deserialize, Serialize};

/// A single audio track in the library.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub artist_id: Option<String>,
    pub album: Option<String>,
    pub album_id: Option<String>,
    pub genre: Option<String>,
    pub year: Option<i32>,
    pub track_number: Option<u32>,
    pub duration_sec: Option<u32>,
    pub path: String,
    pub artwork: Option<String>,
    pub play_count: u32,
    pub rating: u8,
    pub added_at: i64,
    pub last_played: Option<i64>,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub channels: Option<u8>,
    /// Synchronised lyrics, if available.
    pub lyrics_synced: Option<Vec<LyricLine>>,
    /// Unsynchronised lyrics, if available.
    pub lyrics: Option<String>,
}

impl Track {
    pub fn formatted_duration(&self) -> Option<String> {
        let secs = self.duration_sec?;
        let minutes = secs / 60;
        let seconds = secs % 60;
        Some(format!("{}:{:02}", minutes, seconds))
    }

    pub fn subtitle(&self) -> String {
        match (self.artist.is_empty(), self.album.as_deref()) {
            (false, Some(album)) => format!("{} · {}", self.artist, album),
            (false, None) => self.artist.clone(),
            (true, Some(album)) => album.to_string(),
            (true, None) => "Unknown artist".to_string(),
        }
    }
}

/// One line of synchronised lyrics with its start offset in seconds.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LyricLine {
    pub time: f64,
    pub text: String,
}
