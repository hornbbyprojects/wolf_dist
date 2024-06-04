use super::*;
use crate::combinable::CombinedVecs;
use signal_listener_macro::*;

//model: each mob owns 1 allegiance, + some for special allegiances
//zombies etc use links to make them undead-aligned:w
define_signal_listener!(GetAllegiances, &Game -> CombinedVecs<AllegianceId>);

#[derive(Clone)]
pub struct AllegianceComponent {
    pub component_id: ComponentId,
    allegiance_id: AllegianceId,
}

impl AllegianceComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        friends_to_link: Vec<AllegianceId>,
    ) -> ComponentId {
        let mut friends_as_hash_set = WolfHashSet::new();
        for allegiance_id in friends_to_link {
            friends_as_hash_set.insert(allegiance_id);
        }
        let component_id = game.get_id();
        let allegiance_id = Allegiance::new(game, friends_as_hash_set);
        let comp = AllegianceComponent {
            component_id,
            allegiance_id,
        };
        owner_id.add_get_allegiances_signal_listener(game, comp.clone());
        owner_id.add_component(game, comp);
        component_id
    }
}

impl GameObjectId {
    pub fn can_hurt(&self, game: &Game, other: GameObjectId) -> bool {
        let our_allegiances = self.get_allegiances(game);
        let their_allegiances = other.get_allegiances(game);
        AllegianceSystem::can_hurt_multiple(
            &game.allegiance_system,
            &our_allegiances,
            &their_allegiances,
        )
    }
    pub fn get_allegiances(&self, game: &Game) -> Vec<AllegianceId> {
        self.send_get_allegiances_signal(game)
            .map(|allegiances| allegiances.0)
            .unwrap_or(vec![])
    }
    pub fn get_main_allegiance(&self, game: &Game) -> Option<AllegianceId> {
        let allegiances = self.get_allegiances(game);
        if allegiances.is_empty() {
            None
        } else {
            Some(allegiances[0])
        }
    }
}

impl GetAllegiancesSignalListener for AllegianceComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn receive_get_allegiances_signal(
        &self,
        _game: &Game,
        _owner_id: GameObjectId,
    ) -> CombinedVecs<AllegianceId> {
        CombinedVecs(vec![self.allegiance_id])
    }
    fn clone_box(&self) -> Box<dyn GetAllegiancesSignalListener> {
        Box::new(self.clone())
    }
}

impl Component for AllegianceComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_get_allegiances_signal_listener(game, self.component_id);
        Allegiance::remove(game, self.allegiance_id);
    }
}
