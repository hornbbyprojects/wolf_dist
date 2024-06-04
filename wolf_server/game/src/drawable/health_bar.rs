use crate::combinable::CombinedVecs;
use crate::game::*;
use wolf_interface::*;

#[derive(Clone)]
pub struct HealthBarComponent {
    component_id: ComponentId,
}

impl GetClientSideComponentsSignalListener for HealthBarComponent {
    fn clone_box(&self) -> Box<dyn GetClientSideComponentsSignalListener> {
        Box::new(self.clone())
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn receive_get_client_side_components_signal(
        &self,
        game: &Game,
        owner: GameObjectId,
        player_id: PlayerId,
    ) -> CombinedVecs<(ComponentId, u32)> {
        CombinedVecs(vec![(self.component_id, 0)])
    }
}

impl GetClientSideComponentCreateMessageSignalListener for HealthBarComponent {
    fn clone_box(&self) -> Box<dyn GetClientSideComponentCreateMessageSignalListener> {
        Box::new(self.clone())
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn receive_get_client_side_component_create_message_signal(
        &self,
        game: &Game,
        owner: GameObjectId,
        player_id: PlayerId,
    ) -> CombinedVecs<CreateComponentMessage> {
        let message = CreateComponentMessage {
            game_object_id: owner.into(),
            component_id: self.component_id.into(),
            data: CreateComponentData::HealthBar,
        };
        CombinedVecs(vec![message])
    }
}

impl Component for HealthBarComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        owner.remove_get_client_side_components_signal_listener(game, self.component_id);
    }
}

impl HealthBarComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
        let component_id = game.get_id();
        let comp = HealthBarComponent { component_id };
        owner_id.add_get_client_side_components_signal_listener(game, comp);
        owner_id.add_component(game, comp);
        component_id
    }
}
