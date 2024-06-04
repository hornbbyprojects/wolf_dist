use super::*;
use crate::damage::DeathSignalListener;
use std::rc::*;

//like harvestable, but not stored in a spatial map for harvesters to find
#[derive(Clone)]
pub struct ResourceDropperComponent {
    component_id: ComponentId,
    resources: Resources,
}

impl ResourceDropperComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId, resources: Resources) {
        let component_id = game.get_id();
        let comp = Rc::new(ResourceDropperComponent {
            component_id,
            resources,
        });
        owner_id.add_component(game, Rc::clone(&comp));
    }
}
impl DeathSignalListener for Rc<ResourceDropperComponent> {
    fn clone_box(&self) -> Box<(dyn DeathSignalListener + 'static)> {
        Box::new(Rc::clone(self))
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn receive_death_signal(&self, game: &mut Game, owner_id: GameObjectId) {
        let coords = owner_id.get_coords_game(game);
        create_resource_pickup(game, coords, self.resources.clone());
    }
}
impl Component for Rc<ResourceDropperComponent> {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_death_signal_listener(game, self.component_id);
    }
}
