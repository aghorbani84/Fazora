//! Root application component.
//!
//! Wires global state providers, injects the design token stylesheet, mounts
//! the application shell and the client-side router.

use crate::components::layout::AppShell;
use crate::design::css;
use crate::routing::Route;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
fn App() -> Element {
    let app_state = use_signal(AppState::new);

    provide_context(app_state);

    rsx! {
        // Global design token stylesheet injected once at the root.
        style { {css::TOKENS_CSS} }
        // Component-level styles (buttons, cards, sliders, …).
        style { {css::COMPONENTS_CSS} }

        Router::<Route> {
            AppShell {}
        }
    }
}
