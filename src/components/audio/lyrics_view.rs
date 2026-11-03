//! Synchronised lyrics view with current-line highlighting and smooth
//! auto-scroll. Handles missing lyrics gracefully.

use crate::models::{LyricLine, Track};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct LyricsViewProps {
    track: Option<Track>,
    position_sec: f64,
    #[props(default = false)]
    follow: bool,
    on_sync_adjust: Option<EventHandler<f64>>,
}

#[component]
pub fn LyricsView(props: LyricsViewProps) -> Element {
    let lines = props.track.as_ref().and_then(|track| track.lyrics_synced.clone());
    let position_sec = props.position_sec;
    let mut manual_offset = use_signal(|| 0.0_f64);
    let mut highlighted = use_signal(|| None::<usize>);
    let effect_lines = lines.clone();

    use_effect(move || {
        if props.follow {
            let adjusted = position_sec + manual_offset.cloned();
            let mut best = None;
            if let Some(lines) = effect_lines.as_ref() {
                for (index, line) in lines.iter().enumerate() {
                    if line.time <= adjusted {
                        best = Some(index);
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
            match lines.as_ref() {
                Some(lines) if !lines.is_empty() => rsx! {
                    div {
                        class: "fz-lyrics-container",
                        style: "flex: 1; overflow-y: auto; font-size: var(--text-lg); line-height: var(--leading-normal); color: var(--text-secondary); display: flex; flex-direction: column; gap: var(--space-4);",
                        for (index, line) in lines.iter().enumerate() {
                            LyricLineRow {
                                key: "{index}",
                                line: line.clone(),
                                active: Some(index) == highlighted.cloned(),
                                position_sec,
                                offset: manual_offset,
                                on_sync_adjust: props.on_sync_adjust,
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


#[derive(Props, Clone, PartialEq)]
struct LyricLineRowProps {
    line: LyricLine,
    active: bool,
    position_sec: f64,
    offset: Signal<f64>,
    on_sync_adjust: Option<EventHandler<f64>>,
}

#[component]
fn LyricLineRow(props: LyricLineRowProps) -> Element {
    let line_time = props.line.time;
    let text = props.line.text.clone();
    let active = props.active;
    let position_sec = props.position_sec;
    let mut offset = props.offset;
    let sync_adjust = props.on_sync_adjust;

    rsx! {
        div {
            class: if active { "fz-lyric-line active" } else { "fz-lyric-line" },
            style: "transition: color var(--transition-base), transform var(--transition-base); padding: 0 var(--space-2); border-radius: var(--radius-medium); cursor: pointer;",
            onclick: move |_| {
                let next_offset = line_time - position_sec;
                offset.set(next_offset);
                if let Some(handler) = sync_adjust {
                    handler.call(line_time);
                }
            },
            "{text}"
        }
    }
}
