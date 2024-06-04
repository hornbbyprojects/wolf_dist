use wolf_hash_map::WolfHashSet;

use crate::biomes::generate_biome;
use crate::loading::LoadingSystem;
use crate::{game::*, time_system};

pub const LOAD_CHUNKS_WITHIN: i64 = 4;

pub struct ChunkLoader {
    pub loaded_chunks: WolfHashSet<TerrainChunkCoords>,
}
impl ChunkLoader {
    pub fn new() -> Self {
        ChunkLoader {
            loaded_chunks: WolfHashSet::new(),
        }
    }
}

pub fn notify_new_chunk(game: &mut Game, coords: TerrainChunkCoords) {
    if let Some(chunk_load_listeners) = game.terrain.chunk_load_listeners_by_chunk.get_mut(&coords)
    {
        for chunk_load_listener_id in chunk_load_listeners.iter() {
            let chunk_load_listener = game
                .terrain
                .chunk_load_listeners
                .get_mut(*chunk_load_listener_id)
                .unwrap();
            chunk_load_listener.loaded = true;
        }
    }
}
pub fn step_chunk_loaders(game: &mut Game) {
    let mut all_loaded_chunks = WolfHashSet::new();
    for (_id, chunk_loader) in game.terrain.chunk_loaders.iter() {
        all_loaded_chunks.extend(chunk_loader.loaded_chunks.iter().map(|x| *x));
    }
    let mut chunks_to_remove = Vec::new();
    let mut new_chunks_to_load = Vec::new();
    for (coords, _chunk) in game.terrain.chunks.iter() {
        if !all_loaded_chunks.contains(coords) {
            chunks_to_remove.push(*coords);
        }
    }
    for coords in all_loaded_chunks {
        time_system!(if game.terrain.chunks.contains_key(&coords) {
            continue;
        });
        new_chunks_to_load.push(coords);
    }
    for coords in chunks_to_remove {
        time_system!(unload_chunk(game, coords));
        game.terrain.chunks.remove(&coords);
    }
    for coords in new_chunks_to_load {
        generate_biome(game, coords);
        notify_new_chunk(game, coords);
    }
}

fn unload_chunk(game: &mut Game, coords: TerrainChunkCoords) {
    LoadingSystem::unload_chunk(game, coords);
}

pub struct BasicChunkLoader {
    game_object_id: GameObjectId,
}
impl BasicChunkLoader {
    pub fn step(game: &mut Game) {
        for (id, basic_chunk_loader) in game.terrain.basic_chunk_loaders.iter() {
            let owner_coords = basic_chunk_loader.game_object_id.get_chunk_coords(game);
            let chunk_loader = game.terrain.chunk_loaders.get_mut(id).unwrap();
            chunk_loader.loaded_chunks =
                square_of_coords_centered(owner_coords, LOAD_CHUNKS_WITHIN);
        }
    }
}
impl BasicChunkLoader {
    fn new(game: &mut Game, game_object_id: GameObjectId) -> ChunkLoaderId {
        let id = game.get_id();
        let basic_chunk_loader = BasicChunkLoader { game_object_id };
        game.terrain.chunk_loaders.insert(id, ChunkLoader::new());
        game.terrain
            .basic_chunk_loaders
            .insert(id, basic_chunk_loader);
        id
    }
    fn remove(game: &mut Game, id: ChunkLoaderId) {
        game.terrain.chunk_loaders.remove(id);
        game.terrain.basic_chunk_loaders.remove(id);
    }
}

pub struct BasicChunkLoaderComponent {
    component_id: ComponentId,
    chunk_loader_id: ChunkLoaderId,
}

impl BasicChunkLoaderComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let chunk_loader_id = BasicChunkLoader::new(game, owner_id);
        let component_id = game.get_id();
        let comp = BasicChunkLoaderComponent {
            component_id,
            chunk_loader_id,
        };
        owner_id.add_component(game, comp);
    }
}

impl Component for BasicChunkLoaderComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, _owner: GameObjectId) {
        BasicChunkLoader::remove(game, self.chunk_loader_id);
    }
}
