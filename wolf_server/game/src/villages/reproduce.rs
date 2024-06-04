use rand::thread_rng;

use super::*;

const BIRTH_RANGE: f64 = 30.0;
const REPRODUCE_TIME: u32 = 250;
const REPRODUCE_JITTER: f64 = 5.0;

pub struct ReproduceBehaviour {
    mind_id: MindId,
    partner_id: Option<GameObjectId>,
    birth_at: Option<u32>,
    started_looking: bool,
}

impl ReproduceBehaviour {
    pub fn step(game: &mut Game) {
        let mut to_birth = Vec::new();
        let mut to_move = Vec::new();
        let mut partnered: Vec<(BehaviourId, GameObjectId, BehaviourId)> = Vec::new();
        let mut partnered_earlier = HashSet::new();
        let mut incels = Vec::new();
        for (id, behaviour) in game.villages_system.reproduce_behaviours.iter_mut() {
            if let Some(mind) = game.behaviour_system.minds.get_mut(behaviour.mind_id) {
                if mind.active_behaviour != Some(id) {
                    if behaviour.started_looking {
                        behaviour.started_looking = false;
                        game.villages_system.looking_for_partner.remove(&id);
                    }
                    continue;
                }
                let our_coords = mind.game_object_id.get_coords(&game.game_objects);
                if !behaviour.started_looking {
                    game.villages_system.looking_for_partner.insert(id);
                    behaviour.started_looking = true;
                }
                if let Some(birth_at) = behaviour.birth_at {
                    if game.tick_counter > birth_at {
                        to_birth.push(mind.game_object_id.get_coords(&game.game_objects));
                        mind.active_behaviour = None;
                    } else {
                        let dx = thread_rng().gen_range(-REPRODUCE_JITTER..REPRODUCE_JITTER);
                        let dy = thread_rng().gen_range(-REPRODUCE_JITTER..REPRODUCE_JITTER);
                        mind.game_object_id.move_by(
                            &mut game.movement_system.to_move,
                            &mut game.game_objects,
                            dx,
                            dy,
                        );
                    }
                } else if let Some(partner_id) = behaviour.partner_id {
                    let their_coords = partner_id.get_coords(&game.game_objects);
                    let distance = our_coords.get_distance_to(&their_coords);
                    if distance > BIRTH_RANGE {
                        to_move.push((mind.game_object_id, partner_id));
                    } else {
                        behaviour.birth_at = Some(game.tick_counter + REPRODUCE_TIME);
                    }
                } else {
                    if partnered_earlier.contains(&id) {
                        continue;
                    }
                    if let Some(partner_id) = game
                        .villages_system
                        .looking_for_partner
                        .iter()
                        .map(|x| *x)
                        .filter(|x| *x != id)
                        .next()
                    {
                        partnered_earlier.insert(partner_id);
                        game.villages_system.looking_for_partner.remove(&id);
                        game.villages_system.looking_for_partner.remove(&partner_id);
                        partnered.push((id, mind.game_object_id, partner_id));
                    } else {
                        incels.push(mind.game_object_id);
                    }
                }
            }
        }
        for (a, a_game_object_id, b) in partnered {
            let behaviour_b = game
                .villages_system
                .reproduce_behaviours
                .get_mut(b)
                .unwrap();
            let mind_b = game
                .behaviour_system
                .minds
                .get(behaviour_b.mind_id)
                .unwrap();
            behaviour_b.partner_id = Some(a_game_object_id);
            let behaviour_a = game
                .villages_system
                .reproduce_behaviours
                .get_mut(a)
                .unwrap();
            let mind_a = game
                .behaviour_system
                .minds
                .get(behaviour_a.mind_id)
                .unwrap();
            behaviour_a.partner_id = Some(mind_b.game_object_id);
        }
        for id in incels {
            id.speak_safe(
                game,
                format!(
                    "TFW no GF ({} total)",
                    game.villages_system.looking_for_partner.len()
                ),
                Some(game.tick_counter + 100),
            );
        }
        for (a, b) in to_move {
            a.speak_safe(game, "COME HERE", Some(game.tick_counter + 100));
            a.intend_follow(&mut game.movement_system.intend_move_system, b);
        }
        for coords in to_birth {
            VillagesSystem::create_villager(game, coords);
        }
    }
    pub fn begin_behaviour(game: &mut Game, behaviour_id: BehaviourId) {
        let mut behaviour = game
            .villages_system
            .reproduce_behaviours
            .get_mut(behaviour_id)
            .unwrap();
        behaviour.partner_id = None;
        behaviour.birth_at = None;
        behaviour.started_looking = false;
    }
    pub fn new(game: &mut Game, mind_id: MindId) -> BehaviourId {
        let id = game.get_id();
        let behaviour = ReproduceBehaviour {
            mind_id,
            partner_id: None,
            birth_at: None,
            started_looking: false,
        };
        game.villages_system
            .reproduce_behaviours
            .insert(id, behaviour);
        id
    }
    pub fn remove(game: &mut Game, id: BehaviourId) {
        game.villages_system.reproduce_behaviours.remove(id);
    }
}
