use id::IdMap;
use std::collections::hash_map::Entry;
use strum_macros::EnumDiscriminants;
use wolf_hash_map::WolfHashMap;

use crate::abilities::SlotMapping;
use crate::game::Game;
use crate::id_types::*;
use crate::LocomotionModeSelection;
use crate::StickyFacer;

#[derive(EnumDiscriminants)]
#[strum_discriminants(derive(Hash))]
pub enum SharedComponent {
    LocomotionModeSelection(LocomotionModeSelection),
    StickyFacer(StickyFacer),
    SlotMapping(SlotMapping),
}

pub struct SharedComponentState {
    pub shared_component: SharedComponent,
    pub using_counter: u32,
}

pub struct GameObjectComponents {
    pub components: IdMap<ComponentId, Box<dyn Component>>,
    pub shared_components: WolfHashMap<SharedComponentDiscriminants, SharedComponentState>,
}

#[macro_export]
macro_rules! add_shared_component {
    ($func_name:ident, $component_type:ty, $discriminant:ident) => {
        pub fn $func_name(&mut self) -> &mut $component_type {
            let entry = self
                .shared_components
                .entry(SharedComponentDiscriminants::$discriminant);
            match entry {
                std::collections::hash_map::Entry::Occupied(mut occ) => {
                    occ.get_mut().using_counter += 1;
                    if let SharedComponent::$discriminant(ref mut inner) =
                        occ.into_mut().shared_component
                    {
                        inner
                    } else {
                        unreachable!()
                    }
                }
                std::collections::hash_map::Entry::Vacant(vac) => {
                    let new_component = SharedComponentState {
                        shared_component: SharedComponent::$discriminant(<$component_type>::new()),
                        using_counter: 1,
                    };
                    if let SharedComponent::$discriminant(ref mut inner) =
                        vac.insert(new_component).shared_component
                    {
                        inner
                    } else {
                        unreachable!()
                    }
                }
            }
        }
    };
}

#[macro_export]
macro_rules! get_shared_component {
    ($func_name:ident, $mut_func_name: ident, $component_type:ty, $discriminant:ident) => {
        pub fn $func_name(&self) -> Option<&$component_type> {
            if let Some(filled) = self
                .shared_components
                .get(&SharedComponentDiscriminants::$discriminant)
            {
                if let SharedComponent::$discriminant(ref inner) = filled.shared_component {
                    Some(inner)
                } else {
                    unreachable!()
                }
            } else {
                None
            }
        }
        pub fn $mut_func_name(&mut self) -> Option<&mut $component_type> {
            if let Some(filled) = self
                .shared_components
                .get_mut(&SharedComponentDiscriminants::$discriminant)
            {
                if let SharedComponent::$discriminant(ref mut inner) = filled.shared_component {
                    Some(inner)
                } else {
                    unreachable!()
                }
            } else {
                None
            }
        }
    };
}

impl GameObjectComponents {
    pub fn new() -> Self {
        GameObjectComponents {
            components: IdMap::new(),
            shared_components: WolfHashMap::new(),
        }
    }
    pub fn remove_shared_component(&mut self, discriminant: SharedComponentDiscriminants) {
        if let Entry::Occupied(mut occ) = self.shared_components.entry(discriminant) {
            let state = occ.get_mut();
            state.using_counter -= 1;
            if state.using_counter == 0 {
                occ.remove();
            }
        } else {
            panic!("Removed more {:?} than added", discriminant);
        }
    }
}

pub trait Component {
    fn on_remove(self: Box<Self>, game: &mut Game, owner: GameObjectId);
    fn get_component_id(&self) -> ComponentId;
}
