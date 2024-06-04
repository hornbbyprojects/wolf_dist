use super::*;

#[derive(Debug)]
pub struct PlannerState {
    pub states: WolfHashMap<PlannerStateKey, Box<dyn CloneBoxAny>>,
}

impl Clone for PlannerState {
    fn clone(&self) -> PlannerState {
        let mut ret_states: WolfHashMap<PlannerStateKey, Box<dyn CloneBoxAny>> = WolfHashMap::new();
        for (key, value) in self.states.iter() {
            let new_value: Box<dyn CloneBoxAny> = (*value).clone_box_any();
            ret_states.insert(key.clone(), new_value);
        }
        PlannerState { states: ret_states }
    }
}

impl PlannerState {
    pub fn new() -> PlannerState {
        PlannerState {
            states: WolfHashMap::new(),
        }
    }
}
