use rand::thread_rng;
use wolf_hash_map::WolfHashSet;

use crate::terrain::{notify_new_chunk, Chunk, ChunkLoader, TerrainSpriteComponent};

use super::*;

pub fn create_portal(game: &mut Game, coords_1: SquareCoords, coords_2: SquareCoords) {
    let door_1 = GameObject::create_game(game, coords_1.center_pixel());
    TerrainSpriteComponent::add_to(game, door_1, coords_1, DOOR_SPRITE);
    door_1.add_collision_group(game, CollisionGroupId::Portal);
    let door_2 = GameObject::create_game(game, coords_2.center_pixel());
    TerrainSpriteComponent::add_to(game, door_2, coords_2, DOOR_SPRITE);
    door_2.add_collision_group(game, CollisionGroupId::Portal);
    game.villages_system.doors_map.insert(door_1, door_2);
    game.villages_system.doors_map.insert(door_2, door_1);
}
pub fn create_tavern(game: &mut Game, coords: SquareCoords) {
    let new_plane: Plane = game.get_plane();
    let door_2_coords = SquareCoords::new(new_plane, 0, 0);

    // Destroy anything present
    let destroy_width = 5 * SQUARE_SIZE_PIXELS / 2;
    let hit_box = HitBox::new(coords.center_pixel(), destroy_width, destroy_width);
    let colliding = CollisionSystem::get_colliding(game, CollisionGroupId::Damageable, hit_box);
    for id in colliding {
        id.send_death_signal(game);
    }
    // Create door on main plane
    let door_1 = GameObject::create_game(game, coords.center_pixel());
    TerrainSpriteComponent::add_to(game, door_1, coords, DOOR_SPRITE);
    // Add floor
    for dx in -1..2 {
        for dy in -1..2 {
            let floor_coords = coords.translate(dx, dy);
            TerrainSpriteComponent::add_to(game, door_1, floor_coords, FLOOR_SPRITE);
        }
    }
    // Add walls
    let mut wall_coords = coords.translate(-2, 2);
    let mut wdx = 1;
    let mut wdy = 0;
    let facing = thread_rng().gen_range(0..4);
    for side in 0..4 {
        for square in 0..4 {
            if side != facing || square != 2 {
                Scaffold::create(game, wall_coords);
            }
            wall_coords = wall_coords.translate(wdx, wdy)
        }
        // Keep tavern loaded
        let wdx_next = wdy;
        wdy = -wdx;
        wdx = wdx_next;
    }
    door_1.add_collision_group(game, CollisionGroupId::Portal);

    // Create exit door
    let door_2 = GameObject::create_game(game, door_2_coords.center_pixel());
    TerrainSpriteComponent::add_to(game, door_2, door_2_coords, DOOR_SPRITE);
    door_2.add_collision_group(game, CollisionGroupId::Portal);

    // TODO: allow for unloading/reloading taverns
    // Keep tavern loaded
    let chunk_loader = {
        let mut loaded_chunks = WolfHashSet::new();
        for coords in [coords, door_2_coords] {
            let chunk_coords: TerrainChunkCoords = coords.into();
            loaded_chunks.extend(square_of_coords_centered(chunk_coords, 1).into_iter());
        }
        ChunkLoader { loaded_chunks }
    };
    game.terrain
        .chunks
        .insert(door_2_coords.into(), Chunk::new(FLOOR_SPRITE));
    let big_wall_id = game.get_id();
    for dx in -1..2 {
        for dy in -1..2 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let mut chunk = Chunk::new(WALL_SPRITE);
            for i in 0..chunk.chunk_squares.len() {
                chunk.chunk_squares[i].solids.insert(big_wall_id);
            }
            game.terrain.chunks.insert(
                Into::<TerrainChunkCoords>::into(door_2_coords).translate(dx, dy),
                chunk,
            );
        }
    }
    notify_new_chunk(game, door_2_coords.into());
    let tavern_loader_id = game.get_id();
    game.terrain
        .chunk_loaders
        .insert(tavern_loader_id, chunk_loader);
    game.villages_system.doors_map.insert(door_1, door_2);
    game.villages_system.doors_map.insert(door_2, door_1);
}
pub fn traverse_doors(game: &mut Game, game_object_id: GameObjectId) {
    let hit_box = game_object_id.get_hit_box(game);
    let colliding = CollisionSystem::get_colliding(game, CollisionGroupId::Portal, hit_box.clone());
    for other in colliding {
        if let Some(through_door) = game.villages_system.doors_map.get(other) {
            let through_door_coords = through_door.get_coords_game(game);
            game_object_id.move_to_game(game, through_door_coords);
            return;
        }
    }
}
/*
Buildings:
A building is a PORTAL to the BUILDING PLANE
Tavern (Villagers drink here)
*/
