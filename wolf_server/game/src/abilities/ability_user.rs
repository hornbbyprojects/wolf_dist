use super::*;
use crate::combinable::CombinedVecs;
use signal_listener_macro::define_signal_listener;
use wolf_hash_map::WolfHashMap;

define_signal_listener!(
    CastAbility,
    &mut Game,
    ability_id: AbilityId,
    target_coords: PixelCoords
);
define_signal_listener!(SetAbilities, &mut Game, ability_ids: &Vec<AbilityTypeId>);
define_signal_listener!(GetAbilities, &Game -> CombinedVecs<AbilityId>);
define_signal_listener!(GetAbilityDetails, &Game -> CombinedVecs<(AbilityId, AbilityDetails)>);
define_signal_listener!(GetAbilityIcons, &Game -> CombinedVecs<(AbilityId, u32)>);
define_signal_listener!(AbilitiesChanged, &mut Game);

pub struct BasicAbilityUser {
    pub abilities: WolfHashMap<AbilityId, Option<Box<dyn Ability>>>, //the option is used to take the ability while it's being cast, and so will be none mid-cast to prevent self-modification.
}

impl BasicAbilityUser {
    fn new(game: &mut Game, ability_ids: Vec<AbilityTypeId>) -> BasicAbilityUserId {
        let id = game.get_id();
        let abilities: WolfHashMap<AbilityId, Option<Box<dyn Ability>>> =
            ability_ids_to_abilities(game, &ability_ids)
                .into_iter()
                .map(|x| (x.get_ability_id(), Some(x)))
                .collect();
        let basic_ability_user = BasicAbilityUser { abilities };
        game.ability_system
            .basic_ability_users
            .insert(id, basic_ability_user);
        id
    }
}

#[derive(Clone)]
pub struct BasicAbilityUserComponent {
    pub component_id: ComponentId,
    pub basic_ability_user_id: BasicAbilityUserId,
}

impl CastAbilitySignalListener for BasicAbilityUserComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn CastAbilitySignalListener> {
        Box::new(self.clone())
    }
    fn receive_cast_ability_signal(
        &self,
        game: &mut Game,
        owner_id: GameObjectId,
        ability_id: AbilityId,
        target_coords: PixelCoords,
    ) {
        let mut ability = {
            let basic_ability_user = game
                .ability_system
                .basic_ability_users
                .get_mut(self.basic_ability_user_id)
                .unwrap();
            if let Some(ability_slot) = basic_ability_user.abilities.get_mut(&ability_id) {
                ability_slot.take().expect("Ability was cast from a cast!")
            } else {
                return;
            }
        };
        ability.activate(game, owner_id, target_coords);
        if let Some(user) = game
            .ability_system
            .basic_ability_users
            .get_mut(self.basic_ability_user_id)
        {
            user.abilities.insert(ability_id, Some(ability));
        }
    }
}

impl GetAbilitiesSignalListener for BasicAbilityUserComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn GetAbilitiesSignalListener> {
        Box::new(self.clone())
    }
    fn receive_get_abilities_signal(
        &self,
        game: &Game,
        _owner_id: GameObjectId,
    ) -> CombinedVecs<AbilityId> {
        CombinedVecs(
            game.ability_system
                .basic_ability_users
                .get(self.basic_ability_user_id)
                .unwrap()
                .abilities
                .iter()
                .map(|(x, _)| *x)
                .collect(),
        )
    }
}

impl GetAbilityDetailsSignalListener for BasicAbilityUserComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn GetAbilityDetailsSignalListener> {
        Box::new(self.clone())
    }
    fn receive_get_ability_details_signal(
        &self,
        game: &Game,
        _owner_id: GameObjectId,
    ) -> CombinedVecs<(AbilityId, AbilityDetails)> {
        CombinedVecs(
            game.ability_system
                .basic_ability_users
                .get(self.basic_ability_user_id)
                .unwrap()
                .abilities
                .iter()
                .map(|(ability_id, ability)| (*ability_id, ability.as_ref().unwrap().get_details()))
                .collect(),
        )
    }
}

impl GetAbilityIconsSignalListener for BasicAbilityUserComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn GetAbilityIconsSignalListener> {
        Box::new(self.clone())
    }
    fn receive_get_ability_icons_signal(
        &self,
        game: &Game,
        _owner_id: GameObjectId,
    ) -> CombinedVecs<(AbilityId, u32)> {
        CombinedVecs(
            game.ability_system
                .basic_ability_users
                .get(self.basic_ability_user_id)
                .unwrap()
                .abilities
                .iter()
                .map(|(ability_id, ability)| {
                    (*ability_id, ability.as_ref().unwrap().get_ability_icon())
                })
                .collect(),
        )
    }
}

impl SetAbilitiesSignalListener for BasicAbilityUserComponent {
    fn get_listener_id(&self) -> ComponentId {
        self.component_id
    }
    fn clone_box(&self) -> Box<dyn SetAbilitiesSignalListener> {
        Box::new(self.clone())
    }
    fn receive_set_abilities_signal(
        &self,
        game: &mut Game,
        owner_id: GameObjectId,
        ability_type_ids: &Vec<AbilityTypeId>,
    ) {
        let abilities = ability_ids_to_abilities(game, ability_type_ids);
        let ability_user = game
            .ability_system
            .basic_ability_users
            .get_mut(self.basic_ability_user_id)
            .unwrap();
        ability_user.abilities = abilities
            .into_iter()
            .map(|x| (x.get_ability_id(), Some(x)))
            .collect();
        owner_id.send_abilities_changed_signal(game);
    }
}

impl Component for BasicAbilityUserComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_cast_ability_signal_listener(game, self.component_id);
        owner_id.remove_set_abilities_signal_listener(game, self.component_id);
        owner_id.remove_get_abilities_signal_listener(game, self.component_id);
        owner_id.remove_get_ability_details_signal_listener(game, self.component_id);
        owner_id.remove_get_ability_icons_signal_listener(game, self.component_id);
    }
}

impl BasicAbilityUserComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        abilities: Vec<AbilityTypeId>,
    ) -> ComponentId {
        let component_id = game.get_id();
        let basic_ability_user_id = BasicAbilityUser::new(game, abilities);
        let comp = BasicAbilityUserComponent {
            component_id,
            basic_ability_user_id,
        };
        owner_id.add_cast_ability_signal_listener(game, comp.clone());
        owner_id.add_set_abilities_signal_listener(game, comp.clone());
        owner_id.add_get_abilities_signal_listener(game, comp.clone());
        owner_id.add_get_ability_details_signal_listener(game, comp.clone());
        owner_id.add_get_ability_icons_signal_listener(game, comp.clone());
        owner_id.add_component(game, comp);
        component_id
    }
}
