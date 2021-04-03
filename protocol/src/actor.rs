use crate::command::ColonyCommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct PlayerTick {
    pub tick: u64,
    pub player_id: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct PlayerTickResponse {
    pub commands: Vec<ColonyCommand>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct PlayerTarget {
    pub player_id: String,
    pub actor_key: String,
}
