//! FAZORA Media test suite.
//!
//! Tests cover the service layer, data models, and core business logic.
//! Service tests use Dioxus's testing render scope because all services
//! construct `Signal`s via `use_signal()` which requires a component scope.

mod models;
mod playback;
mod queue;
mod library;
mod settings;
mod search;
mod audio;
