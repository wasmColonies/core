use std::sync::RwLock;
use wapc_guest::prelude::*;
use wasmcolonies_protocol as protocol;
use wasmcolonies_protocol::{deserialize, serialize};

lazy_static! {
    #[doc(hidden)]
    static ref PLAYER_TICK: RwLock<Option<fn(protocol::PlayerTick) -> HandlerResult<protocol::PlayerTickResponse>>> =
        RwLock::new(None);
}

#[doc(hidden)]
pub struct Handlers {}

#[doc(hidden)]
impl Handlers {
    pub fn register_player_tick(
        f: fn(protocol::PlayerTick) -> HandlerResult<protocol::PlayerTickResponse>,
    ) {
        *PLAYER_TICK.write().unwrap() = Some(f);
        register_function(&"PlayerTick", player_tick_wrapper);
    }
}

fn player_tick_wrapper(input_payload: &[u8]) -> CallResult {
    let input = deserialize::<protocol::PlayerTick>(input_payload)?;
    let lock = PLAYER_TICK.read().unwrap().unwrap();
    let result = lock(input)?;
    Ok(serialize(result)?)
}
