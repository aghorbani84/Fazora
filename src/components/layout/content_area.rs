//! Main content area wrapper — exposes scrollable region for pages.

use dioxus::prelude::*;

#[derive(Props)]
pub struct ContentAreaProps {
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn ContentArea(props: ContentAreaProps) -> Element {
    rsx! {
        div {
            class: if let Some(c) = &props.class {
                format!("fz-content {}", c)
            } else {
                "fz-content".to_string()
            },
            {props.children}
        }
    }
}
