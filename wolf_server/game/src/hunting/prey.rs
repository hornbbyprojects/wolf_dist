use super::*;

pub struct Prey {
    pub game_object_id: GameObjectId,
}

impl Prey {
    fn new(game: &mut Game, game_object_id: GameObjectId) -> PreyId {
        let id = game.get_id();
        let prey = Prey { game_object_id };
        let chunk_coords = game_object_id.get_chunk_coords(game);
        game.hunting_system.preys.insert(id, prey);
        game.hunting_system.prey_chunk_map.insert(id, chunk_coords);
        id
    }
    fn remove(game: &mut Game, prey_id: PreyId) {
        game.hunting_system.preys.remove(prey_id);
        game.hunting_system.prey_chunk_map.remove(prey_id);
    }
}

#[derive(Clone)]
pub struct PreyComponent {
    component_id: ComponentId,
    prey_id: PreyId,
}

impl PreyComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
        let component_id = game.get_id();
        let prey_id = Prey::new(game, owner_id);
        let comp = PreyComponent {
            component_id,
            prey_id,
        };
        owner_id.add_move_signal_listener(game, comp.clone());
        owner_id.add_component(game, comp);
        component_id
    }
}

impl Component for PreyComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_move_signal_listener(game, self.component_id);
        Prey::remove(game, self.prey_id);
    }
}

impl MoveSignalListener for PreyComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn MoveSignalListener> {
        Box::new(self.clone())
    }
    fn receive_move_signal(
        &self,
        game: &mut Game,
        owner_id: GameObjectId,
        _old_coords: &PixelCoords,
    ) {
        let chunk_coords = owner_id.get_chunk_coords(game);
        game.hunting_system
            .prey_chunk_map
            .move_item(self.prey_id, chunk_coords);
    }
}
