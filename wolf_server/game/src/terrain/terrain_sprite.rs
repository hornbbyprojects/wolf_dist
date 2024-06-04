use super::*;

pub struct TerrainSprite {
    coords: SquareCoords,
    chunk_load_listener_id: ChunkLoadListenerId,
    sprite: u32,
}
impl TerrainSprite {
    pub fn new(
        game: &mut Game,
        owner_id: GameObjectId,
        coords: SquareCoords,
        sprite: u32,
    ) -> TerrainSpriteId {
        let terrain_sprite_id = game.get_id();
        let chunk_load_listener_id =
            ChunkLoadListenerComponent::add_to(game, owner_id, coords.into())
                .chunk_load_listener_id;
        let terrain_sprite = TerrainSprite {
            coords,
            chunk_load_listener_id,
            sprite,
        };
        game.terrain
            .terrain_sprites
            .insert(terrain_sprite_id, terrain_sprite);
        terrain_sprite_id
    }
    pub fn remove(game: &mut Game, id: TerrainSpriteId) {
        if let Some(terrain_sprite) = game.terrain.terrain_sprites.remove(id) {
            let chunk_coords: TerrainChunkCoords = terrain_sprite.coords.into();
            let relative = terrain_sprite
                .coords
                .relative_to_chunk(chunk_coords)
                .unwrap();
            if let Some(chunk) = game.terrain.chunks.get_mut(&chunk_coords) {
                let index = get_chunk_index_from_relative_coords(relative);
                chunk.chunk_squares[index].sprites.remove(id);
                chunk.squares_to_redraw.push(relative);
            }
        }
    }
    pub fn step(game: &mut Game) {
        for (id, terrain_sprite) in game.terrain.terrain_sprites.iter() {
            let chunk_load_listener = game
                .terrain
                .chunk_load_listeners
                .get_mut(terrain_sprite.chunk_load_listener_id)
                .unwrap();
            if chunk_load_listener.loaded {
                chunk_load_listener.loaded = false;
                let chunk_coords: TerrainChunkCoords = terrain_sprite.coords.into();
                let relative = terrain_sprite
                    .coords
                    .relative_to_chunk(chunk_coords)
                    .unwrap();
                let index = get_chunk_index_from_relative_coords(relative);
                if let Some(chunk) = game.terrain.chunks.get_mut(&chunk_coords) {
                    chunk.chunk_squares[index]
                        .sprites
                        .insert(id, terrain_sprite.sprite);
                    chunk.squares_to_redraw.push(relative);
                }
            }
        }
    }
}
pub struct TerrainSpriteComponent {
    component_id: ComponentId,
    terrain_sprite_id: TerrainSpriteId,
}
#[inline]
pub fn get_chunk_index_from_relative_coords(coords: ChunkRelativeSquareCoords) -> usize {
    (coords.0.get_x() as usize)
        + (coords.0.get_y() as usize) * (TERRAIN_CHUNK_SIZE_SQUARES as usize)
}
impl TerrainSpriteComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        coords: SquareCoords,
        sprite: u32,
    ) -> ComponentId {
        let component_id = game.get_id();
        let terrain_sprite_id = TerrainSprite::new(game, owner_id, coords, sprite);
        let comp = TerrainSpriteComponent {
            component_id,
            terrain_sprite_id,
        };
        owner_id.add_component(game, comp);
        component_id
    }
}
impl Component for TerrainSpriteComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        TerrainSprite::remove(game, self.terrain_sprite_id)
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}
