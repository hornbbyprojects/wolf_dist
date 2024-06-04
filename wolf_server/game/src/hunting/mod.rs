use crate::allegiance::AllegianceSystem;
use crate::chunk_map::ChunkMap;
use crate::game::*;

mod basic_hunter_behaviour;
pub use basic_hunter_behaviour::*;
mod prey;
pub use prey::*;

pub struct HuntingSystem {
    pub preys: IdMap<PreyId, Prey>,
    pub prey_chunk_map: ChunkMap<PreyId>,
}

impl HuntingSystem {
    pub fn new() -> Self {
        HuntingSystem {
            preys: IdMap::new(),
            prey_chunk_map: ChunkMap::new(),
        }
    }
    pub fn get_closest_prey(game: &Game, hunter: GameObjectId, radius: f64) -> Option<PreyId> {
        let hunter_coords = hunter.get_coords_game(game);
        let hunter_chunk_coords: TerrainChunkCoords = hunter_coords.into();

        let chunk_radius = radius as i64 / TERRAIN_CHUNK_SIZE_PIXELS;
        let chunks = square_of_coords_centered(hunter_chunk_coords, chunk_radius);

        let mut current_closest_prey_id = None;
        let mut current_distance = 0.0;

        let hunter_allegiances = hunter.get_allegiances(game);

        for chunk_coords in chunks {
            if let Some(preys) = game.hunting_system.prey_chunk_map.get(chunk_coords) {
                for prey_id in preys.iter() {
                    let prey = game.hunting_system.preys.get(*prey_id).unwrap();
                    let prey_coords = prey.game_object_id.get_coords_game(game);
                    let distance = prey_coords.get_distance_to(&hunter_coords);
                    if distance > radius {
                        continue;
                    }
                    if current_closest_prey_id.is_some() {
                        if distance > current_distance {
                            continue;
                        }
                    }
                    let prey_allegiances = prey.game_object_id.get_allegiances(game);
                    if !AllegianceSystem::can_hurt_multiple(
                        &game.allegiance_system,
                        &hunter_allegiances,
                        &prey_allegiances,
                    ) {
                        continue;
                    }
                    current_closest_prey_id = Some(*prey_id);
                    current_distance = distance;
                }
            }
        }
        current_closest_prey_id
    }
}
