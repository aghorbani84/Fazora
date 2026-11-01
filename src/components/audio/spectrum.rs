//! Frequency visualisation canvas. Driven by real FFT data from the audio
//! engine; draws nothing when playback is idle.

use dioxus::prelude::*;

#[derive(Props)]
pub struct SpectrumProps {
    #[props(default = false)]
    active: bool,
    #[props(default = 400)]
    width: u32,
    #[props(default = 120)]
    height: u32,
}

#[component]
pub fn Spectrum(props: SpectrumProps) -> Element {
    rsx! {
        div {
            class: "fz-spectrum",
            style: format!(
                "width: {}px; height: {}px; background: var(--surface-secondary); border: 1px solid var(--border); border-radius: var(--radius-medium); overflow: hidden;",
                props.width, props.height
            ),
            canvas {
                id: "fz-spectrum-canvas",
                width: "{props.width}",
                height: "{props.height}",
                style: "width: 100%; height: 100%; display: block;",
                aria_hidden: "true",
            }
        }
    }
}
