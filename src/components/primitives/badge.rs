//! Badge primitive.

use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BadgeVariant {
    Default,
    Accent,
}

#[derive(Props)]
pub struct BadgeProps {
    #[props(default = BadgeVariant::Default)]
    variant: BadgeVariant,
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    let class = match props.variant {
        BadgeVariant::Default => "fz-badge",
        BadgeVariant::Accent => "fz-badge fz-badge-accent",
    };
    let full_class = match &props.class {
        Some(c) => format!("{} {}", class, c),
        None => class.to_string(),
    };

    rsx! {
        span { class: "{full_class}", {props.children} }
    }
}
