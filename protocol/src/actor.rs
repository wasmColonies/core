use crate::command::ColonyCommand;
use serde::{Deserialize, Serialize};

pub const OP_PLAYER_TICK: &str = "PlayerTick";
#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct PlayerTick {
    pub tick: u64,
    pub player_id: String,
    pub game_state: Option<GameStateColonyView>, // This is populated by middleware on-host
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

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct GameStateColonyView {
    pub placeholder: u64,
}
