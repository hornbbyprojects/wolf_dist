use crate::combinable::OrBool;
use crate::game::*;
use signal_listener_macro::define_signal_listener;
use std::collections::hash_map::*;
use wolf_hash_map::*;

mod locomotion;
pub use locomotion::*;
mod intend_move;
pub use intend_move::*;
mod constant_velocity;
pub use constant_velocity::*;
mod arcing;
pub use arcing::*;
mod speed_mod;
pub use speed_mod::*;

define_signal_listener!(Move, &mut Game, old_coords: &PixelCoords);
define_signal_listener!(GetBlockMovement, &Game -> OrBool);
//movement is done at the end of the tick to allow spatial maps to update correctly

pub struct MovementSystem {
    //to_move: maps id to target coords
    pub intend_move_system: IntendMoveSystem,
    pub locomotion_system: LocomotionSystem,

    pub to_move: WolfHashMap<GameObjectId, PixelCoords>,
    pub to_rotate: WolfHashMap<GameObjectId, Angle>,
    pub last_moved: WolfHashSet<GameObjectId>,

    pub constant_velocities: IdMap<ConstantVelocityId, ConstantVelocity>,
}

impl MovementSystem {
    pub fn new() -> MovementSystem {
        MovementSystem {
            intend_move_system: IntendMoveSystem::new(),
            locomotion_system: LocomotionSystem::new(),

            to_move: WolfHashMap::new(),
            to_rotate: WolfHashMap::new(),
            last_moved: WolfHashSet::new(),
            constant_velocities: IdMap::new(),
        }
    }
    pub fn pre_movement(game: &mut Game) {
        ConstantVelocity::step(game);
        LocomotionSystem::step(game);
        IntendMoveSystem::pre_movement(game);
    }
    /// All movement is queued up by the move_ functions, and then only actually performed in this function. This keeps the position of each object consistent throughout the tick
    pub fn movement(game: &mut Game) {
        // All the objects we attempt to move
        let mut attempt_to_move = WolfHashMap::new();
        // As above, but add in movement to mounts and subtract movements blocked by signal
        let mut actually_moved = WolfHashMap::new();
        let mut player_updates: WolfHashMap<PlayerId, WolfHashSet<GameObjectId>> =
            WolfHashMap::new();
        std::mem::swap(&mut attempt_to_move, &mut game.movement_system.to_move);
        // First, movement that can be blocked by signal
        for (id, target_coords) in attempt_to_move {
            let block = id
                .send_get_block_movement_signal(game)
                .map(|x| x.extract())
                .unwrap_or(false);
            if !block {
                actually_moved.insert(id, target_coords);
            }
        }
        /* Then mount following, which needs to be
        a) after normal movement, to sync position on server
        b) before we update the players, to sync position on client.
        TODO: should this be handled by signal instead?

        */
        for (_id, mount) in game.movement_system.intend_move_system.mounts.iter() {
            if let Some(about_to_move_to) = actually_moved.get(&mount.mounted_id) {
                actually_moved.insert(mount.mounter_id, *about_to_move_to);
            } else {
                let new_coords = mount.mounted_id.get_coords(&game.game_objects);
                actually_moved.insert(mount.mounter_id, new_coords);
            }
        }
        for (id, new_coords) in actually_moved.iter() {
            let (old_coords, player_ids) = {
                let mut game_object = match game.game_objects.get_mut(*id) {
                    Some(game_object) => game_object,
                    None => continue,
                };
                let old_coords = game_object.coords.clone();
                let player_ids = game_object.players_sent_to_last_tick.clone();
                game_object.coords = *new_coords;
                (old_coords, player_ids)
            };
            id.send_move_signal(game, &old_coords);
            for player_id in player_ids {
                let entry = player_updates.entry(player_id);
                let set = entry.or_insert_with(WolfHashSet::new);
                set.insert(*id);
            }
        }
        let mut to_rotate = WolfHashMap::new();
        std::mem::swap(&mut game.movement_system.to_rotate, &mut to_rotate);
        for (id, angle) in to_rotate.iter() {
            if let Some(game_object) = game.game_objects.get_mut(*id) {
                game_object.rotation = *angle;
                let player_ids = game_object.players_sent_to_last_tick.clone();

                // TODO: code duplication
                for player_id in player_ids {
                    let entry = player_updates.entry(player_id);
                    let set = entry.or_insert_with(WolfHashSet::new);
                    set.insert(*id);
                }
            }
        }
        let mut last_moved = actually_moved
            .into_iter()
            .map(|(id, _)| id)
            .chain(to_rotate.into_iter().map(|(id, _)| id))
            .collect();
        std::mem::swap(&mut game.movement_system.last_moved, &mut last_moved);
        for (player_id, set) in player_updates {
            let player = game
                .player_system
                .players
                .get_mut(player_id)
                .expect("Object tried to update a removed player");
            player.game_objects_to_update = set;
        }
    }
}

//these movement functions merely queue up a movement, which is then done or potentially blocked in the actual movement stage
impl GameObjectId {
    pub fn move_to(
        &self,
        to_move: &mut WolfHashMap<GameObjectId, PixelCoords>,
        new_coords: PixelCoords,
    ) {
        to_move.insert(*self, new_coords);
    }
    pub fn move_to_game(&self, game: &mut Game, new_coords: PixelCoords) {
        self.move_to(&mut game.movement_system.to_move, new_coords)
    }
    pub fn move_by(
        &self,
        to_move: &mut WolfHashMap<GameObjectId, PixelCoords>,
        game_objects: &IdMap<GameObjectId, GameObject>,
        dx: f64,
        dy: f64,
    ) {
        let target_coords = match to_move.entry(*self) {
            Entry::Occupied(occ) => occ.get().clone(),
            Entry::Vacant(_) => {
                let game_object = game_objects.get(*self).unwrap();
                game_object.coords.clone()
            }
        }
        .translate(dx, dy);
        to_move.insert(*self, target_coords);
    }
    pub fn move_by_game(&self, game: &mut Game, dx: f64, dy: f64) {
        self.move_by(
            &mut game.movement_system.to_move,
            &game.game_objects,
            dx,
            dy,
        )
    }
    pub fn move_direction_game(&self, game: &mut Game, direction: Angle, distance: f64) {
        self.move_direction(
            &mut game.movement_system.to_move,
            &game.game_objects,
            direction,
            distance,
        )
    }
    pub fn move_direction(
        &self,
        to_move: &mut WolfHashMap<GameObjectId, PixelCoords>,
        game_objects: &IdMap<GameObjectId, GameObject>,
        direction: Angle,
        distance: f64,
    ) {
        let dx = direction.cos() * distance;
        let dy = direction.sin() * distance;
        self.move_by(to_move, game_objects, dx, dy);
    }
}
