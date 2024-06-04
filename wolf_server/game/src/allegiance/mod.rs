//TODO: replace with refcounting?
use crate::game::*;

mod allegiance;
pub use allegiance::*;
mod allegiance_list;
pub use allegiance_list::*;
mod component;
pub use component::*;
mod special_allegiances;
pub use special_allegiances::*;
use wolf_hash_map::WolfHashSet;

//allegiance ids are not guaranteed to not overlap with other ids
pub struct AllegianceSystem {
    pub allegiance_list: AllegianceList,
    pub special_allegiances: SpecialAllegiances,
}
impl AllegianceSystem {
    pub fn new() -> AllegianceSystem {
        let mut allegiance_list = AllegianceList::new();
        let special_allegiances = SpecialAllegiances::new(&mut allegiance_list);
        AllegianceSystem {
            allegiance_list,
            special_allegiances,
        }
    }
    fn get(&self, id: AllegianceId) -> Option<&Allegiance> {
        self.allegiance_list.get(id)
    }
    //graph search
    pub fn can_hurt(&self, hurter: AllegianceId, hurtee: AllegianceId) -> bool {
        {
            let hurter_allegiance = self.get(hurter).unwrap();
            if !hurter_allegiance.friendly_fire && hurter == hurtee {
                return false;
            }
        }
        let mut already_visited: WolfHashSet<AllegianceId> = WolfHashSet::new();
        already_visited.insert(hurter);
        let mut to_visit: Vec<AllegianceId> = vec![hurter];
        while let Some(hurter_allegiance_id) = to_visit.pop() {
            let hurter_allegiance = self.allegiance_list.get(hurter_allegiance_id).unwrap();

            if hurter_allegiance.friends.contains(&hurtee) {
                return false;
            }
            for linker_id in hurter_allegiance.linkers.iter() {
                if !already_visited.contains(linker_id) {
                    to_visit.push(*linker_id);
                    already_visited.insert(*linker_id);
                }
            }
        }
        true
    }
    pub fn can_hurt_multiple(
        &self,
        hurter: &Vec<AllegianceId>,
        hurtee: &Vec<AllegianceId>,
    ) -> bool {
        for hurter_id in hurter.iter() {
            for hurtee_id in hurtee.iter() {
                if !self.can_hurt(*hurter_id, *hurtee_id) {
                    return false;
                }
            }
        }
        true
    }
}
