//! Synchronised lyrics view with current-line highlighting and smooth
//! auto-scroll. Handles missing lyrics gracefully.

use crate::models::Track;
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
                            let line_time = line.time;
                            let offset_signal = manual_offset;
                            let sync_adjust = props.on_sync_adjust;
                            div {
                                key: "{index}",
                                class: if Some(index) == highlighted.cloned() { "fz-lyric-line active" } else { "fz-lyric-line" },
                                style: "transition: color var(--transition-base), transform var(--transition-base); padding: 0 var(--space-2); border-radius: var(--radius-medium); cursor: pointer;",
                                onclick: move |_| {
                                    let mut offset = offset_signal;
                                    offset.set(line_time - position_sec);
                                    if let Some(handler) = sync_adjust {
                                        handler.call(line_time);
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
