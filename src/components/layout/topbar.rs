//! Application header: brand, search, device, settings.

use crate::components::branding::FazoraLogo;
use crate::components::primitives::{Button, IconButton, Input};
use crate::routing::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Props)]
pub struct TopbarProps {
    on_toggle_sidebar: EventHandler<MouseEvent>,
    on_open_queue: EventHandler<MouseEvent>,
}

#[component]
pub fn Topbar(props: TopbarProps) -> Element {
    let navigator = use_navigator();

    let go_home = move |_| navigator.push(Route::Home);
    let go_search = move |_| navigator.push(Route::Search);
    let go_settings = move |_| navigator.push(Route::Settings);

    rsx! {
        header {
            class: "fz-topbar",
            FazoraLogo { size: 20, compact: false }
            div {
                class: "fz-topbar-spacer",
                flex: "1",
                Input {
                    placeholder: "Search artists, albums, tracks…",
                    aria_label: "Global search",
                    oninput: Some(move |_| go_search.call(Default::default())),
                }
            }
            div {
                class: "fz-topbar-actions",
                gap: "var(--space-3)",
                IconButton {
                    title: "Queue",
                    class: "fz-topbar-icon",
                    onclick: move |evt| props.on_open_queue.call(evt),
                    span {
                        class: "fz-icon",
                        width: 18,
                        height: 18,
                        use { href: "#icon-queue" }
                    }
                }
                Button {
                    variant: crate::components::primitives::ButtonVariant::Ghost,
                    onclick: go_settings,
                    label: "Settings",
                }
            }
        }
    }
}
