//! Global search entry.

use crate::components::primitives::Input;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn Search() -> Element {
    let navigator = use_navigator();
    let query = use_signal(|| String::new());

    let submit = move |_| {
        if !query.get().is_empty() {
            navigator.push(crate::routing::Route::SearchResults {
                query: query.get().clone(),
            });
        }
    };

    rsx! {
        div {
            class: "fz-page",
            h1 {
                style: "font-size: var(--text-3xl); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-6);",
                "Search"
            }
            Input {
                placeholder: "Search tracks, artists, albums, videos…",
                aria_label: "Global search",
                value: query.get(),
                oninput: Some(move |evt| query.set(evt.value())),
                onchange: Some(move |_| submit(())),
            }
            div {
                style: "margin-top: var(--space-6); display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: var(--space-5);",
                RecentSearchItem { label: "Recent searches", queries: vec![] }
            }
        }
    }
}

#[component]
fn RecentSearchItem(label: &'static str, queries: Vec<String>) -> Element {
    rsx! {
        div {
            style: "font-size: var(--text-xs); font-weight: 700; color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.08em; margin-bottom: var(--space-2);",
            "{label}"
        }
        if queries.is_empty() {
            p { style: "color: var(--text-muted); font-size: var(--text-sm);", "No recent searches." }
        } else {
            for q in queries {
                div {
                    style: "font-size: var(--text-sm); color: var(--text-secondary); padding: var(--space-2);",
                    "{q}"
                }
            }
        }
    }
}
