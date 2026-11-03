//! Application shell: sidebar, topbar, main content, persistent player dock,
//! optional secondary panel and global overlays.

use crate::components::branding::FazoraLogo;
use crate::components::layout::sidebar::Sidebar;
use crate::components::layout::topbar::Topbar;
use crate::components::player::MiniPlayer;
use crate::components::player::QueuePanel;
use crate::routing::Route;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn AppShell() -> Element {
    let state = use_context::<Signal<AppState>>();
    let navigator = use_navigator();

    let sidebar_open = use_signal(|| false);
    let secondary_open = use_signal(|| false);
    let selected_route = use_signal(|| Route::Home {});

    let toggle_sidebar = move |_| {
        sidebar_open.set(!sidebar_open.cloned());
    };

    let navigate = move |route: Route| {
        navigator.push(route.clone());
        selected_route.set(route);
        sidebar_open.set(false);
    };

    rsx! {
        div {
            class: "fz-app-shell",
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
            div {
                class: "fz-body",
                Topbar {
                    on_toggle_sidebar: toggle_sidebar,
                    on_open_queue: move |_| secondary_open.set(true),
                }
                div {
                    class: "fz-content",
                    main {
                        // The router renders the active route here via Outlet.
                        Outlet::<Route> {}
                    }
                }
                if secondary_open.cloned() {
                    QueuePanel { open: secondary_open, on_close: move |_| secondary_open.set(false) }
                }
            }
            MiniPlayer {}
        }
    }
}
