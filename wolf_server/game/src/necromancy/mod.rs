use crate::drawable::BasicDrawingComponent;
use crate::game::*;

mod corpse;
pub use corpse::*;
mod corpse_on_death;
pub use corpse_on_death::*;
mod necrobolt;
pub use necrobolt::*;
mod corpse_toss;
pub use corpse_toss::*;

pub struct NecromancySystem {
    pub necrobolts: IdMap<NecroboltId, Necrobolt>,
}

impl NecromancySystem {
    pub fn new() -> Self {
        NecromancySystem {
            necrobolts: IdMap::new(),
        }
    }
    pub fn make_corpse(game: &mut Game, coords: PixelCoords) -> GameObjectId {
        let game_object_id = GameObject::create_game(game, coords);
        CorpseComponent::add_to(game, game_object_id);
        BasicDrawingComponent::add_to(game, game_object_id, CORPSE_SPRITE, CORPSE_DEPTH);
        game_object_id
    }
    pub fn step(game: &mut Game) {
        Necrobolt::step(game);
    }
}
