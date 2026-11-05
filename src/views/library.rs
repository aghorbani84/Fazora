//! Library management — media selection and indexing status.

use crate::components::primitives::{Button, Input};
use crate::playback::PlaybackState;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn Library() -> Element {
    let state = use_context::<Signal<AppState>>();
    let app_state = state.read();
    let library_service = app_state.library.clone();
    let playback_service = app_state.playback.clone();
    let playback_state = playback_service.state.read().clone();
    let playback_status = format!("{playback_state:?}");

    let open_picker_service = playback_service.clone();
    let open_picker = move |_| {
        if let Err(message) = open_picker_service.open_audio_picker() {
            open_picker_service.set_state(PlaybackState::Error {
                track: open_picker_service.current_track(),
                message,
            });
        }
    };

    rsx! {
        div {
            class: "fz-page",
            h1 {
                style: "font-size: var(--text-3xl); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-6);",
                "Library"
            }
            div {
                style: "display: flex; align-items: center; justify-content: space-between; gap: var(--space-3); flex-wrap: wrap; margin-bottom: var(--space-6);",
                div {
                    style: "font-size: var(--text-sm); color: var(--text-secondary);",
                    "Choose audio files stored on this device."
                }
                Button {
                    variant: crate::components::primitives::ButtonVariant::Primary,
                    label: "Choose audio files",
                    onclick: open_picker,
                }
            }
            if let PlaybackState::Error { ref message, .. } = playback_state {
                div {
                    role: "alert",
                    style: "padding: var(--space-3); border: 1px solid var(--danger); border-radius: var(--radius-medium); color: var(--danger); margin-bottom: var(--space-4);",
                    "{message}"
                }
            }
            if let Some(track) = playback_service.current_track() {
                div {
                    style: "padding: var(--space-4); border: 1px solid var(--border); border-radius: var(--radius-medium); margin-bottom: var(--space-6);",
                    div { style: "font-weight: 700; color: var(--text-primary);", "{track.title}" }
                    div { style: "font-size: var(--text-sm); color: var(--text-secondary);", "{track.artist}" }
                    div { style: "font-size: var(--text-xs); color: var(--text-secondary); margin-top: var(--space-2);", "Playback status: {playback_status}" }
                }
            }
            div {
                style: "font-size: var(--text-sm); color: var(--text-secondary); margin-bottom: var(--space-3);",
                "Indexed directories"
            }
            div {
                class: "fz-table",
                style: "display: flex; flex-direction: column; gap: var(--space-2);",
                for dir in library_service.media_dirs.read().iter() {
                    div {
                        style: "padding: var(--space-3) var(--space-4); border: 1px solid var(--border); border-radius: var(--radius-medium); display: flex; align-items: center; justify-content: space-between; font-family: var(--font-mono); font-size: var(--text-sm);",
                        span { "{dir.path.display()}" }
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Danger,
                            onclick: move |_| {},
                            label: "Remove",
                        }
                    }
                }
            }
            div {
                style: "margin-top: var(--space-6);",
                Input {
                    placeholder: "Add media directory…",
                    aria_label: "Media directory",
                    oninput: move |_| {},
                }
                Button {
                    variant: crate::components::primitives::ButtonVariant::Primary,
                    label: "Add",
                    onclick: move |_| {},
                }
            }
            div {
                style: "margin-top: var(--space-8); padding: var(--space-4); border: 1px solid var(--border); border-radius: var(--radius-medium); background: var(--surface-secondary);",
                h4 {
                    style: "font-size: var(--text-sm); font-weight: 700; color: var(--text-primary); text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: var(--space-2);",
                    "Indexing status"
                }
                p {
                    style: "font-size: var(--text-sm); color: var(--text-secondary);",
                    "{library_service.stats.read().track_count} tracks indexed"
                }
            }
        }
    }
}
