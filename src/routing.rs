//! Application routing.
//!
//! Every navigable destination is declared here. Dioxus keeps the URL and the
//! UI in sync; the shell renders the active route inside its main content area.

use crate::views::*;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[route("/")]
    Home,
    #[route("/music")]
    Music,
    #[route("/album/:id")]
    Album { id: String },
    #[route("/artist/:id")]
    Artist { id: String },
    #[route("/playlists")]
    Playlists,
    #[route("/playlists/:id")]
    Playlist { id: String },
    #[route("/videos")]
    Videos,
    #[route("/video/:id")]
    Video { id: String },
    #[route("/now-playing")]
    NowPlaying,
    #[route("/queue")]
    Queue,
    #[route("/search")]
    Search,
    #[route("/search/:query")]
    SearchResults { query: String },
    #[route("/equalizer")]
    Equalizer,
    #[route("/settings")]
    Settings,
    #[route("/settings/:section")]
    SettingsSection { section: String },
    #[route("/library")]
    Library,
    #[route("/:..route")]
    NotFound,
}

pub fn resolve_component(route: Route) -> Element {
    match route {
        Route::Home => rsx! { home::Home {} },
        Route::Music => rsx! { music::Music {} },
        Route::Album { id } => rsx! { album::Album { id } },
        Route::Artist { id } => rsx! { artist::ArtistView { id } },
        Route::Playlists => rsx! { playlists::Playlists {} },
        Route::Playlist { id } => rsx! { playlists::Playlist { id } },
        Route::Videos => rsx! { video_library::VideoLibrary {} },
        Route::Video { id } => rsx! { video_player::VideoPlayer { id } },
        Route::NowPlaying => rsx! { now_playing::NowPlaying {} },
        Route::Queue => rsx! { queue::Queue {} },
        Route::Search => rsx! { search::Search {} },
        Route::SearchResults { query } => rsx! { search_results::SearchResults { query } },
        Route::Equalizer => rsx! { equalizer::EqualizerView {} },
        Route::Settings => rsx! { settings::Settings {} },
        Route::SettingsSection { section } => rsx! { settings::SettingsSection { section } },
        Route::Library => rsx! { library::Library {} },
        Route::NotFound => rsx! {
            div { class: "fz-empty",
                h2 { "Not found" }
                p { "The requested destination does not exist." }
                crate::components::primitives::Button {
                    variant: crate::components::primitives::ButtonVariant::Primary,
                    onclick: move |_| {},
                    label: "Return home",
                }
            }
        },
    }
}
