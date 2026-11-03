//! Persistent mini-player dock at the bottom of the desktop app.

use crate::components::media::Artwork;
use crate::components::primitives::IconButton;
use crate::routing::Route;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn MiniPlayer() -> Element {
    let state = use_context::<Signal<AppState>>();
    let playback = state.read().playback;
    let navigator = use_navigator();

    let track = playback.state.read().current_track().cloned();
    let is_playing = playback.state.read().is_playing();

    let open_now_playing = move |_| {
        navigator.push(Route::NowPlaying {});
    };

    let toggle_play = move |_| {
        playback.play_toggle();
    };

    rsx! {
        div {
            class: "fz-player-dock",
            role: "region",
            "aria-label": "Now playing",
            div {
                class: "fz-mini-player",
                onclick: open_now_playing,
                style: "cursor: pointer;",
                div {
                    class: "fz-mini-artwork",
                    if let Some(t) = &track {
                        Artwork {
                            src: t.artwork.clone(),
                            width: 56,
                            height: 56,
                            alt: t.title.clone(),
                        }
                    } else {
                        div {
                            style: "width: 56px; height: 56px; background: var(--surface-tertiary); border: 1px solid var(--border); border-radius: var(--radius-medium); display: flex; align-items: center; justify-content: center; color: var(--text-muted); font-family: var(--font-mono); font-size: var(--text-xs);",
                            "FZ"
                        }
                    }
                }
                div {
                    class: "fz-mini-info",
                    flex: "1",
                    min_width: 0,
                    h4 {
                        style: "font-size: var(--text-sm); font-weight: 700; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                        if let Some(t) = &track { &t.title } else { "No track" }
                    }
                    p {
                        style: "font-size: var(--text-xs); color: var(--text-secondary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                        if let Some(t) = &track { &t.artist } else { "" }
                    }
                }
                div {
                    class: "fz-mini-progress",
                    style: "flex: 1; min-width: 120px;",
                    crate::components::player::timeline::Timeline {
                        position: playback.position_sec.get(),
                        duration: playback.duration_sec.get(),
                        on_seek: move |sec| playback.seek(sec),
                    }
                }
                div {
                    class: "fz-mini-controls",
                    gap: "var(--space-2)",
                    IconButton {
                        active: false,
                        title: if is_playing { "Pause" } else { "Play" },
                        onclick: toggle_play,
                        svg {
                            class: "fz-icon",
                            width: 18,
                            height: 18,
                            use { href: if is_playing { "#icon-pause" } else { "#icon-play" } }
                        }
                    }
                    IconButton {
                        active: false,
                        title: "Next",
                        onclick: move |_| {},
                        svg { class: "fz-icon", width: 18, height: 18, use { href: "#icon-next" } }
                    }
                    IconButton {
                        active: false,
                        title: "Queue",
                        onclick: move |_| {},
                        svg { class: "fz-icon", width: 18, height: 18, use { href: "#icon-queue" } }
                    }
                }
            }
        }
    }
}
