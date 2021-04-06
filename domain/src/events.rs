use crate::DOMAIN_VERSION;

use serde::{Deserialize, Serialize};
use wasmcolonies_protocol::UnitType;

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version(DOMAIN_VERSION)]
#[event_source("events://wacolonies.com/events/colony")]
pub enum ColonyEvent {
    None,
    TickFinished(u64),
    UnitConstructionBegan {
        tick: u64,
        utype: UnitType,
        yield_in: u64,
    },
}
