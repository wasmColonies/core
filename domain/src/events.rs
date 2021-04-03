use crate::DOMAIN_VERSION;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Event)]
#[event_type_version(DOMAIN_VERSION)]
#[event_source("events://wacolonies.com/events/colony")]
pub enum ColonyEvent {
    None,
}
