use super::*;
use signal_listener_macro::define_signal_listener;

define_signal_listener!(GetVisionScale, &Game -> CantCombine<f64>);

#[derive(Clone)]
pub struct WideVisionComponent {
    pub component_id: ComponentId,
}

impl Component for WideVisionComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_get_vision_scale_signal_listener(game, self.component_id);
    }
}
impl GetVisionScaleSignalListener for WideVisionComponent {
    fn clone_box(&self) -> Box<dyn GetVisionScaleSignalListener> {
        Box::new(self.clone())
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn receive_get_vision_scale_signal(
        &self,
        _game: &Game,
        _owner: GameObjectId,
    ) -> CantCombine<f64> {
        CantCombine(2.0)
    }
}

impl WideVisionComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId, component_id: ComponentId) {
        let comp = WideVisionComponent { component_id };
        owner_id.add_component(game, comp.clone());
        owner_id.add_get_vision_scale_signal_listener(game, comp);
    }
}
