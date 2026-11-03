//! Album page: large artwork, metadata, track listing, playback actions.

use crate::components::media::{Artwork, Metadata};
use crate::components::primitives::{Badge, Button};
use crate::models::{Album, Track};
use crate::state::AppState;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AlbumProps {
    id: String,
}

#[component]
pub fn Album(props: AlbumProps) -> Element {
    let state = use_context::<Signal<AppState>>();
    let app_state = state.read();
    let library = app_state.library.clone();

    let album = library
        .library
        .read()
        .albums
        .iter()
        .find(|a| a.id == props.id)
        .cloned();
    let album = match album {
        Some(a) => a,
        None => return rsx! { div { class: "fz-empty", h2 { "Album not found" } } },
    };

    let tracks: Vec<Track> = library
        .library
        .read()
        .tracks
        .iter()
        .filter(|t| {
            t.album_id.as_deref() == Some(&props.id) || t.album.as_deref() == Some(&album.title)
        })
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
                        src: album.artwork.clone(),
                        width: 420,
                        height: 420,
                        alt: album.title.clone(),
                    }
                }
                div {
                    flex: 1,
                    min_width: 0,
                    h1 {
                        style: "font-size: var(--text-5xl); font-weight: 700; color: var(--text-primary); letter-spacing: -0.02em;",
                        "{album.title}"
                    }
                    h2 {
                        style: "font-size: var(--text-xl); font-weight: 600; color: var(--text-secondary); margin-top: var(--space-2);",
                        "{album.artist}"
                    }
                    Metadata {
                        items: vec![
                            ("Year".to_string(), album.year.map(|y| y.to_string()).unwrap_or_else(|| "—".to_string())),
                            ("Genre".to_string(), album.genre.clone().unwrap_or_else(|| "—".to_string())),
                            ("Tracks".to_string(), album.track_count.map(|n| n.to_string()).unwrap_or_else(|| "—".to_string())),
                            ("Duration".to_string(), album.duration_sec.map(|s| {
                                let m = s / 60; format!("{}:{:02}", m, s % 60)
                            }).unwrap_or_else(|| "—".to_string())),
                        ]
                    }
                    div {
                        style: "margin-top: var(--space-6); display: flex; gap: var(--space-2); flex-wrap: wrap;",
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Primary,
                            label: "Play album",
                        }
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Surface,
                            label: "Shuffle",
                        }
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Ghost,
                            label: "Add to queue",
                        }
                    }
                }
            }

            div {
                style: "margin-top: var(--space-10);",
                h3 {
                    style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-4);",
                    "Tracks"
                }
                table {
                    class: "fz-table",
                    thead {
                        tr {
                            th { "#" }
                            th { "Title" }
                            th { "Duration" }
                            th { style: "width: 60px;", "" }
                        }
                    }
                    tbody {
                        for (i, track) in tracks.iter().enumerate() {
                            tr {
                                key: "{track.id}",
                                class: "fz-track-row",
                                onclick: move |_| {},
                                td { style: "color: var(--text-muted); font-family: var(--font-mono);", "{i + 1}" }
                                td { "{track.title}" }
                                td { style: "font-family: var(--font-mono); color: var(--text-muted); text-align: right;", "{track.formatted_duration().unwrap_or_default()}" }
                                td {
                                    style: "text-align: right;",
                                    Button {
                                        variant: crate::components::primitives::ButtonVariant::Ghost,
                                        onclick: move |_| {},
                                        svg { class: "fz-icon", width: 16, height: 16, use { href: "#icon-queue" } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
