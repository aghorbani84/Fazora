//! Home dashboard — data-driven, hides sections without data.
//! Quick actions: Open File, Open Folder, Scan Media, Create Playlist,
//! Open URL, Resume Playback.

use crate::models::Playlist;
use crate::state::AppState;
use crate::components::media::AlbumCard;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let state = use_context::<Signal<AppState>>();
    let app_state = state.read();
    let library_service = app_state.library.clone();
    let stats = library_service.stats.clone();

    rsx! {
        div {
            class: "fz-page",
            style: "max-width: none;",
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: var(--space-8);",
                div {
                    h1 {
                        style: "font-size: var(--text-4xl); font-weight: 700; color: var(--text-primary); letter-spacing: -0.02em;",
                        "Home"
                    }
                    p {
                        style: "font-size: var(--text-sm); color: var(--text-secondary); margin-top: var(--space-1);",
                        "Your media, engineered."
                    }
                }
                div {
                    style: "display: flex; gap: var(--space-2);",
                    crate::components::primitives::Button {
                        variant: crate::components::primitives::ButtonVariant::Primary,
                        label: "Scan media",
                    }
                    crate::components::primitives::Button {
                        variant: crate::components::primitives::ButtonVariant::Surface,
                        label: "Open folder",
                    }
                }
            }

            // Library statistics
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: var(--space-4); margin-bottom: var(--space-10);",
                StatCard {
                    label: "Tracks",
                    value: stats.read().track_count.to_string(),
                    icon: "#icon-track",
                }
                StatCard {
                    label: "Albums",
                    value: stats.read().album_count.to_string(),
                    icon: "#icon-album",
                }
                StatCard {
                    label: "Artists",
                    value: stats.read().artist_count.to_string(),
                    icon: "#icon-artist",
                }
                StatCard {
                    label: "Videos",
                    value: stats.read().video_count.to_string(),
                    icon: "#icon-video",
                }
            }

            // Continue Listening
            section {
                class: "fz-section",
                RecentlyPlayed { library: library_service.library.read().clone() }
            }

            // Recently Added
            section {
                class: "fz-section",
                RecentlyAdded { library: library_service.library.read().clone() }
            }

            // Your Favorites
            section {
                class: "fz-section",
                Favorites { playlists: library_service.library.read().playlists.clone() }
            }

            // Quick actions
            section {
                class: "fz-section",
                div {
                    class: "fz-section-header",
                    h3 {
                        style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary);",
                        "Quick actions"
                    }
                }
                div {
                    style: "display: flex; gap: var(--space-3); flex-wrap: wrap;",
                    QuickAction {
                        label: "Open file",
                        icon: "#icon-folder",
                        on_click: move |_| {},
                    }
                    QuickAction {
                        label: "Open folder",
                        icon: "#icon-folder",
                        on_click: move |_| {},
                    }
                    QuickAction {
                        label: "Scan media",
                        icon: "#icon-clock",
                        on_click: move |_| {
                            library_service.request_scan();
                        },
                    }
                    QuickAction {
                        label: "Create playlist",
                        icon: "#icon-playlist",
                        on_click: move |_| {},
                    }
                    QuickAction {
                        label: "Open URL",
                        icon: "#icon-globe",
                        on_click: move |_| {},
                    }
                    QuickAction {
                        label: "Resume playback",
                        icon: "#icon-play",
                        on_click: move |_| {},
                    }
                }
            }
        }
    }
}

#[component]
fn StatCard(label: &'static str, value: String, icon: &'static str) -> Element {
    rsx! {
        div {
            class: "fz-card",
            style: "padding: var(--space-5); display: flex; align-items: center; gap: var(--space-4);",
            div {
                style: "width: 48px; height: 48px; background: var(--accent-muted); color: var(--accent); border-radius: var(--radius-medium); display: flex; align-items: center; justify-content: center;",
                svg {
                    class: "fz-icon",
                    width: 22,
                    height: 22,
                    use { href: icon }
                }
            }
            div {
                div {
                    style: "font-size: var(--text-2xl); font-weight: 700; color: var(--text-primary); font-variant-numeric: tabular-nums;",
                    "{value}"
                }
                div {
                    style: "font-size: var(--text-xs); color: var(--text-muted); text-transform: uppercase; letter-spacing: 0.05em;",
                    "{label}"
                }
            }
        }
    }
}

#[component]
fn RecentlyPlayed(library: crate::models::Library) -> Element {
    library_section("Continue listening", &library.tracks)
}

#[component]
fn RecentlyAdded(library: crate::models::Library) -> Element {
    library_section("Recently added", &library.tracks)
}

#[component]
fn library_section(title: &'static str, _tracks: &[crate::models::Track]) -> Element {
    rsx! {
        div {
            class: "fz-section-header",
            h3 {
                style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary);",
                "{title}"
            }
        }
        div {
            class: "fz-empty",
            style: "height: auto; padding: var(--space-6);",
            p { "No media yet." }
        }
    }
}

#[component]
fn Favorites(playlists: Vec<Playlist>) -> Element {
    rsx! {
        div {
            class: "fz-section-header",
            h3 {
                style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary);",
                "Your favorites"
            }
        }
        if playlists.is_empty() {
            div {
                class: "fz-empty",
                style: "height: auto; padding: var(--space-6);",
                p { "No favorite playlists yet." }
            }
        } else {
            div {
                class: "fz-grid",
                for playlist in playlists {
                    div {
                        class: "fz-card",
                        style: "padding: var(--space-4);",
                        div {
                            style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary);",
                            "{playlist.title}"
                        }
                        p {
                            style: "font-size: var(--text-sm); color: var(--text-secondary); margin-top: var(--space-1);",
                            {format!("{} tracks", playlist.track_count())}
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn QuickAction(label: &'static str, icon: &'static str, on_click: EventHandler<()>) -> Element {
    rsx! {
        crate::components::primitives::Button {
            variant: crate::components::primitives::ButtonVariant::Surface,
            onclick: move |_| on_click.call(()),
            style: "min-width: 150px; justify-content: flex-start; padding: var(--space-3) var(--space-4); gap: var(--space-3);",
            svg { class: "fz-icon", width: 16, height: 16, use { href: icon } }
            span { "{label}" }
        }
    }
}
