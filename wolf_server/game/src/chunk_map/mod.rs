use wolf_hash_map::WolfHashSet;

use crate::game::*;

pub struct ChunkMap<T> {
    chunk_coords_to_items: wolf_hash_map::WolfHashMap<TerrainChunkCoords, WolfHashSet<T>>,
    items_to_chunk_coords: wolf_hash_map::WolfHashMap<T, TerrainChunkCoords>,
}

impl<ItemType: Eq + std::hash::Hash + Clone> ChunkMap<ItemType> {
    pub fn new() -> Self {
        ChunkMap {
            chunk_coords_to_items: wolf_hash_map::WolfHashMap::new(),
            items_to_chunk_coords: wolf_hash_map::WolfHashMap::new(),
        }
    }
    pub fn insert(&mut self, item: ItemType, coords: TerrainChunkCoords) {
        let chunk_coords_to_items = self
            .chunk_coords_to_items
            .entry(coords)
            .or_insert_with(|| WolfHashSet::new());
        chunk_coords_to_items.insert(item.clone());
        self.items_to_chunk_coords.insert(item, coords);
    }
    pub fn get(&self, coords: TerrainChunkCoords) -> Option<&WolfHashSet<ItemType>> {
        self.chunk_coords_to_items.get(&coords)
    }
    pub fn remove(&mut self, item: ItemType) {
        if let Some(chunk_coords) = self.items_to_chunk_coords.remove(&item) {
            let mut chunk_entry = match self.chunk_coords_to_items.entry(chunk_coords) {
                std::collections::hash_map::Entry::Occupied(occ) => occ,
                _ => panic!("Item lacked place in chunk_map!"),
            };
            let should_remove = {
                let chunk = chunk_entry.get_mut();
                chunk.remove(&item);
                chunk.is_empty()
            };
            if should_remove {
                chunk_entry.remove();
            }
        }
    }
    pub fn move_item(&mut self, item: ItemType, coords: TerrainChunkCoords) {
        let old_coords = self.items_to_chunk_coords.get(&item).unwrap().clone();
        if old_coords == coords {
            return;
        }
        self.remove(item.clone());
        self.insert(item, coords);
    }
}
