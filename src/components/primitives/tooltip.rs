//! Tooltip primitive.

use dioxus::prelude::*;

#[derive(Props)]
pub struct TooltipProps {
    label: String,
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn Tooltip(props: TooltipProps) -> Element {
    rsx! {
        div {
            class: "fz-tooltip",
            span { class: "fz-tooltip-content", "{props.label}" }
            {props.children}
        }
    }
}
