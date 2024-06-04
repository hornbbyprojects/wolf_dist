use super::*;
use crate::abilities::*;
use crate::characters::LichCharacterComponent;

#[derive(Clone)]
pub struct LichBookComponent {
    component_id: ComponentId,
}
impl SpellbookWasAbsorbedSignalListener for LichBookComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn receive_spellbook_was_absorbed_signal(
        &self,
        game: &mut Game,
        _owner_id: GameObjectId,
        absorber: GameObjectId,
    ) {
        LichCharacterComponent::add_to(game, absorber);
    }
    fn clone_box(&self) -> Box<dyn SpellbookWasAbsorbedSignalListener> {
        Box::new(self.clone())
    }
}

impl LichBookComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) {
        let component_id = game.get_id();
        let comp = LichBookComponent { component_id };
        owner_id.add_spellbook_was_absorbed_signal_listener(game, comp.clone());
        owner_id.add_component(game, comp);
    }
}

impl Component for LichBookComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_spellbook_was_absorbed_signal_listener(game, self.component_id);
    }
}
