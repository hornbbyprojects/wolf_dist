use super::*;

pub struct NeedsDeconstruction {
    pub game_object_id: GameObjectId,
    coords: CityBlockChunkCoords,
}

pub struct NeedsDeconstructionComponent {
    component_id: ComponentId,
    needs_deconstruction_id: NeedsDeconstructionId,
}

impl NeedsDeconstructionComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let needs_deconstruction_id = game.get_id();
        let coords = owner_id.get_coords_game(game);
        let needs_deconstruction = NeedsDeconstruction {
            game_object_id: owner_id,
            coords: coords.into(),
        };
        {
            let entry = game
                .villages_system
                .needs_deconstruction_lookup
                .entry(needs_deconstruction.coords)
                .or_insert_with(WolfHashSet::new);
            entry.insert(needs_deconstruction_id);
        }
        game.villages_system
            .needs_deconstruction
            .insert(needs_deconstruction_id, needs_deconstruction);
        let component = NeedsDeconstructionComponent {
            component_id,
            needs_deconstruction_id,
        };
        owner_id.add_component(game, component);
    }
}
impl Component for NeedsDeconstructionComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, _owner: GameObjectId) {
        let needs_deconstruction = game
            .villages_system
            .needs_deconstruction
            .remove(self.needs_deconstruction_id)
            .unwrap();
        game.villages_system
            .unassigned_needs_deconstruction
            .remove(&self.needs_deconstruction_id);
        if let hash_map::Entry::Occupied(mut occ) = game
            .villages_system
            .needs_deconstruction_lookup
            .entry(needs_deconstruction.coords)
        {
            let hs = occ.get_mut();

            hs.remove(&self.needs_deconstruction_id);
            if hs.is_empty() {
                occ.remove();
            }
        } else {
            panic!("No lookup for NeedsDeconstruction");
        }
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}

pub struct DeconstructBehaviour {
    mind_id: MindId,
    target: Option<(NeedsDeconstructionId, GameObjectId)>,
}

const DECONSTRUCT_RANGE: f64 = 50.0;
impl DeconstructBehaviour {
    pub fn step(game: &mut Game) {
        let mut to_deconstruct = Vec::new();
        for (id, behaviour) in game.villages_system.deconstruct_behaviours.iter_mut() {
            let mind = game
                .behaviour_system
                .minds
                .get_mut(behaviour.mind_id)
                .unwrap();
            if mind.active_behaviour != Some(id) {
                continue;
            }
            let my_coords = mind.game_object_id.get_coords(&game.game_objects);
            if let Some((_, target)) = behaviour.target {
                if let Some(target_coords) = target.get_coords_safe(&game.game_objects) {
                    let distance = my_coords.get_distance_to(&target_coords);
                    if distance < DECONSTRUCT_RANGE {
                        to_deconstruct.push(target);
                        behaviour.target = None;
                        mind.active_behaviour = None;
                    } else {
                        mind.game_object_id.intend_move_to_point(
                            &mut game.movement_system.intend_move_system,
                            target_coords,
                        );
                    }
                } else {
                    behaviour.target = None;
                    mind.active_behaviour = None;
                }
            } else {
                panic!("Active deconstruction behaviour with no target!");
            }
        }
        for id in to_deconstruct {
            id.remove(game);
        }
    }
    pub fn new(game: &mut Game, mind_id: MindId) -> BehaviourId {
        let id = game.get_id();
        let behaviour = DeconstructBehaviour {
            mind_id,
            target: None,
        };
        game.villages_system
            .deconstruct_behaviours
            .insert(id, behaviour);
        id
    }
    pub fn remove(game: &mut Game, id: BehaviourId) {
        if let Some(behaviour) = game.villages_system.deconstruct_behaviours.remove(id) {
            if let Some((needs_deconstruction_id, _)) = behaviour.target {
                if game
                    .villages_system
                    .needs_deconstruction
                    .contains_key(needs_deconstruction_id)
                {
                    game.villages_system
                        .unassigned_needs_deconstruction
                        .insert(needs_deconstruction_id);
                }
            }
        }
    }
    pub fn activate(
        game: &mut Game,
        id: BehaviourId,
        needs_deconstruction_id: NeedsDeconstructionId,
    ) {
        let game_object_id = game
            .villages_system
            .needs_deconstruction
            .get(needs_deconstruction_id)
            .unwrap()
            .game_object_id;
        game.villages_system
            .deconstruct_behaviours
            .get_mut(id)
            .unwrap()
            .target = Some((needs_deconstruction_id, game_object_id));
        game.villages_system
            .unassigned_needs_deconstruction
            .remove(&needs_deconstruction_id);
    }
}
