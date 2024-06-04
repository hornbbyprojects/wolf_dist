use crate::terrain::get_chunk_index_from_relative_coords;
use crate::{game::*, terrain::Chunk};

mod grassland;
use grassland::*;
use noise::NoiseFn;
use noise::Perlin;
pub mod caves;
pub mod ravine;

pub struct BiomeSystem {
    pub grass_biomes: IdMap<GrassBiomeId, GrassBiome>,
    pub rockiness: Perlin,
    pub wetness: Perlin,
    pub elevation: Perlin,
}

impl BiomeSystem {
    pub fn new() -> Self {
        BiomeSystem {
            grass_biomes: IdMap::new(),
            rockiness: Perlin::new(rand::thread_rng().gen()),
            wetness: Perlin::new(rand::thread_rng().gen()),
            elevation: Perlin::new(rand::thread_rng().gen()),
        }
    }
    pub fn step(game: &mut Game) {
        GrassBiome::step(game);
    }
}

const NOISE_DISTANCE_SCALING: f64 = 0.5;
const WATER_WETNESS_THRESHOLD: f64 = 0.7;
const WATER_ELEVATION_MAX: f64 = -0.2;
const GRASS_WETNESS_THRESHOLD: f64 = 0.2;
const DIRT_WETNESS_THRESHOLD: f64 = -0.2;
const WALL_ROCKINESS_THRESHOLD: f64 = 0.8;
const GEM_FLOWER_CHANCE_ON_GRASS: f64 = 0.02;
const GEM_FLOWER_CHANCE_ON_DIRT: f64 = 0.005;
const GEM_FLOWER_CHANCE_ON_WATER: f64 = 0.04;
fn chunk_relative_to_noise_coords(chunk: i64, relative: u8) -> f64 {
    (chunk as f64 + (relative as f64 / TERRAIN_CHUNK_SIZE_SQUARES as f64)) * NOISE_DISTANCE_SCALING
}
pub fn generate_biome(game: &mut Game, coords: TerrainChunkCoords) {
    // TODO: technically f64 doesn't cover whole range of terrain coords
    let mut chunk = Chunk::new(DIRT_SPRITE);
    if coords.get_plane() == Plane(0) {
        for rx in 0..TERRAIN_CHUNK_SIZE_SQUARES as u8 {
            let nx = chunk_relative_to_noise_coords(coords.get_x(), rx);
            for ry in 0..TERRAIN_CHUNK_SIZE_SQUARES as u8 {
                let chunk_index =
                    get_chunk_index_from_relative_coords(ChunkRelativeSquareCoords::new(rx, ry));
                let ny = chunk_relative_to_noise_coords(coords.get_y(), ry);
                let rockiness = game.biome_system.rockiness.get([nx, ny]);
                let wetness = game.biome_system.wetness.get([nx, ny]);
                let elevation = game.biome_system.elevation.get([nx, ny]);
                let sprite_chosen = if rockiness > WALL_ROCKINESS_THRESHOLD {
                    chunk.chunk_squares[chunk_index].base_solid = true;
                    WALL_SPRITE
                } else if wetness > WATER_WETNESS_THRESHOLD && elevation < WATER_ELEVATION_MAX {
                    if rand::thread_rng().gen_bool(GEM_FLOWER_CHANCE_ON_WATER) {
                        chunk.chunk_squares[chunk_index]
                            .detritus
                            .push(GEM_FLOWER_SPRITE);
                    }
                    WATER_SPRITE
                } else if wetness > GRASS_WETNESS_THRESHOLD {
                    if rand::thread_rng().gen_bool(GEM_FLOWER_CHANCE_ON_GRASS) {
                        chunk.chunk_squares[chunk_index]
                            .detritus
                            .push(GEM_FLOWER_SPRITE);
                    }
                    GRASS_SPRITE
                } else if wetness > DIRT_WETNESS_THRESHOLD {
                    if rand::thread_rng().gen_bool(GEM_FLOWER_CHANCE_ON_DIRT) {
                        chunk.chunk_squares[chunk_index]
                            .detritus
                            .push(GEM_FLOWER_SPRITE);
                    }
                    DIRT_SPRITE
                } else {
                    SAND_SPRITE
                };
                chunk.chunk_squares[chunk_index].base_sprite = sprite_chosen;
            }
        }
        game.terrain.chunks.insert(coords, chunk);
    } else {
        Chunk::generate(game, coords, VOID_SPRITE);
    }
}

#[cfg(test)]
mod tests {
    use coords::TERRAIN_CHUNK_SIZE_SQUARES;

    use crate::biomes::NOISE_DISTANCE_SCALING;

    use super::chunk_relative_to_noise_coords;

    #[test]
    fn chunk_relative_to_noise_coords_continuous_across_chunks() {
        let at_0 = chunk_relative_to_noise_coords(0, 0);
        let above_it = chunk_relative_to_noise_coords(-1, TERRAIN_CHUNK_SIZE_SQUARES as u8 - 1);
        let at_bottom = chunk_relative_to_noise_coords(0, TERRAIN_CHUNK_SIZE_SQUARES as u8 - 1);
        let below_bottom = chunk_relative_to_noise_coords(1, 0);
        if ((at_0 - above_it) - NOISE_DISTANCE_SCALING / (TERRAIN_CHUNK_SIZE_SQUARES as f64)).abs()
            > 0.001
        {
            panic!(
                "Distance between 0 ({:?}) and square above ({:?}) was {}",
                at_0,
                above_it,
                at_0 - above_it
            );
        }
        if ((below_bottom - at_bottom)
            - NOISE_DISTANCE_SCALING / (TERRAIN_CHUNK_SIZE_SQUARES as f64))
            .abs()
            > 0.001
        {
            panic!(
                "Distance between below_bottom ({:?}) and square above ({:?}) was {}",
                below_bottom,
                at_bottom,
                below_bottom - at_bottom
            );
        }
    }
}
