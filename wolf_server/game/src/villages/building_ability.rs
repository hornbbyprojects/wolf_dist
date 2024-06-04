use crate::abilities::*;
use crate::game::*;
use crate::solids::SolidComponent;
use crate::terrain::TerrainSpriteComponent;

pub fn create_wall(game: &mut Game, coords: SquareCoords) {
    let owner_id = GameObject::create_with_hit_box(
        game,
        coords.center_pixel(),
        HitBox::new_at_zero(SQUARE_SIZE_PIXELS / 2, SQUARE_SIZE_PIXELS / 2),
    );
    TerrainSpriteComponent::add_to(game, owner_id, coords, WALL_SPRITE);
    SolidComponent::add_to(game, owner_id);
    DamageableComponent::add_to(game, owner_id);
    DeleteOnDeathComponent::add_to(game, owner_id);
    DieOnNoHealthComponent::add_to(game, owner_id);
}
pub struct Building {
    pub damageable_id: DamageableId,
}
pub struct BuildingsSystem {}
pub struct BuildingAbility {
    ability_id: AbilityId,
}

impl BuildingAbility {
    pub fn new(ability_id: AbilityId) -> BuildingAbility {
        BuildingAbility { ability_id }
    }
}

impl Ability for BuildingAbility {
    fn get_ability_id(&self) -> AbilityId {
        self.ability_id
    }

    fn activate(
        &mut self,
        game: &mut crate::Game,
        _caster: crate::GameObjectId,
        target_coords: coords::PixelCoords,
    ) {
        create_wall(game, target_coords.into());
    }
}
