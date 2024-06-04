use super::*;

pub struct RemoveComponentAt {
    game_object_id: GameObjectId,
    parent_component_id: ComponentId,
    component_id_to_remove: ComponentId,
    remove_component_at: u32,
}

impl RemoveComponentAt {
    pub fn step(game: &mut Game) {
        let mut to_remove_component = Vec::new();
        for (_id, remove_component_after) in game.generic_system.remove_component_afters.iter() {
            if game.tick_counter >= remove_component_after.remove_component_at {
                to_remove_component.push((
                    remove_component_after.game_object_id,
                    remove_component_after.component_id_to_remove,
                    remove_component_after.parent_component_id,
                ));
            }
        }
        for (id, component_id, parent_component_id) in to_remove_component {
            id.remove_component(game, component_id);
            id.remove_component(game, parent_component_id);
        }
    }
    pub fn new(
        game: &mut Game,
        game_object_id: GameObjectId,
        remove_component_at: u32,
        parent_component_id: ComponentId,
        component_id_to_remove: ComponentId,
    ) -> RemoveComponentAtId {
        let id = game.get_id();
        let remove_component_after = RemoveComponentAt {
            game_object_id,
            remove_component_at,
            parent_component_id,
            component_id_to_remove,
        };
        game.generic_system
            .remove_component_afters
            .insert(id, remove_component_after);
        id
    }
    pub fn remove(game: &mut Game, id: RemoveComponentAtId) {
        game.generic_system.remove_component_afters.remove(id);
    }
}
pub struct RemoveComponentAtComponent {
    pub component_id: ComponentId,
    pub remove_component_after_id: RemoveComponentAtId,
}

impl RemoveComponentAtComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        time: u32,
        component_id_to_remove: ComponentId,
    ) -> ComponentId {
        let component_id = game.get_id();
        let remove_component_after_id =
            RemoveComponentAt::new(game, owner_id, time, component_id, component_id_to_remove);
        let comp = RemoveComponentAtComponent {
            component_id,
            remove_component_after_id,
        };
        owner_id.add_component(game, comp);
        component_id
    }
}

impl Component for RemoveComponentAtComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        RemoveComponentAt::remove(game, self.remove_component_after_id);
    }
}
