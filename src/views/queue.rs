//! Queue view — full page queue management.

use crate::components::player::queue_panel::QueuePanel;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn Queue() -> Element {
    let state = use_context::<Signal<AppState>>();
    let queue = state.read().queue.clone();
    let open = use_signal(|| true);

    rsx! {
        div {
            class: "fz-page",
            h1 {
                style: "font-size: var(--text-3xl); font-weight: 700; color: var(--text-primary); margin-bottom: var(--space-6);",
                "Queue"
            }
            QueuePanel {
                queue: queue.clone(),
                open,
                on_close: move |_| {},
            }
        }
    }
}
