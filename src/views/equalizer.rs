//! Equalizer destination view.

use crate::components::audio::Equalizer;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn EqualizerView() -> Element {
    let state = use_context::<Signal<AppState>>();
    let audio = state.read().audio;

    rsx! {
        div {
            class: "fz-page",
            h1 {
                style: "font-size: var(--text-3xl); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-8);",
                "Soundstage"
            }
            Equalizer {
                service: audio.clone(),
            }
        }
    }
}
