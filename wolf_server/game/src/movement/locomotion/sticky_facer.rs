use crate::{add_shared_component, game::*, get_shared_component};

pub struct StickyFacer {
    facing: Option<CardinalDirection>,
}
impl StickyFacer {
    pub fn new() -> Self {
        StickyFacer { facing: None }
    }
}

impl GameObjectComponents {
    add_shared_component!(add_sticky_facer, StickyFacer, StickyFacer);
    get_shared_component!(
        get_sticky_facer,
        get_sticky_facer_mut,
        StickyFacer,
        StickyFacer
    );
    pub fn remove_sticky_facer(&mut self) {
        self.remove_shared_component(SharedComponentDiscriminants::StickyFacer);
    }
}

impl GameObjectId {
    pub fn add_sticky_facer(&self, game: &mut Game) {
        self.get_mut(game).unwrap().components.add_sticky_facer();
    }
    pub fn remove_sticky_facer(&self, game: &mut Game) {
        self.get_mut(game).unwrap().components.remove_sticky_facer();
    }
    pub fn stop_facing_sticky(&self, game: &mut Game) {
        self.get_mut(game)
            .unwrap()
            .components
            .get_sticky_facer_mut()
            .unwrap()
            .facing = None;
    }
    pub fn set_facing_sticky(&self, game: &mut Game, angle: Angle) {
        let game_object = game.game_objects.get_mut(*self).unwrap();
        if let Some(sticky_facer) = game_object.components.get_sticky_facer_mut() {
            if let Some(existing_facing) = sticky_facer.facing {
                let compatible_directions = CardinalDirection::compatible_with_angle(angle);
                if compatible_directions.contains(&existing_facing) {
                    return;
                }
            }
            let new_facing = CardinalDirection::closest_to_angle(angle);
            sticky_facer.facing = Some(new_facing);
            self.send_set_facing_signal(game, new_facing);
        } else {
            // Todo: refactor this. We want some non sticky facers, with custom responses!
            game_object.rotation = angle;
        }
        let mut mounters_to_face = Vec::new();
        if let Some(mounters) = game
            .movement_system
            .intend_move_system
            .mounts_by_mounted
            .get(self)
        {
            for (_id, mount) in mounters.iter() {
                mounters_to_face.push(mount.mounter_id);
            }
        }
        for id in mounters_to_face {
            id.set_facing_sticky(game, angle);
        }
    }
}
