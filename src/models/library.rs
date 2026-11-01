//! Library aggregate and statistics.

use serde::{Deserialize, Serialize};

use crate::models::{Album, Artist, Playlist, Track, Video};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LibraryStats {
    pub track_count: usize,
    pub album_count: usize,
    pub artist_count: usize,
    pub video_count: usize,
    pub playlist_count: usize,
    pub total_duration_sec: u64,
    pub indexed_dirs: usize,
    pub last_scan: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Library {
    pub tracks: Vec<Track>,
    pub albums: Vec<Album>,
    pub artists: Vec<Artist>,
    pub videos: Vec<Video>,
    pub playlists: Vec<Playlist>,
    pub stats: LibraryStats,
}

impl Library {
    pub fn find_track(&self, id: &str) -> Option<&Track> {
        self.tracks.iter().find(|t| t.id == id)
    }

    pub fn find_album(&self, id: &str) -> Option<&Album> {
        self.albums.iter().find(|a| a.id == id)
    }

    pub fn find_artist(&self, id: &str) -> Option<&Artist> {
        self.artists.iter().find(|a| a.id == id)
    }

    pub fn find_video(&self, id: &str) -> Option<&Video> {
        self.videos.iter().find(|v| v.id == id)
    }

    pub fn find_playlist(&self, id: &str) -> Option<&Playlist> {
        self.playlists.iter().find(|p| p.id == id)
    }
}
