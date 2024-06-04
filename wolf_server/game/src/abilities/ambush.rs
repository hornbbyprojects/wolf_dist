use super::Ability;
use crate::damage::*;
use crate::damage::{melee_attack, MEDIUM_DAMAGE};
use crate::game::*;
use crate::statuses::BleedAttackComponent;

pub struct AmbushAbility {
    ability_id: AbilityId,
}

impl AmbushAbility {
    pub fn new(ability_id: AbilityId) -> Self {
        AmbushAbility { ability_id }
    }
}

pub const AMBUSH_ATTACK_DAMAGE: i32 = MEDIUM_DAMAGE;
pub const AMBUSH_RANGE: f64 = 20.0;
pub const AMBUSH_ATTACK_LIFETIME: u32 = 10;

impl Ability for AmbushAbility {
    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }
    fn activate(&mut self, game: &mut Game, caster: GameObjectId, target_coords: PixelCoords) {
        let starting_coords = caster.get_coords_game(game);
        let angle = starting_coords.get_direction_to(&target_coords);
        let game_object_id = melee_attack(
            game,
            caster,
            AMBUSH_SPRITE,
            AMBUSH_ATTACK_DAMAGE,
            Some(1),
            starting_coords,
            angle,
            AMBUSH_RANGE,
            AMBUSH_ATTACK_LIFETIME,
        );
        BleedAttackComponent::add_to(game, game_object_id, Health(MEDIUM_DAMAGE));
    }
}
