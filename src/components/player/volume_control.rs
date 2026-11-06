//! Volume control with mute toggle and slider.

use crate::components::branding::AppIcon;
use crate::components::primitives::{IconButton, Slider};
use crate::playback::PlaybackService;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct VolumeControlProps {
    service: PlaybackService,
}

#[component]
pub fn VolumeControl(props: VolumeControlProps) -> Element {
    let service = props.service.clone();
    let volume = *service.volume.read();
    let muted = *service.muted.read();
    let effective = if muted { 0.0 } else { volume };
    let percent = (effective * 100.0) as u32;

    let mute_service = service.clone();
    let toggle_mute = move |_| mute_service.toggle_mute();

    let volume_service = service.clone();
    let on_change = move |value: f64| volume_service.set_volume(value as f32);

    rsx! {
        div {
            class: "fz-volume-control",
            style: "display: flex; align-items: center; gap: var(--space-2);",
            IconButton {
                active: muted,
                title: if muted { "Unmute" } else { "Mute" },
                onclick: toggle_mute,
                AppIcon { name: if muted { "volume-mute" } else { "volume" }, size: 18 }
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
