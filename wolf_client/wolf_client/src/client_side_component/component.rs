use super::*;

pub trait Component {
    fn get_component_id(&self) -> ComponentId;
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId);
}

// Stores info on what a component is for removal purposes
pub enum ComponentInfo {
    Speech,
}
