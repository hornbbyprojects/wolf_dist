use super::Ability;
use crate::damage::{ProjectileComponent, SMALL_DAMAGE};
use crate::game::*;
use crate::necromancy::NecroboltComponent;

pub struct NecroboltAbility {
    ability_id: AbilityId,
}

impl NecroboltAbility {
    pub fn new(ability_id: AbilityId) -> Self {
        NecroboltAbility { ability_id }
    }
}

pub const NECROBOLT_PROJECTILE_DAMAGE: i32 = SMALL_DAMAGE;
pub const NECROBOLT_PROJECTILE_SPEED: f64 = 10.0;
pub const NECROBOLT_PROJECTILE_LIFETIME: u32 = 100;

impl Ability for NecroboltAbility {
    fn activate(&mut self, game: &mut Game, caster: GameObjectId, target_coords: PixelCoords) {
        let starting_coords = caster.get_coords_game(game);
        let angle = starting_coords.get_direction_to(&target_coords);
        let game_object_id = ProjectileComponent::fire_basic_projectile(
            game,
            caster,
            NECROBOLT_SPRITE,
            NECROBOLT_PROJECTILE_DAMAGE,
            None,
            starting_coords,
            angle,
            NECROBOLT_PROJECTILE_SPEED,
            NECROBOLT_PROJECTILE_LIFETIME,
        );
        NecroboltComponent::add_to(game, game_object_id);
    }

    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }
}
