use super::*;

#[derive(Debug)]
pub struct StartingPlannerState {
    state: PlannerState,
}

impl StartingPlannerState {
    pub fn new() -> Self {
        StartingPlannerState {
            state: PlannerState::new(),
        }
    }
    pub fn lazy_get<T: Any>(
        &mut self,
        game: &Game,
        owner_id: GameObjectId,
        key: PlannerStateKey,
    ) -> &mut T {
        let entry = self.state.states.entry(key);
        let reference = entry.or_insert_with(|| get_starting_state(game, owner_id, key));
        downcast_clone_box_any_mut(reference).unwrap()
    }
}

fn get_starting_state(
    game: &Game,
    owner_id: GameObjectId,
    key: PlannerStateKey,
) -> Box<dyn CloneBoxAny> {
    match key {}
}
