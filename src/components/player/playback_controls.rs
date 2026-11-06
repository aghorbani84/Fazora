//! Centre playback controls for Now Playing and video surfaces.
//! Buttons reflect the real playback engine state.

use crate::components::branding::AppIcon;
use crate::components::primitives::IconButton;
use crate::playback::{PlaybackService, PlaybackState, RepeatMode};
use crate::queue::{QueueMode, QueueService};
use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
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
    let service = props.service.clone();
    let queue = props.queue.clone();
    let is_playing = service.state.read().is_playing();
    let shuffle_active = *queue.mode.read() == QueueMode::Shuffle;
    let repeat_mode = *service.repeat.read();

    let previous_service = service.clone();
    let previous_queue = queue.clone();
    let previous = move |_| {
        if let Some(track) = previous_queue.move_previous() {
            previous_service.set_state(PlaybackState::Playing { track });
        }
    };

    let next_service = service.clone();
    let next_queue = queue.clone();
    let next = move |_| {
        if let Some(track) = next_queue.move_next() {
            next_service.set_state(PlaybackState::Playing { track });
        }
    };

    let play_service = service.clone();
    let toggle_play = move |_| play_service.play_toggle();

    let shuffle_queue = queue.clone();
    let toggle_shuffle = move |_| shuffle_queue.toggle_shuffle();

    let repeat_service = service.clone();
    let toggle_repeat = move |_| {
        let next_mode = match *repeat_service.repeat.read() {
            RepeatMode::Off => RepeatMode::One,
            RepeatMode::One => RepeatMode::All,
            RepeatMode::All => RepeatMode::Off,
        };
        repeat_service.set_repeat(next_mode);
    };

    rsx! {
        div {
            class: "fz-playback-controls",
            style: "display: flex; align-items: center; justify-content: center; gap: var(--space-3);",
            if props.show_shuffle {
                IconButton {
                    active: shuffle_active,
                    title: "Shuffle",
                    onclick: toggle_shuffle,
                    AppIcon { name: "shuffle", size: 22 }
                }
            }
            IconButton {
                active: false,
                title: "Previous",
                onclick: previous,
                AppIcon { name: "previous", size: 24 }
            }
            IconButton {
                active: is_playing,
                title: if is_playing { "Pause" } else { "Play" },
                onclick: toggle_play,
                AppIcon { name: if is_playing { "pause" } else { "play" }, size: 28 }
            }
            IconButton {
                active: false,
                title: "Next",
                onclick: next,
                AppIcon { name: "next", size: 24 }
            }
            if props.show_repeat {
                IconButton {
                    active: repeat_mode == RepeatMode::All,
                    title: "Repeat",
                    onclick: toggle_repeat,
                    AppIcon { name: "repeat", size: 22 }
                }
            }
        }
    }
}
