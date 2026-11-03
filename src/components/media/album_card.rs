//! Album card for grids: artwork, title, artist, year, track count.
//! Interactions: open album, play album, shuffle, add to queue/playlist,
//! context menu.

use crate::components::primitives::Button;
use crate::models::Album;
use crate::routing::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct AlbumCardProps {
    album: Album,
    #[props(default)]
    on_play: EventHandler<Album>,
    #[props(default)]
    on_shuffle: EventHandler<Album>,
    #[props(default)]
    on_add_queue: EventHandler<Album>,
}

#[component]
pub fn AlbumCard(props: AlbumCardProps) -> Element {
    let navigator = use_navigator();

    let open_album = move |_| {
        navigator.push(Route::Album {
            id: props.album.id.clone(),
        });
    };

    let play_album = move |_| {
        props.on_play.call(props.album.clone());
    };

    rsx! {
        div {
            class: "fz-card",
            role: "button",
            tabindex: 0,
            "aria-label": format!("Album {} by {}", props.album.title, props.album.artist),
            onclick: open_album,
            onkeydown: move |evt: KeyboardEvent| {
                if matches!(evt.key(), Key::Enter) {
                    open_album(());
                }
            },
            div {
                class: "fz-album-cover",
                position: "relative",
                width: "100%",
                style: "aspect-ratio: 1 / 1;",
                Artwork {
                    src: props.album.artwork.clone(),
                    width: 0,
                    height: 0,
                    alt: props.album.title.clone(),
                }
                Button {
                    variant: crate::components::primitives::ButtonVariant::Ghost,
                    class: "fz-album-play-overlay",
                    onclick: play_album,
                    svg {
                        class: "fz-icon",
                        width: 24,
                        height: 24,
                        use { href: "#icon-play" }
                    }
                }
            }
            div {
                style: "padding: var(--space-3) var(--space-3) var(--space-2) var(--space-3);",
                h4 {
                    class: "fz-album-title",
                    style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{props.album.title}"
                }
                p {
                    class: "fz-album-artist",
                    style: "font-size: var(--text-sm); color: var(--text-secondary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{props.album.artist}"
                }
                div {
                    style: "display: flex; justify-content: space-between; margin-top: var(--space-2); font-size: var(--text-xs); color: var(--text-muted);",
                    span {
                        match props.album.year {
                            Some(y) => y.to_string(),
                            None => String::new(),
                        }
                    }
                    span {
                        match props.album.track_count {
                            Some(n) => format!("{} tracks", n),
                            None => "".to_string(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Artwork(
    src: Option<String>,
    width: u32,
    height: u32,
    alt: String,
) -> Element {
    rsx! {
        div {
            style: "width: 100%; height: 100%; background: var(--surface-tertiary); border: 1px solid var(--border); border-radius: var(--radius-medium); overflow: hidden; position: relative;",
            if let Some(src) = src {
                img {
                    src: "{src}",
                    alt: "{alt}",
                    style: "width: 100%; height: 100%; object-fit: cover; display: block;",
                }
            } else {
                div {
                    style: "width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; color: var(--text-muted); font-family: var(--font-mono); font-size: var(--text-xs);",
                    "FZ"
                }
            }
        }
    }
}
