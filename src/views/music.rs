//! Music library destination: statistics, search, display modes and media lists.

use crate::components::media::{AlbumCard, TrackRow};
use crate::components::primitives::{Badge, Button, Input};
use crate::models::Album;
use crate::state::AppState;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DisplayMode {
    Grid,
    List,
}

#[component]
pub fn Music() -> Element {
    let state = use_context::<Signal<AppState>>();
    let library_service = state.read().library.clone();
    let stats = library_service.stats.read().clone();
    let albums = library_service.library.read().albums.clone();
    let tracks = library_service.library.read().tracks.clone();

    let display_mode = use_signal(|| DisplayMode::Grid);
    let query = use_signal(String::new);
    let query_value = query.cloned().to_lowercase();

    let filtered_albums: Vec<Album> = albums
        .into_iter()
        .filter(|album| {
            query_value.is_empty()
                || album.title.to_lowercase().contains(&query_value)
                || album.artist.to_lowercase().contains(&query_value)
        })
        .collect();

    rsx! {
        div {
            class: "fz-page",
            div {
                class: "fz-section-header",
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: var(--space-6); flex-wrap: wrap; gap: var(--space-3);",
                div {
                    h1 { style: "font-size: var(--text-3xl); font-weight: 700; color: var(--text-primary);", "Music library" }
                    p {
                        style: "font-size: var(--text-sm); color: var(--text-secondary); margin-top: var(--space-1);",
                        "{stats.track_count} tracks · {stats.album_count} albums · {stats.artist_count} artists"
                    }
                }
                div {
                    style: "display: flex; gap: var(--space-2);",
                    Badge { variant: crate::components::primitives::BadgeVariant::Accent }
                    Input {
                        placeholder: "Search albums, artists…",
                        aria_label: "Library search",
                        value: query.cloned(),
                        oninput: move |event: FormEvent| {
                            let mut signal = query;
                            signal.set(event.value());
                        },
                    }
                }
            }

            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: var(--space-4);",
                div {
                    style: "display: flex; gap: var(--space-2);",
                    Button {
                        variant: if *display_mode.read() == DisplayMode::Grid {
                            crate::components::primitives::ButtonVariant::Primary
                        } else {
                            crate::components::primitives::ButtonVariant::Surface
                        },
                        onclick: move |_| {
                            let mut signal = display_mode;
                            signal.set(DisplayMode::Grid);
                        },
                        label: "Grid",
                    }
                    Button {
                        variant: if *display_mode.read() == DisplayMode::List {
                            crate::components::primitives::ButtonVariant::Primary
                        } else {
                            crate::components::primitives::ButtonVariant::Surface
                        },
                        onclick: move |_| {
                            let mut signal = display_mode;
                            signal.set(DisplayMode::List);
                        },
                        label: "List",
                    }
                }
            }

            if *display_mode.read() == DisplayMode::Grid {
                div {
                    class: "fz-grid",
                    for album in filtered_albums {
                        AlbumCard {
                            key: "{album.id}",
                            album,
                            on_play: move |_| {},
                            on_shuffle: move |_| {},
                            on_add_queue: move |_| {},
                        }
                    }
                }
            } else {
                table {
                    class: "fz-table",
                    thead {
                        tr {
                            th { "Title" }
                            th { "Artist" }
                            th { "Album" }
                            th { "Duration" }
                        }
                    }
                    tbody {
                        for (index, track) in tracks.iter().cloned().enumerate() {
                            TrackRow {
                                key: "{track.id}",
                                track,
                                index,
                                selected: false,
                                playing: false,
                                on_select: move |_| {},
                                on_toggle_select: move |_| {},
                                on_play: move |_| {},
                                on_add_queue: move |_| {},
                                on_favorite: move |_| {},
                            }
                        }
                    }
                }
            }
        }
    }
}
