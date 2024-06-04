use crate::game::*;

pub struct CorpseComponent {
    component_id: ComponentId,
}

impl CorpseComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let comp = CorpseComponent { component_id };
        owner_id.add_collision_group(game, CollisionGroupId::Corpse);
        owner_id.add_component(game, comp);
    }
}

impl Component for CorpseComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_collision_group(game, CollisionGroupId::Corpse);
    }
}
