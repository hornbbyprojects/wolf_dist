use crate::terrain::get_chunk_index_from_relative_coords;

use super::*;

const RAVINE_START_Y: i64 = 1;
const RAVINE_END_Y: i64 = 4;
const RAVINE_START_X: i64 = -3;
const RAVINE_END_X: i64 = 3;
const RAVINE_OPEN_HALF_WIDTH: i64 = 5;
const RAVINE_OPEN_FULL_WIDTH: i64 = RAVINE_OPEN_HALF_WIDTH * 2 + 1;
const START_X_OFFSET: i64 = (RAVINE_START_X + RAVINE_END_X) * TERRAIN_CHUNK_SIZE_SQUARES / 2;
pub fn generate_ravine(game: &mut Game) {
    let mut current_x_offset = START_X_OFFSET;
    for cy in RAVINE_START_Y..RAVINE_END_Y + 1 {
        let mut x_offsets = Vec::with_capacity(TERRAIN_CHUNK_SIZE_SQUARES as usize);
        for _ in 0..TERRAIN_CHUNK_SIZE_SQUARES {
            x_offsets.push(current_x_offset);
            let chance_right =
                1.0 - ((current_x_offset - START_X_OFFSET + 10).min(10).max(0) as f64) / 10.0;
            current_x_offset += if rand::thread_rng().gen_bool(chance_right) {
                1
            } else {
                -1
            };
        }
        for cx in RAVINE_START_X..RAVINE_END_X + 1 {
            let chunk_coords = ChunkCoords::new(Plane(0), cx, cy);
            Chunk::generate(game, chunk_coords, DIRT_SPRITE);
            let chunk = game.terrain.chunks.get_mut(&chunk_coords).unwrap();
            for ry in 0..TERRAIN_CHUNK_SIZE_SQUARES {
                let x_offset = x_offsets[ry as usize];
                let ravine_right = x_offset + RAVINE_OPEN_HALF_WIDTH;
                let ravine_left = x_offset - RAVINE_OPEN_HALF_WIDTH;
                let my_left = cx * TERRAIN_CHUNK_SIZE_SQUARES;
                let my_right = (cx + 1) * TERRAIN_CHUNK_SIZE_SQUARES - 1;
                let mut in_this_chunk_left = (ravine_right - my_left + 1).max(0);
                if in_this_chunk_left > RAVINE_OPEN_FULL_WIDTH {
                    in_this_chunk_left = 0;
                }
                in_this_chunk_left = in_this_chunk_left.min(TERRAIN_CHUNK_SIZE_SQUARES);

                let mut in_this_chunk_right = (my_right - ravine_left).max(0);
                if in_this_chunk_right > RAVINE_OPEN_FULL_WIDTH || in_this_chunk_left > 0 {
                    in_this_chunk_right = 0;
                }
                in_this_chunk_right = in_this_chunk_right.min(TERRAIN_CHUNK_SIZE_SQUARES);

                for rx in in_this_chunk_left..(TERRAIN_CHUNK_SIZE_SQUARES - in_this_chunk_right) {
                    let relative = ChunkRelativeSquareCoords::new(rx as u8, ry as u8);
                    let index = get_chunk_index_from_relative_coords(relative);
                    let square = &mut chunk.chunk_squares[index];
                    square.base_sprite = WALL_SPRITE;
                    square.base_solid = true;
                }
            }
        }
    }
}
