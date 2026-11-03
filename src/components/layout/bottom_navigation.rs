//! Android-style bottom navigation with a small number of primary
//! destinations: Home, Music, Videos, Library, Settings.

use crate::routing::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum BottomDestination {
    Home,
    Music,
    Videos,
    Library,
    Settings,
}

#[derive(Props)]
pub struct BottomNavigationProps {
    selected: Signal<Route>,
    on_navigate: EventHandler<Route>,
}

#[component]
pub fn BottomNavigation(props: BottomNavigationProps) -> Element {
    let navigator = use_navigator();

    let destinations = [
        (BottomDestination::Home, Route::Home {}, "icon-home", "Home"),
        (BottomDestination::Music, Route::Music {}, "icon-music", "Music"),
        (BottomDestination::Videos, Route::Videos {}, "icon-video", "Videos"),
        (BottomDestination::Library, Route::Library {}, "icon-folder", "Library"),
        (BottomDestination::Settings, Route::Settings {}, "icon-settings", "Settings"),
    ];

    rsx! {
        nav {
            class: "fz-bottom-nav",
            role: "navigation",
            aria_label: "Bottom navigation",
            for (dest, route, icon, label) in destinations {
                button {
                    key: "{route}",
                    class: if props.selected.read().clone() == route.clone() {
                        "fz-bottom-nav-item active"
                    } else {
                        "fz-bottom-nav-item"
                    },
                    role: "button",
                    "aria-selected": if props.selected.read().clone() == route.clone() { "true" } else { "false" },
                    tabindex: 0,
                    onclick: |_| {
                        let r = route.clone();
                        navigator.push(r.clone());
                        props.on_navigate.call(r);
                    },
                    onkeydown: |evt: KeyboardEvent| {
                        if matches!(evt.key(), Key::Enter | Key::Space) {
                            let r = route.clone();
                            navigator.push(r.clone());
                            props.on_navigate.call(r);
                        }
                    },
                    span {
                        class: "fz-icon",
                        width: 20,
                        height: 20,
                        use { href: format!("#{}", icon) }
                    }
                    span { "{label}" }
                }
            }
        }
    }
}
