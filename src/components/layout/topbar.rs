//! Compact mobile-first application header.
use crate::components::branding::{AppIcon, BrandMark};
use crate::routing::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct TopbarProps {
    on_toggle_sidebar: EventHandler<MouseEvent>,
}

#[component]
pub fn Topbar(props: TopbarProps) -> Element {
    let navigator = use_navigator();
    let toggle_sidebar = props.on_toggle_sidebar;
    let search_navigator = navigator.clone();
    let settings_navigator = navigator;

    rsx! {
        header { class: "fz-topbar",
            button {
                class: "fz-header-brand",
                aria_label: "Open navigation",
                onclick: move |event| toggle_sidebar.call(event),
                span { class: "fz-header-mark", BrandMark { size: 28 } }
                span { class: "fz-header-title", "FAZORA" }
            }
            div { class: "fz-topbar-spacer" }
            button {
                class: "fz-header-icon",
                aria_label: "Search",
                title: "Search",
                onclick: move |_| search_navigator.push(Route::Search {}),
                AppIcon { name: "search", size: 21 }
            }
            button {
                class: "fz-header-icon",
                aria_label: "Settings",
                title: "Settings",
                onclick: move |_| settings_navigator.push(Route::Settings {}),
                AppIcon { name: "settings", size: 21 }
            }
        }
    }
}
