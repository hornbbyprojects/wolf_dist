use super::*;

pub struct SprintAbility {
    ability_id: AbilityId,
}

impl SprintAbility {
    pub fn new(ability_id: AbilityId) -> impl Ability {
        SprintAbility { ability_id }.into_ability()
    }
}

impl ComponentsToggledAbility for SprintAbility {
    fn add_components(&self, game: &mut Game, caster: GameObjectId) -> Vec<ComponentId> {
        let component_id = SpeedModComponent::add_to(game, caster, 2.0);
        vec![component_id]
    }

    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }
}
