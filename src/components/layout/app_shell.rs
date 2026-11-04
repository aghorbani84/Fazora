//! Application shell: responsive navigation, content and persistent player.
use crate::components::layout::sidebar::Sidebar;
use crate::components::layout::topbar::Topbar;
use crate::components::layout::bottom_navigation::BottomNavigation;
use crate::components::player::{MiniPlayer, QueuePanel};
use crate::routing::Route;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn AppShell() -> Element {
    let state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();
    let queue = state.read().queue.clone();
    let mut sidebar_open = use_signal(|| false);
    let mut secondary_open = use_signal(|| false);
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
    let navigate_bottom = move |route: Route| {
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
            if sidebar_open.cloned() {
                div {
                    class: "fz-sidebar-overlay open",
                    onclick: move |_| sidebar_open.set(false),
                }
            }
            div { class: "fz-body",
                Topbar {
                    on_toggle_sidebar: toggle_sidebar,
                    on_open_queue: move |_| secondary_open.set(true),
                }
                div { class: "fz-content",
                    main { Outlet::<Route> {} }
                }
                QueuePanel {
                    queue: queue.clone(),
                    open: secondary_open,
                    on_close: move |_| secondary_open.set(false),
                }
            }
            MiniPlayer {}
            BottomNavigation {
                selected: selected_route,
                on_navigate: navigate_bottom,
            }
        }
    }
}
