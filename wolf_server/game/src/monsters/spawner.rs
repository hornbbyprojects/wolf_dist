use crate::game::*;
use crate::monsters::add_zombie;
use rand::Rng;

const SPAWN_WIDTH: i32 = 200;
const ZOMBIE_SPAWN_EVERY: u32 = 50000;

pub struct Spawner {}

impl Spawner {
    pub fn step(game: &mut Game) {
        if (game.tick_counter + 1) % ZOMBIE_SPAWN_EVERY == 0 {
            let x = rand::thread_rng().gen_range(-SPAWN_WIDTH..SPAWN_WIDTH);
            let y = rand::thread_rng().gen_range(-SPAWN_WIDTH..SPAWN_WIDTH);
            add_zombie(game, PixelCoords::new_to_fixed(Plane(0), x, y));
        }
    }
}
