use std::{cell::RefCell, rc::Rc};

use wolf_hash_map::WolfHashMap;
use wolf_interface::CreateComponentData;

use super::*;
use crate::{abilities::*, combinable::OrBool, timers::TimerSystem};

const AFTER_IMAGE_EVERY: f64 = 20.0;
const AFTER_IMAGE_LIFESPAN: u32 = 0; //How long the first after image lasts (minus one increment)
const AFTER_IMAGE_LIFESPAN_INCREMENT: u32 = 1; // How much longer each successive image lasts
const PUNISH_SPACING: f64 = 40.0;
#[derive(Clone)]
struct PaladinCharacterComponentId(ComponentId);
/*
The PALADIN is a holy defender of all that is good.
Core mechanics:
Punishment) Charge to an enemy, and become invincible for 1 second
Abilities:
1) Holy Sword
Deals heavy damage
2) Holy Shield
Time correctly to block any hit.
Punish the source (implemented)
3) Holy Steed
Temporarily move at incredible speed.
Implementation issues: Sprite depth needs to increase, or horse needs to be unnaturally low
4) Holy Spear
Throw a spear, punishing those hit
*/

mod holy_steed;
pub use holy_steed::*;
mod holy_shield;
pub use holy_shield::*;
mod holy_slash;
pub use holy_slash::*;

pub struct PaladinCharacterComponent {
    pub component_id: ComponentId,
    pub base_character_component_id: ComponentId,
    pub coloured_component_id: ComponentId,
    pub drawable_component_id: ComponentId,
}
impl Component for PaladinCharacterComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        owner.remove_get_character_component_ids_signal_listener(game, self.component_id);
        owner.remove_component(game, self.base_character_component_id);
        owner.remove_component(game, self.coloured_component_id);
        owner.remove_component(game, self.drawable_component_id);
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}
impl GetCharacterComponentIdsSignalListener for PaladinCharacterComponentId {
    fn get_listener_id(&self) -> ComponentId {
        self.0
    }
    fn clone_box(&self) -> Box<dyn GetCharacterComponentIdsSignalListener> {
        Box::new(self.clone())
    }
    fn receive_get_character_component_ids_signal(
        &self,
        _game: &Game,
        _owner_id: GameObjectId,
    ) -> CantCombine<ComponentId> {
        CantCombine(self.0)
    }
}
impl PaladinCharacterComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        remove_character(game, owner_id);
        let component_id = game.get_id();
        let base_character_component_id = BaseCharacterComponent::add_to(
            game,
            owner_id,
            KNIGHT_SPEED,
            vec![
                AbilityTypeId::HolyShieldId,
                AbilityTypeId::DebugId,
                AbilityTypeId::HolySlashId,
            ],
            vec![
                game.allegiance_system
                    .special_allegiances
                    .villager_allegiance,
            ],
        );
        let coloured_component_id = add_random_colour(game, owner_id);

        let mut sprites = WolfHashMap::new();
        sprites.insert(CardinalDirection::Left, KNIGHT_SPRITE_LEFT);
        sprites.insert(CardinalDirection::Right, KNIGHT_SPRITE_RIGHT);
        sprites.insert(CardinalDirection::Up, KNIGHT_SPRITE_UP);
        sprites.insert(CardinalDirection::Down, KNIGHT_SPRITE_DOWN);
        let drawable_component_id =
            FacingSpriteComponent::add_to(game, owner_id, sprites, DEFAULT_DEPTH);
        let component = PaladinCharacterComponent {
            component_id,
            base_character_component_id,
            coloured_component_id,
            drawable_component_id,
        };
        owner_id.add_get_character_component_ids_signal_listener(
            game,
            PaladinCharacterComponentId(component_id),
        );
        owner_id.add_component(game, component);
    }
}
