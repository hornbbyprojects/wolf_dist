pub use crate::abilities::AbilitySystem;
use crate::ai::AiSystem;
use crate::allegiance::AllegianceSystem;
pub use crate::ants::AntSystem;
pub use crate::behaviour::BehaviourSystem;
use crate::biomes::BiomeSystem;
pub use crate::collisions::{CollisionGroupId, CollisionSystem};
pub use crate::component::*;
pub use crate::damage::*;
pub use crate::drawable::*;
pub use crate::game_object::*;
use crate::generic::GenericSystem;
use crate::hunting::HuntingSystem;
pub use crate::id_types::*;
use crate::loading::LoadingSystem;
use crate::monsters::Monsters;
pub use crate::movement::MovementSystem;
pub use crate::movement::*;
pub use crate::necromancy::NecromancySystem;
pub use crate::player::*;
use crate::quest::{spawn_quest_guide, QuestSystem};
use crate::resources::ResourceSystem;
use crate::solids::SolidSystem;
pub use crate::spatial_map::*;
use crate::speech::Speech;
use crate::terrain::Terrain;
use crate::timers::TimerSystem;
pub use crate::utilities::*;
pub use crate::villages::VillagesSystem;
pub use crate::wildlife::WildlifeSystem;

pub use coords::*;
pub use id::*;
pub use rand::Rng;
pub use sprite_mappings::*;
pub use std::collections::*;
#[cfg(feature = "timing")]
pub use std::time::{Duration, Instant};

#[cfg(feature = "timing")]
pub const TIME_EVERY: u32 = 100;

#[cfg(feature = "timing")]
use once_cell::sync::Lazy;

#[cfg(feature = "timing")]
pub static TIMINGS: Lazy<std::sync::Mutex<wolf_hash_map::WolfHashMap<String, Duration>>> =
    Lazy::new(|| std::sync::Mutex::new(wolf_hash_map::WolfHashMap::new()));

#[macro_export]
macro_rules! time_system {
    ($e: expr) => {{
        #[cfg(feature = "timing")]
        let start_time = Instant::now();
        let return_value = $e;
        #[cfg(feature = "timing")]
        let passed = Instant::now() - start_time;
        #[cfg(feature = "timing")]
        let old_time = TIMINGS
            .lock()
            .unwrap()
            .remove(stringify!($e))
            .unwrap_or(Duration::from_secs(0));
        #[cfg(feature = "timing")]
        let time_taken = old_time + passed;
        #[cfg(feature = "timing")]
        TIMINGS
            .lock()
            .unwrap()
            .insert(stringify!($e).to_string(), time_taken);
        return_value
    }};
}

pub struct Game {
    id_counter: u32,
    pub tick_counter: u32,
    plane_counter: u32,

    pub game_objects: IdMap<GameObjectId, GameObject>,

    pub to_delete: Vec<GameObjectId>,

    pub ability_system: AbilitySystem,

    pub ai_system: AiSystem,

    pub allegiance_system: AllegianceSystem,

    pub ant_system: AntSystem,

    pub behaviour_system: BehaviourSystem,

    pub biome_system: BiomeSystem,

    pub collision_system: CollisionSystem,

    pub damage_system: DamageSystem,

    pub generic_system: GenericSystem,

    pub hunting_system: HuntingSystem,

    pub loading_system: LoadingSystem,

    pub monsters: Monsters,

    pub movement_system: MovementSystem,

    pub necromancy_system: NecromancySystem,

    pub player_system: PlayerSystem,

    pub quest_system: QuestSystem,

    pub resource_system: ResourceSystem,

    pub solid_system: SolidSystem,

    pub speeches: IdMap<GameObjectId, Speech>,

    pub terrain: Terrain,

    pub timer_system: TimerSystem,

    pub villages_system: VillagesSystem,

