//! Library management — media directories and indexing status.

use crate::components::primitives::{Button, Input};
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn Library() -> Element {
    let state = use_context::<Signal<AppState>>();
    let app_state = state.read();
    let library_service = app_state.library.clone();

    rsx! {
        div {
            class: "fz-page",
            h1 {
                style: "font-size: var(--text-3xl); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-6);",
                "Library"
            }
            div {
                style: "font-size: var(--text-sm); color: var(--text-secondary); margin-bottom: var(--space-6);",
                "Indexed directories"
            }
            div {
                class: "fz-table",
                style: "display: flex; flex-direction: column; gap: var(--space-2);",
                for dir in library_service.media_dirs.read().iter() {
                    div {
                        style: "padding: var(--space-3) var(--space-4); border: 1px solid var(--border); border-radius: var(--radius-medium); display: flex; align-items: center; justify-content: space-between; font-family: var(--font-mono); font-size: var(--text-sm);",
                        span { "{dir.path.display()}" }
                        Button {
                            variant: crate::components::primitives::ButtonVariant::Danger,
                            onclick: move |_| {},
                            label: "Remove",
                        }
                    }
                }
            }
            div {
                style: "margin-top: var(--space-6);",
                Input {
                    placeholder: "Add media directory…",
                    aria_label: "Media directory",
                    oninput: Some(move |_| {}),
                }
                Button {
                    variant: crate::components::primitives::ButtonVariant::Primary,
                    label: "Add",
                    onclick: move |_| {},
                }
            }
            div {
                style: "margin-top: var(--space-8); padding: var(--space-4); border: 1px solid var(--border); border-radius: var(--radius-medium); background: var(--surface-secondary);",
                h4 {
                    style: "font-size: var(--text-sm); font-weight: 700; color: var(--text-primary); text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: var(--space-2);",
                    "Indexing status"
                }
                p {
                    style: "font-size: var(--text-sm); color: var(--text-secondary);",
                    {format!("{} tracks indexed", library_service.stats.read().track_count)}
                }
            }
        }
    }
}
