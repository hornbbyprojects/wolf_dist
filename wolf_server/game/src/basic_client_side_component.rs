use crate::game::*;
use wolf_interface::*;

#[derive(Clone)]
pub struct BasicClientSideComponent {
    pub component_id: ComponentId,
    pub client_side_component_id: ClientSideComponentId,
}

impl Component for BasicClientSideComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        owner.remove_client_side_component(game, self.client_side_component_id);
    }
}

impl BasicClientSideComponent {
    pub fn add_to(
        game: &mut Game,
        owner: GameObjectId,
        data: CreateComponentData,
    ) -> BasicClientSideComponent {
        let component_id = game.get_id();
        let client_side_component_id = owner.add_client_side_component(game, data);
        let comp = BasicClientSideComponent {
            component_id,
            client_side_component_id,
        };
        owner.add_component(game, comp.clone());
        comp
    }
}
