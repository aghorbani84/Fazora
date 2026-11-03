//! Application routing.
//!
//! Every navigable destination is declared here. `AppShell` is the layout for
//! every route (see `#[layout(AppShell)]`); the router renders `Outlet` into
//! `AppShell`'s main content area and each route renders its own view
//! component (specified as the second argument to `#[route]`).

use crate::components::layout::AppShell;
use crate::views::{
    album::Album, artist::Artist, equalizer::EqualizerView, home::Home,
    library::Library, music::Music, now_playing::NowPlaying,
    playlists::{Playlists, Playlist}, queue::Queue, search::Search,
    search_results::SearchResults, settings::{Settings, SettingsSection},
    video_library::VideoLibrary, video_player::VideoPlayer,
};
use dioxus::prelude::*;

#[rustfmt::skip]
#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[layout(AppShell)]
        #[route("/", Home)]
        Home {},
        #[route("/music", Music)]
        Music {},
        #[route("/tracks", Music)]
        Tracks {},
        #[route("/recent", Music)]
        Recent {},
        #[route("/album/:id", Album)]
        Album { id: String },
        #[route("/artist/:id", Artist)]
        Artist { id: String },
        #[route("/albums", Library)]
        Albums {},
        #[route("/artists", Library)]
        Artists {},
        #[route("/playlists", Playlists)]
        Playlists {},
        #[route("/playlists/:id", Playlist)]
        Playlist { id: String },
        #[route("/favorites", Library)]
        Favorites {},
        #[route("/videos", VideoLibrary)]
        Videos {},
        #[route("/video/:id", VideoPlayer)]
        Video { id: String },
        #[route("/now-playing", NowPlaying)]
        NowPlaying {},
        #[route("/queue", Queue)]
        Queue {},
        #[route("/search", Search)]
        Search {},
        #[route("/search/:query", SearchResults)]
        SearchResults { query: String },
        #[route("/equalizer", EqualizerView)]
        Equalizer {},
        #[route("/settings", Settings)]
        Settings {},
        #[route("/settings/:section", SettingsSection)]
        SettingsSection { section: String },
        #[route("/library", Library)]
        Library {},
        #[route("/:..route", not_found)]
        NotFound {},
    #[end_layout]
}

/// 404 destination shown when the URL matches nothing.
#[component]
fn not_found() -> Element {
    let navigator = use_navigator();

    rsx! {
        div { class: "fz-empty",
            h2 { "Not found" }
            p { "The requested destination does not exist." }
            crate::components::primitives::Button {
                variant: crate::components::primitives::ButtonVariant::Primary,
                onclick: move |_| navigator.push(Route::Home {}),
                label: "Return home",
            }
        }
    }
}
