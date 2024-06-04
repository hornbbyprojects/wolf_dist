use crate::{game::*, time_system};

mod chunk_loader;
pub use chunk_loader::*;
mod chunk;
pub use chunk::*;
mod chunk_watcher;
pub use chunk_watcher::*;
mod chunk_load_listener;
pub use chunk_load_listener::*;
mod terrain_sprite;
pub use terrain_sprite::*;
use wolf_hash_map::{WolfHashMap, WolfHashSet};

pub struct Terrain {
    pub chunks: wolf_hash_map::WolfHashMap<TerrainChunkCoords, Chunk>,
    pub chunk_loaders: IdMap<ChunkLoaderId, ChunkLoader>,
    pub basic_chunk_loaders: IdMap<ChunkLoaderId, BasicChunkLoader>,
    pub chunk_watchers: IdMap<ChunkWatcherId, ChunkWatcher>,
    pub chunk_load_listeners: IdMap<ChunkLoadListenerId, ChunkLoadListener>,
    pub chunk_load_listeners_by_chunk:
        WolfHashMap<TerrainChunkCoords, WolfHashSet<ChunkLoadListenerId>>,
    pub terrain_sprites: IdMap<TerrainSpriteId, TerrainSprite>,
}

impl Terrain {
    pub fn new() -> Terrain {
        Terrain {
            chunks: wolf_hash_map::WolfHashMap::new(),
            chunk_loaders: IdMap::new(),
            chunk_watchers: IdMap::new(),
            chunk_load_listeners: IdMap::new(),
            chunk_load_listeners_by_chunk: WolfHashMap::new(),
            terrain_sprites: IdMap::new(),
            basic_chunk_loaders: IdMap::new(),
        }
    }
}

impl Terrain {
    pub fn step(game: &mut Game) {
        time_system!(BasicChunkLoader::step(game));
        time_system!(step_chunk_loaders(game));
        time_system!(ChunkWatcher::step(game));
        time_system!(TerrainSprite::step(game));
    }
}
