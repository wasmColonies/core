use crate::Colony;
use crate::UniverseMap;

/// Provides your colony with access to the game world
pub struct Game {}

impl Game {
    /// Information and controls specific to your colony
    pub fn my_colony() -> Colony {
        Colony {}
    }

    /// Access to the stellar navigation view of the universe
    pub fn map() -> UniverseMap {
        UniverseMap {}
    }
}
