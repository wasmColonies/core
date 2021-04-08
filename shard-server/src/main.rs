use bevy::{
    app::{AppExit, ScheduleRunnerPlugin, ScheduleRunnerSettings},
    core::FixedTimestep,
    ecs::component::{ComponentDescriptor, StorageType},
    log::LogPlugin,
    prelude::*,
};
use construction::ConstructionSite;
use lattice::ColonyInvoker;
use procgen::big_bang;
use rules::{ColoniesStage, GameParameters, WasmColoniesLabels};

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

mod construction;
mod core;
mod lattice;
mod player;
mod procgen;
mod resources;
mod rules;
mod structure;

use construction::construction;
use player::{colony_commands, player_startup};
use resources::mines;

fn main() -> Result<()> {
    let nc = nats::connect("0.0.0.0").unwrap();
    let params = GameParameters::load_from_file("./default_params.json")?; // TODO: make this a command line option

    let cinvoker = ColonyInvoker::new(nc);

    App::build()
        .insert_resource(params)
        .insert_resource(cinvoker)
        .add_plugins(MinimalPlugins)
        .add_plugin(LogPlugin)
        .add_startup_system(big_bang.system().label(WasmColoniesLabels::BigBang))
        .add_startup_system(player_startup.system().after(WasmColoniesLabels::BigBang))
        .add_stage_before(
            CoreStage::Update,
            ColoniesStage::ActorRpc,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system(colony_commands.system()),
        )
        .add_stage(
            ColoniesStage::Resources,
            SystemStage::parallel()
                .with_run_criteria(FixedTimestep::step(1.0))
                .with_system_set(
                    SystemSet::new()
                        .with_system(mines.system())
                        .with_system(construction.system()),
                ),
        )
        .run();

    Ok(())
}
