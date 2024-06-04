use crate::game::*;

mod woodcutter;
pub use woodcutter::*;
mod hunter;
pub use hunter::*;

/*
Outline of plan:
A Behaviour is the basic unit of AI control, and has a step function.
A Mind controls which behaviour is currently active.
Example:
A frog has two behaviours, JumpAroundRandomly and Ribbit.
Each behaviour has a step function that checks mind.active_behaviour, and steps if it is active.
Then a FrogMind is in charge of setting the active behaviour.
More complicated combinations of behaviours can be modeled using nesting -
A behaviour that controls a mind, or passing along (behaviour sets the active behaviour to a subbehaviour)
*/
pub struct BehaviourSystem {
    pub minds: IdMap<MindId, Mind>,
    woodcutter_behaviours: IdMap<BehaviourId, WoodcutterBehaviour>,
    hunter_behaviours: IdMap<BehaviourId, HunterBehaviour>,
}

impl BehaviourSystem {
    pub fn new() -> Self {
        BehaviourSystem {
            minds: IdMap::new(),
            woodcutter_behaviours: IdMap::new(),
            hunter_behaviours: IdMap::new(),
        }
    }
    pub fn step(game: &mut Game) {
        WoodcutterBehaviour::step(game);
        HunterBehaviour::step(game);
    }
}

pub struct Mind {
    pub active_behaviour: Option<BehaviourId>,
    pub game_object_id: GameObjectId,
}

impl Mind {
    fn new(game: &mut Game, game_object_id: GameObjectId) -> MindId {
        let mind_id = game.get_id();
        let mind = Mind {
            active_behaviour: None,
            game_object_id,
        };
        game.behaviour_system.minds.insert(mind_id, mind);
        mind_id
    }
}
pub struct MindComponent {
    mind_id: MindId,
    component_id: ComponentId,
}

impl Component for MindComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, _owner: GameObjectId) {
        game.behaviour_system.minds.remove(self.mind_id);
    }
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}

impl MindComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) -> MindId {
        let component_id = game.get_id();
        let mind_id = Mind::new(game, owner_id);
        let component = MindComponent {
            component_id,
            mind_id,
        };
        owner_id.add_component(game, component);
        mind_id
    }
}

impl MindId {
    pub fn get_active_behaviour(&self, minds: &IdMap<MindId, Mind>) -> Option<BehaviourId> {
        minds.get(*self).and_then(|mind| mind.active_behaviour)
    }
    pub fn set_active_behaviour(
        &self,
        minds: &mut IdMap<MindId, Mind>,
        behaviour: Option<BehaviourId>,
    ) {
        if let Some(mind) = minds.get_mut(*self) {
            mind.active_behaviour = behaviour;
        }
    }
}
