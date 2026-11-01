//! Queue service — the real playback queue, owned by the playback service.

use crate::models::Track;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueEntry {
    pub id: String,
    pub track: Track,
    /// When this entry was added to the queue (epoch ms), for stable ordering.
    pub queued_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum QueueMode {
    #[default]
    Sequential,
    Shuffle,
}

#[derive(Clone)]
pub struct QueueService {
    pub entries: Signal<Vec<QueueEntry>>,
    pub index: Signal<Option<usize>>,
    pub mode: Signal<QueueMode>,
    pub history: Signal<Vec<String>>,
}

impl Default for QueueService {
    fn default() -> Self {
        Self::new()
    }
}

impl QueueService {
    pub fn new() -> Self {
        Self {
            entries: use_signal(|| Vec::new()),
            index: use_signal(|| None),
            mode: use_signal(|| QueueMode::Sequential),
            history: use_signal(|| Vec::new()),
        }
    }

    pub fn current(&self) -> Option<Track> {
        self.index
            .read()
            .and_then(|i| self.entries.read().get(i))
            .map(|e| e.track.clone())
    }

    pub fn upcoming(&self) -> Vec<Track> {
        let entries = self.entries.read();
        let idx = self.index.read().unwrap_or(0);
        entries
            .get(idx + 1..)
            .unwrap_or(&[])
            .iter()
            .map(|e| e.track.clone())
            .collect()
    }

    pub fn add(&self, track: Track, play_next: bool) {
        let entry = QueueEntry {
            id: uuid::Uuid::new_v4().to_string(),
            track,
            queued_at: chrono::Utc::now().timestamp_millis(),
        };
        self.entries.write().with(|entries| {
            if play_next {
                let insert_at = self.index.read().map(|i| i + 1).unwrap_or(0);
                entries.insert(insert_at.clamp(0, entries.len()), entry);
            } else {
                entries.push(entry);
            }
        });
    }

    pub fn add_many(&self, tracks: Vec<Track>) {
        self.entries.write().with(|entries| {
            for track in tracks {
                entries.push(QueueEntry {
                    id: uuid::Uuid::new_v4().to_string(),
                    track,
                    queued_at: chrono::Utc::now().timestamp_millis(),
                });
            }
        });
    }

    pub fn remove(&self, position: usize) {
        self.entries.write().with(|entries| {
            if position < entries.len() {
                entries.remove(position);
                // Clamp index if it pointed at or past the removed entry.
                let mut idx = self.index.write();
                if let Some(i) = *idx {
                    if i >= position && !entries.is_empty() {
                        *idx = Some((i - 1).min(entries.len() - 1));
                    } else if entries.is_empty() {
                        *idx = None;
                    }
                }
            }
        });
    }

    pub fn clear(&self) {
        self.entries.write().clear();
        self.index.set(None);
        self.history.write().clear();
    }

    pub fn reorder(&self, from: usize, to: usize) {
        self.entries.write().with(|entries| {
            if from >= entries.len() || to >= entries.len() {
                return;
            }
            let entry = entries.remove(from);
            entries.insert(to, entry);
        });
    }

    pub fn save_as_playlist(&self, title: String) -> crate::models::Playlist {
        let entries = self.entries.read();
        let track_ids: Vec<String> = entries.iter().map(|e| e.track.id.clone()).collect();
        crate::models::Playlist {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            description: None,
            track_ids,
            artwork: None,
            created_at: chrono::Utc::now().timestamp_millis(),
            modified_at: chrono::Utc::now().timestamp_millis(),
            favorite: false,
        }
    }

    pub fn set_mode(&self, mode: QueueMode) {
        self.mode.set(mode);
    }

    pub fn toggle_shuffle(&self) {
        self.mode.set(match self.mode.get() {
            QueueMode::Sequential => QueueMode::Shuffle,
            QueueMode::Shuffle => QueueMode::Sequential,
        });
    }

    pub fn set_index(&self, index: usize) {
        let entries_len = self.entries.read().len();
        if index < entries_len {
            self.index.set(Some(index));
        }
    }

    pub fn move_next(&self) -> Option<Track> {
        let entries = self.entries.read();
        let idx = self.index.read().unwrap_or(0);
        let next = if idx + 1 < entries.len() { idx + 1 } else { 0 };
        drop(entries);
        self.index.set(Some(next));
        self.current()
    }

    pub fn move_previous(&self) -> Option<Track> {
        let idx = self.index.read().unwrap_or(0);
        let prev = if idx == 0 { 0 } else { idx - 1 };
        self.index.set(Some(prev));
        self.current()
    }
}
