//! Synchronised lyrics view with current-line highlighting and smooth
//! auto-scroll. Handles missing lyrics gracefully.

use crate::models::{LyricLine, Track};
use dioxus::prelude::*;

#[derive(Props)]
pub struct LyricsViewProps {
    track: Option<Track>,
    position_sec: f64,
    #[props(default = false)]
    follow: bool,
    on_sync_adjust: Option<EventHandler<f64>>,
}

#[component]
pub fn LyricsView(props: LyricsViewProps) -> Element {
    let lines = props.track.as_ref().and_then(|t| t.lyrics_synced.as_ref());
    let manual_offset = use_signal(|| 0.0f64);
    let highlighted = use_signal(|| None::<usize>);

    use_effect(move || {
        if props.follow {
            let adjusted = props.position_sec + manual_offset.get();
            let mut best: Option<usize> = None;
            if let Some(ls) = lines {
                for (i, line) in ls.iter().enumerate() {
                    if line.time <= adjusted {
                        best = Some(i);
                    } else {
                        break;
                    }
                }
            }
            highlighted.set(best);
        }
    });

    rsx! {
        div {
            class: "fz-lyrics-view",
            style: "height: 100%; display: flex; flex-direction: column; padding: var(--space-8);",
            match lines {
                Some(ls) if !ls.is_empty() => rsx! {
                    div {
                        class: "fz-lyrics-container",
                        style: "flex: 1; overflow-y: auto; font-size: var(--text-lg); line-height: var(--leading-normal); color: var(--text-secondary); display: flex; flex-direction: column; gap: var(--space-4);",
                        for (i, line) in ls.iter().enumerate() {
                            div {
                                key: "{i}",
                                class: if Some(i) == highlighted.read().clone() { "fz-lyric-line active" } else { "fz-lyric-line" },
                                style: "transition: color var(--transition-base), transform var(--transition-base); padding: 0 var(--space-2); border-radius: var(--radius-medium); cursor: pointer;",
                                onclick: move |_| {
                                    manual_offset.set(line.time - props.position_sec);
                                    if let Some(h) = props.on_sync_adjust.as_ref() {
                                        h.call(line.time);
                                    }
                                },
                                "{line.text}"
                            }
                        }
                    }
                },
                _ => rsx! {
                    div {
                        class: "fz-empty",
                        style: "flex: 1; height: auto;",
                        p { "No lyrics available for this track." }
                    }
                },
            }
        }
    }
}
