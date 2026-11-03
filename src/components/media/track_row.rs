//! Track list row with multi-selection, playback actions and metadata.

use crate::components::primitives::IconButton;
use crate::models::Track;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TrackRowProps {
    track: Track,
    index: usize,
    selected: bool,
    playing: bool,
    on_select: EventHandler<(usize, bool)>,
    on_toggle_select: EventHandler<usize>,
    on_play: EventHandler<Track>,
    on_add_queue: EventHandler<Track>,
    on_favorite: EventHandler<Track>,
}

#[component]
pub fn TrackRow(props: TrackRowProps) -> Element {
    let track = props.track.clone();
    let index = props.index;
    let selected = props.selected;
    let playing = props.playing;
    let duration = track.formatted_duration().unwrap_or_else(|| "--:--".to_string());
    let on_play_click = props.on_play;
    let on_play_key = props.on_play;
    let on_select_click = props.on_select;
    let on_select_key = props.on_select;
    let on_toggle_click = props.on_toggle_select;
    let on_toggle_key = props.on_toggle_select;
    let on_add_queue = props.on_add_queue;
    let _on_favorite = props.on_favorite;

    let click_track = track.clone();
    let keyboard_track = track.clone();
    let queue_track = track.clone();

    rsx! {
        tr {
            class: if playing {
                "fz-track-row playing"
            } else if selected {
                "fz-track-row selected"
            } else {
                "fz-track-row"
            },
            role: "row",
            "aria-selected": selected,
            tabindex: 0,
            onclick: move |_| {
                on_select_click.call((index, true));
                on_play_click.call(click_track.clone());
            },
            onkeydown: move |evt: KeyboardEvent| {
                if matches!(evt.key(), Key::Enter) {
                    on_select_key.call((index, true));
                    on_play_key.call(keyboard_track.clone());
                }
            },
            td {
                class: "fz-row-select",
                width: 36,
                span {
                    class: "fz-checkbox",
                    role: "checkbox",
                    "aria-checked": selected,
                    tabindex: 0,
                    onclick: move |evt| {
                        evt.stop_propagation();
                        on_toggle_click.call(index);
                    },
                    onkeydown: move |evt: KeyboardEvent| {
                        if matches!(evt.key(), Key::Enter) {
                            evt.prevent_default();
                            evt.stop_propagation();
                            on_toggle_key.call(index);
                        }
                    },
                }
            }
            td {
                class: "fz-row-number",
                width: 32,
                style: "color: var(--text-muted);",
                "{index + 1}"
            }
            td { class: "fz-row-title", "{track.title}" }
            td { class: "fz-row-artist", style: "color: var(--text-secondary);", "{track.artist}" }
            td { class: "fz-row-album", style: "color: var(--text-secondary);", {track.album.as_deref().unwrap_or("")} }
            td {
                class: "fz-row-duration",
                style: "font-family: var(--font-mono); color: var(--text-muted); text-align: right; width: 60px;",
                "{duration}"
            }
            td {
                class: "fz-row-actions",
                style: "width: 52px; text-align: right; opacity: 0; transition: opacity var(--transition-fast);",
                IconButton {
                    active: false,
                    title: "Add to queue",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        on_add_queue.call(queue_track.clone());
                    },
                    svg { class: "fz-icon", width: 16, height: 16, use { href: "#icon-queue" } }
                }
            }
        }
    }
}
