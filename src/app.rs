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
    // Service constructors create Dioxus signals. Call them directly in this
    // component's hook scope; do not wrap them in another use_signal
    // initializer, because that would execute hooks from inside a hook's
    // lazy initializer and can corrupt hook ordering on mobile runtimes.
    let playback = PlaybackService::init();
    let queue = QueueService::new();
    let library = LibraryService::init();
    let settings = SettingsService::init();
    let audio = AudioService::init();
    let search = SearchService::init();

    // The aggregate state itself is a signal, but its initializer only
    // assembles already-created service handles and does not invoke hooks.
    let app_state = use_signal(move || {
        AppState::new(
            playback.clone(),
            queue.clone(),
            library.clone(),
            settings.clone(),
            audio.clone(),
            search.clone(),
        )
    });

    provide_context(app_state);

    rsx! {
        style { {css::TOKENS_CSS} }
        style { {css::COMPONENTS_CSS} }
        // Mount the shared SVG symbol sprite once so every icon reference resolves
        // consistently in the Android WebView and desktop renderer.
        div {
            class: "fz-icon-sprite",
            dangerous_inner_html: crate::components::ICONS,
        }
        Router::<Route> {}
    }
}
