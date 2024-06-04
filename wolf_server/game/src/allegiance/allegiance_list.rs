use super::*;

//todo: use this as test to decide whether to use generational IDs for all
pub struct AllegianceList {
    next_id: u32,
    allegiances: IdMap<AllegianceId, Allegiance>,
}

impl AllegianceList {
    pub fn new() -> AllegianceList {
        AllegianceList {
            next_id: 0,
            allegiances: IdMap::new(),
        }
    }
    pub fn get(&self, id: AllegianceId) -> Option<&Allegiance> {
        self.allegiances.get(id)
    }
    pub fn get_mut(&mut self, id: AllegianceId) -> Option<&mut Allegiance> {
        self.allegiances.get_mut(id)
    }
    pub fn add(&mut self, allegiance: Allegiance) -> AllegianceId {
        let id = self.next_id.into();
        self.next_id += 1;
        self.allegiances.insert(id, allegiance);
        id
    }
    pub fn remove(&mut self, id: AllegianceId) -> Option<Allegiance> {
        self.allegiances.remove(id)
    }
}
