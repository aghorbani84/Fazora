//! Android-style bottom navigation with primary destinations.

use crate::routing::Route;
use dioxus::prelude::*;

#[allow(dead_code, reason = "destination enum is retained for the planned navigation analytics metadata")]
#[derive(Clone, Copy, PartialEq, Eq)]
enum BottomDestination {
    Home,
    Music,
    Videos,
    Library,
    Settings,
}

#[derive(Props, Clone, PartialEq)]
pub struct BottomNavigationProps {
    selected: Signal<Route>,
    on_navigate: EventHandler<Route>,
}

#[component]
pub fn BottomNavigation(props: BottomNavigationProps) -> Element {
    let selected = props.selected.cloned();
    let on_navigate = props.on_navigate;
    let destinations = [
        (BottomDestination::Home, Route::Home {}, "icon-home", "Home"),
        (
            BottomDestination::Music,
            Route::Music {},
            "icon-music",
            "Music",
        ),
        (
            BottomDestination::Videos,
            Route::Videos {},
            "icon-video",
            "Videos",
        ),
        (
            BottomDestination::Library,
            Route::Library {},
            "icon-folder",
            "Library",
        ),
        (
            BottomDestination::Settings,
            Route::Settings {},
            "icon-settings",
            "Settings",
        ),
    ];

    rsx! {
        nav {
            class: "fz-bottom-nav",
            role: "navigation",
            aria_label: "Bottom navigation",
            for (destination, route, icon, label) in destinations {
                BottomNavItem {
                    key: "{label}",
                    destination,
                    route: route.clone(),
                    icon,
                    label,
                    selected: selected == route.clone(),
                    on_navigate,
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct BottomNavItemProps {
    destination: BottomDestination,
    route: Route,
    icon: &'static str,
    label: &'static str,
    selected: bool,
    on_navigate: EventHandler<Route>,
}

#[component]
fn BottomNavItem(props: BottomNavItemProps) -> Element {
    let navigator = use_navigator();
    let route = props.route.clone();
    let on_navigate = props.on_navigate;
    let is_selected = props.selected;
    let _destination = props.destination;

    rsx! {
        button {
            class: if is_selected { "fz-bottom-nav-item active" } else { "fz-bottom-nav-item" },
            role: "button",
            "aria-selected": if is_selected { "true" } else { "false" },
            tabindex: 0,
            onclick: move |_| {
                navigator.push(route.clone());
                on_navigate.call(route.clone());
            },
            span {
                class: "fz-icon",
                width: 20,
                height: 20,
                use { href: format!("#{}", props.icon) }
            }
            span { "{props.label}" }
        }
    }
}
