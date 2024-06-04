use super::*;
use crate::abilities::Ability;

pub struct CorpseTossAbility {
    ability_id: AbilityId,
}

impl CorpseTossAbility {
    pub fn new(ability_id: AbilityId) -> Self {
        CorpseTossAbility { ability_id }
    }
}

impl Ability for CorpseTossAbility {
    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }
    fn activate(&mut self, game: &mut Game, caster: GameObjectId, target_coords: PixelCoords) {
        let starting_coords = caster.get_coords_game(game);
        let game_object_id = NecromancySystem::make_corpse(game, starting_coords);
        ArcingComponent::add_to(game, game_object_id, target_coords);
    }
}
