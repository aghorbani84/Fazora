//! FAZORA Media application entry point.
//!
//! Dioxus CLI selects the desktop or mobile Cargo feature according
//! to the requested target. The UI and application services remain shared.

mod app;
mod audio;
mod components;
mod design;
mod library;
mod media_engine;
mod models;
mod playback;
mod queue;
mod routing;
mod search;
mod security;
mod settings;
mod state;
mod views;

use app::App;
use dioxus::prelude::*;

#[cfg(feature = "desktop")]
fn main() {
    use dioxus_desktop::{launch::launch, tao::dpi::LogicalSize, Config, WindowBuilder};
    use tracing_subscriber::{EnvFilter, FmtSubscriber};

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,fazora=info,dioxus=warn"));
    let _ = FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_target(false)
        .with_ansi(true)
        .try_init();

    let config = Config::new().with_window(
        WindowBuilder::new()
            .with_title("FAZORA Media")
            .with_inner_size(LogicalSize::new(1300.0, 850.0))
            .with_min_inner_size(LogicalSize::new(900.0, 600.0))
            .with_resizable(true)
            .with_maximized(false),
    );

    launch(|| rsx! { App {} }, vec![], vec![Box::new(config)]);
}

#[cfg(feature = "mobile")]
fn main() {
    dioxus::launch(App);
}

#[cfg(not(any(feature = "desktop", feature = "mobile")))]
compile_error!("Select either the desktop or mobile Cargo feature.");
