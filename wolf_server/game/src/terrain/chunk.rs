use crate::game::*;
use wolf_hash_map::WolfHashSet;
use wolf_interface::*;

pub trait ChunkComponent {
    fn get_component_id(&self) -> ChunkComponentId;
    fn on_remove(&mut self, game: &mut Game, coords: TerrainChunkCoords);
}

pub struct ChunkSquare {
    pub base_sprite: u32,
    pub detritus: Vec<u32>, // More aesthetic only things on top of the base sprite
    pub base_solid: bool,
    pub sprites: IdMap<TerrainSpriteId, u32>,
    pub solids: WolfHashSet<TerrainSpriteId>,
}
impl ChunkSquare {
    pub fn to_sprites_vec(&self) -> Vec<u32> {
        (self
            .detritus
            .iter()
            .map(|s| *s)
            .chain(self.sprites.iter().map(|(_, s)| *s)))
        .collect()
    }
    pub fn is_solid(&self) -> bool {
        self.base_solid || !self.solids.is_empty()
    }
}

pub struct Chunk {
    pub base_sprite: u32,
    pub chunk_squares: Vec<ChunkSquare>,
    pub squares_to_redraw: Vec<ChunkRelativeSquareCoords>,
    pub chunk_components: IdMap<ChunkComponentId, Box<dyn ChunkComponent>>,
}

impl Chunk {
    pub fn new(base_sprite: u32) -> Self {
        let mut chunk_squares = Vec::new();
        for _i in 0..TERRAIN_CHUNK_SIZE_SQUARES * TERRAIN_CHUNK_SIZE_SQUARES {
            chunk_squares.push(ChunkSquare {
                base_sprite,
                detritus: Vec::new(),
                base_solid: false,
                solids: WolfHashSet::new(),
                sprites: IdMap::new(),
            });
        }
        let chunk = Chunk {
            base_sprite,
            chunk_squares,
            squares_to_redraw: Vec::new(),
            chunk_components: IdMap::new(),
        };
        chunk
    }
    pub fn add_component<T: ChunkComponent + 'static>(
        game: &mut Game,
        chunk_coords: TerrainChunkCoords,
        component: T,
    ) {
        let chunk = game
            .terrain
            .chunks
            .get_mut(&chunk_coords)
            .expect("Attempted to add component to unloaded chunk!");
        chunk
            .chunk_components
            .insert(component.get_component_id(), Box::new(component));
    }
    pub fn generate(game: &mut Game, coords: TerrainChunkCoords, base_sprite: u32) {
        let chunk = Self::new(base_sprite);
        game.terrain.chunks.insert(coords, chunk);
    }
    pub fn get_base_info_message(&self) -> BaseChunkMessage {
        let mut terrain: Vec<Vec<u32>> = Vec::new();
        for square_sprites in self.chunk_squares.iter() {
            let mut sprites_vec = square_sprites.to_sprites_vec();
            if square_sprites.base_sprite != self.base_sprite {
                sprites_vec.insert(0, square_sprites.base_sprite);
            }
            terrain.push(sprites_vec);
        }
        BaseChunkMessage {
            base_sprite: self.base_sprite,
            terrain,
        }
    }
    pub fn get_info_message(&self, coords: TerrainChunkCoords) -> ChunkInfoMessage {
        let base = self.get_base_info_message();
        ChunkInfoMessage { coords, base }
    }
}
