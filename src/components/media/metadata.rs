//! Read-only metadata chip group for track/video details.

use dioxus::prelude::*;

#[derive(Props)]
pub struct MetadataProps {
    items: Vec<(String, String)>,
}

#[component]
pub fn Metadata(props: MetadataProps) -> Element {
    rsx! {
        div {
            class: "fz-metadata",
            style: "display: flex; flex-wrap: wrap; gap: var(--space-3); font-size: var(--text-sm); color: var(--text-secondary); font-family: var(--font-mono);",
            for (label, value) in props.items {
                div {
                    style: "display: flex; align-items: baseline; gap: var(--space-1);",
                    span { style: "color: var(--text-muted);", "{label}" }
                    span { "{value}" }
                }
            }
        }
    }
}
