//! Core / common components and types

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum UnitType {
    Unknown,
    Mine,
}

impl Default for UnitType {
    fn default() -> UnitType {
        UnitType::Unknown
    }
}

/// Indicates a player's position within a game shard. The game is played on 2D planes,
/// each of which represents the usable surface of a planet, asteroid, or moon
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Position {
    /// The solar system index
    pub sys: u8,
    /// The satellite index within the solar system (planet, moon, asteroid)
    pub sat: u8,
    /// X coordinate on flat satellite map
    pub x: f32,
    /// Y coordinate on flat satellite map.
    pub y: f32,
}

impl Position {
    pub fn new(sys: u8, sat: u8, x: f32, y: f32) -> Position {
        Position { sys, sat, x, y }
    }
}

/// A 2-dimensional vector indicating the velocity of an entity in meters per second.
#[derive(Clone, Debug, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for Velocity {
    fn from(source: (f32, f32)) -> Velocity {
        Velocity {
            x: source.0,
            y: source.1,
        }
    }
}

#[derive(Clone, Debug, Bundle)]
pub struct MobileBundle {
    position: Position,
    velocity: Velocity,
}
