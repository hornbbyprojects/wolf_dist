use super::*;

pub struct DeleteAt {
    game_object_id: GameObjectId,
    delete_at: u32,
}

impl DeleteAt {
    pub fn step(game: &mut Game) {
        let mut to_delete = Vec::new();
        for (_id, delete_after) in game.generic_system.delete_afters.iter() {
            if game.tick_counter >= delete_after.delete_at {
                to_delete.push(delete_after.game_object_id);
            }
        }
        for id in to_delete {
            id.remove(game);
        }
    }
    pub fn new(game: &mut Game, game_object_id: GameObjectId, delete_at: u32) -> DeleteAtId {
        let id = game.get_id();
        let delete_after = DeleteAt {
            game_object_id,
            delete_at,
        };
        game.generic_system.delete_afters.insert(id, delete_after);
        id
    }
    pub fn remove(game: &mut Game, id: DeleteAtId) {
        game.generic_system.delete_afters.remove(id);
    }
}
pub struct DeleteAtComponent {
    pub component_id: ComponentId,
    pub delete_after_id: DeleteAtId,
}

impl DeleteAtComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId, time: u32) -> ComponentId {
        let component_id = game.get_id();
        let delete_after_id = DeleteAt::new(game, owner_id, time);
        let comp = DeleteAtComponent {
            component_id,
            delete_after_id,
        };
        owner_id.add_component(game, comp);
        component_id
    }
}

impl Component for DeleteAtComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        DeleteAt::remove(game, self.delete_after_id);
    }
}
