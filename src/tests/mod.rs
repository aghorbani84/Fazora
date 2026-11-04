//! FAZORA Media test suite.
//!
//! Each service harness owns its VirtualDom for the lifetime of the test.
//! Dioxus' default Signal storage is thread-local, so the test hand-off uses
//! thread-local RefCell storage rather than a global Mutex/OnceLock.

use dioxus::prelude::*;
use std::ops::{Deref, DerefMut};

pub struct ServiceHarness<S> {
    service: S,
    _dom: VirtualDom,
}

impl<S> Deref for ServiceHarness<S> {
    type Target = S;

    fn deref(&self) -> &Self::Target {
        &self.service
    }
}

impl<S> DerefMut for ServiceHarness<S> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.service
    }
}

macro_rules! service_harness {
    ($name:ident, $slot:ident, $service:ty, $init:path) => {
        std::thread_local! {
            static $slot: std::cell::RefCell<Option<$service>> = const { std::cell::RefCell::new(None) };
        }

        #[component]
        fn $name() -> Element {
            let service = $init();
            $slot.with(|slot| {
                *slot.borrow_mut() = Some(service.clone());
            });
            rsx! { div {} }
        }

        pub fn with_service() -> super::ServiceHarness<$service> {
            let mut dom = VirtualDom::new($name);
            dom.rebuild_in_place();

            let service = $slot
                .with(|slot| slot.borrow_mut().take())
                .expect("service harness component did not mount");

            super::ServiceHarness { service, _dom: dom }
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
