//! Expanded Now Playing — full-window experience with artwork as the main
//! visual focus, waveform seekbar, queue and lyrics panels.

use crate::components::audio::lyrics_view::LyricsView;
use crate::components::media::Artwork;
use crate::components::player::playback_controls::PlaybackControls;
use crate::components::player::timeline::Timeline;
use crate::components::player::volume_control::VolumeControl;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn NowPlaying() -> Element {
    let state = use_context::<Signal<AppState>>();
    let playback = state.read().playback.clone();
    let queue = state.read().queue.clone();

    let track = playback.state.read().current_track().cloned();

    let show_queue = use_signal(|| false);
    let show_lyrics = use_signal(|| false);
    let seek_service = playback.clone();

    rsx! {
        div {
            class: "fz-now-playing",
            style: "width: 100%; height: 100%; background: var(--background); color: var(--text-primary); display: flex; flex-direction: column; overflow: hidden;",
            header {
                class: "fz-now-playing-header",
                style: "padding: var(--space-6); border-bottom: 1px solid var(--border); display: flex; align-items: center; justify-content: space-between; flex-shrink: 0;",
                h1 {
                    style: "font-size: var(--text-3xl); font-weight: 700; color: var(--text-primary);",
                    "Now playing"
                }
                div {
                    style: "display: flex; gap: var(--space-2);",
                    crate::components::primitives::IconButton {
                        active: show_queue.cloned(),
                        title: "Queue",
                        onclick: move |_| { let mut signal = show_queue; let next = !*signal.read(); signal.set(next); },
                        svg { class: "fz-icon", width: 18, height: 18, use { href: "#icon-queue" } }
                    }
                    crate::components::primitives::IconButton {
                        active: show_lyrics.cloned(),
                        title: "Lyrics",
                        onclick: move |_| {
                            let mut signal = show_lyrics;
                            let next = !*signal.read();
                            signal.set(next);
                        },
                        svg { class: "fz-icon", width: 18, height: 18, use { href: "#icon-lyrics" } }
                    }
                }
            }
            div {
                class: "fz-now-playing-body",
                style: "flex: 1; overflow-y: auto; padding: var(--space-8); display: flex; flex-direction: column; gap: var(--space-8); align-items: center;",
                div {
                    class: "fz-artwork-panel",
                    style: "width: 100%; max-width: 560px; aspect-ratio: 1 / 1; position: relative;",
                    if let Some(t) = &track {
                        Artwork {
                            src: t.artwork.clone(),
                            width: 560,
                            height: 560,
                            alt: t.title.clone(),
                        }
                    } else {
                        div {
                            style: "width: 100%; height: 100%; background: var(--surface-secondary); border: 1px solid var(--border); border-radius: var(--radius-xl); display: flex; align-items: center; justify-content: center; color: var(--text-muted); font-family: var(--font-mono); font-size: var(--text-2xl);",
                            "FZ"
                        }
                    }
                }
                div {
                    class: "fz-info-panel",
                    text_align: "center",
                    if let Some(t) = &track {
                        h2 {
                            style: "font-size: var(--text-4xl); font-weight: 700; color: var(--text-primary); letter-spacing: -0.02em;",
                            "{t.title}"
                        }
                        h3 {
                            style: "font-size: var(--text-xl); font-weight: 600; color: var(--text-secondary); margin-top: var(--space-2);",
                            "{t.artist}"
                        }
                        p {
                            style: "font-size: var(--text-sm); color: var(--text-muted); margin-top: var(--space-2); font-family: var(--font-mono);",
                            {format!("{} · {}", t.album.as_deref().unwrap_or(""), t.year.map(|y| y.to_string()).unwrap_or_else(|| "—".to_string()))}
                        }
                    } else {
                        div {
                            class: "fz-empty",
                            style: "height: auto; width: 100%; max-width: 400px;",
                            h2 { "No track loaded" }
                            p { "Select a track from the library to begin playback." }
                        }
                    }
                }
                div {
                    class: "fz-progress-panel",
                    width: "100%",
                    max_width: "700px",
                    Timeline {
                        position: playback.position_sec.cloned(),
                        duration: playback.duration_sec.cloned(),
                        on_seek: move |sec| seek_service.seek(sec),
                    }
                }
                div {
                    class: "fz-controls-panel",
                    width: "100%",
                    max_width: "700px",
                    PlaybackControls {
                        service: playback.clone(),
                        queue: queue.clone(),
                    }
                }
                div {
                    class: "fz-volume-panel",
                    width: "100%",
                    max_width: "320px",
                    VolumeControl { service: playback.clone() }
                }
            }
            footer {
                class: "fz-now-playing-footer",
                style: "padding: var(--space-4) var(--space-6); border-top: 1px solid var(--border); display: flex; align-items: center; justify-content: space-between; flex-shrink: 0;",
                div {
                    style: "display: flex; align-items: center; gap: var(--space-3);",
                    crate::components::primitives::IconButton {
                        active: false,
                        title: "Favorite",
                        onclick: move |_| {},
                        svg { class: "fz-icon", width: 20, height: 20, use { href: "#icon-star" } }
                    }
                    crate::components::primitives::IconButton {
                        active: false,
                        title: "Share",
                        onclick: move |_| {},
                        svg { class: "fz-icon", width: 20, height: 20, use { href: "#icon-globe" } }
                    }
                }
                div {
                    style: "font-size: var(--text-xs); color: var(--text-muted); font-family: var(--font-mono);",
                    "FAZORA Media"
                }
            }
            // Lyrics panel (overlay slot)
            if show_lyrics.cloned() {
                div {
                    class: "fz-lyrics-panel",
                    style: "position: fixed; inset: 0; background: var(--fazora-obsidian); z-index: 60; display: flex; flex-direction: column;",
                    LyricsView {
                        track: track.clone(),
                        position_sec: playback.position_sec.cloned(),
                        follow: true,
                        on_sync_adjust: None,
                    }
                }
            }
        }
    }
}
