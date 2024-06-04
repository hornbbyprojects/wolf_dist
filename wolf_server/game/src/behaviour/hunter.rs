use crate::abilities::{CastAbilitySignalSender, GetAbilityDetailsSignalSender};

use super::*;

const HUNT_DISTANCE: f64 = 500.0;
const TETHER_DISTANCE: f64 = 600.0;

pub struct HunterBehaviour {
    mind_id: MindId,
    pub current_target: Option<PreyId>,
}

/// Behaviour to chase a specific target, and kill them.
impl HunterBehaviour {
    pub fn step(game: &mut Game) {
        let mut to_follow = Vec::new();
        let mut to_reset = Vec::new();
        for (id, hunter_behaviour) in game.behaviour_system.hunter_behaviours.iter() {
            let mind = game
                .behaviour_system
                .minds
                .get(hunter_behaviour.mind_id)
                .unwrap();
            if mind.active_behaviour != Some(id) {
                continue;
            }
            let game_object_id = mind.game_object_id;
            let current_target = hunter_behaviour
                .current_target
                .expect("Hunter behaviour had no target set");
            let allegiances = game_object_id.get_allegiances(game);
            let prey = if let Some(prey) = game.hunting_system.preys.get(current_target) {
                prey
            } else {
                // Prey is vanished
                to_reset.push(hunter_behaviour.mind_id);
                continue;
            };
            let distance = game_object_id.get_distance_to(&game.game_objects, &prey.game_object_id);
            if distance > TETHER_DISTANCE {
                // Prey has escaped
                to_reset.push(hunter_behaviour.mind_id);
                continue;
            }
            let target_allegiances = &prey.game_object_id.get_allegiances(game);
            if !game
                .allegiance_system
                .can_hurt_multiple(&allegiances, target_allegiances)
            {
                // Prey is no longer a valid target
                to_reset.push(hunter_behaviour.mind_id);
                continue;
            }
            to_follow.push((game_object_id, prey.game_object_id));
        }
        for (id, target_id) in to_follow {
            if game.tick_counter % 50 == 0 {
                let abilities = id
                    .send_get_ability_details_signal(game)
                    .map(|x| x.0)
                    .unwrap_or(Vec::new());
                for (ability_id, ability) in abilities {
                    if ability.is_attack_ability {
                        id.send_cast_ability_signal(
                            game,
                            ability_id,
                            target_id.get_coords(&game.game_objects),
                        );
                        break;
                    }
                }
            }

            id.intend_follow(&mut game.movement_system.intend_move_system, target_id);
        }
        for id in to_reset {
            id.set_active_behaviour(&mut game.behaviour_system.minds, None);
        }
    }
    fn new(game: &mut Game, mind_id: MindId) -> BehaviourId {
        let hunter_behaviour_id = game.get_id();
        let hunter_behaviour = HunterBehaviour {
            mind_id,
            current_target: None,
        };
        game.behaviour_system
            .hunter_behaviours
            .insert(hunter_behaviour_id, hunter_behaviour);
        hunter_behaviour_id
    }
    pub fn remove(game: &mut Game, id: BehaviourId) {
        game.behaviour_system.hunter_behaviours.remove(id);
    }
}

#[derive(Clone)]
pub struct HunterBehaviourComponent {
    pub component_id: ComponentId,
    pub hunter_behaviour: BehaviourId,
}

impl Component for HunterBehaviourComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        HunterBehaviour::remove(game, self.hunter_behaviour);
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}
impl HunterBehaviourComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        mind_id: MindId,
    ) -> HunterBehaviourComponent {
        let component_id = game.get_id();
        let hunter_behaviour = HunterBehaviour::new(game, mind_id);
        let comp = HunterBehaviourComponent {
            hunter_behaviour,
            component_id,
        };
        owner_id.add_component(game, comp.clone());
        comp
    }
}

impl HunterBehaviour {
    pub fn set_target(game: &mut Game, behaviour_id: BehaviourId, target: PreyId) {
        game.behaviour_system
            .hunter_behaviours
            .get_mut(behaviour_id)
            .unwrap()
            .current_target = Some(target);
    }
}
