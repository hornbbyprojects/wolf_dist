use super::*;
use crate::characters::{HolyShieldAbility, HolySlashAbility, HolySteedAbility};
use crate::necromancy::CorpseTossAbility;
use crate::villages::BuildingAbility;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum AbilityTypeId {
    NecroboltId,
    FireballId,
    RailgunId,
    CorpseTossId,
    DebugId,
    CloakId,
    AmbushId,
    SprintId,
    HarvestId,
    BuildingId,
    PlaneWalkId,
    HolyShieldId,
    HolySteedId,
    HolySlashId,
}

fn id_to_ability(type_id: AbilityTypeId, ability_id: AbilityId) -> Box<dyn Ability> {
    match type_id {
        AbilityTypeId::NecroboltId => Box::new(NecroboltAbility::new(ability_id)),
        AbilityTypeId::FireballId => Box::new(FireballAbility::new(ability_id)),
        AbilityTypeId::RailgunId => Box::new(RailgunAbility::new(ability_id)),
        AbilityTypeId::DebugId => Box::new(DebugAbility::new(ability_id)),
        AbilityTypeId::CorpseTossId => Box::new(CorpseTossAbility::new(ability_id)),
        AbilityTypeId::CloakId => Box::new(CloakAbility::new(ability_id)),
        AbilityTypeId::AmbushId => Box::new(AmbushAbility::new(ability_id)),
        AbilityTypeId::SprintId => Box::new(SprintAbility::new(ability_id)),
        AbilityTypeId::HarvestId => Box::new(HarvestAbility::new(ability_id)),
        AbilityTypeId::BuildingId => Box::new(BuildingAbility::new(ability_id)),
        AbilityTypeId::PlaneWalkId => Box::new(PlaneWalkAbility::new(ability_id)),
        AbilityTypeId::HolyShieldId => Box::new(HolyShieldAbility::new(ability_id)),
        AbilityTypeId::HolySteedId => Box::new(HolySteedAbility::new(ability_id)),
        AbilityTypeId::HolySlashId => Box::new(HolySlashAbility::new(ability_id)),
    }
}
pub fn ability_ids_to_abilities(
    game: &mut Game,
    ids: &Vec<AbilityTypeId>,
) -> Vec<Box<dyn Ability>> {
    let mut ret = Vec::new();
    for id in ids {
        ret.push(id_to_ability(*id, game.get_id()));
    }
    ret
}
