use rand::thread_rng;

use super::*;

pub struct BuildHouseBehaviour {
    mind_id: MindId,
    targeted_coords: Option<SquareCoords>,
}

const BUILD_DISTANCE: f64 = 200.0;

impl BuildHouseBehaviour {
    pub fn step(game: &mut Game) {
        let mut to_create = Vec::new();
        let mut city_blocks_to_create = Vec::new();
        let mut to_say_start = Vec::new();
        let mut to_say_build = Vec::new();
        for (id, behaviour) in game.villages_system.build_house_behaviours.iter_mut() {
            let game_object_id = {
                let mind = game.behaviour_system.minds.get(behaviour.mind_id).unwrap();
                if mind.active_behaviour != Some(id) {
                    continue;
                }
                mind.game_object_id
            };
            let my_coords = game_object_id.get_coords(&game.game_objects);
            if let Some(targeted_coords) = behaviour.targeted_coords {
                let pixel_coords = targeted_coords.center_pixel();
                let distance = my_coords.get_distance_to(&pixel_coords);
                if distance < BUILD_DISTANCE {
                    // Build it
                    to_say_build.push(game_object_id);
                    to_create.push(targeted_coords);
                    behaviour.targeted_coords = None;
                    behaviour
                        .mind_id
                        .set_active_behaviour(&mut game.behaviour_system.minds, None);
                } else {
                    game_object_id.intend_move_to_point(
                        &mut game.movement_system.intend_move_system,
                        pixel_coords,
                    );
                }
            } else {
                let mut new_city_block = true;
                if let Some(next_city_block) = &game.villages_system.next_city_block {
                    if !next_city_block.is_full() {
                        new_city_block = false;
                    }
                }
                if new_city_block {
                    let next_coords = game.villages_system.city_block_spiraler.get_next_coords();
                    game.villages_system.next_city_block =
                        Some(CityBlock::new(next_coords.center_square().into()));
                    city_blocks_to_create.push(next_coords);
                }
                let target = game
                    .villages_system
                    .next_city_block
                    .as_mut()
                    .unwrap()
                    .get_reservation();
                let dx = thread_rng().gen_range(-1..2);
                let dy = thread_rng().gen_range(-1..2);
                let sx: SquareCoords = target.center_square();
                let sx_offset = sx.translate(dx, dy);
                behaviour.targeted_coords = Some(sx_offset);
                to_say_start.push(game_object_id);
            }
        }
        for id in to_say_build {
            id.speak_safe(game, "TOK TOK TOK", Some(game.tick_counter + 100));
        }
        for id in to_say_start {
            id.speak_safe(game, "Time to build house!", Some(game.tick_counter + 100));
        }
        for coords in to_create {
            create_tavern(game, coords)
        }
        for coords in city_blocks_to_create {
            if let Some(to_deconstruct) = game
                .villages_system
                .needs_deconstruction_lookup
                .get(&coords)
            {
                game.villages_system
                    .unassigned_needs_deconstruction
                    .extend(to_deconstruct.iter().map(|x| *x));
            }
            if !game.villages_system.reserved.insert(coords) {
                println!("WARNING: Attempted to recreate city block!");
            }
            let mut square = coords.bottom_left();
            let mut direction = (1, 0);
            for _side in 0..4 {
                for _square in 0..CITY_BLOCK_SIZE - 1 {
                    Scaffold::create(game, square);
                    square = square.translate(direction.0, direction.1);
                }
                direction = (-direction.1, direction.0);
            }
        }
    }
    pub fn new(game: &mut Game, mind_id: MindId) -> BehaviourId {
        let behaviour_id = game.get_id();
        let behaviour = BuildHouseBehaviour {
            mind_id,
            targeted_coords: None,
        };
        game.villages_system
            .build_house_behaviours
            .insert(behaviour_id, behaviour);
        behaviour_id
    }
    pub fn remove(game: &mut Game, id: BehaviourId) {
        game.villages_system.build_house_behaviours.remove(id);
    }
}
