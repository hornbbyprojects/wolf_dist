use super::Ability;
use crate::damage::{DamagerComponent, ProjectileComponent, LARGE_DAMAGE};
use crate::drawable::BasicDrawingComponent;
use crate::game::*;
use crate::generic::DeleteAtComponent;
use crate::timers::TimerSystem;

pub struct RailgunAbility {
    ability_id: AbilityId,
}

impl RailgunAbility {
    pub fn new(ability_id: AbilityId) -> Self {
        RailgunAbility { ability_id }
    }
}

pub const RAILGUN_PROJECTILE_DAMAGE: i32 = LARGE_DAMAGE;
pub const RAILGUN_PROJECTILE_SPEED: f64 = 50.0;
pub const RAILGUN_PROJECTILE_LIFETIME: u32 = 100;
pub const RAILGUN_DELAY: u32 = 15;

impl Ability for RailgunAbility {
    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }
    fn activate(&mut self, game: &mut Game, caster: GameObjectId, target_coords: PixelCoords) {
        let starting_coords = caster.get_coords_game(game);
        let game_object_id = GameObject::create_game(game, starting_coords);
        let angle = starting_coords.get_direction_to(&target_coords);
        BasicDrawingComponent::add_to(game, game_object_id, FIREBALL_SPRITE, PROJECTILE_DEPTH);
        let callback = Box::new(move |game: &mut Game| {
            let damager_id = DamagerComponent::add_to(
                game,
                game_object_id,
                caster,
                Some(1),
                RAILGUN_PROJECTILE_DAMAGE,
            )
            .damager_id;
            DeleteAtComponent::add_to(
                game,
                game_object_id,
                game.tick_counter + RAILGUN_PROJECTILE_LIFETIME,
            );
            ProjectileComponent::add_to(
                game,
                game_object_id,
                damager_id,
                angle,
                RAILGUN_PROJECTILE_SPEED,
            );
        });
        TimerSystem::add_timer(game, callback, RAILGUN_DELAY);
    }
}
