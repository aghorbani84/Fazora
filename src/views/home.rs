//! Home dashboard — responsive, data-driven premium media overview.

use crate::models::{Library, Playlist};
use crate::routing::Route;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let state = use_context::<Signal<AppState>>();
    let app_state = state.read();
    let library_service = app_state.library.clone();
    let library = library_service.library.read().clone();
    let stats = library_service.stats.read().clone();
    let navigator = use_navigator();
    let open_library = navigator.clone();

    rsx! {
        div {
            class: "fz-page fz-home",
            header {
                class: "fz-home-hero",
                div {
                    h1 { class: "fz-home-title", "Your Library." }
                    p { class: "fz-home-subtitle", "Your media, engineered." }
                }
                div {
                    class: "fz-home-actions",
                    crate::components::primitives::Button {
                        variant: crate::components::primitives::ButtonVariant::Primary,
                        onclick: move |_| open_library.push(Route::Library {}),
                        label: "Open library",
                    }
                    crate::components::primitives::Button {
                        variant: crate::components::primitives::ButtonVariant::Surface,
                        onclick: move |_| navigator.push(Route::SettingsSection { section: "library".to_string() }),
                        label: "Library settings",
                    }
                }
            }

            div {
                class: "fz-stat-grid",
                StatCard { label: "Tracks", value: stats.track_count.to_string(), icon: "#icon-track" }
                StatCard { label: "Albums", value: stats.album_count.to_string(), icon: "#icon-album" }
                StatCard { label: "Artists", value: stats.artist_count.to_string(), icon: "#icon-artist" }
                StatCard { label: "Videos", value: stats.video_count.to_string(), icon: "#icon-video" }
            }

            section {
                class: "fz-home-section",
                h2 { class: "fz-home-section-title", "Continue listening" }
                RecentlyPlayed { library: library.clone() }
            }

            section {
                class: "fz-home-section",
                h2 { class: "fz-home-section-title", "Recently added" }
                RecentlyAdded { library: library.clone() }
            }

            section {
                class: "fz-home-section",
                h2 { class: "fz-home-section-title", "Your playlists" }
                Favorites { playlists: library.playlists.clone() }
            }
        }
    }
}

#[component]
fn StatCard(label: &'static str, value: String, icon: &'static str) -> Element {
    rsx! {
        div {
            class: "fz-stat-card",
            div {
                class: "fz-stat-icon",
                svg {
                    class: "fz-icon",
                    width: 21,
                    height: 21,
                    aria_hidden: "true",
                    use { href: icon }
                }
            }
            div {
                div { class: "fz-stat-value", "{value}" }
                div { class: "fz-stat-label", "{label}" }
            }
        }
    }
}

#[component]
fn RecentlyPlayed(library: Library) -> Element {
    let mut tracks = library.tracks.clone();
    tracks.retain(|track| track.last_played.is_some());
    tracks.sort_by_key(|track| std::cmp::Reverse(track.last_played.unwrap_or_default()));
    tracks.truncate(4);

    if tracks.is_empty() {
        rsx! {
            EmptyState {
                icon: "#icon-headphones",
                title: "Your listening journey starts here",
                message: "Your recently played tracks will appear here."
            }
        }
    } else {
        rsx! {
            div {
                class: "fz-recent-list",
                for track in tracks {
                    div {
                        key: "{track.id}",
                        class: "fz-recent-item",
                        div {
                            class: "fz-recent-art",
                            svg { class: "fz-icon", width: 19, height: 19, use { href: "#icon-music" } }
                        }
                        div {
                            div { class: "fz-recent-title", "{track.title}" }
                            div { class: "fz-recent-subtitle", "{track.subtitle()}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn RecentlyAdded(library: Library) -> Element {
    let mut tracks = library.tracks.clone();
    tracks.sort_by_key(|track| std::cmp::Reverse(track.added_at));
    tracks.truncate(4);

    if tracks.is_empty() {
        rsx! {
            EmptyState {
                icon: "#icon-sparkles",
                title: "Nothing in your library yet",
                message: "Open your library to start organizing your media."
            }
        }
    } else {
        rsx! {
            div {
                class: "fz-recent-list",
                for track in tracks {
                    div {
                        key: "{track.id}",
                        class: "fz-recent-item",
                        div {
                            class: "fz-recent-art",
                            svg { class: "fz-icon", width: 19, height: 19, use { href: "#icon-music" } }
                        }
                        div {
                            div { class: "fz-recent-title", "{track.title}" }
                            div { class: "fz-recent-subtitle", "{track.subtitle()}" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EmptyState(icon: &'static str, title: &'static str, message: &'static str) -> Element {
    rsx! {
        div {
            class: "fz-home-empty",
            div {
                class: "fz-home-empty-icon",
                svg { class: "fz-icon", width: 22, height: 22, use { href: icon } }
            }
            div {
                div { class: "fz-home-empty-title", "{title}" }
                p { class: "fz-home-empty-copy", "{message}" }
            }
        }
    }
}

#[component]
fn Favorites(playlists: Vec<Playlist>) -> Element {
    if playlists.is_empty() {
        rsx! {
            EmptyState {
                icon: "#icon-playlist",
                title: "No playlists yet",
                message: "Your playlists will appear here when you create them."
            }
        }
    } else {
        rsx! {
            div {
                class: "fz-recent-list",
                for playlist in playlists {
                    div {
                        key: "{playlist.id}",
                        class: "fz-recent-item",
                        div {
                            class: "fz-recent-art",
                            svg { class: "fz-icon", width: 19, height: 19, use { href: "#icon-playlist" } }
                        }
                        div {
                            div { class: "fz-recent-title", "{playlist.title}" }
                            div { class: "fz-recent-subtitle", "{playlist.track_count()} tracks" }
                        }
                    }
                }
            }
        }
    }
}
