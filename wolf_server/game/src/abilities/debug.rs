use super::Ability;
use crate::{game::*, quest::CrossDesertQuest};

pub struct PlaneWalkAbility {
    ability_id: AbilityId,
}

impl PlaneWalkAbility {
    pub fn new(ability_id: AbilityId) -> Self {
        PlaneWalkAbility { ability_id }
    }
}

impl Ability for PlaneWalkAbility {
    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }

    fn activate(&mut self, game: &mut Game, caster: GameObjectId, target_coords: PixelCoords) {
        let starting_coords = caster.get_coords_game(game);
        if starting_coords.get_plane() == Plane(0) {
            caster.move_to_game(game, starting_coords.set_plane(Plane(1)));
        } else {
            caster.move_to_game(game, starting_coords.set_plane(Plane(0)));
        }
    }
}

pub struct DebugAbility {
    ability_id: AbilityId,
}

impl DebugAbility {
    pub fn new(ability_id: AbilityId) -> Self {
        DebugAbility { ability_id }
    }
}

impl Ability for DebugAbility {
    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }
    fn activate(&mut self, game: &mut Game, caster: GameObjectId, target_coords: PixelCoords) {
        CrossDesertQuest::begin(game, caster);
    }
}
