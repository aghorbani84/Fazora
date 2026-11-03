//! Tooltip primitive.

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TooltipProps {
    label: String,
    #[props(default)]
    class: Option<String>,
    #[props(default)]
    children: Option<Element>,
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
