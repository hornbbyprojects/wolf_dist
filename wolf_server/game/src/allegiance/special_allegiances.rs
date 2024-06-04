use crate::allegiance::*;
use crate::id_types::*;

pub struct SpecialAllegiances {
    pub holy_allegiance: AllegianceId,
    pub undead_allegiance: AllegianceId,
    pub villager_allegiance: AllegianceId,
    pub wolf_allegiance: AllegianceId,
}

impl SpecialAllegiances {
    pub fn new(allegiances: &mut AllegianceList) -> SpecialAllegiances {
        let holy_allegiance = Allegiance::new_on_list(allegiances, WolfHashSet::new());
        let undead_allegiance = Allegiance::new_on_list(allegiances, WolfHashSet::new());
        let villager_allegiance = Allegiance::new_on_list(allegiances, WolfHashSet::new());
        let wolf_allegiance = Allegiance::new_on_list(allegiances, WolfHashSet::new());
        SpecialAllegiances {
            holy_allegiance,
            undead_allegiance,
            villager_allegiance,
            wolf_allegiance,
        }
    }
}
