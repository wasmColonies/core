//! Data-driven game parameters and settings

use std::{collections::HashMap, fs::File, path::PathBuf, str::FromStr};

use crate::core::UnitType;
use crate::Result;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::io::Read;

#[derive(Clone, Debug, PartialEq, Hash, Eq, StageLabel)]
pub enum ColoniesStage {
    ActorRpc,
    Resources,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, SystemLabel)]
pub enum WasmColoniesLabels {
    BigBang,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GameParameters {
    pub construction_times: HashMap<UnitType, u16>,
}

impl GameParameters {
    pub fn load_from_file(path: &str) -> Result<GameParameters> {
        let path = PathBuf::from_str(path)?;
        let mut f = File::open(path)?;
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;
        Ok(serde_json::from_slice(&buf)?)
    }
}
