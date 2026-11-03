//! Music library destination: statistics, search, filters, sorting,
//! display modes, album grid and track list.

use crate::components::media::{AlbumCard, ArtistCard, TrackRow};
use crate::components::primitives::{Badge, Button, Input};
use crate::models::{Album, Artist, Track};
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
    let app_state = state.read();
    let library_service = app_state.library.clone();
    let stats = library_service.stats.clone();

    let display_mode = use_signal(|| DisplayMode::Grid);
    let query = use_signal(|| String::new());
    let selected_tracks = use_signal(|| std::collections::HashSet::<String>::new());

    let filtered_albums: Vec<Album> = library_service.library.read()
        .albums.iter()
        .filter(|a| {
            let q = query.get().to_lowercase();
            q.is_empty() || a.title.to_lowercase().contains(&q) || a.artist.to_lowercase().contains(&q)
        })
        .cloned()
        .collect();

    rsx! {
        div {
            class: "fz-page",
            div {
                class: "fz-section-header",
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: var(--space-6); flex-wrap: wrap; gap: var(--space-3);",
                div {
                    h1 {
                        style: "font-size: var(--text-3xl); font-weight: 700; color: var(--text-primary);",
                        "Music library"
                    }
                    p {
                        style: "font-size: var(--text-sm); color: var(--text-secondary); margin-top: var(--space-1);",
                        format!("{} tracks · {} albums · {} artists", stats.read().track_count, stats.read().album_count, stats.read().artist_count)
                    }
                }
                div {
                    style: "display: flex; gap: var(--space-2);",
                    Badge { variant: crate::components::primitives::BadgeVariant::Accent }
                    Input {
                        placeholder: "Search albums, artists…",
                        aria_label: "Library search",
                        oninput: Some(move |evt| query.set(evt.value())),
                    }
                }
            }

            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: var(--space-4);",
                div {
                    style: "display: flex; gap: var(--space-2);",
                    Button {
                        variant: if display_mode.read() == &DisplayMode::Grid {
                            crate::components::primitives::ButtonVariant::Primary
                        } else {
                            crate::components::primitives::ButtonVariant::Surface
                        },
                        onclick: move |_| display_mode.set(DisplayMode::Grid),
                        label: "Grid",
                    }
                    Button {
                        variant: if display_mode.read() == &DisplayMode::List {
                            crate::components::primitives::ButtonVariant::Primary
                        } else {
                            crate::components::primitives::ButtonVariant::Surface
                        },
                        onclick: move |_| display_mode.set(DisplayMode::List),
                        label: "List",
                    }
                }
            }

            if display_mode.read() == &DisplayMode::Grid {
                div {
                    class: "fz-grid",
                    for album in filtered_albums {
                        AlbumCard {
                            key: "{album.id}",
                            album,
                            on_play: Some(move |_| {}),
                            on_shuffle: Some(move |_| {}),
                            on_add_queue: Some(move |_| {}),
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
                            th { "Year" }
                            th { style: "text-align: right;", "Duration" }
                        }
                    }
                    tbody {
                        for track in &library_service.library.read().tracks {
                            TrackRow {
                                key: "{track.id}",
                                track: track.clone(),
                                index: 0,
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

#[component]
fn ArtistGridItem(artist: Artist) -> Element {
    rsx! {
        ArtistCard { artist }
    }
}
