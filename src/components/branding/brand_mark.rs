//! Application brand mark (monogram only) for compact navigation and icons.

use dioxus::prelude::*;

#[derive(Props)]
pub struct BrandMarkProps {
    #[props(default = 24)]
    size: u32,
    #[props(default = "var(--fazora-gold)".to_string())]
    color: String,
}

#[component]
pub fn BrandMark(props: BrandMarkProps) -> Element {
    let stroke = (props.size as f32 * 0.18) as u32;

    rsx! {
        svg {
            width: "{props.size}",
            height: "{props.size}",
            view_box: "0 0 64 64",
            fill: "none",
            xmlns: "http://www.w3.org/2000/svg",
            path {
                d: "M20 4v14h8v8H20v14H8V4h12z",
                stroke: "{props.color}",
                "stroke-width": "{stroke}",
                "stroke-linecap": "square",
                "stroke-linejoin": "miter",
                fill: "none",
            }
            path {
                d: "M20 32h8v8h-8z",
                fill: "{props.color}",
            }
        }
    }
}
