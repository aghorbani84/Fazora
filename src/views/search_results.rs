//! Search results view — tracks, artists, albums, videos and playlists.

use crate::models::{Album, Artist, Track, Video};
use crate::state::AppState;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SearchResultsProps {
    query: String,
}

#[component]
pub fn SearchResults(props: SearchResultsProps) -> Element {
    let state = use_context::<Signal<AppState>>();
    let app_state = state.read();
    let library = app_state.library.clone();

    let query = props.query.clone();
    let q = query.to_lowercase();

    let tracks: Vec<Track> = library
        .library
        .read()
        .tracks
        .iter()
        .filter(|t| t.title.to_lowercase().contains(&q) || t.artist.to_lowercase().contains(&q))
        .cloned()
        .collect();

    let albums: Vec<Album> = library
        .library
        .read()
        .albums
        .iter()
        .filter(|a| a.title.to_lowercase().contains(&q) || a.artist.to_lowercase().contains(&q))
        .cloned()
        .collect();

    let artists: Vec<Artist> = library
        .library
        .read()
        .artists
        .iter()
        .filter(|a| a.name.to_lowercase().contains(&q))
        .cloned()
        .collect();

    let videos: Vec<Video> = library
        .library
        .read()
        .videos
        .iter()
        .filter(|v| v.title.to_lowercase().contains(&q))
        .cloned()
        .collect();

    let empty = tracks.is_empty() && albums.is_empty() && artists.is_empty() && videos.is_empty();

    rsx! {
        div {
            class: "fz-page",
            div {
                class: "fz-section-header",
                style: "margin-bottom: var(--space-6);",
                h1 {
                    style: "font-size: var(--text-3xl); font-weight: 700; color: var(--text-primary);",
                    "Search results"
                }
                p {
                    style: "font-size: var(--text-sm); color: var(--text-secondary); margin-top: var(--space-1);",
                    "\"{query}\""
                }
            }

            if empty {
                div {
                    class: "fz-empty",
                    style: "height: auto;",
                    p { "No results found for \"{query}\"" }
                }
            } else {
                if !tracks.is_empty() {
                    section {
                        class: "fz-section",
                        h3 {
                            style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-4);",
                            "Tracks"
                        }
                        table {
                            class: "fz-table",
                            thead {
                                tr {
                                    th { "Title" }
                                    th { "Artist" }
                                    th { "Album" }
                                    th { style: "text-align: right; width: 60px;", "Duration" }
                                }
                            }
                            tbody {
                                for track in tracks {
                                    tr {
                                        key: "{track.id}",
                                        class: "fz-track-row",
                                        td { "{track.title}" }
                                        td { style: "color: var(--text-secondary);", "{track.artist}" }
                                        td { style: "color: var(--text-secondary);", {track.album.as_deref().unwrap_or("")} }
                                        td { style: "font-family: var(--font-mono); color: var(--text-muted); text-align: right;", "{track.formatted_duration().unwrap_or_default()}" }
                                    }
                                }
                            }
                        }
                    }
                }
                if !albums.is_empty() {
                    section {
                        class: "fz-section",
                        h3 {
                            style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-4);",
                            "Albums"
                        }
                        div {
                            class: "fz-grid",
                            for album in albums {
                                crate::components::media::AlbumCard {
                                    key: "{album.id}",
                                    album,
                                    on_play: move |_| {},
                                    on_shuffle: move |_| {},
                                    on_add_queue: move |_| {},
                                }
                            }
                        }
                    }
                }
                if !artists.is_empty() {
                    section {
                        class: "fz-section",
                        h3 {
                            style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-4);",
                            "Artists"
                        }
                        div {
                            class: "fz-grid",
                            for artist in artists {
                                crate::components::media::ArtistCard {
                                    key: "{artist.id}",
                                    artist,
                                }
                            }
                        }
                    }
                }
                if !videos.is_empty() {
                    section {
                        class: "fz-section",
                        h3 {
                            style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-4);",
                            "Videos"
                        }
                        div {
                            class: "fz-grid",
                            for video in videos {
                                crate::components::media::VideoCard {
                                    key: "{video.id}",
                                    video,
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
