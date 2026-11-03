//! FAZORA Media — application entry point.
//!
//! Bootstraps logging, initialises shared application state and launches the
//! Dioxus desktop window.

mod app;
mod audio;
mod components;
mod design;
mod library;
mod models;
mod playback;
mod queue;
mod routing;
mod search;
mod settings;
mod state;
mod views;

use app::App;
use dioxus_desktop::{launch::launch, Config, WindowBuilder};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

fn main() {
    // Initialise structured logging. Override with e.g. `FAZORA_LOG=debug`.
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
            .with_inner_size((1300.0, 850.0))
            .with_min_inner_size((900.0, 600.0))
            .with_resizable(true)
            .with_maximized(false),
    );

    launch(
        || rsx! { App {} },
        vec![],
        vec![Box::new(config)],
    );
}
