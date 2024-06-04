use super::*;
use combinable::CantCombine;
use signal_listener_macro::define_signal_listener;

define_signal_listener!(GetHealthProportion, &Game -> CantCombine<f64>);

#[derive(Clone)]
pub struct HealthProportionComponent {
    pub component_id: ComponentId,
    pub health_proportion: f64,
}

impl GetHealthProportionSignalListener for HealthProportionComponent {
    fn clone_box(&self) -> Box<dyn GetHealthProportionSignalListener> {
        Box::new(self.clone())
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn receive_get_health_proportion_signal(
        &self,
        game: &Game,
        owner: GameObjectId,
    ) -> CantCombine<f64> {
        CantCombine(self.health_proportion)
    }
}

impl Component for HealthProportionComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_get_health_proportion_signal_listener(game, self.component_id)
    }
}
impl HealthProportionComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        component_id: ComponentId,
        health_proportion: f64,
    ) {
        let comp = HealthProportionComponent {
            component_id,
            health_proportion,
        };
        owner_id.add_component(game, comp.clone());
        owner_id.add_get_health_proportion_signal_listener(game, comp);
    }
}
