use super::*;
use signal_listener_macro::define_signal_listener;

define_signal_listener!(Death, &mut Game);

#[derive(Clone)]
pub struct DieOnNoHealthComponent {
    component_id: ComponentId,
}

impl DamageTakenSignalListener for DieOnNoHealthComponent {
    fn clone_box(&self) -> Box<(dyn damageable::DamageTakenSignalListener + 'static)> {
        Box::new(self.clone())
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn receive_damage_taken_signal(
        &self,
        game: &mut Game,
        owner: GameObjectId,
        _amount: Health,
        health_remaining: Health,
    ) {
        if health_remaining.0 <= 0 {
            owner.send_death_signal(game);
        }
    }
}
impl Component for DieOnNoHealthComponent {
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId) {
        owner.remove_damage_taken_signal_listener(game, self.component_id);
    }

    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
}

impl DieOnNoHealthComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
        let component_id = game.get_id();
        let comp = DieOnNoHealthComponent { component_id };
        owner_id.add_component(game, comp.clone());
        owner_id.add_damage_taken_signal_listener(game, comp);
        component_id
    }
}

#[derive(Clone)]
pub struct DeleteOnDeathComponent {
    component_id: ComponentId,
}

impl DeathSignalListener for DeleteOnDeathComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn DeathSignalListener> {
        Box::new(self.clone())
    }
    fn receive_death_signal(&self, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove(game);
    }
}

impl Component for DeleteOnDeathComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_death_signal_listener(game, self.component_id);
    }
}

impl DeleteOnDeathComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
        let component_id = game.get_id();
        let comp = DeleteOnDeathComponent { component_id };
        owner_id.add_component(game, comp.clone());
        owner_id.add_death_signal_listener(game, comp);
        component_id
    }
}
