//! Premium, data-driven Home dashboard for desktop and Android.
use crate::components::branding::AppIcon;
use crate::library::ScanStatus;
use crate::models::{Playlist, Track};
use crate::playback::PlaybackState;
use crate::routing::Route;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let state = use_context::<Signal<AppState>>();
    let app_state = state.read();
    let library_service = app_state.library.clone();
    let queue_service = app_state.queue.clone();
    let playback_service = app_state.playback.clone();
    let library = library_service.library.read().clone();
    let stats = library_service.stats.read().clone();
    let scan_status = library_service.scan_status.read().clone();

    let mut continue_tracks: Vec<Track> = library
        .tracks
        .iter()
        .filter(|track| track.last_played.is_some())
        .cloned()
        .collect();
    continue_tracks.sort_by(|a, b| b.last_played.cmp(&a.last_played));

    let mut recently_added = library.tracks.clone();
    recently_added.sort_by(|a, b| b.added_at.cmp(&a.added_at));

    let favorites: Vec<Playlist> = library
        .playlists
        .iter()
        .filter(|playlist| playlist.favorite)
        .cloned()
        .collect();

    let is_scanning = matches!(&scan_status, ScanStatus::Scanning | ScanStatus::Indexing);
    let has_media = !library.tracks.is_empty() || !library.videos.is_empty();
    let mut show_permission_details = use_signal(|| false);
    let navigator = use_navigator();
    let recent_navigator = navigator.clone();
    let quick_recent_navigator = navigator.clone();
    let favorites_navigator = navigator.clone();
    let quick_favorites_navigator = navigator;

    let play_tracks = library.tracks.clone();
    let play_queue = queue_service.clone();
    let play_service = playback_service.clone();
    let play_all = move |_| {
        if let Some(first) = play_tracks.first().cloned() {
            play_queue.clear();
            play_queue.add_many(play_tracks.clone());
            play_queue.set_index(0);
            play_service.set_state(PlaybackState::Playing { track: first });
        }
    };

    let shuffle_tracks = library.tracks.clone();
    let shuffle_queue = queue_service.clone();
    let shuffle_playback = playback_service.clone();
    let shuffle_all = move |_| {
        let mut tracks = shuffle_tracks.clone();
        let mut seed = chrono::Utc::now().timestamp_millis() as u64;
        for index in (1..tracks.len()).rev() {
            seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            let swap_index = (seed as usize) % (index + 1);
            tracks.swap(index, swap_index);
        }
        if let Some(first) = tracks.first().cloned() {
            shuffle_queue.clear();
            shuffle_queue.add_many(tracks);
            shuffle_queue.set_index(0);
            shuffle_queue.set_mode(crate::queue::QueueMode::Shuffle);
            shuffle_playback.set_state(PlaybackState::Playing { track: first });
        }
        if !*shuffle_playback.shuffle.read() {
            shuffle_playback.toggle_shuffle();
        }
    };

    rsx! {
        div { class: "fz-page fz-home",
            section { class: "fz-home-hero",
                div { class: "fz-home-heading",
                    span { class: "fz-eyebrow", "YOUR PERSONAL MEDIA SPACE" }
                    h1 { "Home" }
                    p { class: "fz-home-tagline", "Your media, engineered." }
                }
            }

            section { class: "fz-stats-grid", aria_label: "Library statistics",
                StatCard { label: "Tracks", value: stats.track_count.to_string(), icon: "track", route: Route::Tracks {} }
                StatCard { label: "Albums", value: stats.album_count.to_string(), icon: "album", route: Route::Albums {} }
                StatCard { label: "Artists", value: stats.artist_count.to_string(), icon: "artist", route: Route::Artists {} }
                StatCard { label: "Videos", value: stats.video_count.to_string(), icon: "video", route: Route::Videos {} }
            }

            if is_scanning {
                section { class: "fz-scan-status",
                    div { class: "fz-scan-status-copy",
                        strong { "Building your library" }
                        span { "Reading media metadata…" }
                    }
                    div { class: "fz-progress-track", role: "progressbar", aria_label: "Media scan in progress", aria_valuetext: "Scanning" ,
                        div { class: "fz-progress-indeterminate" }
                    }
                }
            }
            if let ScanStatus::Finished = &scan_status {
                section { class: "fz-scan-result",
                    strong { "Library scan complete" }
                    span { "{stats.track_count} tracks · {stats.album_count} albums · {stats.artist_count} artists · {stats.video_count} videos" }
                }
            }
            if let ScanStatus::Error { message } = &scan_status {
                section { class: "fz-scan-error", role: "alert",
                    strong { "We couldn't scan your media" }
                    span { "{message}" }
                }
            }

            if !has_media {
                section { class: "fz-first-run-card",
                    div { class: "fz-first-run-icon", AppIcon { name: "library", size: 28 } }
                    span { class: "fz-eyebrow", "LET'S GET STARTED" }
                    h2 { "Your library starts here" }
                    p { "FAZORA needs permission to read audio and video metadata so it can organize your music and videos. Your media stays on your device." }
                    crate::components::primitives::Button {
                        variant: crate::components::primitives::ButtonVariant::Primary,
                        onclick: move |_| show_permission_details.set(!show_permission_details()),
                        label: "Set up media access",
                    }
                    if show_permission_details() {
                        div { class: "fz-permission-note", role: "status",
                            strong { "Android access setup" }
                            span { "The Android runtime permission request and MediaStore scanner are the next implementation phase. No permission has been requested and no scan has been started by this UI yet." }
                        }
                    }
                }
            }

            if !continue_tracks.is_empty() {
                section { class: "fz-section",
                    div { class: "fz-section-header",
                        h2 { class: "fz-section-title", "Continue listening" }
                        span { class: "fz-section-meta", "RECENTLY PLAYED" }
                    }
                    div { class: "fz-track-list",
                        for track in continue_tracks.iter().take(4) {
                            TrackRow { key: "{track.id}", track: track.clone() }
                        }
                    }
                }
            }

            if !recently_added.is_empty() {
                section { class: "fz-section",
                    div { class: "fz-section-header",
                        h2 { class: "fz-section-title", "Recently added" }
                        button {
                            class: "fz-text-action",
                            onclick: move |_| { recent_navigator.push(Route::Library {}); },
                            "View library"
                        }
                    }
                    div { class: "fz-track-list",
                        for track in recently_added.iter().take(4) {
                            TrackRow { key: "added-{track.id}", track: track.clone() }
                        }
                    }
                }
            }

            if !favorites.is_empty() {
                section { class: "fz-section",
                    div { class: "fz-section-header",
                        h2 { class: "fz-section-title", "Your favorites" }
                        button {
                            class: "fz-text-action",
                            onclick: move |_| { favorites_navigator.push(Route::Favorites {}); },
                            "See all"
                        }
                    }
                    div { class: "fz-favorites-grid",
                        for playlist in favorites.iter().take(4) {
                            div { class: "fz-card fz-favorite-card", key: "{playlist.id}",
                                div { class: "fz-favorite-art", AppIcon { name: "playlist", size: 22 } }
                                strong { "{playlist.title}" }
                                span { "{playlist.track_count()} tracks" }
                            }
                        }
                    }
                }
            }

            section { class: "fz-section fz-quick-actions",
                div { class: "fz-section-header",
                    h2 { class: "fz-section-title", "Quick actions" }
                }
                div { class: "fz-quick-actions-grid",
                    QuickAction { label: "Play all", icon: "play", disabled: library.tracks.is_empty(), on_click: play_all }
                    QuickAction { label: "Shuffle", icon: "shuffle", disabled: library.tracks.is_empty(), on_click: shuffle_all }
                    QuickAction {
                        label: "Recently added",
                        icon: "clock",
                        disabled: false,
                        on_click: move |_| { quick_recent_navigator.push(Route::Library {}); },
                    }
                    QuickAction {
                        label: "Favorites",
                        icon: "favorite",
                        disabled: false,
                        on_click: move |_| { quick_favorites_navigator.push(Route::Favorites {}); },
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(label: &'static str, value: String, icon: &'static str, route: Route) -> Element {
    let navigator = use_navigator();
    rsx! {
        button {
            class: "fz-card fz-stat-card",
            aria_label: "{label}: {value}. Open {label} library.",
            onclick: move |_| { navigator.push(route.clone()); },
            span { class: "fz-stat-icon", AppIcon { name: icon, size: 22 } }
            span { class: "fz-stat-copy",
                strong { "{value}" }
                span { "{label}" }
            }
            AppIcon { name: "next", size: 16 }
        }
    }
}

#[component]
fn TrackRow(track: Track) -> Element {
    rsx! {
        div { class: "fz-home-track",
            div { class: "fz-home-track-art",
                AppIcon { name: "music", size: 19 }
            }
            div { class: "fz-home-track-copy",
                strong { "{track.title}" }
                span { "{track.artist}" }
            }
            if let Some(duration) = track.formatted_duration() {
                span { class: "fz-track-duration", "{duration}" }
            }
        }
    }
}

#[component]
fn QuickAction(
    label: &'static str,
    icon: &'static str,
    disabled: bool,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            class: "fz-quick-action",
            disabled,
            onclick: move |event| on_click.call(event),
            AppIcon { name: icon, size: 19 }
            span { "{label}" }
        }
    }
}
