use crate::game::*;

mod delete_after;
pub use delete_after::*;
mod remove_component_after;
pub use remove_component_after::*;

//a system for the boring stuff
pub struct GenericSystem {
    delete_afters: IdMap<DeleteAtId, DeleteAt>,
    remove_component_afters: IdMap<RemoveComponentAtId, RemoveComponentAt>,
}

impl GenericSystem {
    pub fn new() -> Self {
        GenericSystem {
            delete_afters: IdMap::new(),
            remove_component_afters: IdMap::new(),
        }
    }
    pub fn step(game: &mut Game) {
        DeleteAt::step(game);
        RemoveComponentAt::step(game);
    }
}
