//! Global application state.
//! All services are constructed once inside the component scope and exposed
//! through Dioxus context so views and components stay decoupled.

pub mod app_state;

pub use app_state::AppState;
