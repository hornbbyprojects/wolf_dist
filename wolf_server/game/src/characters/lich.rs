use super::*;
use crate::abilities::AbilityTypeId;
use crate::drawable::BasicDrawingComponent;

#[derive(Clone)]
struct LichCharacterComponentId(ComponentId);

pub struct LichCharacterComponent {
    pub component_id: ComponentId,
    pub basic_drawable_component_id: ComponentId,
    pub base_character_component_id: ComponentId,
}

const NECROMANCER_SPEED: f64 = 4.0;

impl LichCharacterComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        remove_character(game, owner_id);
        let component_id = game.get_id();

        let base_character_component_id = BaseCharacterComponent::add_to(
            game,
            owner_id,
            NECROMANCER_SPEED,
            vec![AbilityTypeId::NecroboltId, AbilityTypeId::CorpseTossId],
            vec![game.allegiance_system.special_allegiances.undead_allegiance],
        );

        let basic_drawable_component_id =
            BasicDrawingComponent::add_to(game, owner_id, NECROMANCER_SPRITE, DEFAULT_DEPTH)
                .component_id;

        let comp = LichCharacterComponent {
            component_id,
            basic_drawable_component_id,
            base_character_component_id,
        };
        owner_id.add_component(game, comp);
        owner_id.add_get_character_component_ids_signal_listener(
            game,
            LichCharacterComponentId(component_id),
        );
    }
}

impl Component for LichCharacterComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_component(game, self.basic_drawable_component_id);
        owner_id.remove_component(game, self.base_character_component_id);
        owner_id.remove_get_character_component_ids_signal_listener(game, self.component_id);
    }
}

impl GetCharacterComponentIdsSignalListener for LichCharacterComponentId {
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
