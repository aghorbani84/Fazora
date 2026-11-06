//! Video card: thumbnail, title, duration, resolution, watched progress.

use crate::models::Video;
use crate::routing::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct VideoCardProps {
    video: Video,
}

#[component]
pub fn VideoCard(props: VideoCardProps) -> Element {
    let navigator = use_navigator();
    let click_target = Route::Video {
        id: props.video.id.clone(),
    };
    let keyboard_target = click_target.clone();
    let click_navigator = navigator;
    let keyboard_navigator = navigator;
    let progress = props.video.progress_ratio().clamp(0.0, 1.0);
    let duration = props
        .video
        .formatted_duration()
        .unwrap_or_else(|| "--:--".to_string());
    let open_video = move |_| {
        click_navigator.push(click_target.clone());
    };

    rsx! {
        div {
            class: "fz-card",
            role: "button",
            tabindex: 0,
            "aria-label": format!("Video {}", props.video.title),
            onclick: open_video,
            onkeydown: move |evt: KeyboardEvent| {
                if matches!(evt.key(), Key::Enter) {
                    keyboard_navigator.push(keyboard_target.clone());
                }
            },
            div {
                class: "fz-video-cover",
                style: "aspect-ratio: 16 / 9;",
                div {
                    style: "width: 100%; height: 100%; background: var(--surface-tertiary); border: 1px solid var(--border); border-radius: var(--radius-medium); overflow: hidden; position: relative;",
                    if let Some(src) = &props.video.thumbnail {
                        img {
                            src: "{src}",
                            alt: props.video.title.clone(),
                            style: "width: 100%; height: 100%; object-fit: cover; display: block;",
                        }
                    } else {
                        div {
                            style: "width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; color: var(--text-muted); font-family: var(--font-mono); font-size: var(--text-xs);",
                            "FZ"
                        }
                    }
                    if props.video.watched {
                        div {
                            style: "position: absolute; inset: 0; background: rgba(11, 13, 16, 0.55); display: flex; align-items: flex-end; justify-content: flex-end; padding: var(--space-2);",
                            span {
                                style: "background: var(--fazora-obsidian); color: var(--fazora-ivory); font-size: var(--text-xs); font-weight: 700; padding: 2px 6px; border-radius: var(--radius-small);",
                                "Watched"
                            }
                        }
                    } else if progress > 0.0 {
                        div {
                            style: "position: absolute; bottom: 0; left: 0; right: 0; height: 3px; background: rgba(120, 207, 193, 0.35);",
                            div {
                                style: format!("width: {}%; background: var(--fazora-teal);", progress * 100.0),
                            }
                        }
                    }
                    div {
                        style: "position: absolute; bottom: var(--space-2); right: var(--space-2); font-family: var(--font-mono); font-size: var(--text-xs); background: rgba(11, 13, 16, 0.6); color: var(--fazora-ivory); padding: 2px 6px; border-radius: var(--radius-small);",
                        "{duration}"
                    }
                }
            }
            div {
                style: "padding: var(--space-3);",
                h4 {
                    style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{props.video.title}"
                }
                if let Some(res) = props.video.resolution_label() {
                    p {
                        style: "font-size: var(--text-sm); color: var(--text-secondary); margin-top: var(--space-1);",
                        "{res}"
                    }
                }
            }
        }
    }
}
