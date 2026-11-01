//! Centre playback controls for Now Playing and video surfaces.
//! Buttons reflect the real playback engine state.

use crate::playback::PlaybackService;
use crate::queue::QueueService;
use crate::components::primitives::IconButton;
use dioxus::prelude::*;

#[derive(Props)]
pub struct PlaybackControlsProps {
    service: PlaybackService,
    queue: QueueService,
    #[props(default = true)]
    show_shuffle: bool,
    #[props(default = true)]
    show_repeat: bool,
}

#[component]
pub fn PlaybackControls(props: PlaybackControlsProps) -> Element {
    let state = props.service.state.read();
    let is_playing = state.is_playing();

    let previous = move |_| {
        if let Some(track) = props.queue.move_previous() {
            props.service.load(track.clone());
            props.service.state.set(crate::playback::PlaybackState::Playing { track });
        }
    };

    let next = move |_| {
        if let Some(track) = props.queue.move_next() {
            props.service.load(track.clone());
            props.service.state.set(crate::playback::PlaybackState::Playing { track });
        }
    };

    let toggle_play = move |_| {
        props.service.play_toggle();
    };

    let toggle_shuffle = move |_| {
        props.queue.toggle_shuffle();
    };

    let toggle_repeat = move |_| {
        let mode = match state.repeat() ... 
    };
    // repeat handling below uses signal directly

    rsx! {
        div {
            class: "fz-playback-controls",
            style: "display: flex; align-items: center; justify-content: center; gap: var(--space-3);",
            if props.show_shuffle {
                IconButton {
                    active: matches!(props.queue.mode.get(), crate::queue::QueueMode::Shuffle),
                    title: "Shuffle",
                    onclick: toggle_shuffle,
                    svg { class: "fz-icon", width: 22, height: 22, use { href: "#icon-shuffle" } }
                }
            }
            IconButton {
                active: false,
                title: "Previous",
                onclick: previous,
                svg { class: "fz-icon", width: 24, height: 24, use { href: "#icon-previous" } }
            }
            IconButton {
                active: is_playing,
                title: if is_playing { "Pause" } else { "Play" },
                onclick: toggle_play,
                svg {
                    class: "fz-icon",
                    width: 28,
                    height: 28,
                    use { href: if is_playing { "#icon-pause" } else { "#icon-play" } }
                }
            }
            IconButton {
                active: false,
                title: "Next",
                onclick: next,
                svg { class: "fz-icon", width: 24, height: 24, use { href: "#icon-next" } }
            }
            if props.show_repeat {
                IconButton {
                    active: matches!(state.repeat(), crate::playback::RepeatMode::All),
                    title: "Repeat",
                    onclick: move |_| {
                        let next_mode = match state.repeat() {
                            crate::playback::RepeatMode::Off => crate::playback::RepeatMode::One,
                            crate::playback::RepeatMode::One => crate::playback::RepeatMode::All,
                            crate::playback::RepeatMode::All => crate::playback::RepeatMode::Off,
                        };
                        props.service.set_repeat(next_mode);
                    },
                    svg { class: "fz-icon", width: 22, height: 22, use { href: "#icon-repeat" } }
                }
            }
        }
    }
}
