use super::*;

use crate::abilities::AbilityTypeId;
use crate::abilities::BasicAbilityUserComponent;
use crate::allegiance::AllegianceComponent;
use crate::hunting::PreyComponent;
use crate::solids::BlockableMoverComponent;

//not actually a charactercomponent, but the base for building one
#[derive(Clone)]
pub struct BaseCharacterComponent {
    pub component_id: ComponentId,
    pub allegiance_component_id: ComponentId,
    pub basic_ability_user_component_id: ComponentId,
    pub blockable_mover_component_id: ComponentId,
    pub prey_component_id: ComponentId,
    pub health_bar_component_id: ComponentId,
    pub voluntary_mover_component_id: ComponentId,
    pub delete_on_death_component_id: ComponentId,
    pub die_on_no_health_component_id: ComponentId,
}

impl BaseCharacterComponent {
    pub fn add_to(
        game: &mut Game,
        owner_id: GameObjectId,
        speed: f64,
        abilities: Vec<AbilityTypeId>,
        allegiances: Vec<AllegianceId>,
    ) -> ComponentId {
        let component_id = game.get_id();

        owner_id.set_hit_box(game, HitBox::default());

        let prey_component_id = PreyComponent::add_to(game, owner_id);

        let health_bar_component_id = add_health_bar(game, owner_id);

        let blockable_mover_component_id = BlockableMoverComponent::add_to(game, owner_id);
        let voluntary_mover_component_id =
            WalkerComponent::add_to(game, owner_id, speed, speed / 8.0);

        let allegiance_component_id = AllegianceComponent::add_to(game, owner_id, allegiances);

        let basic_ability_user_component_id =
            BasicAbilityUserComponent::add_to(game, owner_id, abilities);

        let delete_on_death_component_id = DeleteOnDeathComponent::add_to(game, owner_id);

        let die_on_no_health_component_id = DieOnNoHealthComponent::add_to(game, owner_id);

        let comp = BaseCharacterComponent {
            component_id,
            allegiance_component_id,
            basic_ability_user_component_id,
            blockable_mover_component_id,
            prey_component_id,
            health_bar_component_id,
            voluntary_mover_component_id,
            delete_on_death_component_id,
            die_on_no_health_component_id,
        };

        owner_id.add_component(game, comp);
        component_id
    }
}
impl Component for BaseCharacterComponent {
    fn get_component_id(&self) -> ComponentId {
        self.component_id
    }
    fn on_remove(self: Box<Self>, game: &mut Game, owner_id: GameObjectId) {
        owner_id.remove_component(game, self.allegiance_component_id);
        owner_id.remove_component(game, self.basic_ability_user_component_id);
        owner_id.remove_component(game, self.prey_component_id);
        owner_id.remove_component(game, self.health_bar_component_id);
        owner_id.remove_component(game, self.voluntary_mover_component_id);
        owner_id.remove_component(game, self.blockable_mover_component_id);
        owner_id.remove_component(game, self.delete_on_death_component_id);
        owner_id.remove_component(game, self.die_on_no_health_component_id);
    }
}
