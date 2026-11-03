//! Videos destination: poster/thumbnail grid, continue watching, favorites.

use crate::components::media::VideoCard;
use crate::models::Video;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn VideoLibrary() -> Element {
    let state = use_context::<Signal<AppState>>();
    let library = state.read().library.clone();
    let videos = library.library.read().videos.clone();

    let continue_watching: Vec<Video> = videos.iter()
        .filter(|v| v.watched_position_sec.unwrap_or(0) > 0 && !v.watched)
        .cloned()
        .collect();

    let favorites: Vec<Video> = videos.iter()
        .filter(|v| v.favorite)
        .cloned()
        .collect();

    rsx! {
        div {
            class: "fz-page",
            div {
                class: "fz-section-header",
                style: "margin-bottom: var(--space-6);",
                h1 {
                    style: "font-size: var(--text-3xl); font-weight: 700; color: var(--text-primary);",
                    "Videos"
                }
            }

            if !continue_watching.is_empty() {
                section {
                    class: "fz-section",
                    div {
                        style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-4);",
                        "Continue watching"
                    }
                    div {
                        class: "fz-grid",
                        for video in continue_watching {
                            VideoCard { key: "{video.id}", video }
                        }
                    }
                }
            }

            section {
                class: "fz-section",
                div {
                    style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-4);",
                    "Video library"
                }
                if videos.is_empty() {
                    div {
                        class: "fz-empty",
                        style: "height: auto; padding: var(--space-6);",
                        p { "No videos in the library." }
                    }
                } else {
                    div {
                        class: "fz-grid",
                        for video in &videos {
                            VideoCard { key: "{video.id}", video: video.clone() }
                        }
                    }
                }
            }

            if !favorites.is_empty() {
                section {
                    class: "fz-section",
                    div {
                        style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-4);",
                        "Favorites"
                    }
                    div {
                        class: "fz-grid",
                        for video in favorites {
                            VideoCard { key: "{video.id}", video }
                        }
                    }
                }
            }
        }
    }
}
