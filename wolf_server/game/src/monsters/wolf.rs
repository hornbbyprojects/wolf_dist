use std::f64::consts::PI;

use wolf_hash_map::WolfHashSet;

use crate::{allegiance::AllegianceComponent, game::*};
pub struct WolfLeader {
    scouts_remaining: u32,
    waves_remaining: u32,
}

/* Wolf types:

Holds still, anime style dash
Charges fast across screen
Wizard (dark rifts?)
*/

impl WolfLeader {
    pub fn step(game: &mut Game) {
        //
    }
}
pub struct WolfSystem {
    dashers: IdMap<GameObjectId, Dasher>,
    chargers: IdMap<GameObjectId, Charger>,
    chargers_by_target: IdMap<GameObjectId, WolfHashSet<GameObjectId>>,
}
impl WolfSystem {
    pub fn new() -> Self {
        WolfSystem {
            dashers: IdMap::new(),
            chargers: IdMap::new(),
            chargers_by_target: IdMap::new(),
        }
    }
    pub fn step(game: &mut Game) {
        Dasher::step(game);
        Charger::step(game);
    }
}

struct Dasher {
    direction: Angle,
    die_at: u32,
}

const SOLDIER_SPEED: f64 = 4.0;
const SOLDIER_LIFETIME: u32 = 50;
impl Dasher {
    fn create(game: &mut Game, spawn_coords: PixelCoords, target_coords: PixelCoords) {
        let game_object_id = GameObject::create_game(game, spawn_coords);
        let direction = spawn_coords.get_direction_to(&target_coords);
        game_object_id.set_rotation(game, direction);
        BasicDrawingComponent::add_to(game, game_object_id, WOLF_SPRITE, DEFAULT_DEPTH);
        DamageableComponent::add_to(game, game_object_id);
        DieOnNoHealthComponent::add_to(game, game_object_id);
        DeleteOnDeathComponent::add_to(game, game_object_id);
        WalkerComponent::add_to(game, game_object_id, SOLDIER_SPEED, SOLDIER_SPEED);
        add_health_bar(game, game_object_id);
        game.monsters.wolf_system.dashers.insert(
            game_object_id,
            Dasher {
                direction,
                die_at: game.tick_counter + SOLDIER_LIFETIME,
            },
        );
    }
    fn step(game: &mut Game) {
        let mut to_delete = Vec::new();
        let mut to_kill = Vec::new();
        for (id, wolf) in game.monsters.wolf_system.dashers.iter() {
            if wolf.die_at <= game.tick_counter {
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
                wolf.direction,
            );
        }
        for id in to_kill {
            id.remove(game);
        }
        for id in to_delete {
            game.monsters.wolf_system.dashers.remove(id);
        }
    }
}
const CHARGER_SPEED: f64 = 8.0;
const CHARGER_HUNT_DISTANCE: f64 = 400.0;
const CHARGER_CHARGE_DISTANCE: f64 = 200.0;
const CHARGER_SEPARATION_DISTANCE: f64 = 200.0;
pub struct Charger {
    target: Option<GameObjectId>,
}
impl Charger {
    pub fn new(
        game: &mut Game,
        starting_coords: PixelCoords,
        target: GameObjectId,
    ) -> GameObjectId {
        let id = GameObject::create_game(game, starting_coords);
        AllegianceComponent::add_to(
            game,
            id,
            vec![game.allegiance_system.special_allegiances.wolf_allegiance],
        );
        DamagerComponent::add_to(game, id, id, None, DEFAULT_HEALTH.0 / 8);
        BasicDrawingComponent::add_to(game, id, WOLF_SPRITE, DEFAULT_DEPTH);
        DamageableComponent::add_to(game, id);
        add_health_bar(game, id);
        WalkerComponent::add_to(game, id, CHARGER_SPEED, CHARGER_SPEED / 32.0);
        DieOnNoHealthComponent::add_to(game, id);
        DeleteOnDeathComponent::add_to(game, id);
        game.monsters.wolf_system.chargers.insert(
            id,
            Charger {
                target: Some(target),
            },
        );
        game.monsters
            .wolf_system
            .chargers_by_target
            .entry(target)
            .or_insert_with(WolfHashSet::new)
            .insert(id);
        id
    }
    pub fn step(game: &mut Game) {
        /* Based on wolf hunting techniques, hunt via:
        1) Get to hunt distance to prey
        2) Move away from other wolves that are also near hunt distance to prey
        3) Attack using anime style magic moves (See Hagrid 2006)
        */
        let mut to_delete = Vec::new();
        for (id, charger) in game.monsters.wolf_system.chargers.iter() {
            let charger_coords = match id.get_coords_safe(&game.game_objects) {
                Some(x) => x,
                None => {
                    to_delete.push(id);
                    continue;
                }
            };
            if let Some(target) = charger.target {
                if let Some(target_coords) = target.get_coords_safe(&game.game_objects) {
                    if target_coords.get_plane() != charger_coords.get_plane() {
                        // TODO: Handle gracefully
                        continue;
                    }
                    let distance = charger_coords.get_distance_to(&target_coords);
                    if distance < CHARGER_CHARGE_DISTANCE {
                        id.intend_follow(&mut game.movement_system.intend_move_system, target);
                    } else if distance > CHARGER_HUNT_DISTANCE {
                        id.intend_follow(&mut game.movement_system.intend_move_system, target);
                    } else {
                        let mut dx: f64 = 0.0;
                        let mut dy: f64 = 0.0;
                        let mut closest = CHARGER_SEPARATION_DISTANCE;
                        let mut found_other = false;
                        let other_hunters = game
                            .monsters
                            .wolf_system
                            .chargers_by_target
                            .get(target)
                            .expect("At least one wolf should be hunting a hunter's target");
                        for other_hunter in other_hunters.iter() {
                            if *other_hunter == id {
                                continue;
                            }
                            if let Some(other_hunter_coords) =
                                other_hunter.get_coords_safe(&game.game_objects)
                            {
                                let other_hunter_distance_to_target =
                                    other_hunter_coords.get_distance_to(&target_coords);
                                if other_hunter_distance_to_target > CHARGER_HUNT_DISTANCE * 2.0 {
                                    continue;
                                }
                                let other_hunter_distance =
                                    charger_coords.get_distance_to(&other_hunter_coords);
                                if other_hunter_distance < closest {
                                    dx = other_hunter_coords.get_x().to_num::<f64>()
                                        - charger_coords.get_x().to_num::<f64>();
                                    dy = other_hunter_coords.get_y().to_num::<f64>()
                                        - charger_coords.get_y().to_num::<f64>();
                                    closest = other_hunter_distance;
                                    found_other = true
                                }
                            }
                        }
                        if !found_other {
                            // Don't need to run from others, charge target
                            id.intend_follow(&mut game.movement_system.intend_move_system, target);
                        } else {
                            let direction = if dx.abs() + dy.abs() < 1.0 {
                                // If too close, go in random direction
                                Angle::enforce_range(rand::thread_rng().gen_range(0.0..2.0 * PI))
                            } else {
                                PixelCoords::new_at_zero().get_direction_to(
                                    &PixelCoords::new_to_fixed(
                                        charger_coords.get_plane(),
                                        -dx,
                                        -dy,
                                    ),
                                )
                            };
                            id.intend_move_in_direction_minimal(
                                &mut game.movement_system.intend_move_system,
                                direction,
                            )
                        }
                    }
                }
            }
        }
        for id in to_delete {
            let charger = game.monsters.wolf_system.chargers.remove(id).unwrap();
            if let Some(target) = charger.target {
                match game.monsters.wolf_system.chargers_by_target.entry(target) {
                    hash_map::Entry::Occupied(mut occ) => {
                        let set = occ.get_mut();
                        set.remove(&id);
                        if set.is_empty() {
                            occ.remove();
                        }
                    }
                    hash_map::Entry::Vacant(_) => {
                        panic!("Wolf died with target but not registered on target")
                    }
                };
            }
        }
    }
}
struct Wolfzard {}
