use wolf_hash_map::WolfHashSet;

use crate::game::*;

pub struct ChunkLoadListener {
    pub coords: TerrainChunkCoords,
    pub loaded: bool,
}

impl ChunkLoadListener {
    pub fn new(game: &mut Game, coords: TerrainChunkCoords) -> ChunkLoadListenerId {
        let chunk_load_listener_id = game.get_id();
        let loaded = game.terrain.chunks.contains_key(&coords);
        let chunk_load_listener = ChunkLoadListener { coords, loaded };
        game.terrain
            .chunk_load_listeners
            .insert(chunk_load_listener_id, chunk_load_listener);
        let by_chunk = game
            .terrain
            .chunk_load_listeners_by_chunk
            .entry(coords)
            .or_insert_with(WolfHashSet::new);
        by_chunk.insert(chunk_load_listener_id);
        chunk_load_listener_id
    }
    pub fn remove(game: &mut Game, id: ChunkLoadListenerId) {
        if let Some(chunk_load_listener) = game.terrain.chunk_load_listeners.get(id) {
            match game
                .terrain
                .chunk_load_listeners_by_chunk
                .entry(chunk_load_listener.coords)
            {
                hash_map::Entry::Occupied(mut occ) => {
                    let by_chunk = occ.get_mut();
                    by_chunk.remove(&id);
                    if by_chunk.is_empty() {
                        occ.remove();
                    }
                }
                hash_map::Entry::Vacant(_) => {
                    panic!("Chunk load listener missing chunk mapping");
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct ChunkLoadListenerComponent {
    pub component_id: ComponentId,
    pub chunk_load_listener_id: ChunkLoadListenerId,
}

impl Component for ChunkLoadListenerComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        ChunkLoadListener::remove(game, self.chunk_load_listener_id);
    }
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}

impl ChunkLoadListenerComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        coords: TerrainChunkCoords,
    ) -> ChunkLoadListenerComponent {
        let component_id = game.get_id();
        let chunk_load_listener_id = ChunkLoadListener::new(game, coords);
        let comp = ChunkLoadListenerComponent {
            component_id,
            chunk_load_listener_id,
        };
        owner_id.add_component(game, comp.clone());
        comp
    }
}
