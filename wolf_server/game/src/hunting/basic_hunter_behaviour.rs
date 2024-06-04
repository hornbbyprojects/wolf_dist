use super::*;
use crate::ai::*;
use crate::combinable::*;

const BASIC_HUNTER_AGGRO_RANGE: f64 = 200.0;
const BASIC_HUNTER_DEAGGRO_RANGE: f64 = 450.0;

#[derive(Clone)]
pub struct BasicHunterGoal {}

pub struct BasicHunterPlan {
    target_id: GameObjectId,
}

impl BasicHunterGoal {
    pub fn new() -> Box<dyn Goal> {
        Box::new(BasicHunterGoal {})
    }
}
impl Goal for BasicHunterGoal {
    fn get_importance(&self, _game: &Game, _owner_id: GameObjectId) -> Option<GoalImportance> {
        Some(GoalImportance(100))
    }
    fn get_method(&self, game: &Game, owner_id: GameObjectId) -> GoalResult {
        let prey_id = HuntingSystem::get_closest_prey(game, owner_id, BASIC_HUNTER_AGGRO_RANGE);
        let target_id = prey_id.map(|prey_id| {
            game.hunting_system
                .preys
                .get(prey_id)
                .unwrap()
                .game_object_id
        });
        if let Some(target_id) = target_id {
            GoalResult::SimplePlan(Box::new(BasicHunterPlan { target_id }))
        } else {
            GoalResult::Failure
        }
    }
}

impl SimplePlan for BasicHunterPlan {
    fn step(&mut self, game: &mut Game, owner_id: GameObjectId) -> ActionResult {
        let target_coords = if let Some(target) = game.game_objects.get(self.target_id) {
            target.coords
        } else {
            return ActionResult::Success;
        };
        if !owner_id.can_hurt(game, self.target_id) {
            return ActionResult::Failure;
        }
        let owner_coords = owner_id.get_coords_game(game);
        let distance = owner_coords.get_distance_to(&target_coords);
        if distance > BASIC_HUNTER_DEAGGRO_RANGE {
            return ActionResult::Failure;
        }
        owner_id.intend_follow_game(game, self.target_id);
        ActionResult::Continue
    }
}

#[derive(Clone)]
pub struct BasicHunterBehaviourComponent {
    component_id: ComponentId,
}

impl BasicHunterBehaviourComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let comp = BasicHunterBehaviourComponent { component_id };
        owner_id.add_get_goals_signal_listener(game, comp.clone());
        owner_id.add_component(game, comp);
    }
}

impl GetGoalsSignalListener for BasicHunterBehaviourComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn GetGoalsSignalListener> {
        Box::new(self.clone())
    }
    fn receive_get_goals_signal(
        &self,
        _game: &Game,
        _owner_id: GameObjectId,
    ) -> CombinedVecs<Box<dyn Goal>> {
        CombinedVecs(vec![BasicHunterGoal::new()])
    }
}

impl Component for BasicHunterBehaviourComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_get_goals_signal_listener(game, self.component_id);
    }
}
