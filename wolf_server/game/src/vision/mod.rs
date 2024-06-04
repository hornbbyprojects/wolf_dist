use crate::game::*;
use std::rc::Rc;
use wolf_interface::*;

pub struct WideVisionComponent {
    pub component_id: ComponentId,
    pub client_side_component_id: ClientSideComponentId,
}

impl Component for Rc<WideVisionComponent> {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        owner.remove_client_side_component(game, self.client_side_component_id);
    }
}

impl WideVisionComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
        let component_id = game.get_id();
        let client_side_component_id = owner_id.add_client_side_component_with_visibility(
            game,
            CreateComponentData::WideVision,
            ClientSideComponentVisibility::BoundPlayerOnly,
        );
        let comp = Rc::new(WideVisionComponent {
            component_id,
            client_side_component_id,
        });
        owner_id.add_component(game, comp);
        component_id
    }
}
