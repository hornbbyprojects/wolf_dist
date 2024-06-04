use super::*;

#[derive(Debug)]
pub struct DerivedPlannerState {
    pub changed_state: PlannerState,
}

impl Clone for DerivedPlannerState {
    fn clone(&self) -> Self {
        let changed_state = self.changed_state.clone();
        DerivedPlannerState { changed_state }
    }
}

impl DerivedPlannerState {
    pub fn new() -> Self {
        DerivedPlannerState {
            changed_state: PlannerState::new(),
        }
    }
    pub fn get<'a, 'b: 'a, T: Any>(
        &'a self,
        game: &Game,
        owner_id: GameObjectId,
        starting_state: &'b mut StartingPlannerState,
        key: PlannerStateKey,
    ) -> &T {
        if let Some(in_self) = self.changed_state.states.get(&key) {
            downcast_clone_box_any(in_self).unwrap()
        } else {
            starting_state.lazy_get(game, owner_id, key)
        }
    }
    pub fn get_mut<'a, 'b: 'a, T: Any>(
        &'a mut self,
        game: &Game,
        owner_id: GameObjectId,
        starting_state: &'b mut StartingPlannerState,
        key: PlannerStateKey,
    ) -> &mut T {
        if let Some(in_self) = self.changed_state.states.get_mut(&key) {
            downcast_clone_box_any_mut(in_self).unwrap()
        } else {
            let reference = starting_state.lazy_get(game, owner_id, key);
            reference
        }
    }
}
