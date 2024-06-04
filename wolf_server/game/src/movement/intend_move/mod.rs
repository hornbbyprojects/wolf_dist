/*
Functions to allow an object to intend to move in a direction.
This will then be converted into real movement by a component that pays attention,
e.g. LegsComponent.
*/

use std::collections::hash_map::Entry;

use crate::game::*;
use wolf_hash_map::WolfHashMap;

mod mount;
pub use mount::*;

#[derive(Clone)]
pub enum IntendedMovements {
    MoveInDirection(Angle),
    Follow(GameObjectId),
    MoveToPoint(PixelCoords),
    Confusion,
}

pub struct IntendMoveSystem {
    pub intended_movements: WolfHashMap<GameObjectId, IntendedMovements>,
    pub mounts: IdMap<MountId, Mount>,
    // TODO: both of these don't need to store the whole mount
    pub mounts_by_mounter: WolfHashMap<GameObjectId, IdMap<MountId, Mount>>,
    pub mounts_by_mounted: WolfHashMap<GameObjectId, IdMap<MountId, Mount>>,
}

impl IntendMoveSystem {
    pub fn new() -> Self {
        IntendMoveSystem {
            intended_movements: WolfHashMap::new(),
            mounts: IdMap::new(),
            mounts_by_mounter: WolfHashMap::new(),
            mounts_by_mounted: WolfHashMap::new(),
        }
    }
    fn intend_move(&mut self, game_object_id: GameObjectId, movement: IntendedMovements) {
        if let Some(mounts) = self.mounts_by_mounter.get(&game_object_id) {
            let mut to_move = Vec::new();
            for (_id, mount) in mounts.iter() {
                // TODO: maybe prevent loops of mounts? Perhaps as arg to this function?
                to_move.push(mount.mounted_id);
            }
            for mounted_id in to_move {
                self.intend_move(mounted_id, movement.clone());
            }
        } else {
            let entry = self.intended_movements.entry(game_object_id);
            match entry {
                Entry::Occupied(mut occ) => {
                    occ.insert(movement);
                }
                Entry::Vacant(vac) => {
                    vac.insert(movement);
                }
            }
        }
    }
    pub fn pre_movement(_game: &mut Game) {}
}

impl Default for IntendMoveSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl GameObjectId {
    pub fn intend_stop(&self, intend_move_system: &mut IntendMoveSystem) {
        let mut to_stop = Vec::new();
        if let Some(mounts) = intend_move_system.mounts_by_mounter.get(self) {
            for (_id, mount) in mounts.iter() {
                to_stop.push(mount.mounted_id);
            }
        }
        for id in to_stop {
            id.intend_stop(intend_move_system);
        }
        intend_move_system.intended_movements.remove(self);
    }
    pub fn intend_follow(
        &self,
        intend_move_system: &mut IntendMoveSystem,
        to_follow: GameObjectId,
    ) {
        intend_move_system.intend_move(*self, IntendedMovements::Follow(to_follow))
    }
    pub fn intend_follow_game(&self, game: &mut Game, to_follow: GameObjectId) {
        self.intend_follow(&mut game.movement_system.intend_move_system, to_follow)
    }
    pub fn intend_move_in_direction_minimal(
        &self,
        intend_move_system: &mut IntendMoveSystem,
        direction: Angle,
    ) {
        intend_move_system.intend_move(*self, IntendedMovements::MoveInDirection(direction))
    }
    pub fn intend_move_in_direction(&self, game: &mut Game, direction: Angle) {
        game.movement_system
            .intend_move_system
            .intend_move(*self, IntendedMovements::MoveInDirection(direction))
    }
    pub fn intend_move_to_point(
        &self,
        intend_move_system: &mut IntendMoveSystem,
        point: PixelCoords,
    ) {
        intend_move_system.intend_move(*self, IntendedMovements::MoveToPoint(point));
    }
    pub fn intend_move_to_point_game(&self, game: &mut Game, point: PixelCoords) {
        self.intend_move_to_point(&mut game.movement_system.intend_move_system, point);
    }
}
