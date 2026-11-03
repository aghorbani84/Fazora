//! Divider primitive.

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct DividerProps {
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn Divider(props: DividerProps) -> Element {
    rsx! {
        hr {
            class: if let Some(c) = &props.class {
                format!("fz-divider {}", c)
            } else {
                "fz-divider".to_string()
            },
        }
    }
}
