use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct GoalImportance(pub u32);

pub enum GoalResult {
    SimplePlan(Box<dyn SimplePlan>),
    Needs(Needs),
    Failure,
}

pub trait Goal {
    fn get_importance(&self, game: &Game, owner_id: GameObjectId) -> Option<GoalImportance>; //if none, don't do at all
    fn get_method(&self, game: &Game, owner_id: GameObjectId) -> GoalResult;
}
