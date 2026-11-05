//! Data-driven Home dashboard. Sections are rendered only when local data supports them.

use crate::components::media::Artwork;
use crate::library::ScanStatus;
use crate::models::{Library, Playlist, Track};
use crate::routing::Route;
use crate::state::AppState;
use dioxus::prelude::*;
use std::collections::HashMap;

#[component]
pub fn Home() -> Element {
    let state = use_context::<Signal<AppState>>();
    let app_state = state.read();
    let library_service = app_state.library.clone();
    let library: Library = library_service.library.read().clone();
    let stats = library_service.stats.read().clone();
    let scan_status = library_service.scan_status.read().clone();
    let scan_is_active = matches!(scan_status, ScanStatus::Scanning | ScanStatus::Indexing);
    let scan_error = match &scan_status {
        ScanStatus::Error { message } => Some(message.clone()),
        _ => None,
    };
    let playback = app_state.playback.clone();
    let current_track = playback.current_track();
    let position = *playback.position_sec.read();
    let duration = *playback.duration_sec.read();
    let progress_width = if duration > 0.0 { (position / duration * 100.0).clamp(0.0, 100.0) } else { 0.0 };
    let navigator = use_navigator();

    let mut recently_played: Vec<Track> = library
        .tracks
        .iter()
        .filter(|track| track.last_played.is_some())
        .cloned()
        .collect();
    recently_played.sort_by_key(|track| std::cmp::Reverse(track.last_played.unwrap_or_default()));
    recently_played.truncate(12);

    let mut recently_added = library.tracks.clone();
    recently_added.sort_by_key(|track| std::cmp::Reverse(track.added_at));
    recently_added.truncate(12);

    let mut most_played: Vec<Track> = library
        .tracks
        .iter()
        .filter(|track| track.play_count > 0)
        .cloned()
        .collect();
    most_played.sort_by_key(|track| std::cmp::Reverse(track.play_count));
    most_played.truncate(12);

    let favorite_playlists: Vec<Playlist> = library
        .playlists
        .iter()
        .filter(|playlist| playlist.favorite)
        .cloned()
        .collect();

    let mut artist_counts: HashMap<String, usize> = HashMap::new();
    let mut decade_counts: HashMap<i32, usize> = HashMap::new();
    for track in &library.tracks {
        if !track.artist.trim().is_empty() {
            *artist_counts.entry(track.artist.clone()).or_insert(0) += 1;
        }
        if let Some(year) = track.year {
            *decade_counts.entry(year.div_euclid(10) * 10).or_insert(0) += 1;
        }
    }
    let mut artist_mixes: Vec<(String, usize)> = artist_counts.into_iter().collect();
    artist_mixes.sort_by_key(|item| std::cmp::Reverse(item.1));
    artist_mixes.truncate(4);
    let mut decade_mixes: Vec<(i32, usize)> = decade_counts.into_iter().collect();
    decade_mixes.sort_by_key(|item| item.0);
    let has_library = !library.tracks.is_empty() || !library.videos.is_empty();

    rsx! {
        div { class: "fz-page fz-home",
            header { class: "fz-home-hero",
                div {
                    span { class: "fz-home-eyebrow", "YOUR PERSONAL MEDIA SPACE" }
                    h1 { "Good evening." }
                    p { "A quieter way to stay close to what you love." }
                }
                div { class: "fz-home-stats",
                    span { "{stats.track_count} tracks" }
                    span { "{stats.album_count} albums" }
                    span { "{stats.artist_count} artists" }
                }
            }

            if !has_library {
                section { class: "fz-home-empty",
                    div { class: "fz-empty-mark", "FZ" }
                    h2 { "Your library starts here" }
                    p { "FAZORA has not indexed any local media yet. Add a media folder from Library to begin." }
                    button {
                        class: "fz-button fz-button-primary fz-home-cta",
                        onclick: move |_| navigator.push(Route::Library {}),
                        "Open Library"
                    }
                    if scan_is_active {
                        p { class: "fz-state-note", "Library indexing is in progress." }
                    }
                    if let Some(message) = scan_error {
                        p { class: "fz-state-error", "Last scan failed: {message}" }
                    }
                }
            } else {
                if let Some(track) = current_track {
                    section { class: "fz-resume-section",
                        div { class: "fz-section-heading",
                            div { span { class: "fz-home-eyebrow", "PICK UP WHERE YOU LEFT OFF" }
                                h2 { "Continue listening" }
                            }
                        }
                        div { class: "fz-resume-card",
                            Artwork { src: track.artwork.clone(), width: 76, height: 76, alt: track.title.clone() }
                            div { class: "fz-resume-copy",
                                strong { "{track.title}" }
                                span { "{track.artist}" }
                                div { class: "fz-champagne-trace",
                                    div { class: "fz-champagne-trace-track" }
                                    div { class: "fz-champagne-trace-progress",
                                        style: "width: {progress_width}%;"
                                    }
                                    span { class: "fz-champagne-trace-marker" }
                                }
                            }
                            span { class: "fz-resume-time",
                                if duration > 0.0 { "{(position as u64 / 60)}:{(position as u64 % 60):02}" }
                                else { "READY" }
                            }
                        }
                    }
                }

                if !recently_added.is_empty() {
                    TrackSection { title: "Recently added", eyebrow: "FRESH IN YOUR LIBRARY", tracks: recently_added }
                }
                if !most_played.is_empty() {
                    TrackSection { title: "Most played", eyebrow: "YOUR ROTATION", tracks: most_played }
                }
                if !recently_played.is_empty() {
                    TrackSection { title: "Recently played", eyebrow: "BACK IN THE MOMENT", tracks: recently_played }
                }
                if !favorite_playlists.is_empty() {
                    PlaylistSection { playlists: favorite_playlists }
                }
                if !artist_mixes.is_empty() || !decade_mixes.is_empty() {
                    section { class: "fz-home-section",
                        div { class: "fz-section-heading",
                            div { span { class: "fz-home-eyebrow", "BUILT FROM YOUR LIBRARY" }
                                h2 { "Made around your taste" }
                            }
                        }
                        div { class: "fz-mix-grid",
                            for (artist, count) in artist_mixes {
                                div { key: "artist-{artist}", class: "fz-mix-card fz-cutstone-tile",
                                    span { class: "fz-mix-kicker", "ARTIST MIX" }
                                    strong { "{artist}" }
                                    span { "{count} tracks in your library" }
                                }
                            }
                            for (decade, count) in decade_mixes {
                                div { key: "decade-{decade}", class: "fz-mix-card",
                                    span { class: "fz-mix-kicker", "DECADE MIX" }
                                    strong { "{decade}s" }
                                    span { "{count} tracks with known release years" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct TrackSectionProps {
    title: &'static str,
    eyebrow: &'static str,
    tracks: Vec<Track>,
}

#[component]
fn TrackSection(props: TrackSectionProps) -> Element {
    rsx! {
        section { class: "fz-home-section",
            div { class: "fz-section-heading",
                div { span { class: "fz-home-eyebrow", "{props.eyebrow}" }
                    h2 { "{props.title}" }
                }
                span { class: "fz-section-count", "{props.tracks.len()} ITEMS" }
            }
            div { class: "fz-track-strip",
                for track in props.tracks.iter() {
                    div { key: "{track.id}", class: "fz-track-tile",
                        div { class: "fz-track-art",
                            Artwork { src: track.artwork.clone(), width: 132, height: 132, alt: track.title.clone() }
                        }
                        strong { "{track.title}" }
                        span { "{track.artist}" }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct PlaylistSectionProps {
    playlists: Vec<Playlist>,
}

#[component]
fn PlaylistSection(props: PlaylistSectionProps) -> Element {
    rsx! {
        section { class: "fz-home-section",
            div { class: "fz-section-heading",
                div { span { class: "fz-home-eyebrow", "SAVED BY YOU" }
                    h2 { "Favorite playlists" }
                }
            }
            div { class: "fz-mix-grid",
                for playlist in props.playlists.iter() {
                    div { key: "{playlist.id}", class: "fz-mix-card",
                        span { class: "fz-mix-kicker", "PLAYLIST" }
                        strong { "{playlist.title}" }
                        span { "{playlist.track_count()} tracks" }
                    }
                }
            }
        }
    }
}
