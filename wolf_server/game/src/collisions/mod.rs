use wolf_hash_map::WolfHashSet;

use crate::game::*;
use std::cell::RefCell;

pub struct CollisionGroup {
    pub collision_map: RefCell<CollisionMap<GameObjectId>>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CollisionGroupId {
    Damageable,
    Solid,
    Corpse,
    Spellbook,
    Resource,
    ClientSideComponent,
    Harvestable,
    Portal,
}

impl From<u32> for CollisionGroupId {
    fn from(id: u32) -> Self {
        match id {
            0 => CollisionGroupId::Damageable,
            1 => CollisionGroupId::Solid,
            2 => CollisionGroupId::Corpse,
            3 => CollisionGroupId::Spellbook,
            4 => CollisionGroupId::Resource,
            5 => CollisionGroupId::ClientSideComponent,
            6 => CollisionGroupId::ClientSideComponent,
            7 => CollisionGroupId::Portal,
            _ => panic!("Invalid collision group id: {}", id),
        }
    }
}
impl Into<u32> for CollisionGroupId {
    fn into(self) -> u32 {
        match self {
            CollisionGroupId::Damageable => 0,
            CollisionGroupId::Solid => 1,
            CollisionGroupId::Corpse => 2,
            CollisionGroupId::Spellbook => 3,
            CollisionGroupId::Resource => 4,
            CollisionGroupId::ClientSideComponent => 5,
            CollisionGroupId::Harvestable => 6,
            CollisionGroupId::Portal => 7,
        }
    }
}

impl CollisionGroup {
    pub fn new() -> Self {
        CollisionGroup {
            collision_map: RefCell::new(CollisionMap::new()),
        }
    }
}
pub struct CollisionSystem {
    pub collision_groups: IdMap<CollisionGroupId, CollisionGroup>,
}

impl CollisionSystem {
    pub fn new() -> Self {
        CollisionSystem {
            collision_groups: IdMap::new(),
        }
    }
    pub fn get_colliding(
        game: &Game,
        group_id: CollisionGroupId,
        hit_box: HitBox,
    ) -> Vec<GameObjectId> {
        game.collision_system
            .collision_groups
            .get(group_id)
            .map(|collision_group| {
                collision_group
                    .collision_map
                    .borrow()
                    .get_colliding_game(game, hit_box)
            })
            .unwrap_or(vec![])
    }
    pub fn get_within_box(
        &self,
        group_id: CollisionGroupId,
        center_coords: SquareCoords,
        box_width: i64,
        box_height: i64,
    ) -> WolfHashSet<GameObjectId> {
        self.collision_groups
            .get(group_id)
            .map(|collision_group| {
                collision_group.collision_map.borrow().0.get_within_box(
                    center_coords,
                    box_width,
                    box_height,
                )
            })
            .unwrap_or_else(WolfHashSet::new)
    }
    pub fn get_collision_group(game: &Game, group_id: CollisionGroupId) -> Option<&CollisionGroup> {
        game.collision_system.collision_groups.get(group_id)
    }
    pub fn post_movement(game: &mut Game) {
        for game_object_id in game.movement_system.last_moved.iter() {
            let group_ids = {
                let mut group_ids = Vec::new();
                let game_object = game.game_objects.get(*game_object_id).unwrap();
                for (group_id, _count) in game_object.collision_groups.iter() {
                    group_ids.push(*group_id);
                }
                group_ids
            };
            for group_id in group_ids {
                let collision_group = game
                    .collision_system
                    .collision_groups
                    .get(group_id)
                    .unwrap();
                let mut collision_map = collision_group.collision_map.borrow_mut();
                collision_map.move_item(game, *game_object_id);
            }
        }
    }
}

impl GameObjectId {
    pub fn add_collision_group(&self, game: &mut Game, group_id: CollisionGroupId) {
        let game_object = game.game_objects.get_mut(*self).unwrap();
        let old_amount = game_object.collision_groups.entry(group_id).or_insert(0);
        *old_amount += 1;
        if *old_amount == 1 {
            //we are the first component to add this group, add to the collision group
            let collision_group = match game.collision_system.collision_groups.get(group_id) {
                Some(x) => x,
                None => {
                    game.collision_system
                        .collision_groups
                        .insert(group_id, CollisionGroup::new());
                    game.collision_system
                        .collision_groups
                        .get(group_id)
                        .unwrap()
                }
            };
            collision_group.collision_map.borrow_mut().add(game, *self);
        }
    }
    pub fn remove_collision_group(&self, game: &mut Game, group_id: CollisionGroupId) {
        let game_object = game.game_objects.get_mut(*self).unwrap();
        let old_amount = game_object
            .collision_groups
            .get_mut(&group_id)
            .expect(&format!(
                "collision group counter null for group id {:?}",
                group_id
            ));
        if *old_amount > 1 {
            *old_amount -= 1;
        } else {
            game_object.collision_groups.remove(&group_id);
            let collision_group =
                game.collision_system
                    .collision_groups
                    .get(group_id)
                    .expect(&format!(
                        "Tried to remove from nonexistent group {:?}",
                        group_id
                    ));
            let mut collision_map = collision_group.collision_map.borrow_mut();
            collision_map.remove(*self);
            if collision_map.is_empty() {
                drop(collision_map);
                game.collision_system.collision_groups.remove(group_id);
            }
        }
    }
}
