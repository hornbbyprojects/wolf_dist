use wolf_hash_map::WolfHashMap;

use super::*;
use crate::collisions::CollisionSystem;
use crate::combinable::OrBool;

define_signal_listener!(DamagerFizzledOut, &mut Game, damager_id: DamagerId);
define_signal_listener!(DealtDamage, &mut Game, target_id: GameObjectId);
/*
Returns whether damage is BLOCKED.
*/
define_signal_listener!(AboutToBeDamaged, &mut Game, damager_id: GameObjectId, firer_id: GameObjectId -> OrBool);

pub struct Damager {
    pub game_object_id: GameObjectId,
    pub firer_id: GameObjectId,
    pub hits_remaining: Option<u32>,
    pub damage: Health,
    pub last_hurt_timings: WolfHashMap<GameObjectId, u32>,
}

const DAMAGE_COOLDOWN: u32 = 50;
impl Damager {
    pub fn new(
        game: &mut Game,
        game_object_id: GameObjectId,
        firer_id: GameObjectId,
        hits_remaining: Option<u32>,
        damage: Health,
    ) -> DamagerId {
        let id = game.get_id();
        let damager = Damager {
            game_object_id,
            firer_id,
            hits_remaining,
            damage,
            last_hurt_timings: WolfHashMap::new(),
        };
        game.damage_system.damagers.insert(id, damager);
        id
    }
    pub fn remove(game: &mut Game, damager_id: DamagerId) {
        game.damage_system.damagers.remove(damager_id);
    }
    pub fn step(game: &mut Game) {
        let mut to_hit = Vec::new();
        let mut new_hits_remaining = Vec::new();
        let mut fizzled_out = Vec::new();
        {
            let collision_group =
                match CollisionSystem::get_collision_group(game, CollisionGroupId::Damageable) {
                    Some(x) => x,
                    None => return, //nothing to hit
                };
            let collision_map = collision_group.collision_map.borrow();
            for (id, damager) in game.damage_system.damagers.iter() {
                if let Some(ref hits_remaining) = damager.hits_remaining {
                    if *hits_remaining == 0 {
                        new_hits_remaining.push((id, Some(0))); //so we fizzle out
                        continue;
                    }
                }
                let mut hit_something = false;
                let mut hits_remaining = damager.hits_remaining.clone();
                let colliding = collision_map
                    .get_colliding_game(game, damager.game_object_id.get_hit_box(game));
                if colliding.is_empty() {
                    continue;
                }
                let allegiances = damager.firer_id.get_allegiances(game);
                for hit in colliding {
                    let damageable_allegiances = hit.get_allegiances(game);
                    if !game
                        .allegiance_system
                        .can_hurt_multiple(&allegiances, &damageable_allegiances)
                    {
                        continue;
                    }
                    if let Some(last_hit) = damager.last_hurt_timings.get(&hit) {
                        if last_hit + DAMAGE_COOLDOWN > game.tick_counter {
                            continue;
                        }
                    }
                    to_hit.push((id, hit));
                    if let Some(ref mut hits_remaining) = hits_remaining {
                        hit_something = true;
                        *hits_remaining -= 1;
                        if *hits_remaining == 0 {
                            break;
                        }
                    }
                }
                if hit_something {
                    new_hits_remaining.push((id, hits_remaining));
                }
            }
        }
        for (damager_id, hit_id) in to_hit {
            let (damager_object_id, firer_id, damage) = {
                let damager = game.damage_system.damagers.get_mut(damager_id).unwrap();
                damager.last_hurt_timings.insert(hit_id, game.tick_counter);
                (damager.game_object_id, damager.firer_id, damager.damage)
            };
            let blocked = hit_id
                .send_about_to_be_damaged_signal(game, damager_object_id, firer_id)
                .map(OrBool::extract)
                .unwrap_or(false);
            if !blocked {
                hit_id.send_damage_signal(game, damage);
                damager_object_id.send_dealt_damage_signal(game, hit_id);
            }
        }
        for (id, hits_remaining) in new_hits_remaining {
            let damager = game.damage_system.damagers.get_mut(id).unwrap();
            damager.hits_remaining = hits_remaining;
            if let Some(ref hits_remaining) = damager.hits_remaining {
                if *hits_remaining == 0 {
                    fizzled_out.push(id);
                }
            }
        }
        for id in fizzled_out {
            //in case another signal listener kills the damager
            let game_object_id = if let Some(damager) = game.damage_system.damagers.get(id) {
                damager.game_object_id
            } else {
                continue;
            };
            game_object_id.send_damager_fizzled_out_signal(game, id);
        }
    }
}

#[derive(Clone)]
pub struct DamagerComponent {
    pub component_id: ComponentId,
    pub damager_id: DamagerId,
}

impl DamagerComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        firer_id: GameObjectId,
        hits_remaining: Option<u32>,
        damage: i32,
    ) -> Self {
        let component_id = game.get_id();
        let damager_id = Damager::new(game, owner_id, firer_id, hits_remaining, Health(damage));
        let comp = DamagerComponent {
            component_id,
            damager_id,
        };
        owner_id.add_component(game, comp.clone());
        comp
    }
}
impl Component for DamagerComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        game.damage_system.damagers.remove(self.damager_id);
    }
}
