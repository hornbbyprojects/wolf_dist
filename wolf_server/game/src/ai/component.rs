use super::*;

#[derive(Clone)]
pub struct AiComponent {
    component_id: ComponentId,
    ai_id: AiId,
}

impl Component for AiComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, _owner_id: GameObjectId) {
        Ai::remove(game, self.ai_id);
    }
}

impl AiComponent {
    #[allow(dead_code)]
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let ai_id = Ai::new(game, owner_id);
        let comp = AiComponent {
            component_id,
            ai_id,
        };
        owner_id.add_component(game, comp);
    }
}
