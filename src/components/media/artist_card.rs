//! Artist card for grids.

use crate::models::Artist;
use crate::routing::Route;
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ArtistCardProps {
    artist: Artist,
}

#[component]
pub fn ArtistCard(props: ArtistCardProps) -> Element {
    let navigator = use_navigator();
    let target = Route::Artist {
        id: props.artist.id.clone(),
    };
    let click_navigator = navigator.clone();
    let keyboard_navigator = navigator;
    let open_artist = move |_| {
        click_navigator.push(target.clone());
    };

    rsx! {
        div {
            class: "fz-card",
            role: "button",
            tabindex: 0,
            "aria-label": format!("Artist {}", props.artist.name),
            onclick: open_artist,
            onkeydown: move |evt: KeyboardEvent| {
                if matches!(evt.key(), Key::Enter) {
                    keyboard_navigator.push(target.clone());;
                }
            },
            div {
                class: "fz-artist-cover",
                style: "aspect-ratio: 1 / 1;",
                div {
                    style: "width: 100%; height: 100%; background: var(--surface-tertiary); border: 1px solid var(--border); border-radius: var(--radius-medium); overflow: hidden; position: relative;",
                    if let Some(src) = &props.artist.artwork {
                        img {
                            src: "{src}",
                            alt: props.artist.name.clone(),
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
            div {
                style: "padding: var(--space-3);",
                h4 {
                    style: "font-size: var(--text-lg); font-weight: 700; color: var(--text-primary); white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                    "{props.artist.name}"
                }
                p {
                    style: "font-size: var(--text-sm); color: var(--text-secondary); margin-top: var(--space-1);",
                    match props.artist.album_count {
                        Some(n) => format!("{} albums", n),
                        None => "".to_string(),
                    }
                }
            }
        }
    }
}
