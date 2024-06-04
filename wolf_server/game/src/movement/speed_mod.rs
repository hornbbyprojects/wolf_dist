use super::*;
use crate::combinable::Added;
use signal_listener_macro::define_signal_listener;

define_signal_listener!(GetSpeedMod, &Game -> Added<f64>);

pub struct SpeedModComponentId(ComponentId);

#[derive(Clone)]
pub struct SpeedModComponent {
    component_id: ComponentId,
    speed_mod: f64,
}

impl GetSpeedModSignalListener for SpeedModComponent {
    fn receive_get_speed_mod_signal(&self, _game: &Game, _owner_id: GameObjectId) -> Added<f64> {
        Added(self.speed_mod)
    }
    fn clone_box(&self) -> Box<dyn GetSpeedModSignalListener> {
        Box::new(self.clone())
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
}

impl SpeedModComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId, speed_mod: f64) -> ComponentId {
        let component_id = game.get_id();
        let comp = SpeedModComponent {
            component_id,
            speed_mod,
        };
        owner_id.add_component(game, SpeedModComponentId(component_id));
        owner_id.add_get_speed_mod_signal_listener(game, comp);
        component_id
    }
}

impl Component for SpeedModComponentId {
    fn get_component_id(&self) -> ComponentId {
        self.0
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_get_speed_mod_signal_listener(game, self.0);
    }
}
