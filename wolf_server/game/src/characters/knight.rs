use wolf_hash_map::WolfHashMap;

use super::*;
use crate::abilities::*;
use crate::drawable::*;
use crate::resources::*;

pub const KNIGHT_SPEED: f64 = 6.0;
#[derive(Clone)]
struct KnightCharacterComponentId(ComponentId);

pub struct KnightCharacterComponent {
    pub component_id: ComponentId,
    pub coloured_component_id: ComponentId,
    pub drawable_component_id: ComponentId,
    pub base_character_component_id: ComponentId,
    pub resource_holder_component_id: ComponentId,
    pub resource_collector_component_id: ComponentId,
}

impl KnightCharacterComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        remove_character(game, owner_id);
        let component_id = game.get_id();
        let coloured_component_id = add_random_colour(game, owner_id);

        let mut sprites = WolfHashMap::new();
        sprites.insert(CardinalDirection::Left, KNIGHT_SPRITE_LEFT);
        sprites.insert(CardinalDirection::Right, KNIGHT_SPRITE_RIGHT);
        sprites.insert(CardinalDirection::Up, KNIGHT_SPRITE_UP);
        sprites.insert(CardinalDirection::Down, KNIGHT_SPRITE_DOWN);
        let drawable_component_id =
            FacingSpriteComponent::add_to(game, owner_id, sprites, DEFAULT_DEPTH);

        let base_character_component_id = BaseCharacterComponent::add_to(
            game,
            owner_id,
            KNIGHT_SPEED,
            vec![
                AbilityTypeId::BuildingId,
                AbilityTypeId::RailgunId,
                AbilityTypeId::AmbushId,
                AbilityTypeId::PlaneWalkId,
                AbilityTypeId::SprintId,
            ],
            vec![
                game.allegiance_system
                    .special_allegiances
                    .villager_allegiance,
            ],
        );

        let resource_holder_component_id =
            ResourceHolderComponent::add_to(game, owner_id).get_component_id();
        let resource_collector_component_id = ResourceCollectorComponent::add_to(game, owner_id);

        let comp = KnightCharacterComponent {
            component_id,
            drawable_component_id,
            base_character_component_id,
            coloured_component_id,
            resource_holder_component_id,
            resource_collector_component_id,
        };
        owner_id.add_component(game, comp);
        owner_id.add_get_character_component_ids_signal_listener(
            game,
            KnightCharacterComponentId(component_id),
        );
    }
}

impl Component for KnightCharacterComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_component(game, self.coloured_component_id);
        owner_id.remove_component(game, self.drawable_component_id);
        owner_id.remove_component(game, self.base_character_component_id);
        owner_id.remove_component(game, self.resource_holder_component_id);
        owner_id.remove_component(game, self.resource_collector_component_id);
        owner_id.remove_get_character_component_ids_signal_listener(game, self.component_id);
    }
}

impl GetCharacterComponentIdsSignalListener for KnightCharacterComponentId {
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
