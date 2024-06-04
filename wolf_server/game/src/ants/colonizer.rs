use super::*;

pub struct ColonizerAnt {
    direction: Angle,
    spawn_at: u32,
}
const COLONIZER_SPEED: f64 = 1.0;
const SPAWN_TIME: u32 = 200;
impl ColonizerAnt {
    pub fn create(game: &mut Game, coords: PixelCoords) {
        let game_object_id = GameObject::create_game(game, coords);
        let direction = Angle::enforce_range(thread_rng().gen_range(-PI..PI));
        game_object_id.set_rotation(game, direction);
        BasicDrawingComponent::add_to(game, game_object_id, ANT_SPRITE, DEFAULT_DEPTH);
        DamageableComponent::add_to(game, game_object_id);
        DieOnNoHealthComponent::add_to(game, game_object_id);
        DeleteOnDeathComponent::add_to(game, game_object_id);
        WalkerComponent::add_to(game, game_object_id, COLONIZER_SPEED, COLONIZER_SPEED);
        add_health_bar(game, game_object_id);

        game.ant_system.colonizer_ants.insert(
            game_object_id,
            ColonizerAnt {
                direction,
                spawn_at: game.tick_counter + SPAWN_TIME,
            },
        );
    }
    pub fn step(game: &mut Game) {
        let mut to_delete = Vec::new();
        let mut to_spawn = Vec::new();
        for (id, ant) in game.ant_system.colonizer_ants.iter() {
            if id.is_deleted(&game.game_objects) {
                to_delete.push(id);
                continue;
            }
            if ant.spawn_at <= game.tick_counter {
                to_spawn.push(id);
                to_delete.push(id);
            }
            id.intend_move_in_direction_minimal(
                &mut game.movement_system.intend_move_system,
                ant.direction,
            );
        }
        for id in to_spawn {
            let coords = id.get_coords_game(game);
            id.remove(game);
            AntSpawner::create(game, coords);
        }
        for id in to_delete {
            game.ant_system.colonizer_ants.remove(id);
        }
    }
}
