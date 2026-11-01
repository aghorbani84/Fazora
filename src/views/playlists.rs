//! Playlists index.

use crate::components::primitives::Button;
use crate::models::Playlist;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn Playlists() -> Element {
    let state = use_context::<Signal<AppState>>();
    let playlists = state.read().library.read().playlists.clone();

    rsx! {
        div {
            class: "fz-page",
            div {
                class: "fz-section-header",
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: var(--space-6);",
                div {
                    h1 {
                        style: "font-size: var(--text-3xl); font-weight: 700; color: var(--text-primary);",
                        "Playlists"
                    }
                }
                Button {
                    variant: crate::components::primitives::ButtonVariant::Primary,
                    label: "Create playlist",
                }
            }
            if playlists.is_empty() {
                div {
                    class: "fz-empty",
                    p { "No playlists yet. Create your first playlist from the queue or library." }
                }
            } else {
                div {
                    class: "fz-grid",
                    for playlist in playlists {
                        div {
                            key: "{playlist.id}",
                            class: "fz-card",
                            style: "padding: var(--space-4); cursor: pointer;",
                            h4 {
                                style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                                "{playlist.title}"
                            }
                            p {
                                style: "font-size: var(--text-sm); color: var(--text-secondary); margin-top: var(--space-1);",
                                "{} tracks", playlist.track_count()
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Playlist(id: String) -> Element {
    rsx! {
        div {
            class: "fz-page",
            h2 { "Playlist" }
            p { "Playlist id: {id}" }
        }
    }
}
