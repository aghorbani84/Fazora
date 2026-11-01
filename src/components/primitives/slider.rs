//! Slider primitive used by timeline, volume and equalizer controls.

use dioxus::prelude::*;

#[derive(Props)]
pub struct SliderProps {
    value: f64,
    min: Option<f64>,
    max: Option<f64>,
    step: Option<f64>,
    onchange: EventHandler<f64>,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    aria_label: Option<String>,
}

#[component]
pub fn Slider(props: SliderProps) -> Element {
    let min = props.min.unwrap_or(0.0);
    let max = props.max.unwrap_or(1.0);
    let step = props.step.unwrap_or(0.01);
    let percent = if max > min {
        ((props.value - min) / (max - min)).clamp(0.0, 1.0) * 100.0
    } else {
        0.0
    };

    let oninput = move |evt: FormEvent| {
        if let Ok(v) = evt.value().parse::<f64>() {
            props.onchange.call(v);
        }
    };

    rsx! {
        div {
            class: "fz-slider-container",
            input {
                class: "fz-slider",
                r#type: "range",
                min: min.to_string(),
                max: max.to_string(),
                step: step.to_string(),
                value: props.value.to_string(),
                aria_label: props.aria_label.as_deref().unwrap_or(""),
                tabindex: 0,
                oninput,
                onchange: move |evt: FormEvent| {
                    if let Ok(v) = evt.value().parse::<f64>() {
                        props.onchange.call(v);
                    }
                },
            }
            div {
                class: "fz-slider-track-fill",
                style: "transform: scaleX({percent / 100.0})",
            }
        }
    }
}
