//! Artist page: artwork, name, album/track counts, top tracks, albums.

use crate::models::{Album, Artist, Track};
use crate::components::media::{Artwork, Metadata};
use crate::components::primitives::Button;
use crate::state::AppState;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ArtistViewProps {
    id: String,
}

#[component]
pub fn Artist(props: ArtistViewProps) -> Element {
    let state = use_context::<Signal<AppState>>();
    let app_state = state.read();
    let library = app_state.library.clone();

    let artist = library.library.read().artists.iter().find(|a| a.id == props.id).cloned();
    let artist = match artist {
        Some(a) => a,
        None => return rsx! { div { class: "fz-empty", h2 { "Artist not found" } } },
    };

    let artist_albums: Vec<Album> = library.library.read()
        .albums.iter()
        .filter(|a| a.artist_id.as_deref() == Some(&props.id) || a.artist == artist.name)
        .cloned()
        .collect();

    let artist_tracks: Vec<Track> = library.library.read()
        .tracks.iter()
        .filter(|t| t.artist_id.as_deref() == Some(&props.id) || t.artist == artist.name)
        .cloned()
        .collect();

    rsx! {
        div {
            class: "fz-page",
            div {
                style: "display: flex; gap: var(--space-8); align-items: flex-start;",
                div {
                    style: "flex: 0 0 420px;",
                    Artwork {
                        src: artist.artwork.clone(),
                        width: 420,
                        height: 420,
                        alt: artist.name.clone(),
                    }
                }
                div {
                    flex: 1,
                    min_width: 0,
                    h1 {
                        style: "font-size: var(--text-5xl); font-weight: 700; color: var(--text-primary); letter-spacing: -0.02em;",
                        "{artist.name}"
                    }
                    Metadata {
                        items: vec![
                            ("Albums".to_string(), artist.album_count.map(|n| n.to_string()).unwrap_or_else(|| artist_albums.len().to_string())),
                            ("Tracks".to_string(), artist.track_count.map(|n| n.to_string()).unwrap_or_else(|| artist_tracks.len().to_string())),
                            ("Genre".to_string(), artist.genre.clone().unwrap_or_else(|| "—".to_string())),
                        ]
                    }
                    div {
                        style: "margin-top: var(--space-6); display: flex; gap: var(--space-2); flex-wrap: wrap;",
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Primary,
                            label: "Play all",
                        }
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Surface,
                            label: "Shuffle",
                        }
                    }
                }
            }

            div {
                style: "margin-top: var(--space-10);",
                h3 {
                    style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-4);",
                    "Albums"
                }
                if artist_albums.is_empty() {
                    p { style: "color: var(--text-muted);", "No albums found." }
                } else {
                    div {
                        class: "fz-grid",
                        for album in artist_albums {
                            crate::components::media::AlbumCard {
                                key: "{album.id}",
                                album,
                                on_play: Some(move |_| {}),
                                on_shuffle: Some(move |_| {}),
                                on_add_queue: Some(move |_| {}),
                            }
                        }
                    }
                }
            }

            div {
                style: "margin-top: var(--space-10);",
                h3 {
                    style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-4);",
                    "Top tracks"
                }
                table {
                    class: "fz-table",
                    thead {
                        tr {
                            th { "#" }
                            th { "Title" }
                            th { "Album" }
                            th { style: "text-align: right;", "Duration" }
                        }
                    }
                    tbody {
                        for (i, track) in artist_tracks.iter().enumerate() {
                            tr {
                                key: "{track.id}",
                                class: "fz-track-row",
                                td { style: "color: var(--text-muted); font-family: var(--font-mono);", "{i + 1}" }
                                td { "{track.title}" }
                                td { style: "color: var(--text-secondary);", "{track.album.as_deref().unwrap_or("")}" }
                                td { style: "font-family: var(--font-mono); color: var(--text-muted); text-align: right;", "{track.formatted_duration().unwrap_or_default()}" }
                            }
                        }
                    }
                }
            }
        }
    }
}
