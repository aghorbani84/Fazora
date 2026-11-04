//! Four-destination Android bottom navigation.
use crate::components::branding::AppIcon;
use crate::routing::Route;
use dioxus::prelude::*;

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
        (Route::Home {}, "home", "Home"),
        (Route::Library {}, "library", "Library"),
        (Route::Playlists {}, "playlist", "Playlists"),
        (Route::Search {}, "search", "Search"),
    ];

    rsx! {
        nav {
            class: "fz-bottom-nav",
            role: "navigation",
            aria_label: "Primary navigation",
            for (route, icon, label) in destinations {
                button {
                    key: "{label}",
                    class: if selected == route { "fz-bottom-nav-item active" } else { "fz-bottom-nav-item" },
                    aria_label: "{label}",
                    aria_current: if selected == route { "page" } else { "false" },
                    onclick: move |_| on_navigate.call(route.clone()),
                    AppIcon { name: icon, size: 21 }
                    span { "{label}" }
                }
            }
        }
    }
}
