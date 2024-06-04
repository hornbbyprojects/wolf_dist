use super::*;

pub struct CorpseOnDeathComponent {
    component_id: ComponentId,
}

impl CorpseOnDeathComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let comp = CorpseOnDeathComponent { component_id };
        owner_id.add_component(game, comp);
    }
}

impl Component for CorpseOnDeathComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        let coords = owner_id.get_coords_game(game);
        NecromancySystem::make_corpse(game, coords);
    }
}
