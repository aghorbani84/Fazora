//! Application header: brand, search, queue and settings.

use crate::components::branding::FazoraLogo;
use crate::components::primitives::{Button, IconButton, Input};
use crate::routing::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TopbarProps {
    on_toggle_sidebar: EventHandler<MouseEvent>,
    on_open_queue: EventHandler<MouseEvent>,
}

#[component]
pub fn Topbar(props: TopbarProps) -> Element {
    let navigator = use_navigator();
    let search_navigator = navigator.clone();
    let settings_navigator = navigator.clone();
    let mut query = use_signal(String::new);
    let toggle_sidebar = props.on_toggle_sidebar;
    let open_queue = props.on_open_queue;

    rsx! {
        header {
            class: "fz-topbar",
            IconButton {
                title: "Toggle navigation",
                onclick: move |event| toggle_sidebar.call(event),
                svg { class: "fz-icon", width: 20, height: 20, use { href: "#icon-menu" } }
            }
            FazoraLogo { size: 20, compact: false }
            div {
                class: "fz-topbar-spacer",
                flex: "1",
                Input {
                    placeholder: "Search artists, albums, tracks…",
                    aria_label: "Global search",
                    value: query.cloned(),
                    oninput: move |event| {
                        let mut signal = query;
                        signal.set(event.value());
                    },
                    onkeydown: move |event: KeyboardEvent| {
                        if matches!(event.key(), Key::Enter) && !query.cloned().trim().is_empty() {
                            search_navigator.push(Route::SearchResults { query: query.cloned() });
                        }
                    },
                }
            }
            div {
                class: "fz-topbar-actions",
                gap: "var(--space-3)",
                IconButton {
                    title: "Queue",
                    class: "fz-topbar-icon",
                    onclick: move |event| open_queue.call(event),
                    svg { class: "fz-icon", width: 18, height: 18, use { href: "#icon-queue" } }
                }
                Button {
                    variant: crate::components::primitives::ButtonVariant::Ghost,
                    onclick: move |_| {
                        settings_navigator.push(Route::Settings {});
                    },
                    label: "Settings",
                }
            }
        }
    }
}
