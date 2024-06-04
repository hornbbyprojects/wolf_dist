use super::*;

pub trait ToggledAbility {
    fn toggle_on(&mut self, game: &mut Game, caster: GameObjectId) -> bool;
    fn toggle_off(&mut self, game: &mut Game, caster: GameObjectId) -> bool;
    fn get_ability_id(&self) -> AbilityId;
}

pub struct ToggledAbilityContainer<T: ToggledAbility> {
    ability: T,
    is_active: bool,
}

impl<T: ToggledAbility> Ability for ToggledAbilityContainer<T> {
    fn get_ability_id(&self) -> AbilityId {
        self.ability.get_ability_id()
    }
    fn activate(&mut self, game: &mut Game, caster: GameObjectId, _target_coords: PixelCoords) {
        if self.is_active {
            if self.ability.toggle_off(game, caster) {
                self.is_active = false;
            }
        } else {
            if self.ability.toggle_on(game, caster) {
                self.is_active = true;
            }
        }
    }
}

impl<T: ToggledAbility> From<T> for ToggledAbilityContainer<T> {
    fn from(ability: T) -> Self {
        ToggledAbilityContainer {
            ability,
            is_active: false,
        }
    }
}

pub trait ComponentsToggledAbility {
    fn add_components(&self, game: &mut Game, caster: GameObjectId) -> Vec<ComponentId>;
    fn get_ability_id(&self) -> AbilityId;
}

pub struct ComponentsToggledAbilityContainer<T: ComponentsToggledAbility> {
    ability: T,
    component_ids: Option<Vec<ComponentId>>,
}

impl<T: ComponentsToggledAbility> ToggledAbility for ComponentsToggledAbilityContainer<T> {
    fn toggle_on(&mut self, game: &mut Game, caster: GameObjectId) -> bool {
        self.component_ids = Some(self.ability.add_components(game, caster));
        true
    }
    fn toggle_off(&mut self, game: &mut Game, caster: GameObjectId) -> bool {
        if let Some(component_ids) = self.component_ids.take() {
            for component_id in component_ids {
                caster.remove_component(game, component_id);
            }
        }
        true
    }
    fn get_ability_id(&self) -> AbilityId {
        self.ability.get_ability_id()
    }
}

impl<T: ComponentsToggledAbility> From<T> for ComponentsToggledAbilityContainer<T> {
    fn from(ability: T) -> Self {
        ComponentsToggledAbilityContainer {
            ability,
            component_ids: None,
        }
    }
}

impl<T: ComponentsToggledAbility> IntoAbility for T {
    type AbilityType = ToggledAbilityContainer<ComponentsToggledAbilityContainer<T>>;
    fn into_ability(self) -> Self::AbilityType {
        Into::<ComponentsToggledAbilityContainer<T>>::into(self).into()
    }
}
