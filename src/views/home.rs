//! Home dashboard — responsive, data-driven media overview.
use crate::models::Playlist;
use crate::routing::Route;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let state = use_context::<Signal<AppState>>();
    let app_state = state.read();
    let library_service = app_state.library.clone();
    let stats = library_service.stats.read().clone();
    let library = library_service.library.read().clone();
    let scan_status = library_service.scan_status.read().clone();
    let mut recent_tracks = library.tracks.clone();
    recent_tracks.retain(|track| track.last_played.is_some());
    recent_tracks.sort_by(|a, b| b.last_played.cmp(&a.last_played));
    let mut recently_added = library.tracks.clone();
    recently_added.sort_by(|a, b| b.added_at.cmp(&a.added_at));
    let navigator = use_navigator();
    let empty_state_navigator = navigator.clone();
    let open_library = move |_| { navigator.push(Route::Library {}); };
    let open_library_empty = move |_| { empty_state_navigator.push(Route::Library {}); };

    let scan_label = match &scan_status {
        crate::library::ScanStatus::Scanning | crate::library::ScanStatus::Indexing => "Scanning…",
        _ => "Scan media",
    };

    rsx! {
        div {
            class: "fz-page fz-home",
            section {
                class: "fz-home-hero",
                div {
                    class: "fz-home-copy",
                    span { class: "fz-eyebrow", "YOUR PERSONAL MEDIA SPACE" }
                    h1 { "Your library." }
                    p { "Every track, album and moment — in one place." }
                }
                div {
                    class: "fz-home-actions",
                    crate::components::primitives::Button {
                        variant: crate::components::primitives::ButtonVariant::Primary,
                        disabled: matches!(&scan_status, crate::library::ScanStatus::Scanning | crate::library::ScanStatus::Indexing),
                        onclick: move |_| library_service.request_scan(),
                        label: scan_label,
                    }
                    crate::components::primitives::Button {
                        variant: crate::components::primitives::ButtonVariant::Surface,
                        onclick: open_library,
                        label: "Manage library",
                    }
                }
            }

            section {
                class: "fz-stats-grid",
                StatCard { label: "Tracks", value: stats.track_count.to_string(), icon: "#icon-track" }
                StatCard { label: "Albums", value: stats.album_count.to_string(), icon: "#icon-album" }
                StatCard { label: "Artists", value: stats.artist_count.to_string(), icon: "#icon-artist" }
                StatCard { label: "Videos", value: stats.video_count.to_string(), icon: "#icon-video" }
            }

            section {
                class: "fz-section fz-home-section",
                div { class: "fz-section-header",
                    h3 { class: "fz-section-title", "Continue listening" }
                    span { class: "fz-section-meta", "RECENT" }
                }
                if recent_tracks.is_empty() {
                    EmptyLibraryState {
                        title: "Your next favorite starts here",
                        description: if library.tracks.is_empty() { "Scan your device or choose a media folder to build your personal library." } else { "Tracks you play will appear here, ready to pick up where you left off." },
                        action: "Explore library",
                        on_action: open_library_empty,
                    }
                } else {
                    div { class: "fz-home-list",
                        for track in recent_tracks.iter().take(5) {
                            div { class: "fz-home-track", key: "{track.id}",
                                div { class: "fz-home-track-art",
                                    svg { class: "fz-icon", width: 20, height: 20, use { href: "#icon-music" } }
                                }
                                div { class: "fz-home-track-copy",
                                    strong { "{track.title}" }
                                    span { "{track.artist}" }
                                }
                            }
                        }
                    }
                }
            }

            section {
                class: "fz-section fz-home-section",
                div { class: "fz-section-header",
                    h3 { class: "fz-section-title", "Recently added" }
                    span { class: "fz-section-meta", "LIBRARY" }
                }
                if recently_added.is_empty() && library.videos.is_empty() {
                    div { class: "fz-inline-empty",
                        svg { class: "fz-icon", width: 18, height: 18, use { href: "#icon-folder" } }
                        span { "New media will appear here after your first scan." }
                    }
                } else {
                    div { class: "fz-home-list",
                        for track in recently_added.iter().take(4) {
                            div { class: "fz-home-track", key: "added-{track.id}",
                                div { class: "fz-home-track-art",
                                    svg { class: "fz-icon", width: 20, height: 20, use { href: "#icon-music" } }
                                }
                                div { class: "fz-home-track-copy",
                                    strong { "{track.title}" }
                                    span { "{track.artist}" }
                                }
                            }
                        }
                        if !library.videos.is_empty() {
                            div { class: "fz-inline-empty",
                                svg { class: "fz-icon", width: 18, height: 18, use { href: "#icon-video" } }
                                span { "{library.videos.len()} videos in your library." }
                            }
                        }
                    }
                }
            }

            section {
                class: "fz-section fz-home-section",
                Favorites { playlists: library.playlists.clone() }
            }
        }
    }
}

#[component]
fn StatCard(label: &'static str, value: String, icon: &'static str) -> Element {
    rsx! {
        div { class: "fz-card fz-stat-card",
            div { class: "fz-stat-icon",
                svg { class: "fz-icon", width: 22, height: 22, use { href: icon } }
            }
            div { class: "fz-stat-copy",
                strong { "{value}" }
                span { "{label}" }
            }
        }
    }
}

#[component]
fn EmptyLibraryState(
    title: &'static str,
    description: &'static str,
    action: &'static str,
    on_action: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "fz-empty fz-home-empty",
            div { class: "fz-empty-art",
                svg { class: "fz-icon", width: 28, height: 28, use { href: "#icon-music" } }
            }
            h2 { "{title}" }
            p { "{description}" }
            crate::components::primitives::Button {
                variant: crate::components::primitives::ButtonVariant::Surface,
                onclick: move |event| on_action.call(event),
                label: action,
            }
        }
    }
}

#[component]
fn Favorites(playlists: Vec<Playlist>) -> Element {
    rsx! {
        div { class: "fz-section-header",
            h3 { class: "fz-section-title", "Your playlists" }
        }
        if playlists.is_empty() {
            div { class: "fz-inline-empty",
                svg { class: "fz-icon", width: 18, height: 18, use { href: "#icon-playlist" } }
                span { "Your playlists will be ready when you create one." }
            }
        } else {
            div { class: "fz-grid",
                for playlist in playlists {
                    div { class: "fz-card fz-playlist-card", key: "{playlist.id}",
                        strong { "{playlist.title}" }
                        span { "{playlist.track_count()} tracks" }
                    }
                }
            }
        }
    }
}
