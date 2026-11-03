//! FAZORA Media test suite.
//!
//! Tests cover the service layer, data models, and core business logic.
//! Dioxus 0.7 removed `dioxus::testing::RenderScope`. Because every service
//! builds its `Signal`s via `use_signal()` (which must run inside a component
//! scope), each service is exercised through a small `VirtualDom` harness:
//! the harness mounts the service component, captures a clone of the service
//! (clones share the same underlying signal state), and the test drives it
//! directly.

/// Generate a per-service test harness.
///
/// `$name`  - name of the generated harness component
/// `$slot`  - name of the static storage slot
/// `$service` - the service type
/// `$init`  - service constructor path, e.g. `PlaybackService::init`
macro_rules! service_harness {
    ($name:ident, $slot:ident, $service:ty, $init:path) => {
        static $slot: std::sync::OnceLock<std::sync::Mutex<Option<$service>>> =
            std::sync::OnceLock::new();

        #[component]
        fn $name() -> Element {
            let service = use_signal($init);
            let cloned = (*service.read()).clone();
            *$slot
                .get_or_init(|| std::sync::Mutex::new(None))
                .lock()
                .unwrap() = Some(cloned);
            rsx! { div {} }
        }

        /// Mount the harness in a fresh `VirtualDom` and return the service.
        pub fn with_service() -> $service {
            let slot = $slot.get_or_init(|| std::sync::Mutex::new(None));
            *slot.lock().unwrap() = None;
            let mut vdom = VirtualDom::new($name);
            vdom.rebuild();
            slot.lock()
                .unwrap()
                .take()
                .expect("service harness did not run")
        }
    };
}

mod audio;
mod library;
mod models;
mod playback;
mod queue;
mod search;
mod settings;
