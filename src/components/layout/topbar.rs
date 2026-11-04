//! Application header: compact brand, responsive search and global actions.
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
        header { class: "fz-topbar",
            IconButton {
                title: "Toggle navigation",
                onclick: move |event| toggle_sidebar.call(event),
                svg { class: "fz-icon", width: 20, height: 20, use { href: "#icon-menu" } }
            }
            FazoraLogo { size: 24, compact: false }
            div { class: "fz-topbar-spacer",
                Input {
                    placeholder: "Search tracks, albums, artists…",
                    aria_label: "Search your media library",
                    value: query.cloned(),
                    oninput: move |event: FormEvent| query.set(event.value()),
                    onkeydown: move |event: KeyboardEvent| {
                        if matches!(event.key(), Key::Enter) && !query.cloned().trim().is_empty() {
                            search_navigator.push(Route::SearchResults { query: query.cloned() });
                        }
                    },
                }
            }
            div { class: "fz-topbar-actions",
                IconButton {
                    title: "Open playback queue",
                    class: "fz-topbar-icon",
                    onclick: move |event| open_queue.call(event),
                    svg { class: "fz-icon", width: 18, height: 18, use { href: "#icon-queue" } }
                }
                Button {
                    variant: crate::components::primitives::ButtonVariant::Ghost,
                    onclick: move |_| { settings_navigator.push(Route::Settings {}); },
                    label: "Settings",
                }
            }
        }
    }
}
