// Philosophical note: These two main structs (Invocation and InvocationResponse) exist inside
// the wasmcloud-host crate. If this crate were to refer to that, we'd create a cyclical reference
// and all perish inside a black hole.
//
// The other alternative is to create a new, third crate shared by the two, but that creates a brand
// new "reason to change" now for three crates and produces a "dependency as a liability"
//
// Therefore, it's easier to simply create copies of the data types since we only need them to be
// able to push them on the wire. The "real" Invocation and InvocationResponse types are in the wasmcloud-host
// crate in the dispatch module because we need to implement other traits on those types.

use crate::Result;
use data_encoding::HEXUPPER;
use ring::digest::{Context, Digest, SHA256};
use serde::{Deserialize, Serialize};
use std::{io::Read, time::Duration};
use tracing::error;
use uuid::Uuid;
use wascap::jwt::Claims;
use wascap::prelude::KeyPair;
use wasmcolonies_protocol::{
    deserialize, serialize, ColonyCommand, GameStateColonyView, PlayerTick, PlayerTickResponse,
    OP_PLAYER_TICK,
};

const URL_SCHEME: &str = "wasmbus";
const RPC_TIMEOUT_MILLIS: u64 = 1_000;

pub struct ColonyInvoker {
    nc: nats::Connection,
    hk: KeyPair,
}

impl ColonyInvoker {
    pub fn new(nc: nats::Connection) -> ColonyInvoker {
        ColonyInvoker {
            nc,
            hk: KeyPair::new_server(),
        }
    }

    pub fn fetch_commands(
        &self,
        player_id: &str,
        actor_key: &str,
        gs: GameStateColonyView,
    ) -> Result<Vec<ColonyCommand>> {
        let pt = PlayerTick {
            tick: 0,
            player_id: player_id.to_string(),
            game_state: Some(gs),
        };

        let inv = Invocation::new(
            &self.hk,
            Entity::Actor("system".to_string()),
            Entity::Actor(actor_key.to_string()),
            OP_PLAYER_TICK,
            serialize(pt).unwrap(),
        );
        let subject = &rpc_subject(None, &actor_key);
        let res = self.nc.request_timeout(
            subject,
            &serialize(inv).unwrap(),
            Duration::from_millis(RPC_TIMEOUT_MILLIS),
        );
        let tr: std::result::Result<PlayerTickResponse, std::io::Error> = res
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
                    deserialize::<PlayerTickResponse>(&ir.msg).map_err(|e| {
                        std::io::Error::new(std::io::ErrorKind::Other, format!("{}", e))
                    })
                }
            });
        Ok(match tr {
            Ok(tr) => tr.commands,
            Err(e) => {
                error!("{}", e);
                vec![ColonyCommand::Pass(0)]
            }
        })
    }
}

fn rpc_subject(prefix: Option<String>, actor: &str) -> String {
    format!(
        "wasmbus.rpc.{}.{}",
        prefix.as_ref().unwrap_or(&"default".to_string()),
        actor
    )
}

/// An immutable representation of an invocation within wasmcloud
#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Invocation {
    pub origin: Entity,
    pub target: Entity,
    pub operation: String,
    pub msg: Vec<u8>,
    pub id: String,
    pub encoded_claims: String,
    pub host_id: String,
}

impl Invocation {
    /// Creates a new invocation. All invocations are signed with the host key as a way
    /// of preventing them from being forged over the network when connected to a lattice,
    /// so an invocation requires a reference to the host (signing) key
    pub fn new(
        hostkey: &KeyPair,
        origin: Entity,
        target: Entity,
        op: &str,
        msg: Vec<u8>,
    ) -> Invocation {
        let subject = format!("{}", Uuid::new_v4());
        let issuer = hostkey.public_key();
        let target_url = format!("{}/{}", target.url(), op);
        let claims = Claims::<wascap::prelude::Invocation>::new(
            issuer.to_string(),
            subject.to_string(),
            &target_url,
            &origin.url(),
            &invocation_hash(&target_url, &origin.url(), &msg),
        );
        Invocation {
            origin,
            target,
            operation: op.to_string(),
            msg,
            id: subject,
            encoded_claims: claims.encode(&hostkey).unwrap(),
            host_id: issuer,
        }
    }
}

/// The response to an invocation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InvocationResponse {
    pub msg: Vec<u8>,
    pub error: Option<String>,
    pub invocation_id: String,
}

/// Represents an entity within the host runtime that can be the source
/// or target of an invocation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum Entity {
    Actor(String),
    Capability {
        id: String,
        contract_id: String,
        link_name: String,
    },
}

impl Entity {
    /// The URL of the entity
    pub fn url(&self) -> String {
        match self {
            Entity::Actor(pk) => format!("{}://{}", URL_SCHEME, pk),
            Entity::Capability {
                id,
                contract_id,
                link_name,
            } => format!(
                "{}://{}/{}/{}",
                URL_SCHEME,
                contract_id
                    .replace(":", "/")
                    .replace(" ", "_")
                    .to_lowercase(),
                link_name.replace(" ", "_").to_lowercase(),
                id
            ),
        }
    }
}

fn sha256_digest<R: Read>(
    mut reader: R,
) -> std::result::Result<Digest, Box<dyn std::error::Error>> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

pub(crate) fn invocation_hash(target_url: &str, origin_url: &str, msg: &[u8]) -> String {
    use std::io::Write;
    let mut cleanbytes: Vec<u8> = Vec::new();
    cleanbytes.write_all(origin_url.as_bytes()).unwrap();
    cleanbytes.write_all(target_url.as_bytes()).unwrap();
    cleanbytes.write_all(msg).unwrap();
    let digest = sha256_digest(cleanbytes.as_slice()).unwrap();
    HEXUPPER.encode(digest.as_ref())
}
