use std::f64::consts::PI;

use crate::{game::*, hunting::PreyComponent};
use id::IdMap;
use rand::thread_rng;

mod colonizer;
use colonizer::*;
use wolf_hash_map::WolfHashSet;

use crate::GameObjectId;

pub struct AntSystem {
    soldier_ants: IdMap<GameObjectId, SoldierAnt>,
    colonizer_ants: IdMap<GameObjectId, ColonizerAnt>,
    ant_spawners: IdMap<GameObjectId, AntSpawner>,
    spawner_claims: WolfHashSet<TerrainChunkCoords>,
}

impl AntSystem {
    pub fn new() -> Self {
        AntSystem {
            soldier_ants: IdMap::new(),
            ant_spawners: IdMap::new(),
            colonizer_ants: IdMap::new(),
            spawner_claims: WolfHashSet::new(),
        }
    }
    pub fn step(game: &mut Game) {
        SoldierAnt::step(game);
        ColonizerAnt::step(game);
        AntSpawner::step(game);
    }
}
struct AntSpawner {
    spawn_offset: u32,
}

const SPAWN_EVERY: u32 = 50;
const SOLDIER_CHANCE: f64 = 0.8;
impl AntSpawner {
    fn create(game: &mut Game, coords: PixelCoords) {
        let chunk_coords = coords.into();
        if game.ant_system.spawner_claims.contains(&chunk_coords) {
            return;
        }
        game.ant_system.spawner_claims.insert(chunk_coords);

        let game_object_id = GameObject::create_game(game, coords);
        BasicDrawingComponent::add_to(game, game_object_id, ANT_SPAWNER_SPRITE, DEFAULT_DEPTH);
        DamageableComponent::add_to(game, game_object_id);
        add_health_bar(game, game_object_id);
        DieOnNoHealthComponent::add_to(game, game_object_id);
        DeleteOnDeathComponent::add_to(game, game_object_id);
        PreyComponent::add_to(game, game_object_id);
        let spawn_offset = thread_rng().gen_range(0..SPAWN_EVERY);
        game.ant_system
            .ant_spawners
            .insert(game_object_id, AntSpawner { spawn_offset });
    }
    fn step(game: &mut Game) {
        let spawn_offset = game.tick_counter % SPAWN_EVERY;
        let mut to_delete = Vec::new();
        let mut to_create = Vec::new();
        for (id, spawner) in game.ant_system.ant_spawners.iter() {
            if spawner.spawn_offset != spawn_offset {
                continue;
            }
            if let Some(coords) = id.get_coords_safe(&game.game_objects) {
                to_create.push(coords);
            } else {
                to_delete.push(id);
            }
        }
        for coords in to_create {
            if thread_rng().gen_bool(SOLDIER_CHANCE) {
                SoldierAnt::create(game, coords);
            } else {
                ColonizerAnt::create(game, coords);
            }
        }
        for id in to_delete {
            game.ant_system.ant_spawners.remove(id);
        }
    }
}

struct SoldierAnt {
    direction: Angle,
    die_at: u32,
}

const SOLDIER_SPEED: f64 = 4.0;
const SOLDIER_LIFETIME: u32 = 50;
impl SoldierAnt {
    fn create(game: &mut Game, coords: PixelCoords) {
        let game_object_id = GameObject::create_game(game, coords);
        let direction = Angle::enforce_range(thread_rng().gen_range(-PI..PI));
        game_object_id.set_rotation(game, direction);
        BasicDrawingComponent::add_to(game, game_object_id, ANT_SPRITE, DEFAULT_DEPTH);
        DamageableComponent::add_to(game, game_object_id);
        DieOnNoHealthComponent::add_to(game, game_object_id);
        DeleteOnDeathComponent::add_to(game, game_object_id);
        WalkerComponent::add_to(game, game_object_id, SOLDIER_SPEED, SOLDIER_SPEED);
        add_health_bar(game, game_object_id);
        game.ant_system.soldier_ants.insert(
            game_object_id,
            SoldierAnt {
                direction,
                die_at: game.tick_counter + SOLDIER_LIFETIME,
            },
        );
    }
    fn step(game: &mut Game) {
        let mut to_delete = Vec::new();
        let mut to_kill = Vec::new();
        for (id, ant) in game.ant_system.soldier_ants.iter() {
            if ant.die_at <= game.tick_counter {
                to_kill.push(id);
                to_delete.push(id);
                continue;
            }
            if id.is_deleted(&game.game_objects) {
                to_delete.push(id);
                continue;
            }
            id.intend_move_in_direction_minimal(
                &mut game.movement_system.intend_move_system,
                ant.direction,
            );
        }
        for id in to_kill {
            id.remove(game);
        }
        for id in to_delete {
            game.ant_system.soldier_ants.remove(id);
        }
    }
}
