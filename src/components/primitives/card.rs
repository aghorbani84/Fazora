//! Card primitive.

use dioxus::prelude::*;

#[derive(Props)]
pub struct CardProps {
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            class: if let Some(c) = &props.class {
                format!("fz-card {}", c)
            } else {
                "fz-card".to_string()
            },
            {props.children}
        }
    }
}
