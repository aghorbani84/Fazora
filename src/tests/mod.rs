//! FAZORA Media test suite.
//!
//! Service signals are created inside a Dioxus component scope. Each test
//! harness retains its VirtualDom so the scope and its signals remain alive
//! while the test uses the service. Local Rc/RefCell storage avoids putting
//! non-Send Dioxus signals in a process-wide Mutex.

use dioxus::prelude::*;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

/// Owns a service and the VirtualDom scope that allocated its signals.
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

/// Generate a per-service test harness.
macro_rules! service_harness {
    ($name:ident, $slot:ident, $service:ty, $init:path) => {
        #[component]
        fn $name(slot: Rc<RefCell<Option<$service>>>) -> Element {
            let service = use_signal($init);
            *slot.borrow_mut() = Some((*service.read()).clone());
            rsx! { div {} }
        }

        pub fn with_service() -> ServiceHarness<$service> {
            let slot = Rc::new(RefCell::new(None));
            let mut dom = VirtualDom::new_with_props($name, slot.clone());
            dom.rebuild_in_place();

            let service = slot
                .borrow_mut()
                .take()
                .expect("service harness component did not mount");

            ServiceHarness { service, _dom: dom }
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
