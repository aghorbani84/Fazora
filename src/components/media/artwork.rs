//! Artwork display with graceful fallback to a generated monogram when no
//! artwork exists. Never invent posters or metadata.

use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ArtworkProps {
    src: Option<String>,
    #[props(default = 120)]
    width: u32,
    #[props(default = 120)]
    height: u32,
    #[props(default = true)]
    radius: bool,
    #[props(default = "Artwork".to_string())]
    alt: String,
    #[props(default)]
    class: Option<String>,
}

#[component]
pub fn Artwork(props: ArtworkProps) -> Element {
    let class = match &props.class {
        Some(c) => format!("fz-artwork {}", c),
        None => "fz-artwork".to_string(),
    };

    rsx! {
        div {
            class: "{class}",
            style: format!(
                "width: {}px; height: {}px; border-radius: {}px; background: var(--surface-tertiary); border: 1px solid var(--border); overflow: hidden; position: relative;",
                props.width, props.height,
                if props.radius { "12" } else { "0" }
            ),
            if let Some(src) = &props.src {
                img {
                    src: "{src}",
                    alt: "{props.alt}",
                    width: "{props.width}",
                    height: "{props.height}",
                    style: "width: 100%; height: 100%; object-fit: cover; display: block;",
                }
            } else {
                div {
                    style: "width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; color: var(--text-muted); font-family: var(--font-mono); font-size: var(--text-xs); letter-spacing: 0.05em;",
                    "FZ"
                }
            }
        }
    }
}
