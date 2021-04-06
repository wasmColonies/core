#[macro_use]
extern crate eventsourcing_derive;

mod construction;
mod events;

pub const DOMAIN_VERSION: &str = "1.0";

pub use events::*;
