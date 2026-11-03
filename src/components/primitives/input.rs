//! Text input primitive.

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct InputProps {
    #[props(default)]
    value: String,
    #[props(default)]
    placeholder: Option<String>,
    #[props(default)]
    disabled: bool,
    #[props(default)]
    oninput: EventHandler<FormEvent>,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    aria_label: Option<String>,
}

#[component]
pub fn Input(props: InputProps) -> Element {
    rsx! {
        input {
            class: if let Some(c) = &props.class {
                format!("fz-input {}", c)
            } else {
                "fz-input".to_string()
            },
            value: "{props.value}",
            placeholder: props.placeholder.as_deref().unwrap_or(""),
            disabled: props.disabled,
            aria_label: props.aria_label.as_deref().unwrap_or(""),
            tabindex: 0,
            oninput: move |evt| {
                props.oninput.call(evt);
            },
        }
    }
}
