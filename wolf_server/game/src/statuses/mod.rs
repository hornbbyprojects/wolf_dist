use super::*;
use crate::combinable::OrBool;
use signal_listener_macro::define_signal_listener;
use std::cell::RefCell;
use std::rc::Rc;

//Returns TRUE if we already have a bleed handler
define_signal_listener!(Bleed, &mut Game, amount: Health -> OrBool);

#[derive(Clone)]
pub struct BleedComponent {
    pub component_id: ComponentId,
    pub damage_remaining: Health,
}

pub struct BleedComponentId(ComponentId);

impl Component for BleedComponentId {
    fn get_component_id(&self) -> ComponentId {
        self.0
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_bleed_signal_listener(game, self.0);
    }
}
impl BleedSignalListener for Rc<RefCell<BleedComponent>> {
    fn get_listener_id(&self) -> ComponentId {
        self.borrow().component_id
    }
    fn clone_box(&self) -> Box<dyn BleedSignalListener> {
        Box::new(Rc::clone(self))
    }
    fn receive_bleed_signal(
        &self,
        game: &mut Game,
        owner_id: GameObjectId,
        amount: Health,
    ) -> OrBool {
        let mut this = self.borrow_mut();
        let instant_damage = this.damage_remaining.clone();
        this.damage_remaining += amount;
        drop(this);
        owner_id.send_damage_signal(game, instant_damage);
        OrBool(true)
    }
}

impl BleedComponent {
    fn add_to(game: &mut Game, owner_id: GameObjectId, amount: Health) {
        let component_id = game.get_id();
        let comp = Rc::new(RefCell::new(BleedComponent {
            component_id,
            damage_remaining: amount,
        }));
        owner_id.add_bleed_signal_listener(game, comp);
        owner_id.add_component(game, BleedComponentId(component_id));
    }
}

pub fn bleed(game: &mut Game, target: GameObjectId, amount: Health) {
    if let Some(already_bled) = target.send_bleed_signal(game, amount) {
        if already_bled.extract() {
            return;
        }
    };
    BleedComponent::add_to(game, target, amount);
}

#[derive(Clone)]
pub struct BleedAttackComponent {
    component_id: ComponentId,
    damage: Health,
}
pub struct BleedAttackComponentId(ComponentId);

impl BleedAttackComponent {
    pub fn add_to(game: &mut Game, owner_id: GameObjectId, damage: Health) {
        let component_id = game.get_id();
        let comp = BleedAttackComponent {
            component_id,
            damage,
        };
        owner_id.add_dealt_damage_signal_listener(game, comp);
        owner_id.add_component(game, BleedAttackComponentId(component_id));
    }
}
impl DealtDamageSignalListener for BleedAttackComponent {
    fn receive_dealt_damage_signal(
        &self,
        game: &mut Game,
        _owner_id: GameObjectId,
        target_id: GameObjectId,
    ) {
        bleed(game, target_id, self.damage);
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn DealtDamageSignalListener> {
        Box::new(self.clone())
    }
}

impl Component for BleedAttackComponentId {
    fn get_component_id(&self) -> ComponentId {
        self.0
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_dealt_damage_signal_listener(game, self.0);
    }
}
