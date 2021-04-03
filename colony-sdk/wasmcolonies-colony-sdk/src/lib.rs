#[macro_use]
extern crate lazy_static;

use std::sync::RwLock;
pub use wasmcolonies_colony_sdk_derive::tick;
use wasmcolonies_protocol as protocol;

pub use colony::*;
pub use game::*;
pub use handlers::Handlers;
pub use map::*;

mod colony;
mod game;
mod handlers;
mod map;

lazy_static! {
    #[doc(hidden)]
    static ref __STATE: RwLock<protocol::GameStateColonyView> =
        RwLock::new(protocol::GameStateColonyView::default());
    #[doc(hidden)]
    static ref __CMDSTACK: RwLock<Vec<protocol::ColonyCommand>> = RwLock::new(vec![]);
}

#[doc(hidden)]
pub fn set_state(state: Option<protocol::GameStateColonyView>) {
    *__CMDSTACK.write().unwrap() = vec![];
    *__STATE.write().unwrap() = state.unwrap_or(protocol::GameStateColonyView::default());
}

#[doc(hidden)]
pub fn get_cmdstack() -> Vec<protocol::ColonyCommand> {
    __CMDSTACK.read().unwrap().clone()
}
