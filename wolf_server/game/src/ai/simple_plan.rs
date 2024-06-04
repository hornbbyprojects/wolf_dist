use super::*;

pub trait SimplePlan {
    fn step(&mut self, _game: &mut Game, _owner_id: GameObjectId) -> ActionResult;
    fn stop(&self, _game: &mut Game, _owner_id: GameObjectId) {}
}
