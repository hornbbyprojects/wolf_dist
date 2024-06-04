use crate::id_types::*;
use id::*;

mod ability_ids;
pub use ability_ids::*;

mod fireball;
pub use fireball::*;

mod necrobolt;
pub use necrobolt::*;

mod railgun;
pub use railgun::*;

mod cloak;
pub use cloak::*;

mod ambush;
pub use ambush::*;

mod sprint;
pub use sprint::*;

mod ability;
pub use ability::*;

mod ability_user;
pub use ability_user::*;

mod toggled;
pub use toggled::*;

mod spellbook;
pub use spellbook::*;

mod harvest;
pub use harvest::*;

mod slot_mapping;
pub use slot_mapping::*;

mod debug;
pub use debug::*;

use crate::game::*;

pub struct AbilitySystem {
    pub basic_ability_users: IdMap<BasicAbilityUserId, BasicAbilityUser>,
    pub spellbook_system: SpellbookSystem,
    pub harvesters: IdMap<HarvesterId, Harvester>,
}

impl AbilitySystem {
    pub fn new() -> Self {
        AbilitySystem {
            basic_ability_users: IdMap::new(),
            spellbook_system: SpellbookSystem::new(),
            harvesters: IdMap::new(),
        }
    }
    pub fn step(game: &mut Game) {
        SpellbookSystem::step(game);
        Harvester::step(game);
    }
}
