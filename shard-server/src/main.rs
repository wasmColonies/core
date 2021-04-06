use bevy::{
    app::{AppExit, ScheduleRunnerPlugin, ScheduleRunnerSettings},
    log::LogPlugin,
    prelude::*,
};
use lattice::ColonyInvoker;
use procgen::big_bang;
use rules::{GameParameters, WasmColoniesLabels};

use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

mod lattice;
mod player;
mod procgen;
mod rules;

use player::{colony_commands, player_startup};

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
        .add_system_to_stage(CoreStage::PreUpdate, colony_commands.system())
        .run();

    Ok(())
}
