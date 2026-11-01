//! Playlist model.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Playlist {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub track_ids: Vec<String>,
    pub artwork: Option<String>,
    pub created_at: i64,
    pub modified_at: i64,
    pub favorite: bool,
}

impl Playlist {
    pub fn track_count(&self) -> usize {
        self.track_ids.len()
    }
}
