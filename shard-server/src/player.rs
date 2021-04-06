use crate::{lattice::ColonyInvoker, rules::GameParameters};
use bevy::{prelude::*, tasks::ComputeTaskPool};

use tracing::{debug, error, event, info, instrument, span, Level};
use wasmcolonies_protocol::GameStateColonyView;

const BATCH_SIZE: usize = 10;

pub fn player_startup(mut commands: Commands, game_params: Res<GameParameters>) {
    info!(
        "Game parameters available, {} construction times defined",
        game_params.construction_times.len()
    );
    info!("Injecting initial players");
    // Add some players to our world. Players start with a score of 0 ... we want our game to be fair!
    commands.spawn_batch(vec![(
        Player {
            id: "player1".to_string(),
            actor_key: "MDVVUGY5RK7TMJGJOOCOTFY6QA3M3W4ODENO43HWEKLO5OMTVKF5KAWJ".to_string(),
        },
        Score { value: 0 },
    )]);
}

pub fn colony_commands(
    pool: Res<ComputeTaskPool>,
    mut commands: Commands,
    invoker: Res<ColonyInvoker>,
    mut query: Query<(&Player, &Score)>,
) {
    trace!("Fetching player commands");
    query.par_for_each(&pool, BATCH_SIZE, |(player, score)| {
        let cmds = invoker.fetch_commands(
            &player.id,
            &player.actor_key,
            GameStateColonyView::default(),
        );
        match cmds {
            Ok(cmds) => {
                info!("{:?}", cmds);
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    });
}

#[derive(Default, Debug, Clone)]
pub struct Player {
    pub actor_key: String,
    pub id: String,
}

#[derive(Default, Debug)]
pub struct Score {
    pub value: usize,
}