    pub wildlife_system: WildlifeSystem,
}
impl Game {
    pub fn get_id<T: From<u32>>(&mut self) -> T {
        let ret = self.id_counter;
        self.id_counter += 1;
        ret.into()
    }
    pub fn get_plane(&mut self) -> Plane {
        self.plane_counter += 1;
        Plane(self.plane_counter)
    }
    pub fn new() -> Self {
        Game {
            id_counter: 0,
            tick_counter: 0,
            plane_counter: 1,

            generic_system: GenericSystem::new(),

            game_objects: IdMap::new(),

            to_delete: Vec::new(),

            ability_system: AbilitySystem::new(),

            ai_system: AiSystem::new(),

            ant_system: AntSystem::new(),

            allegiance_system: AllegianceSystem::new(),

            biome_system: BiomeSystem::new(),

            damage_system: DamageSystem::new(),

            collision_system: CollisionSystem::new(),

            hunting_system: HuntingSystem::new(),

            loading_system: LoadingSystem::new(),

            behaviour_system: BehaviourSystem::new(),

            monsters: Monsters::new(),

            movement_system: MovementSystem::new(),

            necromancy_system: NecromancySystem::new(),

            player_system: PlayerSystem::new(),

            quest_system: QuestSystem::new(),

            resource_system: ResourceSystem::new(),

            speeches: IdMap::new(),

            solid_system: SolidSystem::new(),

            terrain: Terrain::new(),

            timer_system: TimerSystem::new(),

            villages_system: VillagesSystem::new(),

            wildlife_system: WildlifeSystem::new(),
        }
    }
    pub fn initialise(&mut self) {
        spawn_quest_guide(self);
    }

    //naming convention: step for normal things
    //pre-movement for things that need to happen after step and just before the movement phase
    //post-movement for things that need to happen just after the movement phase
    //end-step for things that must happen last
    pub fn step(&mut self) {
        #[cfg(feature = "timing")]
        {
            if self.tick_counter % TIME_EVERY == 0 {
                println!("------------------------------------------------------");
            };
        }
        #[cfg(feature = "timing")]
        let tick_start = Instant::now();

        time_system!(BehaviourSystem::step(self));

        time_system!(TimerSystem::step(self));

        time_system!(GenericSystem::step(self));

        time_system!(AiSystem::step(self));

        time_system!(AntSystem::step(self));

        time_system!(Projectile::step(self));

        time_system!(DamageSystem::step(self));

        time_system!(Monsters::step(self));

        time_system!(WildlifeSystem::step(self));

        time_system!(Terrain::step(self));

        time_system!(ResourceSystem::step(self));

        time_system!(AbilitySystem::step(self));

        time_system!(NecromancySystem::step(self));

        time_system!(BiomeSystem::step(self));

        time_system!(Speech::step(self));

        time_system!(VillagesSystem::step(self));

        time_system!(QuestSystem::step(self));

        time_system!(MovementSystem::pre_movement(self));

        time_system!(SolidSystem::pre_movement(self));

        time_system!(MovementSystem::movement(self));

        time_system!(CollisionSystem::post_movement(self));

        time_system!(SolidSystem::post_movement(self));

        time_system!(PlayerSystem::end_step(self));

        time_system!(self.delete_objects());

        #[cfg(feature = "timing")]
        let tick_end = Instant::now();
        #[cfg(feature = "timing")]
        let time_taken = tick_end - tick_start;
        #[cfg(feature = "timing")]
        {
            let old_time = TIMINGS
                .lock()
                .unwrap()
                .remove("tick")
                .unwrap_or(Duration::from_secs(0));
            TIMINGS
                .lock()
                .unwrap()
                .insert("tick".to_string(), old_time + time_taken);
        }
        #[cfg(feature = "timing")]
        {
            if self.tick_counter % TIME_EVERY == 0 {
                let mut timings = TIMINGS.lock().unwrap();
                let mut total_time_taken = None;
                let mut results = Vec::new();
                for (name, timing) in timings.drain() {
                    let insert_pos = match results.binary_search_by_key(&timing, |(_, t)| *t) {
                        Ok(x) => x,
                        Err(x) => x,
                    };
                    results.insert(insert_pos, (name, timing));
                }
                for (name, time_taken) in results {
                    if name == "tick" {
                        total_time_taken = Some(time_taken);
                    } else {
                        println!(
                            "{} took {:?} over {} ticks (average {:?})",
                            name,
                            time_taken,
                            TIME_EVERY,
                            time_taken / TIME_EVERY
                        );
                    }
                }
                println!(
                    "\nTotal: {:?} over {} ticks (average {:?})",
                    total_time_taken.unwrap(),
                    TIME_EVERY,
                    total_time_taken.unwrap() / TIME_EVERY
                );
            };
        }
        self.tick_counter += 1;
    }
    fn delete_objects(&mut self) {
        if self.to_delete.is_empty() {
            return;
        }
        let to_delete = std::mem::replace(&mut self.to_delete, Vec::new());
        for id in to_delete {
            GameObject::_remove(self, id);
        }
    }
}
