use crate::{
    abilities::{Harvester, MAX_HARVEST_ANCHOR_DISTANCE},
    game::*,
};

pub struct WoodcutterBehaviour {
    mind_id: MindId,
    current_target: Option<GameObjectId>,
    current_harvester: Option<HarvesterId>,
}

impl WoodcutterBehaviour {
    pub fn step(game: &mut Game) {
        let mut to_delete = Vec::new();
        let mut harvesters_to_create = Vec::new();
        {
            for (id, woodcutter) in game.behaviour_system.woodcutter_behaviours.iter_mut() {
                let game_object_id = {
                    if let Some(mind) = game.behaviour_system.minds.get_mut(woodcutter.mind_id) {
                        if mind.active_behaviour != Some(id) {
                            continue;
                        }
                        if let Some(current_harvester) = woodcutter.current_harvester {
                            if !game
                                .ability_system
                                .harvesters
                                .contains_key(current_harvester)
                            {
                                woodcutter.current_harvester = None;
                                woodcutter.current_target = None;
                                mind.active_behaviour = None;
                            }
                            continue;
                        }
                        mind.game_object_id
                    } else {
                        to_delete.push(id);
                        continue;
                    }
                };
                let coords = game_object_id.get_coords(&game.game_objects);
                if let Some(current_target_id) = woodcutter.current_target {
                    if let Some(target_object) = game.game_objects.get(current_target_id) {
                        let target_coords = target_object.coords;
                        let current_distance = coords.get_distance_to(&target_coords);
                        if current_distance < MAX_HARVEST_ANCHOR_DISTANCE {
                            harvesters_to_create.push((id, game_object_id, target_coords));
                        } else {
                            game_object_id.intend_move_to_point(
                                &mut game.movement_system.intend_move_system,
                                target_coords,
                            );
                        }
                    } else {
                        woodcutter.current_target = None;
                    }
                } else {
                    let nearby_harvestables = game.collision_system.get_within_box(
                        CollisionGroupId::Harvestable,
                        coords.into(),
                        10,
                        10,
                    );

                    if let Some(target) = nearby_harvestables.into_iter().next() {
                        woodcutter.current_target = Some(target);
                    } else {
                        woodcutter
                            .mind_id
                            .set_active_behaviour(&mut game.behaviour_system.minds, None);
                    }
                }
            }
        }
        for (id, game_object_id, target_coords) in harvesters_to_create {
            game_object_id.intend_stop(&mut game.movement_system.intend_move_system);
            let harvester_id = Harvester::new(game, game_object_id, target_coords);
            let woodcutter_behaviour = game
                .behaviour_system
                .woodcutter_behaviours
                .get_mut(id)
                .unwrap();
            woodcutter_behaviour.current_harvester = Some(harvester_id);
        }
        for id in to_delete {
            WoodcutterBehaviour::remove(game, id);
        }
    }
    pub fn new(game: &mut Game, mind_id: MindId) -> BehaviourId {
        let behaviour_id = game.get_id();
        let woodcutter_behaviour = WoodcutterBehaviour {
            mind_id,
            current_target: None,
            current_harvester: None,
        };
        game.behaviour_system
            .woodcutter_behaviours
            .insert(behaviour_id, woodcutter_behaviour);
        behaviour_id
    }
    fn remove(game: &mut Game, id: BehaviourId) {
        game.behaviour_system.woodcutter_behaviours.remove(id);
    }
}
