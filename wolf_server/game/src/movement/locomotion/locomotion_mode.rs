use crate::{add_shared_component, game::*, get_shared_component};
use wolf_hash_map::WolfHashSet;

pub struct LocomotionMode {
    pub priority: u32, //Higher is better
}

pub struct LocomotionModeSelection {
    pub selected_locomotion_mode: Option<LocomotionModeId>,
    pub available_locomotion_modes: WolfHashSet<LocomotionModeId>,
}

impl LocomotionModeSelection {
    pub fn new() -> Self {
        LocomotionModeSelection {
            selected_locomotion_mode: None,
            available_locomotion_modes: WolfHashSet::new(),
        }
    }
}

impl GameObjectComponents {
    add_shared_component!(
        add_locomotion_mode_selection,
        LocomotionModeSelection,
        LocomotionModeSelection
    );
    get_shared_component!(
        get_locomotion_mode_selection,
        get_locomotion_mode_selection_mut,
        LocomotionModeSelection,
        LocomotionModeSelection
    );
    pub fn remove_locomotion_mode_selection(&mut self) {
        self.remove_shared_component(SharedComponentDiscriminants::LocomotionModeSelection);
    }
}

impl GameObjectId {
    pub fn add_locomotion_mode_id(&self, game: &mut Game, id: LocomotionModeId) {
        let locomotion_mode_selection = self
            .get_mut(game)
            .unwrap()
            .components
            .add_locomotion_mode_selection();
        locomotion_mode_selection
            .available_locomotion_modes
            .insert(id);
        locomotion_mode_selection.selected_locomotion_mode = Some(id);
    }
    pub fn remove_locomotion_mode_id(&self, game: &mut Game, id: LocomotionModeId) {
        let components = &mut self.get_mut(game).unwrap().components;
        let locomotion_mode_selection = components.get_locomotion_mode_selection_mut().unwrap();
        locomotion_mode_selection
            .available_locomotion_modes
            .remove(&id);
        if locomotion_mode_selection.selected_locomotion_mode == Some(id) {
            locomotion_mode_selection.selected_locomotion_mode = locomotion_mode_selection
                .available_locomotion_modes
                .iter()
                .next()
                .map(|x| *x)
        }
        components.remove_locomotion_mode_selection();
    }
}
