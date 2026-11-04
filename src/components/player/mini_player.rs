//! Persistent compact mini-player dock.
use crate::components::branding::AppIcon;
use crate::components::media::Artwork;
use crate::playback::PlaybackState;
use crate::routing::Route;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn MiniPlayer() -> Element {
    let state = use_context::<Signal<AppState>>();
    let playback = state.read().playback.clone();
    let queue = state.read().queue.clone();
    let navigator = use_navigator();

    let track = playback.state.read().current_track().cloned();
    let title = track.as_ref().map(|item| item.title.clone()).unwrap_or_else(|| "Nothing playing".to_string());
    let artist = track.as_ref().map(|item| item.artist.clone()).unwrap_or_else(|| "Choose a track from your library".to_string());
    let is_playing = playback.state.read().is_playing();
    let progress = playback.progress_ratio() * 100.0;

    let open_now_playing = move |_| { navigator.push(Route::NowPlaying {}); };
    let play_service = playback.clone();
    let next_playback = playback.clone();
    let next_queue = queue.clone();

    rsx! {
        div { class: "fz-player-dock",
            div { class: "fz-mini-player",
                div { class: "fz-mini-artwork",
                    if let Some(current) = &track {
                        Artwork {
                            src: current.artwork.clone(),
                            width: 44,
                            height: 44,
                            alt: current.title.clone(),
                        }
                    } else {
                        div { class: "fz-mini-artwork-fallback",
                            AppIcon { name: "music", size: 20 }
                        }
                    }
                }
                button {
                    class: "fz-mini-info",
                    aria_label: "Open now playing",
                    onclick: open_now_playing,
                    strong { "{title}" }
                    span { "{artist}" }
                }
                div { class: "fz-mini-progress",
                    div { class: "fz-mini-progress-track",
                        div { class: "fz-mini-progress-fill", style: "width: {progress}%;" }
                    }
                }
                div { class: "fz-mini-controls",
                    button {
                        class: "fz-mini-control",
                        aria_label: if is_playing { "Pause" } else { "Play" },
                        disabled: track.is_none(),
                        onclick: move |_| play_service.play_toggle(),
                        AppIcon { name: if is_playing { "pause" } else { "play" }, size: 19 }
                    }
                    button {
                        class: "fz-mini-control",
                        aria_label: "Next track",
                        disabled: queue.entries.read().is_empty(),
                        onclick: move |_| {
                            if let Some(next) = next_queue.move_next() {
                                next_playback.set_state(PlaybackState::Playing { track: next });
                            }
                        },
                        AppIcon { name: "next", size: 19 }
                    }
                }
            }
        }
    }
}
