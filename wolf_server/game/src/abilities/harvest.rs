use crate::game::*;
use crate::resources::HarvestSignalSender;

use super::Ability;

pub const MAX_HARVEST_ANCHOR_DISTANCE: f64 = 100.0;
const HARVEST_TIME: u32 = 100;

pub struct Harvester {
    game_object_id: GameObjectId,
    owner_id: GameObjectId,  // Claims the rewards
    anchor_id: GameObjectId, // Disappears if far from this
    harvest_countdown: u32,
}

impl Harvester {
    pub fn new(
        game: &mut Game,
        owner_id: GameObjectId,
        starting_coords: PixelCoords,
    ) -> HarvesterId {
        let game_object_id = GameObject::create_game(game, starting_coords);
        BasicDrawingComponent::add_to(game, game_object_id, APPLE_SPRITE, PROJECTILE_DEPTH);
        let harvester_id = game.get_id();
        let harvester = Harvester {
            game_object_id,
            owner_id,
            anchor_id: owner_id,
            harvest_countdown: HARVEST_TIME,
        };
        game.ability_system
            .harvesters
            .insert(harvester_id, harvester);
        harvester_id
    }
    pub fn step(game: &mut Game) {
        let mut to_delete = Vec::new();
        let mut to_harvest = Vec::new();
        let mut to_tick_down = Vec::new();
        {
            for (id, harvester) in game.ability_system.harvesters.iter() {
                if harvester.anchor_id.is_deleted(&game.game_objects) {
                    to_delete.push(id);
                    continue;
                }
                let distance_from_anchor = harvester
                    .game_object_id
                    .get_distance_to_game(game, &harvester.anchor_id);
                if distance_from_anchor > MAX_HARVEST_ANCHOR_DISTANCE {
                    to_delete.push(id);
                    continue;
                }
                if let Some(game_object) = game.game_objects.get(harvester.game_object_id) {
                    let hit_box = game_object.get_hit_box();
                    if harvester.harvest_countdown == 0 {
                        let harvestables = CollisionSystem::get_colliding(
                            game,
                            CollisionGroupId::Harvestable,
                            hit_box,
                        );
                        to_harvest.push((harvester.owner_id, harvestables));
                        to_delete.push(id);
                    } else {
                        to_tick_down.push(id);
                    }
                } else {
                    to_delete.push(id)
                }
            }
        }
        for (owner_id, harvestables) in to_harvest {
            for harvestable_id in harvestables {
                harvestable_id.send_harvest_signal(game, owner_id);
            }
        }
        for id in to_tick_down {
            let harvester = game.ability_system.harvesters.get_mut(id).unwrap();
            harvester.harvest_countdown -= 1;
        }
        for id in to_delete {
            Harvester::remove(game, id);
        }
    }

    fn remove(game: &mut Game, id: HarvesterId) {
        if let Some(harvester) = game.ability_system.harvesters.remove(id) {
            harvester.game_object_id.remove(game);
        }
    }
}

pub struct HarvestAbility {
    ability_id: AbilityId,
}
impl HarvestAbility {
    pub fn new(ability_id: AbilityId) -> Self {
        HarvestAbility { ability_id }
    }
}
impl Ability for HarvestAbility {
    fn activate(&mut self, game: &mut Game, caster: GameObjectId, target_coords: PixelCoords) {
        Harvester::new(game, caster, target_coords);
    }

    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }
}
