//! Five-destination mobile navigation for the primary FAZORA surfaces.

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
        (Route::Home {}, "icon-home", "Home"),
        (Route::Library {}, "icon-folder", "Library"),
        (Route::Search {}, "icon-search", "Search"),
        (Route::Playlists {}, "icon-playlist", "Playlists"),
        (Route::Settings {}, "icon-settings", "Settings"),
    ];

    rsx! {
        nav {
            class: "fz-bottom-nav",
            aria_label: "Primary navigation",
            for (route, icon, label) in destinations {
                button {
                    key: "{label}",
                    class: if selected == route { "fz-bottom-nav-item active" } else { "fz-bottom-nav-item" },
                    type: "button",
                    aria_current: if selected == route { "page" } else { "false" },
                    onclick: move |_| on_navigate.call(route.clone()),
                    span { class: "fz-bottom-nav-icon",
                        svg { class: "fz-icon", width: 20, height: 20, aria_hidden: "true",
                            use { href: icon }
                        }
                    }
                    span { class: "fz-bottom-nav-label", "{label}" }
                }
            }
        }
    }
}
