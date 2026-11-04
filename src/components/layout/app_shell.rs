//! Responsive application shell with sidebar, content, mini-player and bottom nav.
use crate::components::layout::bottom_navigation::BottomNavigation;
use crate::components::layout::sidebar::Sidebar;
use crate::components::layout::topbar::Topbar;
use crate::components::player::MiniPlayer;
use crate::routing::Route;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn AppShell() -> Element {
    let _state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();
    let mut sidebar_open = use_signal(|| false);
    let mut selected_route = use_signal(|| Route::Home {});

    let toggle_sidebar = move |_| {
        let next = !*sidebar_open.read();
        sidebar_open.set(next);
    };

    let navigate = move |route: Route| {
        navigator.push(route.clone());
        selected_route.set(route);
        sidebar_open.set(false);
    };

    rsx! {
        div { class: "fz-app-shell",
            Sidebar {
                open: sidebar_open,
                selected: selected_route,
                on_navigate: navigate,
            }
            if sidebar_open() {
                div {
                    class: "fz-sidebar-overlay open",
                    aria_label: "Close navigation",
                    onclick: move |_| sidebar_open.set(false),
                }
            }
            div { class: "fz-body",
                Topbar { on_toggle_sidebar: toggle_sidebar }
                div { class: "fz-content",
                    main { Outlet::<Route> {} }
                }
            }
            MiniPlayer {}
            BottomNavigation {
                selected: selected_route,
                on_navigate: navigate,
            }
        }
    }
}
