use tracing;
use tracing_subscriber;
use tracing_subscriber::prelude::*;
use tracing_subscriber::FmtSubscriber;
use tracing_subscriber::{fmt, EnvFilter};

use wacolonies_domain::ColonyEvent;
use wacolonies_protocol::*;

use tracing::{event, instrument, span, Level};

fn main() {
    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();

    let mut tick = 0_u64;
    loop {
        game_loop(tick);

        tick += 1;
    }
}

#[instrument(name = "Tick")]
fn game_loop(tick: u64) {
    event!(Level::TRACE, "Game tick started");

    let players = collect_players();

    let all_commands = execute_player_ticks(tick, &players);

    process_events(&apply_commands(&all_commands));

    advance_game_components(tick);

    event!(Level::TRACE, "Game tick finished");
}

fn execute_player_ticks(tick: u64, players: &[PlayerTarget]) -> Vec<ColonyCommand> {
    players.iter().fold(vec![], |mut acc, p| {
        acc.extend_from_slice(&player_tick(p, tick));
        acc
    })
}

fn collect_players() -> Vec<PlayerTarget> {
    vec![
        PlayerTarget {
            player_id: "player1".to_string(),
            actor_key: "Mx".to_string(),
        },
        PlayerTarget {
            player_id: "player2".to_string(),
            actor_key: "Mxx".to_string(),
        },
    ]
}

fn player_tick(player: &PlayerTarget, tick: u64) -> Vec<ColonyCommand> {
    vec![ColonyCommand::Pass]
}

fn apply_commands(commands: &[ColonyCommand]) -> Vec<ColonyEvent> {
    event!(Level::TRACE, cmdcount = commands.len(), "Applying commands");
    vec![ColonyEvent::None]
}

fn process_events(events: &[ColonyEvent]) {}

fn advance_game_components(tick: u64) {}
