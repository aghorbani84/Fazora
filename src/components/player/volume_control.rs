//! Volume control with mute toggle and slider.

use crate::playback::PlaybackService;
use crate::components::primitives::{IconButton, Slider};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct VolumeControlProps {
    service: PlaybackService,
}

#[component]
pub fn VolumeControl(props: VolumeControlProps) -> Element {
    let volume = props.service.volume.get();
    let muted = props.service.muted.get();
    let effective = if muted { 0.0 } else { volume };
    let percent = (effective * 100.0) as u32;

    let toggle_mute = move |_| {
        props.service.toggle_mute();
    };

    let on_change = move |v: f64| {
        props.service.set_volume(v as f32);
    };

    rsx! {
        div {
            class: "fz-volume-control",
            style: "display: flex; align-items: center; gap: var(--space-2);",
            IconButton {
                active: muted,
                title: if muted { "Unmute" } else { "Mute" },
                onclick: toggle_mute,
                svg {
                    class: "fz-icon",
                    width: 18,
                    height: 18,
                    use { href: if muted { "#icon-volume-mute" } else { "#icon-volume" } }
                }
            }
            Slider {
                value: effective as f64,
                min: 0.0,
                max: 1.0,
                step: 0.01,
                onchange: on_change,
                aria_label: "Volume",
            }
            span {
                style: "font-size: var(--text-xs); font-family: var(--font-mono); color: var(--text-muted); min-width: 24px; text-align: right;",
                "{percent}"
            }
        }
    }
}
