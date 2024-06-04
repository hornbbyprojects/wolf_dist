use super::*;
enum PlanResult {
    Success,
    Failure,
    Continue
}
///Represents the goals that still need solving at a stage in a plan
#[derive(Hash, Eq, PartialEq, Clone)]
struct GoalsState {
    resources_needed: Option<Box<Resources>>,
    house_needed: bool,
}
impl GoalsState {
    fn satisfied(&self) -> bool {
        self.resources_needed.is_none()
            && !self.house_needed
    }
    ///The estimated cost of solving all goals
    fn estimated_cost(&self) -> u32 {
        let mut cost = 0;
        if self.resources_needed.is_some() {
            cost += 100;
        }
        if self.house_needed {
            cost += 100;
        }
        return cost;
    }
    fn need_resources(&mut self, resources_needed: Resources) {
        if let Some(old_resources_needed) = self.resources_needed {
            self.resources_needed = Some(resources_needed + old_resources_needed);
        }
        else {
            self.resources_needed = Some(resources_needed);
        }
    }
}
#[derive(Hash, Eq, PartialEq)]
pub struct AiState {
    world_state: WorldState,
    goals_state: GoalState,
}

pub struct Action {
    convert_state: Box<dyn Fn(WorldState) -> WorldState>,
}
