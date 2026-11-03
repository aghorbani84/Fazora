//! Audio output device selector.

use crate::audio::{AudioDevice, AudioService};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AudioDeviceProps {
    service: AudioService,
}

#[component]
pub fn AudioDevice(props: AudioDeviceProps) -> Element {
    let state = props.service.state.read();

    rsx! {
        div {
            class: "fz-audio-device",
            style: "display: flex; flex-direction: column; gap: var(--space-3);",
            h4 {
                style: "font-size: var(--text-sm); font-weight: 700; color: var(--text-primary);",
                "Audio output"
            }
            select {
                style: "background: var(--surface-tertiary); border: 1px solid var(--border); color: var(--text-primary); border-radius: var(--radius-medium); padding: 6px 10px; font-size: var(--text-sm); width: 260px;",
                onchange: move |evt: FormEvent| {
                    props.service.set_output_device(&evt.value());
                },
                option { value: "default", selected: state.output_device.is_none(), "System default" }
                for device in &state.devices {
                    option {
                        value: "{device.id}",
                        selected: state.output_device.as_deref() == Some(&device.id),
                        "{device.name}" {
                            if device.is_default {
                                span { style: "color: var(--text-muted); font-size: var(--text-xs);", " (default)" }
                            }
                        }
                    }
                }
            }
            if state.devices.is_empty() {
                p {
                    style: "font-size: var(--text-xs); color: var(--text-muted);",
                    "No output devices available"
                }
            }
        }
    }
}
