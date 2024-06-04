use crate::ai::AiComponent;
use crate::allegiance::AllegianceComponent;
use crate::damage::DieOnNoHealthComponent;
use crate::damage::{DamageableComponent, DamagerComponent, DeleteOnDeathComponent, LARGE_DAMAGE};
use crate::drawable::BasicDrawingComponent;
use crate::game::*;
use crate::hunting::BasicHunterBehaviourComponent;
use crate::loot::LootComponent;
use crate::necromancy::CorpseOnDeathComponent;

pub const ZOMBIE_DAMAGE: i32 = LARGE_DAMAGE / 30;
pub const ZOMBIE_SPEED: f64 = 2.0;

pub fn add_zombie(game: &mut Game, coords: PixelCoords) {
    let game_object_id = GameObject::create_game(game, coords);
    DamageableComponent::add_to(game, game_object_id);
    BasicDrawingComponent::add_to(game, game_object_id, ZOMBIE_SPRITE, DEFAULT_DEPTH);
    DamagerComponent::add_to(game, game_object_id, game_object_id, None, ZOMBIE_DAMAGE);
    BasicHunterBehaviourComponent::add_to(game, game_object_id);
    AiComponent::add_to(game, game_object_id);
    WalkerComponent::add_to(game, game_object_id, ZOMBIE_SPEED, ZOMBIE_SPEED / 4.0);
    add_health_bar(game, game_object_id);
    DieOnNoHealthComponent::add_to(game, game_object_id);
    DeleteOnDeathComponent::add_to(game, game_object_id);
    let allegiance_id = game.allegiance_system.special_allegiances.undead_allegiance;
    AllegianceComponent::add_to(game, game_object_id, vec![allegiance_id]);
    CorpseOnDeathComponent::add_to(game, game_object_id);
    LootComponent::add_to(game, game_object_id);
}
