use std::collections::hash_map::Entry;
use std::hash::Hash;
use wolf_hash_map::WolfHashSet;
mod collision_map;
mod spatial_mappable;
mod utilities;

use self::utilities::*;

pub use collision_map::*;
use coords::*;
pub use spatial_mappable::*;

use crate::game::Game;

pub struct SpatialMap<ItemType> {
    chunks_to_items: wolf_hash_map::WolfHashMap<TerrainChunkCoords, WolfHashSet<ItemType>>,
    items_to_chunks: wolf_hash_map::WolfHashMap<ItemType, WolfHashSet<TerrainChunkCoords>>,
}

//Creation
impl<ItemType: SpatialMappable + Eq + Hash> SpatialMap<ItemType> {
    pub fn new() -> Self {
        SpatialMap {
            chunks_to_items: wolf_hash_map::WolfHashMap::new(),
            items_to_chunks: wolf_hash_map::WolfHashMap::new(),
        }
    }
}

//Adding/removing items
impl<ItemType: SpatialMappable + Eq + Hash + Clone> SpatialMap<ItemType> {
    pub fn add(&mut self, game: &Game, new_item: ItemType) {
        let overlapping_chunks = new_item.get_overlapping_chunks(game);
        for chunk in overlapping_chunks.iter() {
            insert_lazy(&mut self.chunks_to_items, chunk.clone(), new_item.clone())
        }
        self.items_to_chunks.insert(new_item, overlapping_chunks);
    }
    pub fn remove(&mut self, item: ItemType) {
        let old_chunks_entry = self.items_to_chunks.entry(item.clone());
        match old_chunks_entry {
            Entry::Occupied(old_chunks_occupied_entry) => {
                let old_chunks = old_chunks_occupied_entry.remove();
                for chunk in old_chunks {
                    remove_lazy(&mut self.chunks_to_items, chunk, &item);
                }
            }
            _ => {}
        }
    }
    pub fn reserve(&mut self, additional: usize) {
        self.items_to_chunks.reserve(additional);
    }
    pub fn is_empty(&self) -> bool {
        self.items_to_chunks.is_empty()
    }
}

//Moving items
impl<ItemType: SpatialMappable + Hash + Eq + Clone> SpatialMap<ItemType> {
    pub fn move_item(&mut self, game: &Game, item: ItemType) {
        let new_chunks = item.get_overlapping_chunks(game);
        let old_chunks_entry = self.items_to_chunks.entry(item.clone());
        if let Entry::Occupied(mut old_chunks_occupied_entry) = old_chunks_entry {
            let old_chunks = old_chunks_occupied_entry.get_mut();
            if new_chunks.eq(old_chunks) {
                return;
            }
            let added_chunks = new_chunks.difference(&old_chunks);
            for chunk in added_chunks {
                insert_lazy(&mut self.chunks_to_items, *chunk, item.clone());
            }
            let removed_chunks = old_chunks.difference(&new_chunks);
            for chunk in removed_chunks {
                remove_lazy(&mut self.chunks_to_items, *chunk, &item);
            }
        }
        self.items_to_chunks.insert(item.clone(), new_chunks);
    }
}

//getting items
impl<ItemType: SpatialMappable + Eq + Hash + Clone> SpatialMap<ItemType> {
    //guaranteed to include all within box, not to exclude outsiders
    pub fn get_within_box(
        &self,
        center_coords: SquareCoords,
        box_width: i64,
        box_height: i64,
    ) -> WolfHashSet<ItemType> {
        let mut ret = WolfHashSet::new();

        let top_left: TerrainChunkCoords = center_coords.translate(-box_width, -box_height).into();
        let bottom_right: TerrainChunkCoords =
            center_coords.translate(box_width, box_height).into();
        let chunk_coords: WolfHashSet<TerrainChunkCoords> =
            square_of_coords(top_left, bottom_right);

        for chunk in chunk_coords {
            if let Some(items) = self.chunks_to_items.get(&chunk) {
                for item in items.iter() {
                    ret.insert(item.clone());
                }
            }
        }
        ret
    }
}
