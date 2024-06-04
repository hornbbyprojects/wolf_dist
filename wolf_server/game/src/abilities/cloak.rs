use super::*;
use crate::combinable::CantCombine;
use crate::player::GetViewCoordsSignalListener;
use crate::timers::TimerSystem;
use std::cell::*;
use std::rc::Rc;

pub const DECLOAK_TIME: u32 = 100;

pub struct CloakAbility {
    ability_id: AbilityId,
    current_cloak_component_id: Option<ComponentId>,
    current_timer_id: Option<TimerId>,
}

impl CloakAbility {
    pub fn new(ability_id: AbilityId) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(CloakAbility {
            ability_id,
            current_cloak_component_id: None,
            current_timer_id: None,
        }))
    }
}

#[derive(Clone)]
pub struct CloakComponent {
    component_id: ComponentId,
    starting_coords: PixelCoords,
}

impl CloakComponent {
    fn add_to(game: &mut Game, owner_id: GameObjectId) -> ComponentId {
        let component_id = game.get_id();
        let starting_coords = owner_id.get_coords_game(game);
        let comp = CloakComponent {
            component_id,
            starting_coords,
        };
        owner_id.add_get_view_coords_signal_listener(game, comp.clone());
        owner_id.add_component(game, comp);
        component_id
    }
}
impl Component for CloakComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_get_view_coords_signal_listener(game, self.component_id);
    }
}

impl GetViewCoordsSignalListener for CloakComponent {
    fn receive_get_view_coords_signal(
        &self,
        _game: &Game,
        _owner_id: GameObjectId,
    ) -> CantCombine<PixelCoords> {
        CantCombine(self.starting_coords)
    }
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn GetViewCoordsSignalListener> {
        Box::new(self.clone())
    }
}
impl Ability for Rc<RefCell<CloakAbility>> {
    fn get_ability_id(&self) -> AbilityId {
        self.borrow().ability_id
    }
    fn activate(&mut self, game: &mut Game, caster: GameObjectId, _target_coords: PixelCoords) {
        let self_cloned = Rc::clone(self);
        let mut this = self.borrow_mut();
        match this.current_timer_id {
            Some(current_timer_id) => {
                let timer_id = current_timer_id.clone();
                //decloak
                drop(this);
                TimerSystem::activate_immediately(game, timer_id);
            }
            None => {
                this.current_cloak_component_id = Some(CloakComponent::add_to(game, caster));
                let timer_id = TimerSystem::add_timer(
                    game,
                    Box::new(move |game| {
                        let this = self_cloned.borrow();
                        if let Some(current_cloak_component_id) = this.current_cloak_component_id {
                            drop(this);
                            caster.remove_component(game, current_cloak_component_id);
                            let mut this = self_cloned.borrow_mut();
                            this.current_cloak_component_id = None;
                            this.current_timer_id = None;
                        }
                    }),
                    DECLOAK_TIME,
                );
                this.current_timer_id = Some(timer_id);
            }
        }
    }
}
