use crate::ColonyEvent;
use eventsourcing::{Aggregate, AggregateState};
use wasmcolonies_protocol::{ColonyCommand, UnitType};

pub struct ConstructionSite;
impl Aggregate for ConstructionSite {
    type Event = ColonyEvent;
    type Command = ColonyCommand;
    type State = ConstructionSiteData;

    fn apply_event(state: &Self::State, evt: &Self::Event) -> eventsourcing::Result<Self::State> {
        Ok(match evt {
            ColonyEvent::TickFinished(_t) => ConstructionSiteData {
                remaining: state.remaining.saturating_sub(1),
                ..state.clone()
            },
            ColonyEvent::UnitConstructionBegan {
                tick,
                utype,
                yield_in,
            } => ConstructionSiteData {
                began: *tick,
                yields: utype.clone(),
                remaining: *yield_in,
                ..state.clone()
            },
            _ => state.clone(),
        })
    }

    fn handle_command(
        _state: &Self::State,
        cmd: &Self::Command,
    ) -> eventsourcing::Result<Vec<Self::Event>> {
        Ok(match cmd {
            ColonyCommand::ConstructUnit(tick, ut) => {
                vec![ColonyEvent::UnitConstructionBegan {
                    tick: *tick,
                    utype: ut.clone(),
                    yield_in: 1000, // TODO: TURN THIS INTO A GAME PARAMETER
                }]
            }
            _ => vec![],
        })
    }
}

#[derive(Debug, Clone)]
pub struct ConstructionSiteData {
    id: u64,
    began: u64,
    generation: u64,
    remaining: u64,
    yields: UnitType,
}

impl AggregateState for ConstructionSiteData {
    fn generation(&self) -> u64 {
        self.generation
    }
}
