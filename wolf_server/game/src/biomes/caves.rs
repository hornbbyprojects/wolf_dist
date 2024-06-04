use crate::{
    game::*,
    terrain::{get_chunk_index_from_relative_coords, Chunk},
    villages::create_portal,
};

const CAVERN_COUNT: usize = 5;
const CAVERN_SPREAD: i64 = 160;
const CAVERN_HALF_WIDTH: i64 = 10;
const CAVERN_MIN_DISTANCE: f64 = CAVERN_HALF_WIDTH as f64 * 2.0 + 10.0;
const CAVERN_LAYER_HALF_WIDTH: i64 = CAVERN_SPREAD + CAVERN_HALF_WIDTH + 20;
const CAVERN_LAYER_HALF_WIDTH_CHUNKS: i64 =
    (CAVERN_LAYER_HALF_WIDTH + TERRAIN_CHUNK_SIZE_SQUARES - 1) / TERRAIN_CHUNK_SIZE_SQUARES;
pub fn generate_caves(game: &mut Game) {
    println!("Generating caves...");
    let caves_plane = game.get_plane();

    for cx in -CAVERN_LAYER_HALF_WIDTH_CHUNKS..CAVERN_LAYER_HALF_WIDTH_CHUNKS + 1 {
        for cy in -CAVERN_LAYER_HALF_WIDTH_CHUNKS..CAVERN_LAYER_HALF_WIDTH_CHUNKS + 1 {
            let chunk_coords = ChunkCoords::new(caves_plane, cx, cy);
            Chunk::generate(game, chunk_coords, WALL_SPRITE);
            let mut chunk = game.terrain.chunks.get_mut(&chunk_coords).unwrap();
            for ry in 0..TERRAIN_CHUNK_SIZE_SQUARES {
                for rx in 0..TERRAIN_CHUNK_SIZE_SQUARES {
                    let index = get_chunk_index_from_relative_coords(
                        ChunkRelativeSquareCoords::new(rx as u8, ry as u8),
                    );
                    chunk.chunk_squares[index].base_solid = true;
                }
            }
        }
    }
    // First, randomly sprinkle some caves
    let mut cavern_points = Vec::new();
    let mut iterations = 0;
    for _ in 0..CAVERN_COUNT {
        let mut center_x;
        let mut center_y;
        if cavern_points.is_empty() {
            center_x = 0;
            center_y = 0;
        } else {
            loop {
                center_x = rand::thread_rng().gen_range(-CAVERN_SPREAD..CAVERN_SPREAD);
                center_y = rand::thread_rng().gen_range(-CAVERN_SPREAD..CAVERN_SPREAD);
                let mut collided = false;
                for (ox, oy) in cavern_points.iter() {
                    let distance =
                        (((ox - center_x).pow(2) + (ox - center_x).pow(2)) as f64).sqrt();
                    if distance < CAVERN_MIN_DISTANCE {
                        collided = true;
                        break;
                    }
                }
                if !collided {
                    break;
                }
                iterations += 1;
                if iterations > 1000 {
                    panic!("Generating caves took > 1000 iterations!");
                }
            }
        }
        // TODO: very inefficient
        for dx in -CAVERN_HALF_WIDTH..CAVERN_HALF_WIDTH + 1 {
            for dy in -CAVERN_HALF_WIDTH..CAVERN_HALF_WIDTH + 1 {
                let coords = SquareCoords::new(caves_plane, center_x + dx, center_y + dy);
                let chunk_coords: TerrainChunkCoords = coords.into();
                let chunk = game.terrain.chunks.get_mut(&chunk_coords).unwrap();
                let relative = coords.relative_to_chunk(chunk_coords).unwrap();
                let index = get_chunk_index_from_relative_coords(relative);
                chunk.chunk_squares[index].base_sprite = DIRT_SPRITE;
                chunk.chunk_squares[index].base_solid = false;
            }
        }
        cavern_points.push((center_x, center_y));
    }
    // Now make paths between caves
    for (i_minus_1, (center_x, center_y)) in
        cavern_points[1..cavern_points.len()].iter().enumerate()
    {
        let (ox, oy) = cavern_points[0..i_minus_1 + 1]
            .iter()
            .min_by_key(|(ox, oy)| ((center_x - ox).pow(2) + (center_y - oy).pow(2)))
            .unwrap();
        let mut x = *center_x;
        let mut y = *center_y;
        while x != *ox || y != *oy {
            let square_coords = SquareCoords::new(caves_plane, x, y);
            let chunk_coords: TerrainChunkCoords = square_coords.into();
            let chunk = game
                .terrain
                .chunks
                .get_mut(&chunk_coords)
                .unwrap_or_else(|| {
                    panic!(
                        "Haven't yet generated chunk {:?}. Should have generated up to {}",
                        chunk_coords, CAVERN_LAYER_HALF_WIDTH_CHUNKS
                    )
                });
            let relative = square_coords.relative_to_chunk(chunk_coords).unwrap();
            let index = get_chunk_index_from_relative_coords(relative);
            chunk.chunk_squares[index].base_sprite = DIRT_SPRITE;
            chunk.chunk_squares[index].base_solid = false;
            let dx = ox - x;
            let dy = oy - y;
            if dx.abs() > dy.abs() {
                if dx > 0 {
                    x += 1;
                } else {
                    x -= 1;
                }
            } else {
                if dy > 0 {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
    }

    // Now generate a door
    let cave_door_coords = SquareCoords::new(caves_plane, 0, 0);
    let wd_x = 0;
    let wd_y = 0;
    let world_door_coords = SquareCoords::new(Plane(0), wd_x, wd_y);
    create_portal(game, cave_door_coords, world_door_coords);
}
