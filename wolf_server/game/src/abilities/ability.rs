use super::*;

#[derive(Debug, Clone)]
pub struct AbilityDetails {
    pub is_attack_ability: bool,
}

impl Default for AbilityDetails {
    fn default() -> Self {
        Self {
            is_attack_ability: false,
        }
    }
}

/// Info about the ability for AI use
impl AbilityDetails {
    pub fn attack_ability(&self) -> Self {
        let mut ret = self.clone();
        ret.is_attack_ability = true;
        ret
    }
}

pub trait Ability {
    fn get_ability_icon(&self) -> u32 {
        UNKNOWN_ABILITY_SPRITE
    }
    fn get_ability_id(&self) -> AbilityId;
    fn activate(&mut self, game: &mut Game, caster: GameObjectId, target_coords: PixelCoords);
    fn get_details(&self) -> AbilityDetails {
        AbilityDetails::default()
    }
}

pub trait IntoAbility {
    type AbilityType;
    fn into_ability(self) -> Self::AbilityType;
}
