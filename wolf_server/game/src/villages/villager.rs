use super::*;

pub struct VillagerBehaviour {
    mind_id: MindId,
}

impl VillagerBehaviour {
    pub fn step(game: &mut Game) {
        let mut to_delete = Vec::new();
        let mut to_throw = Vec::new();
        for (id, villager) in game.villages_system.villager_behaviours.iter() {
            if let Some(mind) = game.behaviour_system.minds.get_mut(villager.mind_id) {
                if mind.active_behaviour != Some(id) {
                    continue;
                }
                let direction = Angle::enforce_range(rand::thread_rng().gen_range(0.0..PI * 2.0));
                mind.game_object_id.intend_move_in_direction_minimal(
                    &mut game.movement_system.intend_move_system,
                    direction,
                );
                if rand::thread_rng().gen_range(0..100) == 0 {
                    mind.active_behaviour = None;
                    to_throw.push(mind.game_object_id);
                }
            } else {
                to_delete.push(id);
            }
        }
        for id in to_throw {
            id.drop_resources(game);
        }
        for id in to_delete {
            VillagerBehaviour::remove(game, id);
        }
    }
    pub fn remove(game: &mut Game, id: BehaviourId) {
        game.villages_system.villager_behaviours.remove(id);
    }
    pub fn new(game: &mut Game, mind_id: MindId) -> BehaviourId {
        let villager_behaviour_id = game.get_id();
        let villager_behaviour = VillagerBehaviour { mind_id };
        game.villages_system
            .villager_behaviours
            .insert(villager_behaviour_id, villager_behaviour);
        villager_behaviour_id
    }
}

pub struct VillagerMind {
    mind_id: MindId,
    woodcutter_behaviour_id: BehaviourId,
    villager_behaviour_id: BehaviourId,
    reproduce_behaviour_id: BehaviourId,
    hunter_behaviour_id: BehaviourId,
    build_house_behaviour_id: BehaviourId,
    build_scaffold_behaviour_id: BehaviourId,
    deconstruct_behaviour_id: BehaviourId,
}

impl VillagerMind {
    pub fn step(game: &mut Game) {
        let mut to_delete = Vec::new();
        let mut scaffold_behaviours_to_activate = Vec::new();
        let mut deconstruct_behaviours_to_activate = Vec::new();
        let mut new_reproduce = Vec::new();
        for (id, villager_mind) in game.villages_system.villager_minds.iter() {
            let mut new_behaviour_id = None;
            if let Some(mind) = game.behaviour_system.minds.get(villager_mind.mind_id) {
                if mind.active_behaviour.is_none() {
                    let resources = mind.game_object_id.get_resources(game);
                    let reproduce_chance = 1.0 / (game.villages_system.villager_minds.len() as f64);
                    let scaffold_id = game.villages_system.unassigned_scaffolds.iter().next();
                    let needs_deconstruction_id = game
                        .villages_system
                        .unassigned_needs_deconstruction
                        .iter()
                        .next();
                    if resources.get_resource_amount(ResourceType::Wood).0 > 3 {
                        new_behaviour_id = Some(villager_mind.villager_behaviour_id);
                    } else if let Some(needs_deconstruction_id) = needs_deconstruction_id {
                        new_behaviour_id = Some(villager_mind.deconstruct_behaviour_id);
                        deconstruct_behaviours_to_activate.push((
                            villager_mind.deconstruct_behaviour_id,
                            *needs_deconstruction_id,
                        ));
                    } else if rand::thread_rng().gen_bool(0.3) && scaffold_id.is_some() {
                        new_behaviour_id = Some(villager_mind.build_scaffold_behaviour_id);
                        scaffold_behaviours_to_activate.push((
                            villager_mind.build_scaffold_behaviour_id,
                            *scaffold_id.unwrap(),
                        ));
                    } else if rand::thread_rng().gen_bool(0.5) {
                        new_behaviour_id = Some(villager_mind.build_house_behaviour_id);
                    } else if rand::thread_rng().gen_bool(reproduce_chance) {
                        new_reproduce.push(villager_mind.reproduce_behaviour_id);
                        new_behaviour_id = Some(villager_mind.reproduce_behaviour_id);
                    } else {
                        new_behaviour_id = Some(villager_mind.woodcutter_behaviour_id);
                    }
                }
            } else {
                to_delete.push(id);
                continue;
            }
            if new_behaviour_id.is_some() {
                game.behaviour_system
                    .minds
                    .get_mut(villager_mind.mind_id)
                    .unwrap()
                    .active_behaviour = new_behaviour_id;
            }
        }
        for id in new_reproduce {
            ReproduceBehaviour::begin_behaviour(game, id);
        }
        for (id, scaffold_id) in scaffold_behaviours_to_activate {
            BuildScaffoldBehaviour::activate(game, id, scaffold_id);
        }
        for (id, needs_deconstruction_id) in deconstruct_behaviours_to_activate {
            DeconstructBehaviour::activate(game, id, needs_deconstruction_id);
        }
        for id in to_delete {
            VillagerMind::delete(game, id);
        }
    }
    pub fn new(game: &mut Game, owner_id: GameObjectId) {
        let villager_mind_id = game.get_id();
        let mind_id = MindComponent::add_to(game, owner_id);
        let woodcutter_behaviour_id = WoodcutterBehaviour::new(game, mind_id);
        let villager_behaviour_id = VillagerBehaviour::new(game, mind_id);
        let hunter_behaviour_id =
            HunterBehaviourComponent::add_to(game, owner_id, mind_id).hunter_behaviour;
        let build_house_behaviour_id = BuildHouseBehaviour::new(game, mind_id);
        let reproduce_behaviour_id = ReproduceBehaviour::new(game, mind_id);
        let build_scaffold_behaviour_id = BuildScaffoldBehaviour::new(game, mind_id);
        let deconstruct_behaviour_id = DeconstructBehaviour::new(game, mind_id);
        let villager_mind = VillagerMind {
            mind_id,
            woodcutter_behaviour_id,
            villager_behaviour_id,
            hunter_behaviour_id,
            build_house_behaviour_id,
            reproduce_behaviour_id,
            build_scaffold_behaviour_id,
            deconstruct_behaviour_id,
        };
        game.villages_system
            .villager_minds
            .insert(villager_mind_id, villager_mind);
    }

    fn delete(game: &mut Game, id: VillagerMindId) {
        if let Some(mind) = game.villages_system.villager_minds.remove(id) {
            BuildScaffoldBehaviour::remove(game, mind.build_scaffold_behaviour_id);
            ReproduceBehaviour::remove(game, mind.reproduce_behaviour_id);
            BuildHouseBehaviour::remove(game, mind.build_house_behaviour_id);
            DeconstructBehaviour::remove(game, mind.deconstruct_behaviour_id);
        }
    }
}
