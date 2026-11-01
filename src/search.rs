//! Search service — global search across the library.

use crate::models::{Album, Artist, Track, Video};
use dioxus::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct SearchResults {
    pub tracks: Vec<Track>,
    pub albums: Vec<Album>,
    pub artists: Vec<Artist>,
    pub videos: Vec<Video>,
}

#[derive(Debug, Clone, Default)]
pub struct SearchState {
    pub query: String,
    pub results: SearchResults,
    pub loading: bool,
}

#[derive(Clone)]
pub struct SearchService {
    pub state: Signal<SearchState>,
}

impl SearchService {
    pub fn init() -> Self {
        Self {
            state: use_signal(SearchState::default),
        }
    }

    pub fn set_query(&self, query: String) {
        self.state.write().with(|s| s.query = query);
    }

    pub fn set_results(&self, results: SearchResults) {
        self.state.write().with(|s| {
            s.results = results;
            s.loading = false;
        });
    }

    pub fn set_loading(&self, loading: bool) {
        self.state.write().with(|s| s.loading = loading);
    }
}
