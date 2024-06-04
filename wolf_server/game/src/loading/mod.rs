use crate::game::*;

struct ChunkDependent {
    chunk_coords: TerrainChunkCoords,
    deloaded: bool,
}

impl ChunkDependent {
    pub fn new(
        game: &mut Game,
        game_object_id: GameObjectId,
        chunk_coords: TerrainChunkCoords,
    ) -> ChunkDependentId {
        let id = game.get_id();
        let chunk_dependent = ChunkDependent {
            chunk_coords,
            deloaded: false,
        };
        game.loading_system
            .chunk_dependents
            .insert(id, chunk_dependent);
        let container = game
            .loading_system
            .chunk_dependents_by_chunk
            .entry(chunk_coords)
            .or_insert(ChunkDependentsContainer::new());
        container.game_object_ids.push(game_object_id);
        container.chunk_dependent_ids.push(id);
        id
    }
    fn remove(game: &mut Game, id: ChunkDependentId) {
        if let Some(chunk_dependent) = game.loading_system.chunk_dependents.remove(id) {
            if !chunk_dependent.deloaded {
                let mut container_entry = match game
                    .loading_system
                    .chunk_dependents_by_chunk
                    .entry(chunk_dependent.chunk_coords)
                {
                    std::collections::hash_map::Entry::Occupied(x) => x,
                    _ => panic!("chunk_dependent not deloaded with no chunk"),
                };
                let remove = {
                    let container = container_entry.get_mut();
                    let mut index = 0;
                    loop {
                        if index > container.chunk_dependent_ids.len() {
                            panic!("chunkdependent missing place in chunk lookup!");
                        }
                        if container.chunk_dependent_ids[index] == id {
                            break;
                        };
                        index += 1;
                    }
                    container.remove(index);
                    container.game_object_ids.is_empty()
                };
                if remove {
                    container_entry.remove();
                }
            }
        }
    }
}
struct ChunkDependentsContainer {
    game_object_ids: Vec<GameObjectId>,
    chunk_dependent_ids: Vec<ChunkDependentId>,
}

impl ChunkDependentsContainer {
    fn new() -> Self {
        ChunkDependentsContainer {
            game_object_ids: Vec::new(),
            chunk_dependent_ids: Vec::new(),
        }
    }
    fn remove(&mut self, index: usize) {
        self.game_object_ids.remove(index);
        self.chunk_dependent_ids.remove(index);
    }
}

#[derive(Clone)]
pub struct ChunkDependentComponent {
    component_id: ComponentId,
    chunk_dependent_id: ChunkDependentId,
}

impl Component for ChunkDependentComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        ChunkDependent::remove(game, self.chunk_dependent_id);
    }
}
impl ChunkDependentComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId, chunk_coords: TerrainChunkCoords) {
        let chunk_dependent_id = ChunkDependent::new(game, owner_id, chunk_coords);
        let component_id = game.get_id();
        let comp = ChunkDependentComponent {
            component_id,
            chunk_dependent_id,
        };
        owner_id.add_component(game, comp);
    }
}
pub struct LoadingSystem {
    chunk_dependents: IdMap<ChunkDependentId, ChunkDependent>,
    chunk_dependents_by_chunk:
        wolf_hash_map::WolfHashMap<TerrainChunkCoords, ChunkDependentsContainer>,
}

impl LoadingSystem {
    pub fn unload_chunk(game: &mut Game, chunk_coords: TerrainChunkCoords) {
        if let Some(chunk_dependents_container) = game
            .loading_system
            .chunk_dependents_by_chunk
            .remove(&chunk_coords)
        {
            for index in 0..chunk_dependents_container.game_object_ids.len() {
                //problem: will cause wasteful removing because of ChunkDependentComponent
                game.loading_system
                    .chunk_dependents
                    .get_mut(chunk_dependents_container.chunk_dependent_ids[index])
                    .unwrap()
                    .deloaded = true;
                chunk_dependents_container.game_object_ids[index].remove(game);
            }
        }

        let chunk_components = std::mem::replace(
            &mut game
                .terrain
                .chunks
                .get_mut(&chunk_coords)
                .unwrap()
                .chunk_components,
            IdMap::new(),
        );
        for (_id, mut component) in chunk_components {
            component.on_remove(game, chunk_coords);
        }
        game.terrain.chunks.remove(&chunk_coords);
    }
    pub fn new() -> Self {
        LoadingSystem {
            chunk_dependents: IdMap::new(),
            chunk_dependents_by_chunk: wolf_hash_map::WolfHashMap::new(),
        }
    }
}
