use super::*;
use crate::drawable::BasicDrawingComponent;
use crate::generic::DeleteAtComponent;

pub fn melee_attack(
    game: &mut Game,
    attacker_id: GameObjectId,
    sprite: u32,
    damage: i32,
    hits: Option<u32>,
    starting_coords: PixelCoords,
    rotation: Angle,
    range: f64,
    lifetime: u32,
) -> GameObjectId {
    let spawn_point = starting_coords.offset_direction(rotation, range);
    let game_object_id = GameObject::create_game(game, spawn_point);
    game_object_id.set_rotation(game, rotation);
    DamagerComponent::add_to(game, game_object_id, attacker_id, hits, damage);
    BasicDrawingComponent::add_to(game, game_object_id, sprite, PROJECTILE_DEPTH);
    DeleteAtComponent::add_to(game, game_object_id, game.tick_counter + lifetime);
    game_object_id
}
