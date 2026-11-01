//! Toggle switch primitive.

use dioxus::prelude::*;

#[derive(Props)]
pub struct ToggleProps {
    checked: bool,
    onchange: EventHandler<bool>,
    #[props(default)]
    label: Option<String>,
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn Toggle(props: ToggleProps) -> Element {
    rsx! {
        label {
            class: if let Some(c) = &props.class { c } else { "fz-toggle-group".to_string() },
            span {
                class: "fz-toggle",
                role: "switch",
                "aria-checked": props.checked,
                tabindex: 0,
                onclick: move |_| props.onchange.call(!props.checked),
                onkeydown: move |evt: KeyboardEvent| {
                    if matches!(evt.key(), Key::Enter | Key::Space) {
                        props.onchange.call(!props.checked);
                    }
                },
                span { class: "fz-toggle-thumb" }
            }
            if let Some(label) = &props.label {
                span { class: "fz-toggle-label", "{label}" }
            }
        }
    }
}
