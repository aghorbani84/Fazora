//! FAZORA geometric F monogram + wordmark.
//!
//! The standalone monogram is used for application icons and compact
//! navigation. The wordmark is used in onboarding, headers and branding
//! locations. Never invent a different logo in individual screens.

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct FazoraLogoProps {
    #[props(default = 28)]
    size: u32,
    #[props(default = false)]
    compact: bool,
}

#[component]
pub fn FazoraLogo(props: FazoraLogoProps) -> Element {
    let scale = if props.compact { 1.0 } else { 1.25 };
    let stroke = (props.size as f32 * 0.18) as u32;
    let view = props.size;

    rsx! {
        svg {
            width: "{view}",
            height: "{view}",
            view_box: "0 0 64 64",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M20 4v14h8v8H20v14H8V4h12z",
                stroke: "var(--fazora-gold)",
                "stroke-width": "{stroke}",
                "stroke-linecap": "square",
                "stroke-linejoin": "miter",
                fill: "none",
            }
            path {
                d: "M20 32h8v8h-8z",
                fill: "var(--fazora-gold)",
            }
        }
        if !props.compact {
            span {
                class: "fz-wordmark",
                "aria-hidden": "true",
                "FAZORA"
            }
        }
    }
}

/// Brand wordmark without the monogram, for headers and onboarding.
#[component]
pub fn FazoraWordmark(props: FazoraLogoProps) -> Element {
    rsx! {
        div {
            class: "fz-wordmark-container",
            FazoraLogo { size: props.size, compact: true }
            span { class: "fz-wordmark", "FAZORA" }
        }
    }
}
