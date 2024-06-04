use crate::basic_client_side_component::*;
use crate::game::*;
use wolf_interface::*;

pub const DEFAULT_DEPTH: i32 = 0;
pub const PROJECTILE_DEPTH: i32 = -1;
pub const CORPSE_DEPTH: i32 = 1;

mod basic_drawing_component;
pub use basic_drawing_component::*;

pub fn add_random_colour(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
    let r = rand::thread_rng().gen_range(0..255);
    let g = rand::thread_rng().gen_range(0..255);
    let b = rand::thread_rng().gen_range(0..255);
    add_colour(game, owner_id, r, g, b)
}
pub fn add_colour(game: &mut Game, owner_id: GameObjectId, r: u8, g: u8, b: u8) -> ComponentId {
    BasicClientSideComponent::add_to(
        game,
        owner_id,
        CreateComponentData::Coloured(CreateColouredData { r, g, b }),
    )
    .component_id
}

pub fn add_health_bar(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
    BasicClientSideComponent::add_to(game, owner_id, CreateComponentData::HealthBar).component_id
}
