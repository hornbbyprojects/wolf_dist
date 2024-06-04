use crate::game::*;

mod locomotion_mode;
pub use locomotion_mode::*;
mod sticky_facer;
pub use sticky_facer::*;
mod hopper;
mod walker;
pub use hopper::*;
pub use walker::*;

pub struct LocomotionSystem {
    walkers: IdMap<WalkerId, Walker>,
    hoppers: IdMap<HopperId, Hopper>,
}

impl LocomotionSystem {
    pub fn new() -> Self {
        LocomotionSystem {
            walkers: IdMap::new(),
            hoppers: IdMap::new(),
        }
    }
    pub fn step(game: &mut Game) {
        Walker::step(game);
        Hopper::step(game);
    }
}
