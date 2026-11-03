//! Root application component.
//!
//! Wires global state providers, injects the design token stylesheet and
//! mounts the client-side router. The `AppShell` layout is mounted by the
//! router via `#[layout(AppShell)]` declared in `routing.rs`; the router
//! renders `Outlet` into `AppShell`'s content area.

use crate::audio::AudioService;
use crate::design::css;
use crate::library::LibraryService;
use crate::playback::PlaybackService;
use crate::queue::QueueService;
use crate::routing::Route;
use crate::search::SearchService;
use crate::settings::SettingsService;
use crate::state::AppState;
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    // Build every application service behind a `Signal`. Each service
    // constructor calls Dioxus hooks directly, so they must run in component
    // scope (on every render, in the same order).
    let playback = use_signal(PlaybackService::init);
    let queue = use_signal(QueueService::new);
    let library = use_signal(LibraryService::init);
    let settings = use_signal(SettingsService::init);
    let audio = use_signal(AudioService::init);
    let search = use_signal(SearchService::init);

    let app_state = use_signal(move || {
        AppState::new(
            (*playback.read()).clone(),
            (*queue.read()).clone(),
            (*library.read()).clone(),
            (*settings.read()).clone(),
            (*audio.read()).clone(),
            (*search.read()).clone(),
        )
    });

    provide_context(app_state);

    rsx! {
        // Global design token stylesheet injected once at the root.
        style { {css::TOKENS_CSS} }
        // Component-level styles (buttons, cards, sliders, …).
        style { {css::COMPONENTS_CSS} }

        Router::<Route> { }
    }
}
