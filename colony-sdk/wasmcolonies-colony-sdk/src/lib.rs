#[macro_use]
extern crate lazy_static;

use std::sync::RwLock;
pub use wasmcolonies_colony_sdk_derive::tick;
use wasmcolonies_protocol as protocol;

pub use handlers::Handlers;

mod handlers;

lazy_static! {
    pub(crate) static ref __STATE: RwLock<protocol::GameStateColonyView> =
        RwLock::new(protocol::GameStateColonyView::default());
}

#[doc(hidden)]
pub fn set_state(state: Option<protocol::GameStateColonyView>) {
    *__STATE.write().unwrap() = state.unwrap_or(protocol::GameStateColonyView::default());
}
