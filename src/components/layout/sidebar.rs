//! Navigation sidebar — collapsible, with active destination indicator.

use crate::components::branding::BrandMark;
use crate::components::primitives::IconButton;
use crate::routing::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum NavItem {
    Home,
    Music,
    Albums,
    Artists,
    Tracks,
    Playlists,
    Videos,
    Favorites,
    Recent,
    Search,
    Settings,
}

#[derive(Props, Clone, PartialEq)]
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
        (
            NavItem::Artists,
            Route::Artists {},
            "icon-artist",
            "Artists",
        ),
        (NavItem::Tracks, Route::Tracks {}, "icon-track", "Tracks"),
        (
            NavItem::Playlists,
            Route::Playlists {},
            "icon-playlist",
            "Playlists",
        ),
        (NavItem::Videos, Route::Videos {}, "icon-video", "Videos"),
        (
            NavItem::Favorites,
            Route::Favorites {},
            "icon-star",
            "Favorites",
        ),
        (
            NavItem::Recent,
            Route::Recent {},
            "icon-clock",
            "Recently Played",
        ),
        (NavItem::Search, Route::Search {}, "icon-search", "Search"),
        (
            NavItem::Settings,
            Route::Settings {},
            "icon-settings",
            "Settings",
        ),
    ];
    let compact = !props.open.cloned();
    let selected_route = props.selected.cloned();
    let on_navigate = props.on_navigate;

    rsx! {
        nav {
            class: if compact { "fz-sidebar collapsed" } else { "fz-sidebar" },
            aria_label: "Main navigation",
            div {
                class: "fz-sidebar-brand",
                padding: "var(--space-4)",
                display: "flex",
                align_items: "center",
                gap: "var(--space-3)",
                BrandMark { size: 32 }
                if !compact {
                    span { class: "fz-sidebar-wordmark", font_weight: "700", letter_spacing: "-0.02em", "FAZORA" }
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
                        NavLink {
                            key: "{label}",
                            item,
                            icon,
                            label,
                            compact,
                            selected: selected_route == route.clone(),
                            on_click: move |_| on_navigate.call(route.clone()),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Props, Clone, PartialEq)]
struct NavLinkProps {
    item: NavItem,
    icon: &'static str,
    label: &'static str,
    compact: bool,
    selected: bool,
    on_click: EventHandler<()>,
}

#[component]
fn NavLink(props: NavLinkProps) -> Element {
    let _item = props.item;
    let active_class = if props.selected { " fz-nav-active" } else { "" };
    rsx! {
        button {
            class: format!("fz-nav-link{}", active_class),
            role: "menuitem",
            "aria-selected": props.selected,
            tabindex: 0,
            onclick: move |_| props.on_click.call(()),
            onkeydown: move |event: KeyboardEvent| {
                if matches!(event.key(), Key::Enter) {
                    props.on_click.call(());
                }
            },
            div {
                class: "fz-nav-icon",
                svg { class: "fz-icon", width: 20, height: 20, use { href: format!("#{}", props.icon) } }
            }
            if !props.compact {
                span { class: "fz-nav-label", "{props.label}" }
            }
        }
    }
}

#[component]
fn Divider() -> Element {
    rsx! {
        hr { class: "fz-divider", style: "margin: var(--space-4) 0; border-color: var(--border);" }
    }
}
