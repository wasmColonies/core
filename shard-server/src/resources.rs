use bevy::prelude::*;

use crate::construction::ConstructionSite;

#[derive(Debug, Clone)]
pub enum ResourceType {
    Wasmium,
}

#[derive(Debug, Clone)]
pub struct Mine {
    resource_type: ResourceType,
    /// The total amount of this resource that exists in the "resource deposit" underlying this mine
    deposit_qty: u32,
    /// Maximum amount of this resource this mine can hold at any time
    max_qty: u32,
    /// Amount of this resource available for pickup from the mine
    current_qty: u32,
    /// Units yielded per second
    yield_rate_ups: u32,
}

impl Mine {
    pub fn new(
        resource_type: ResourceType,
        max_qty: u32,
        deposit_qty: u32,
        yield_rate_ups: u32,
    ) -> Mine {
        Mine {
            deposit_qty,
            resource_type,
            max_qty,
            current_qty: 0,
            yield_rate_ups,
        }
    }
}

/// A mine is a component that will gradually store a resource that has been extracted from
/// an underlying resource deposit. During construction of a mine, the deposit goes away
// and is converted into a mine, hence the tracking of the original deposit quantity
pub fn mines(mut query: Query<(&mut Mine, &ConstructionSite)>) {
    for (mut mine, site) in query.iter_mut() {
        if site.progress < 100 {
            continue;
        }
        if mine.current_qty < mine.max_qty {
            mine.current_qty = mine
                .current_qty
                .saturating_add(mine.yield_rate_ups)
                .clamp(0, mine.max_qty);

            mine.deposit_qty = mine.deposit_qty.saturating_sub(mine.yield_rate_ups);

            if mine.deposit_qty == 0 {
                info!("Mine has been depleted");
            }
            if mine.current_qty == mine.max_qty {
                info!("Mine has reached capacity.");
            }
        }
    }
}
