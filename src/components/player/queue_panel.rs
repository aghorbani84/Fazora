//! Queue panel — current track, upcoming tracks, reorder, remove, save as
//! playlist. Queue changes update the real playback service.

use crate::components::branding::AppIcon;
use crate::components::primitives::IconButton;
use crate::queue::{QueueEntry, QueueService};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct QueuePanelProps {
    queue: QueueService,
    open: Signal<bool>,
    on_close: EventHandler<()>,
}

#[component]
pub fn QueuePanel(props: QueuePanelProps) -> Element {
    let queue = props.queue.clone();
    let entries = queue.entries.read().clone();
    let current_index = *queue.index.read();
    let is_open = props.open.cloned();
    let queue_for_save = queue.clone();
    let queue_for_clear = queue.clone();

    rsx! {
        div {
            class: "fz-secondary-panel",
            style: if is_open { "display: flex;" } else { "display: none;" },
            div {
                class: "fz-panel-header",
                style: "padding: var(--space-4); border-bottom: 1px solid var(--border); display: flex; align-items: center; justify-content: space-between;",
                h3 {
                    style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary);",
                    "Queue"
                }
                IconButton {
                    active: false,
                    title: "Close",
                    onclick: move |_| props.on_close.call(()),
                    AppIcon { name: "chevron-left", size: 18 }
                }
            }
            div {
                class: "fz-queue-list",
                style: "flex: 1; overflow-y: auto; padding: var(--space-4); display: flex; flex-direction: column; gap: var(--space-3);",
                if entries.is_empty() {
                    div {
                        class: "fz-empty",
                        style: "height: auto;",
                        p { "Queue is empty" }
                    }
                } else {
                    for (i, entry) in entries.iter().enumerate() {
                        QueueRow {
                            key: "{entry.id}",
                            entry: entry.clone(),
                            current: current_index == Some(i),
                            on_remove: {
                                let queue = queue.clone();
                                move |_| queue.remove(i)
                            },
                            on_play: {
                                let queue = queue.clone();
                                move |_| queue.set_index(i)
                            },
                        }
                    }
                    div {
                        style: "margin-top: var(--space-4); display: flex; gap: var(--space-2);",
                        crate::components::primitives::Button {
                            variant: crate::components::primitives::ButtonVariant::Surface,
                            onclick: move |_| {
                                let playlist = queue_for_save.save_as_playlist("Saved queue".to_string());
                                let _ = playlist;
                            },
                            label: "Save as playlist",
                        }
                        crate::components::primitives::Button {
                            variant: crate::components::primitives::ButtonVariant::Danger,
                            onclick: {
                                move |_| queue_for_clear.clear()
                            },
                            label: "Clear",
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct QueueRowProps {
    entry: QueueEntry,
    current: bool,
    on_remove: EventHandler<()>,
    on_play: EventHandler<()>,
}

#[component]
fn QueueRow(props: QueueRowProps) -> Element {
    let track = props.entry.track.clone();
    let on_play = props.on_play;
    let on_remove = props.on_remove;
    let duration = track
        .formatted_duration()
        .unwrap_or_else(|| "--:--".to_string());
    let (bg, border) = if props.current {
        ("rgba(216, 181, 106, 0.10)", "rgba(216, 181, 106, 0.35)")
    } else {
        ("transparent", "var(--border)")
    };

    rsx! {
        div {
            style: format!(
                "display: flex; align-items: center; gap: var(--space-3); padding: var(--space-2) var(--space-3); background: {}; border-radius: var(--radius-medium); border: 1px solid {}; cursor: pointer;",
                bg, border
            ),
            onclick: move |_| on_play.call(()),
            div {
                style: "font-family: var(--font-mono); font-size: var(--text-xs); color: var(--text-muted); min-width: 16px; text-align: center;",
                "{props.entry.id.chars().take(2).collect::<String>()}"
            }
            div {
                style: "flex: 1; min-width: 0;",
                div {
                    style: "font-size: var(--text-sm); font-weight: 600; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{track.title}"
                }
                div {
                    style: "font-size: var(--text-xs); color: var(--text-secondary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{track.artist}"
                }
            }
            div {
                style: "font-size: var(--text-xs); font-family: var(--font-mono); color: var(--text-muted);",
                "{duration}"
            }
            IconButton {
                active: false,
                title: "Remove from queue",
                onclick: move |_| on_remove.call(()),
                AppIcon { name: "chevron-down", size: 16 }
            }
        }
    }
}
