use crate::game::*;
mod spawner;
use spawner::*;
mod zombie;
pub use zombie::*;
mod wolf;
pub use wolf::*;

pub struct Monsters {
    wolf_system: WolfSystem,
}

impl Monsters {
    pub fn new() -> Self {
        Monsters {
            wolf_system: WolfSystem::new(),
        }
    }
    pub fn step(game: &mut Game) {
        Spawner::step(game);
        WolfSystem::step(game);
    }
}
