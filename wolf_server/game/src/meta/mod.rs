use crate::game::*;

struct ComponentListComponent {
    component_id: ComponentId,
    managed_component_ids: Vec<ComponentId>,
}

impl Component for ComponentListComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        for managed_component_id in self.managed_component_ids {
            owner.remove_component(game, managed_component_id);
        }
    }
}

impl ComponentListComponent {
    fn add_to(
        game: &mut Game,
        owner: GameObjectId,
        component_constructors: impl IntoIterator<
            Item = Box<dyn FnOnce(&mut Game, GameObjectId) -> ComponentId>,
        >,
    ) -> ComponentId {
        let component_id = game.get_id();
        let managed_component_ids = component_constructors
            .into_iter()
            .map(|constructor| constructor(game, owner))
            .collect();
        let comp = ComponentListComponent {
            component_id,
            managed_component_ids,
        };
        owner.add_component(game, comp);
        component_id
    }
}
