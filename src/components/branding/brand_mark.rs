//! Application brand mark and dependency-free inline SVG icon set.
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
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
            "aria-hidden": "true",
            path {
                d: "M20 4v14h8v8H20v14H8V4h12z",
                stroke: "{props.color}",
                "stroke-width": "{stroke}",
                "stroke-linecap": "square",
                "stroke-linejoin": "miter",
                fill: "none",
            }
            path { d: "M20 32h8v8h-8z", fill: "{props.color}" }
        }
    }
}

/// Small inline SVG icons. No sprite sheet, font, URL, or external asset is used.
#[derive(Props, Clone, PartialEq)]
pub struct AppIconProps {
    name: &'static str,
    #[props(default = 20)]
    size: u32,
}

#[component]
pub fn AppIcon(props: AppIconProps) -> Element {
    let name = props.name;
    rsx! {
        svg {
            width: "{props.size}",
            height: "{props.size}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "1.8",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            "aria-hidden": "true",
            {
                match name {
                    "menu" => rsx! { path { d: "M4 6h16M4 12h16M4 18h16" } },
                    "search" => rsx! {
                        circle { cx: "11", cy: "11", r: "7" }
                        path { d: "m20 20-4-4" }
                    },
                    "settings" => rsx! {
                        path { d: "M12 8.5a3.5 3.5 0 1 0 0 7 3.5 3.5 0 0 0 0-7Z" }
                        path { d: "m19.4 15 .1.1 1.4 1.1-1.4 2.4-1.7-.6a8 8 0 0 1-1.6.9l-.3 1.8h-2.8l-.3-1.8a8 8 0 0 1-1.6-.9l-1.7.6-1.4-2.4 1.4-1.1a7 7 0 0 1 0-1.9l-1.4-1.1 1.4-2.4 1.7.6a8 8 0 0 1 1.6-.9l.3-1.8h2.8l.3 1.8a8 8 0 0 1 1.6.9l1.7-.6 1.4 2.4-1.4 1.1a7 7 0 0 1-.1 1.8Z" }
                    },
                    "home" => rsx! { path { d: "m3 10 9-7 9 7v10a1 1 0 0 1-1 1h-5v-6H9v6H4a1 1 0 0 1-1-1V10Z" } },
                    "library" | "folder" => rsx! { path { d: "M3 7a2 2 0 0 1 2-2h5l2 2h7a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V7Z" } },
                    "playlist" => rsx! {
                        path { d: "M4 6h11M4 11h11M4 16h7" }
                        path { d: "M18 4v12.5a2.5 2.5 0 1 1-2-2.45V7l6-1.5v8.5a2.5 2.5 0 1 1-2-2.45V4.5L18 4Z" }
                    },
                    "play" => rsx! { path { d: "m8 5 11 7-11 7V5Z", fill: "currentColor", stroke: "none" } },
                    "pause" => rsx! {
                        path { d: "M7 5h3v14H7z", fill: "currentColor", stroke: "none" }
                        path { d: "M14 5h3v14h-3z", fill: "currentColor", stroke: "none" }
                    },
                    "next" => rsx! {
                        path { d: "M5 5v14l10-7L5 5Z" }
                        path { d: "M19 5v14" }
                    },
                    "previous" | "chevron-left" => rsx! { path { d: "m15 18-6-6 6-6" } },
                    "chevron-down" => rsx! { path { d: "m6 9 6 6 6-6" } },
                    "repeat" => rsx! {
                        path { d: "m17 2 4 4-4 4M3 11V9a3 3 0 0 1 3-3h15M7 22l-4-4 4-4M21 13v2a3 3 0 0 1-3 3H3" }
                    },
                    "queue" => rsx! {
                        rect { x: "3", y: "4", width: "18", height: "13", rx: "2" }
                        path { d: "M3 9h18M7 20h14M7 23h11" }
                    },
                    "volume" => rsx! {
                        path { d: "M4 10v4h4l5 4V6l-5 4H4Z" }
                        path { d: "M17 9a5 5 0 0 1 0 6M19.5 6.5a9 9 0 0 1 0 11" }
                    },
                    "volume-mute" => rsx! {
                        path { d: "M4 10v4h4l5 4V6l-5 4H4Z" }
                        path { d: "m17 9 5 6m0-6-5 6" }
                    },
                    "lyrics" => rsx! {
                        path { d: "M4 6h16M4 11h16M4 16h10" }
                        path { d: "m17 17 2 2 3-4" }
                    },
                    "globe" => rsx! {
                        circle { cx: "12", cy: "12", r: "9" }
                        path { d: "M3 12h18M12 3a14 14 0 0 1 0 18M12 3a14 14 0 0 0 0 18" }
                    },
                    "track" | "music" => rsx! {
                        path { d: "M9 18V5l12-2v13" }
                        circle { cx: "6", cy: "18", r: "3" }
                        circle { cx: "18", cy: "16", r: "3" }
                    },
                    "album" => rsx! {
                        rect { x: "3.5", y: "3.5", width: "17", height: "17", rx: "3" }
                        circle { cx: "12", cy: "12", r: "3.5" }
                    },
                    "artist" => rsx! {
                        circle { cx: "9", cy: "8", r: "3.5" }
                        path { d: "M2.5 20a6.5 6.5 0 0 1 13 0M16 5.5a3.5 3.5 0 0 1 0 6.8M18 14a5.5 5.5 0 0 1 3.5 5" }
                    },
                    "video" => rsx! {
                        rect { x: "3", y: "5", width: "18", height: "14", rx: "2.5" }
                        path { d: "m10 9 5 3-5 3V9Z" }
                    },
                    "shuffle" => rsx! {
                        path { d: "M16 3h5v5M4 20 21 3M21 16v5h-5M4 4l5 5M15 15l6 6" }
                    },
                    "favorite" | "star" => rsx! { path { d: "M20.8 8.7c0 5.2-8.8 10.3-8.8 10.3S3.2 13.9 3.2 8.7A4.7 4.7 0 0 1 12 6.2a4.7 4.7 0 0 1 8.8 2.5Z" } },
                    "clock" => rsx! {
                        circle { cx: "12", cy: "12", r: "9" }
                        path { d: "M12 7v5l3 2" }
                    },
                    _ => rsx! { circle { cx: "12", cy: "12", r: "8" } },
                }
            }
        }
    }
}
