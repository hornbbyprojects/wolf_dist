use crate::game::*;

pub struct SpellbookComponent {
    component_id: ComponentId,
}

impl SpellbookComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let comp = SpellbookComponent { component_id };
        owner_id.add_collision_group(game, CollisionGroupId::Spellbook);
        owner_id.add_component(game, comp);
    }
}
impl Component for SpellbookComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_collision_group(game, CollisionGroupId::Spellbook);
    }
}
