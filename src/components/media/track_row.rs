//! Track list row with multi-selection, playback actions and metadata.

use crate::models::Track;
use crate::state::AppState;
use dioxus::prelude::*;

#[derive(Props)]
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
    let state = use_context::<Signal<AppState>>();
    let _ = state;

    let duration = props.track.formatted_duration().unwrap_or_else(|| "--:--".to_string());

    rsx! {
        tr {
            class: if props.playing {
                "fz-track-row playing"
            } else if props.selected {
                "fz-track-row selected"
            } else {
                "fz-track-row"
            },
            role: "row",
            "aria-selected": props.selected,
            tabindex: 0,
            onclick: move |_| {
                props.on_play.call(props.track.clone());
            },
            onkeydown: move |evt: KeyboardEvent| {
                if matches!(evt.key(), Key::Enter) {
                    props.on_play.call(props.track.clone());
                }
                if evt.key() == Key::Space {
                    evt.prevent_default();
                    props.on_play.call(props.track.clone());
                }
            },
            td {
                class: "fz-row-select",
                width: 36,
                span {
                    class: "fz-checkbox",
                    role: "checkbox",
                    "aria-checked": props.selected,
                    tabindex: 0,
                    onclick: move |evt| {
                        evt.stop_propagation();
                        props.on_toggle_select.call(props.index);
                    },
                    onkeydown: move |evt: KeyboardEvent| {
                        if matches!(evt.key(), Key::Enter | Key::Space) {
                            evt.prevent_default();
                            props.on_toggle_select.call(props.index);
                        }
                    },
                }
            }
            td {
                class: "fz-row-number",
                width: 32,
                style: "color: var(--text-muted);",
                "{props.index + 1}"
            }
            td {
                class: "fz-row-title",
                "{props.track.title}"
            }
            td {
                class: "fz-row-artist",
                style: "color: var(--text-secondary);",
                "{props.track.artist}"
            }
            td {
                class: "fz-row-album",
                style: "color: var(--text-secondary);",
                "{props.track.album.as_deref().unwrap_or("")}"
            }
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
                    onclick: move |_| props.on_add_queue.call(props.track.clone()),
                    svg { class: "fz-icon", width: 16, height: 16, use { href: "#icon-queue" } }
                }
            }
        }
    }
}
