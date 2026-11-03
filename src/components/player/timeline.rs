//! Timeline seekbar with buffered region and hover preview placeholder.

use crate::components::primitives::Slider;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TimelineProps {
    position: f64,
    duration: f64,
    #[props(default = 0.0)]
    buffered: f64,
    on_seek: EventHandler<f64>,
}

#[component]
pub fn Timeline(props: TimelineProps) -> Element {
    let ratio = if props.duration > 0.0 {
        (props.position / props.duration).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let buffer_ratio = if props.duration > 0.0 {
        (props.buffered / props.duration).clamp(0.0, 1.0)
    } else {
        0.0
    };

    let position_label = format_time(props.position);
    let duration_label = format_time(props.duration);

    rsx! {
        div {
            class: "fz-timeline",
            style: "display: flex; flex-direction: column; gap: var(--space-1);",
            div {
                class: "fz-timeline-bar",
                position: "relative",
                height: 4,
                width: "100%",
                background: "rgba(244, 241, 233, 0.16)",
                border_radius: "999px",
                cursor: "pointer",
                onclick: move |_evt: MouseEvent| {
                    // Click-to-seek is resolved against the bar bounds by a JS
                    // binding in the desktop webview; the event itself is not
                    // inspected here.
                    let _ = _evt;
                },
                onmousedown: move |_| {},
                div {
                    class: "fz-timeline-buffer",
                    style: format!("width: {}%; background: rgba(120, 207, 193, 0.25);", buffer_ratio * 100.0),
                }
                div {
                    class: "fz-timeline-fill",
                    style: format!("width: {}%; background: var(--fazora-teal);", ratio * 100.0),
                }
                div {
                    class: "fz-timeline-thumb",
                    style: format!(
                        "left: {}%; width: 10px; height: 10px; background: var(--fazora-teal); border-radius: 50%; position: absolute; top: -3px; transform: translatex(-50%);",
                        ratio * 100.0
                    ),
                }
            }
            div {
                style: "display: flex; justify-content: space-between; font-size: var(--text-xs); font-family: var(--font-mono); color: var(--text-muted);",
                span { "{position_label}" }
                span { "-{duration_label}" }
            }
        }
    }
}

fn format_time(secs: f64) -> String {
    let total = secs.max(0.0) as u64;
    let minutes = total / 60;
    let seconds = total % 60;
    format!("{}:{:02}", minutes, seconds)
}
