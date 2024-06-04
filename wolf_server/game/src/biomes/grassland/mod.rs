use crate::damage::*;
use crate::drawable::BasicDrawingComponent;
use crate::game::*;
use crate::loading::ChunkDependentComponent;
use crate::resources::*;
use crate::solids::SolidComponent;
use crate::terrain::Chunk;
use crate::terrain::ChunkComponent;
use crate::villages::NeedsDeconstructionComponent;
use crate::wildlife::create_hopper_creature;

const HOPPER_CREATURE_SPAWN_CHANCE: i64 = 50 * 6000;

pub struct GrassBiome {
    coords: TerrainChunkCoords,
}

impl GrassBiome {
    pub fn new(game: &mut Game, coords: TerrainChunkCoords) -> GrassBiomeId {
        let grass_biome_id = game.get_id();
        game.biome_system
            .grass_biomes
            .insert(grass_biome_id, GrassBiome { coords });
        grass_biome_id
    }
    pub fn remove(game: &mut Game, id: GrassBiomeId) {
        game.biome_system.grass_biomes.remove(id);
    }
}

pub struct GrassBiomeComponent {
    component_id: ChunkComponentId,
    grass_biome_id: GrassBiomeId,
}

impl ChunkComponent for GrassBiomeComponent {
    fn get_component_id(&self) -> ChunkComponentId {
        self.component_id
    }

    fn on_remove(&mut self, game: &mut Game, coords: TerrainChunkCoords) {
        GrassBiome::remove(game, self.grass_biome_id);
    }
}
impl GrassBiomeComponent {
    fn add_to(game: &mut Game, coords: TerrainChunkCoords) {
        let component_id = game.get_id();
        let grass_biome_id = GrassBiome::new(game, coords);
        let comp = GrassBiomeComponent {
            component_id,
            grass_biome_id,
        };
        Chunk::add_component(game, coords, comp);
    }
}

pub fn add_tree(game: &mut Game, coords: PixelCoords) {
    if game.villages_system.is_reserved(coords.into()) {
        return;
    }
    let game_object_id = GameObject::create_game(game, coords);
    BasicDrawingComponent::add_to(game, game_object_id, TREE_SPRITE, DEFAULT_DEPTH);
    SolidComponent::add_to(game, game_object_id);
    ChunkDependentComponent::add_to(game, game_object_id, coords.into());
    HarvestableComponent::add_to(game, game_object_id, Resources::wood(ResourceAmount(1)));
    DamageableComponent::add_to(game, game_object_id);
    DieOnNoHealthComponent::add_to(game, game_object_id);
    DeleteOnDeathComponent::add_to(game, game_object_id);
    NeedsDeconstructionComponent::add_to(game, game_object_id);
}

impl GrassBiome {
    pub fn generate(game: &mut Game, coords: TerrainChunkCoords) {
        Chunk::generate(game, coords, GRASS_SPRITE);
        GrassBiomeComponent::add_to(game, coords);
        for _ in 0..10 {
            let x = rand::thread_rng().gen_range(0..TERRAIN_CHUNK_SIZE_PIXELS);
            let y = rand::thread_rng().gen_range(0..TERRAIN_CHUNK_SIZE_PIXELS);
            let pixel_offset = PixelCoords::new_to_fixed(coords.get_plane(), x, y);
            add_tree(game, coords.pixel_offset(pixel_offset));
        }
    }

    pub fn step(game: &mut Game) {
        let mut to_spawn = Vec::new();
        for (_id, grass_biome) in game.biome_system.grass_biomes.iter() {
            if rand::thread_rng().gen_range(0..HOPPER_CREATURE_SPAWN_CHANCE) == 0 {
                let dx = rand::thread_rng()
                    .gen_range(-TERRAIN_CHUNK_SIZE_PIXELS / 2..TERRAIN_CHUNK_SIZE_PIXELS / 2);
                let dy = rand::thread_rng()
                    .gen_range(-TERRAIN_CHUNK_SIZE_PIXELS / 2..TERRAIN_CHUNK_SIZE_PIXELS / 2);
                let spawn_at = grass_biome.coords.center_pixel().translate(dx, dy);
                to_spawn.push(spawn_at);
            }
        }
        for spawn_at in to_spawn {
            create_hopper_creature(game, spawn_at);
        }
    }
}
