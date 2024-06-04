use crate::game::*;

pub struct LootComponent {
    component_id: ComponentId,
}

impl Component for LootComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        let coords = owner_id.get_coords_game(game);
        crate::abilities::SpellbookSystem::create_spellbook(game, coords);
    }
}

impl LootComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let comp = LootComponent { component_id };
        owner_id.add_component(game, comp);
    }
}
