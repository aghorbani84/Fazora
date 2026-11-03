//! Navigation sidebar — collapsible, with icon-only compact mode, active
//! destination indicator and tooltips.

use crate::components::branding::BrandMark;
use crate::components::primitives::IconButton;
use crate::routing::Route;
use crate::state::AppState;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum NavItem {
    Home,
    Music,
    Albums,
    Artists,
    Tracks,
    Genres,
    Playlists,
    Videos,
    Folders,
    Favorites,
    Recent,
    History,
    Statistics,
    Search,
    Settings,
}

#[derive(Props)]
pub struct SidebarProps {
    open: Signal<bool>,
    selected: Signal<Route>,
    on_navigate: EventHandler<Route>,
}

#[component]
pub fn Sidebar(props: SidebarProps) -> Element {
    let items = [
        (NavItem::Home, Route::Home {}, "icon-home", "Home"),
        (NavItem::Music, Route::Music {}, "icon-music", "Music"),
        (NavItem::Albums, Route::Albums {}, "icon-album", "Albums"),
        (NavItem::Artists, Route::Artists {}, "icon-artist", "Artists"),
        (NavItem::Tracks, Route::Tracks {}, "icon-track", "Tracks"),
        (NavItem::Playlists, Route::Playlists {}, "icon-playlist", "Playlists"),
        (NavItem::Videos, Route::Videos {}, "icon-video", "Videos"),
        (NavItem::Favorites, Route::Favorites {}, "icon-star", "Favorites"),
        (NavItem::Recent, Route::Recent {}, "icon-clock", "Recently Played"),
        (NavItem::Search, Route::Search {}, "icon-search", "Search"),
        (NavItem::Settings, Route::Settings {}, "icon-settings", "Settings"),
    ];

    let compact = !props.open.get();

    rsx! {
        nav {
            class: if compact {
                "fz-sidebar collapsed"
            } else {
                "fz-sidebar"
            },
            aria_label: "Main navigation",
            div {
                class: "fz-sidebar-brand",
                padding: "var(--space-4)",
                display: "flex",
                align_items: "center",
                gap: "var(--space-3)",
                BrandMark { size: 32 }
                if !compact {
                    span {
                        class: "fz-sidebar-wordmark",
                        font_weight: "700",
                        letter_spacing: "-0.02em",
                        "FAZORA"
                    }
                }
            }
            Divider {}
            div {
                class: "fz-sidebar-scroll",
                overflow_y: "auto",
                flex: "1",
                ul {
                    class: "fz-nav-list",
                    for (item, route, icon, label) in items {
                        li {
                            key: "{route}",
                            NavLink {
                                item,
                                route,
                                icon,
                                label,
                                compact,
                                selected: props.selected.read().clone() == route.clone(),
                                on_click: move |_| props.on_navigate.call(route.clone()),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavLink(
    item: NavItem,
    route: Route,
    icon: &'static str,
    label: &'static str,
    compact: bool,
    selected: bool,
    on_click: EventHandler<()>,
) -> Element {
    let active_class = if selected { " fz-nav-active" } else { "" };

    rsx! {
        button {
            class: format!("fz-nav-link{}", active_class),
            role: "menuitem",
            "aria-selected": selected,
            tabindex: 0,
            onclick: move |_| on_click.call(()),
            onkeydown: move |evt: KeyboardEvent| {
                if matches!(evt.key(), Key::Enter | Key::Space) {
                    on_click.call(());
                }
            },
            div {
                class: "fz-nav-icon",
                svg {
                    class: "fz-icon",
                    width: 20,
                    height: 20,
                    use { href: format!("#{}", icon) }
                }
            }
            if !compact {
                span { class: "fz-nav-label", "{label}" }
            }
        }
    }
}

#[component]
fn Divider() -> Element {
    rsx! {
        hr {
            class: "fz-divider",
            style: "margin: var(--space-4) 0; border-color: var(--border);",
        }
    }
}
