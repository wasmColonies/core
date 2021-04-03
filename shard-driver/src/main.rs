use std::{error::Error, time::Duration};

use inv::{Entity, Invocation, InvocationResponse};
use nats::Connection;
use tracing;
use tracing_subscriber;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use wascap::prelude::KeyPair;
use wasmcolonies_domain::ColonyEvent;
use wasmcolonies_protocol::*;

use tracing::{debug, error, event, info, instrument, span, Level};

mod inv;

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
    let nc = nats::connect("0.0.0.0").unwrap();
    let hk = KeyPair::new_server();

    loop {
        game_loop(tick, &nc, &hk);

        tick += 1;
    }
}

#[instrument(name = "Tick")]
fn game_loop(tick: u64, nc: &Connection, hk: &KeyPair) {
    event!(Level::TRACE, "Game tick started");

    let players = collect_players();

    let all_commands = execute_player_ticks(tick, &players, nc, hk);

    process_events(&apply_commands(&all_commands));

    advance_game_components(tick);

    event!(Level::TRACE, "Game tick finished");
}

fn execute_player_ticks(
    tick: u64,
    players: &[PlayerTarget],
    nc: &Connection,
    hk: &KeyPair,
) -> Vec<ColonyCommand> {
    players.iter().fold(vec![], |mut acc, p| {
        acc.extend_from_slice(&player_tick(p, tick, nc, hk));
        acc
    })
}

fn collect_players() -> Vec<PlayerTarget> {
    vec![PlayerTarget {
        player_id: "player1".to_string(),
        actor_key: "MDVVUGY5RK7TMJGJOOCOTFY6QA3M3W4ODENO43HWEKLO5OMTVKF5KAWJ".to_string(), // replace with yours
    }]
}

fn player_tick(
    player: &PlayerTarget,
    tick: u64,
    nc: &Connection,
    hk: &KeyPair,
) -> Vec<ColonyCommand> {
    let pt = PlayerTick {
        tick: tick,
        player_id: player.player_id.to_string(),
        game_state: None, // Populated by middleware upon receipt
    };
    let inv = Invocation::new(
        hk,
        Entity::Actor("system".to_string()),
        Entity::Actor(player.actor_key.to_string()),
        "PlayerTick",
        serialize(pt).unwrap(),
    );
    let subject = &rpc_subject(None, &player.actor_key);
    let res = nc.request_timeout(
        subject,
        &serialize(inv).unwrap(),
        Duration::from_millis(100),
    );
    let tr: Result<PlayerTickResponse, std::io::Error> = res
        .and_then(|m| {
            deserialize::<InvocationResponse>(&m.data)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))
        })
        .and_then(|ir| {
            if let Some(e) = ir.error {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("{}", e),
                ))
            } else {
                deserialize::<PlayerTickResponse>(&ir.msg)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e)))
            }
        });
    if let Ok(tr) = tr {
        tr.commands
    } else {
        vec![ColonyCommand::Pass]
    }
}

fn rpc_subject(prefix: Option<String>, actor: &str) -> String {
    format!(
        "wasmbus.rpc.{}.{}",
        prefix.as_ref().unwrap_or(&"default".to_string()),
        actor
    )
}

fn apply_commands(commands: &[ColonyCommand]) -> Vec<ColonyEvent> {
    event!(Level::TRACE, cmdcount = commands.len(), "Applying commands");
    vec![ColonyEvent::None]
}

fn process_events(events: &[ColonyEvent]) {}

fn advance_game_components(tick: u64) {}
