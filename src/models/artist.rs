//! Artist model.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub album_count: Option<u32>,
    pub track_count: Option<u32>,
    pub artwork: Option<String>,
    pub genre: Option<String>,
    pub added_at: i64,
}
