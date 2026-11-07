//! Search service — global search across the library.

use crate::models::{Album, Artist, Track, Video};
use crate::security::sanitize_query;
use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SearchResults {
    pub tracks: Vec<Track>,
    pub albums: Vec<Album>,
    pub artists: Vec<Artist>,
    pub videos: Vec<Video>,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct SearchState {
    pub query: String,
    pub results: SearchResults,
    pub loading: bool,
}

#[derive(Clone, PartialEq)]
pub struct SearchService {
    pub state: Signal<SearchState>,
}

impl SearchService {
    pub fn init() -> Self {
        Self {
            state: use_signal(SearchState::default),
        }
    }

    #[allow(dead_code, reason = "P4 search engine integration will use this")]
    pub fn set_query(&self, query: String) {
        let mut state = self.state;
        state.write().query = sanitize_query(&query);
    }

    #[allow(
        dead_code,
        reason = "reserved for the P4 search engine integration; Phase 1 is CI-only"
    )]
    pub fn set_results(&self, results: SearchResults) {
        let mut state = self.state;
        let mut current = state.write();
        current.results = results;
        current.loading = false;
    }

    #[allow(
        dead_code,
        reason = "reserved for the P4 search engine integration; Phase 1 is CI-only"
    )]
    pub fn set_loading(&self, loading: bool) {
        let mut state = self.state;
        state.write().loading = loading;
    }
}
