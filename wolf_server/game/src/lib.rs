#![allow(incomplete_features)]
#![feature(proc_macro_hygiene)]
#![feature(step_trait)]
#![feature(entry_insert)]
#![feature(hash_raw_entry)]
mod abilities;
mod ai;
mod allegiance;
mod ants;
mod basic_body;
mod basic_client_side_component;
mod behaviour;
mod biomes;
mod characters;
mod chunk_map;
mod collisions;
mod combinable;
mod component;
mod damage;
mod drawable;
mod game;
mod game_object;
mod generic;
mod hunting;
mod id_types;
mod loading;
mod loot;
mod meta;
mod monsters;
mod movement;
mod necromancy;
mod player;
mod quest;
mod resources;
mod solids;
mod spatial_map;
mod speech;
mod statuses;
mod terrain;
mod timers;
mod utilities;
mod villages;
mod vision;
mod wildlife;

pub use self::game::*;
pub use self::id_types::*;
