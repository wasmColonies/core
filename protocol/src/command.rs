use serde::{Deserialize, Serialize};

use crate::UnitType;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub enum ColonyCommand {
    Pass(u64),
    ConstructUnit(u64, UnitType),
}
