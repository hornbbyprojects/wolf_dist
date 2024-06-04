use super::*;

#[derive(Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Debug)]
pub struct ActionCost(pub i32);

#[allow(dead_code)]
pub enum ActionResult {
    Failure,
    Success,
    Continue,
}

pub trait Action: std::fmt::Debug {
    fn step(&mut self, game: &mut Game, owner_id: GameObjectId) -> ActionResult;
    fn get_name(&self) -> String;
}

pub trait ActionSeed: std::fmt::Debug {
    fn get_action(self: Box<Self>, game: &Game, owner_id: GameObjectId) -> Option<Box<dyn Action>>; //returns None if can't germinate
}

pub trait ActionGenerator {
    //starting_state is used to load and cache data from the world
    fn generate_actions(
        &self,
        game: &Game,
        owner_id: GameObjectId,
        starting_state: &mut StartingPlannerState,
        state: &DerivedPlannerState,
        needs: &Needs,
    ) -> Vec<(DerivedPlannerState, Needs, ActionCost, Box<dyn ActionSeed>)>;
}

impl std::ops::Add<i32> for ActionCost {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        ActionCost(self.0 + rhs)
    }
}
impl std::ops::Add for ActionCost {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        ActionCost(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign<i32> for ActionCost {
    fn add_assign(&mut self, rhs: i32) {
        self.0 += rhs
    }
}

impl std::ops::Mul<i32> for ActionCost {
    type Output = ActionCost;
    fn mul(self, rhs: i32) -> Self::Output {
        ActionCost(self.0 * rhs)
    }
}
