//! Structure management components and systems

use bevy::prelude::*;

use crate::core::Position;

#[derive(Default, Debug, Clone)]
pub struct Structure {
    max_hp: u16,
    hp: u16,
    attackable: bool,
    ar: u8,
}

impl Structure {
    pub fn player_base() -> Structure {
        Structure {
            max_hp: 1000,
            hp: 1000,
            attackable: true,
            ar: 125,
        }
    }
}

#[derive(Default, Debug, Clone, Bundle)]
pub struct PlayerBaseBundle {
    pub structure: Structure,
    pub position: Position,
}
