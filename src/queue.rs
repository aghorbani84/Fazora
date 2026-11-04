//! Queue service — the real playback queue, owned by the playback service.

use crate::models::Track;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueueEntry {
    pub id: String,
    pub track: Track,
    pub queued_at: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum QueueMode {
    #[default]
    Sequential,
    Shuffle,
}

#[derive(Clone, PartialEq)]
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
            entries: use_signal(Vec::new),
            index: use_signal(|| None),
            mode: use_signal(|| QueueMode::Sequential),
            history: use_signal(Vec::new),
        }
    }

    pub fn current(&self) -> Option<Track> {
        let index = *self.index.read();
        let entries = self.entries.read();
        index
            .and_then(|i| entries.get(i))
            .map(|entry| entry.track.clone())
    }

    pub fn upcoming(&self) -> Vec<Track> {
        let entries = self.entries.read();
        match *self.index.read() {
            Some(index) => entries
                .get(index.saturating_add(1)..)
                .unwrap_or(&[])
                .iter()
                .map(|entry| entry.track.clone())
                .collect(),
            None => entries.iter().map(|entry| entry.track.clone()).collect(),
        }
    }

    pub fn add(&self, track: Track, play_next: bool) {
        let entry = QueueEntry {
            id: uuid::Uuid::new_v4().to_string(),
            track,
            queued_at: chrono::Utc::now().timestamp_millis(),
        };
        let mut entries_signal = self.entries;
        let mut entries = entries_signal.write();
        if play_next {
            let current_index = *self.index.read();
            let insert_at = current_index.map(|i| i.saturating_add(1)).unwrap_or(0);
            let len = entries.len();
            entries.insert(insert_at.min(len), entry);
        } else {
            entries.push(entry);
        }
    }

    pub fn add_many(&self, tracks: Vec<Track>) {
        let mut entries_signal = self.entries;
        let mut entries = entries_signal.write();
        for track in tracks {
            entries.push(QueueEntry {
                id: uuid::Uuid::new_v4().to_string(),
                track,
                queued_at: chrono::Utc::now().timestamp_millis(),
            });
        }
    }

    pub fn remove(&self, position: usize) {
        let mut entries_signal = self.entries;
        let mut entries = entries_signal.write();
        if position >= entries.len() {
            return;
        }
        entries.remove(position);
        let new_len = entries.len();
        drop(entries);

        let mut index_signal = self.index;
        let mut index = index_signal.write();
        *index = match *index {
            Some(_) if new_len == 0 => None,
            Some(i) if i > position => Some(i - 1),
            Some(i) if i == position => (position < new_len).then_some(position),
            other => other,
        };
    }

    pub fn clear(&self) {
        let mut entries = self.entries;
        entries.write().clear();
        let mut index = self.index;
        index.set(None);
        let mut history = self.history;
        history.write().clear();
    }

    pub fn reorder(&self, from: usize, to: usize) {
        let mut entries_signal = self.entries;
        let mut entries = entries_signal.write();
        if from >= entries.len() || to >= entries.len() || from == to {
            return;
        }
        let entry = entries.remove(from);
        entries.insert(to, entry);
        drop(entries);

        let mut index_signal = self.index;
        let mut index = index_signal.write();
        if let Some(current) = *index {
            *index = Some(if current == from {
                to
            } else if from < current && current <= to {
                current - 1
            } else if to <= current && current < from {
                current + 1
            } else {
                current
            });
        }
    }

    pub fn save_as_playlist(&self, title: String) -> crate::models::Playlist {
        let entries = self.entries.read();
        let track_ids = entries.iter().map(|entry| entry.track.id.clone()).collect();
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
        let mut signal = self.mode;
        signal.set(mode);
    }

    pub fn toggle_shuffle(&self) {
        let mut signal = self.mode;
        let next_mode = match *signal.read() {
            QueueMode::Sequential => QueueMode::Shuffle,
            QueueMode::Shuffle => QueueMode::Sequential,
        };
        signal.set(next_mode);
    }

    pub fn set_index(&self, index: usize) {
        let len = self.entries.read().len();
        if index < len {
            let mut signal = self.index;
            signal.set(Some(index));
        }
    }

    pub fn move_next(&self) -> Option<Track> {
        let len = self.entries.read().len();
        if len == 0 {
            return None;
        }

        let next = match *self.index.read() {
            None => 0,
            Some(current) if current + 1 < len => current + 1,
            Some(_) => 0,
        };

        let mut signal = self.index;
        signal.set(Some(next));
        self.current()
    }

    pub fn move_previous(&self) -> Option<Track> {
        let len = self.entries.read().len();
        if len == 0 {
            return None;
        }

        let previous = match *self.index.read() {
            None => len - 1,
            Some(0) => len - 1,
            Some(current) => current - 1,
        };

        let mut signal = self.index;
        signal.set(Some(previous));
        self.current()
    }
}
