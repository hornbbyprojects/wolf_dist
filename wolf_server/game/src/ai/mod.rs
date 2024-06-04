use crate::game::*;

mod action;
mod ai;
mod component;
mod goal;
mod needs;
mod planner;
mod planner_state;
mod simple_plan;

pub use action::*;
pub use ai::*;
pub use component::*;
pub use goal::*;
pub use needs::*;
pub use planner::*;
pub use planner_state::*;
pub use simple_plan::*;

pub struct AiSystem {
    ais: IdMap<AiId, Ai>,
}

impl AiSystem {
    pub fn new() -> AiSystem {
        AiSystem { ais: IdMap::new() }
    }
    pub fn step(game: &mut Game) {
        Ai::step(game);
    }
}
