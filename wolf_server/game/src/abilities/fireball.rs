use super::*;
use crate::damage::{ProjectileComponent, MEDIUM_DAMAGE};
use crate::game::*;

pub struct FireballAbility {
    ability_id: AbilityId,
}

impl FireballAbility {
    pub fn new(ability_id: AbilityId) -> Self {
        FireballAbility { ability_id }
    }
}

pub const FIREBALL_PROJECTILE_DAMAGE: i32 = MEDIUM_DAMAGE;
pub const FIREBALL_PROJECTILE_SPEED: f64 = 10.0;
pub const FIREBALL_PROJECTILE_LIFETIME: u32 = 100;

impl Ability for FireballAbility {
    fn activate(&mut self, game: &mut Game, caster: GameObjectId, target_coords: PixelCoords) {
        let starting_coords = caster.get_coords_game(game);
        let angle = starting_coords.get_direction_to(&target_coords);
        ProjectileComponent::fire_basic_projectile(
            game,
            caster,
            FIREBALL_SPRITE,
            FIREBALL_PROJECTILE_DAMAGE,
            Some(1),
            starting_coords,
            angle,
            FIREBALL_PROJECTILE_SPEED,
            FIREBALL_PROJECTILE_LIFETIME,
        );
    }
    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }
    fn get_details(&self) -> super::AbilityDetails {
        AbilityDetails::default().attack_ability()
    }
}
