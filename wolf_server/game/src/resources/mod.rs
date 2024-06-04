use crate::game::*;

mod resources;
pub use resources::*;

mod resource_pickup;
pub use resource_pickup::*;

mod harvestable;
pub use harvestable::*;

mod resource_holder;
pub use resource_holder::*;

mod resource_collector;
pub use resource_collector::*;

mod resource_dropper;
pub use resource_dropper::*;

pub struct ResourceSystem {
    pub harvestables: IdMap<HarvestableId, Harvestable>,

    pub resource_collectors: IdMap<ResourceCollectorId, ResourceCollector>,

    pub resource_holders: IdMap<ResourceHolderId, ResourceHolder>,
}

impl ResourceSystem {
    pub fn new() -> Self {
        ResourceSystem {
            harvestables: IdMap::new(),

            resource_collectors: IdMap::new(),

            resource_holders: IdMap::new(),
        }
    }
    pub fn step(game: &mut Game) {
        ResourceCollector::step(game);
    }
}
