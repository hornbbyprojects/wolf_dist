use crate::abilities::AbilityTypeId;
use crate::abilities::BasicAbilityUserComponent;
use crate::allegiance::AllegianceComponent;
use crate::behaviour::*;
use crate::game::*;
use crate::resources::ResourceHolderComponent;
use crate::resources::ResourceType;
use std::f64::consts::PI;

mod build_house;
mod building_ability;
mod city_block;
mod guards;
mod layout;
mod needs_deconstruction;
mod reproduce;
mod reservations;
mod scaffold;
mod tavern;
mod villager;

pub use build_house::*;
pub use building_ability::*;
pub use city_block::*;
pub use guards::*;
pub use layout::*;
pub use needs_deconstruction::*;
pub use reproduce::*;
pub use reservations::*;
pub use scaffold::*;
pub use tavern::*;
pub use villager::*;
use wolf_hash_map::WolfHashMap;
use wolf_hash_map::WolfHashSet;

const VILLAGER_SPEED: f64 = 2.0;

pub struct VillagesSystem {
    build_house_behaviours: IdMap<BehaviourId, BuildHouseBehaviour>,
    reproduce_behaviours: IdMap<BehaviourId, ReproduceBehaviour>,
    looking_for_partner: WolfHashSet<BehaviourId>,
    villager_behaviours: IdMap<BehaviourId, VillagerBehaviour>,
    villager_minds: IdMap<VillagerMindId, VillagerMind>,
    doors_map: IdMap<GameObjectId, GameObjectId>,
    guard_minds: IdMap<MindId, GuardMind>,
    squads: IdMap<SquadId, Squad>,
    reserved: WolfHashSet<CityBlockChunkCoords>,
    city_block_spiraler: Spiraler<CITY_BLOCK_SIZE>,
    next_city_block: Option<CityBlock>,
    needs_deconstruction: IdMap<NeedsDeconstructionId, NeedsDeconstruction>,
    unassigned_needs_deconstruction: WolfHashSet<NeedsDeconstructionId>,
    needs_deconstruction_lookup:
        WolfHashMap<CityBlockChunkCoords, WolfHashSet<NeedsDeconstructionId>>,
    deconstruct_behaviours: IdMap<BehaviourId, DeconstructBehaviour>,
    scaffolds: IdMap<ScaffoldId, Scaffold>,
    unassigned_scaffolds: WolfHashSet<ScaffoldId>,
    build_scaffold_behaviours: IdMap<BehaviourId, BuildScaffoldBehaviour>,
}

impl VillagesSystem {
    pub fn new() -> Self {
        VillagesSystem {
            build_house_behaviours: IdMap::new(),
            villager_behaviours: IdMap::new(),
            reproduce_behaviours: IdMap::new(),
            looking_for_partner: WolfHashSet::new(),
            villager_minds: IdMap::new(),
            doors_map: IdMap::new(),
            guard_minds: IdMap::new(),
            squads: IdMap::new(),
            reserved: WolfHashSet::new(),
            city_block_spiraler: Spiraler::new(),
            next_city_block: None,
            needs_deconstruction: IdMap::new(),
            unassigned_needs_deconstruction: WolfHashSet::new(),
            needs_deconstruction_lookup: WolfHashMap::new(),
            deconstruct_behaviours: IdMap::new(),
            scaffolds: IdMap::new(),
            unassigned_scaffolds: WolfHashSet::new(),
            build_scaffold_behaviours: IdMap::new(),
        }
    }
    pub fn is_reserved(&self, coords: SquareCoords) -> bool {
        self.reserved.contains(&coords.into())
    }
    pub fn step(game: &mut Game) {
        VillagerMind::step(game);
        VillagerBehaviour::step(game);
        ReproduceBehaviour::step(game);
        BuildHouseBehaviour::step(game);
        BuildScaffoldBehaviour::step(game);
        DeconstructBehaviour::step(game);
        GuardMind::step(game);
        Squad::step(game);
    }
    pub fn create_villager(game: &mut Game, coords: PixelCoords) {
        let game_object_id = GameObject::create_game(game, coords);
        WalkerComponent::add_to(game, game_object_id, VILLAGER_SPEED, VILLAGER_SPEED / 2.0);
        DamageableComponent::add_to(game, game_object_id);
        BasicDrawingComponent::add_to(game, game_object_id, VILLAGER_SPRITE, DEFAULT_DEPTH);
        DieOnNoHealthComponent::add_to(game, game_object_id);
        DeleteOnDeathComponent::add_to(game, game_object_id);
        ResourceHolderComponent::add_to(game, game_object_id);
        let allegiances = vec![
            game.allegiance_system
                .special_allegiances
                .villager_allegiance,
        ];
        AllegianceComponent::add_to(game, game_object_id, allegiances);
        BasicAbilityUserComponent::add_to(game, game_object_id, vec![AbilityTypeId::FireballId]);
        add_health_bar(game, game_object_id);
        VillagerMind::new(game, game_object_id);
    }
}
