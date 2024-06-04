use super::*;
use crate::resources::Resources;

impl Need {
    fn as_resources(&self) -> Option<&Resources> {
        match self {
            &Need::Resources(ref resources) => Some(resources),
            _ => None,
        }
    }
    fn as_resources_mut(&mut self) -> Option<&mut Resources> {
        match self {
            &mut Need::Resources(ref mut resources) => Some(resources),
            _ => None,
        }
    }
}
impl Needs {
    fn require_resources_by_key(&mut self, key: NeedsKey, resources: &Resources) {
        let entry = self.needs.entry(key);
        match entry {
            std::collections::hash_map::Entry::Occupied(occ) => {
                let old_resources = occ.into_mut().as_resources_mut().unwrap();
                *old_resources += resources;
            }
            std::collections::hash_map::Entry::Vacant(vac) => {
                vac.insert(Need::Resources(Box::new(resources.clone())));
            }
        }
    }
    fn resources_by_key(&self, key: NeedsKey) -> Option<&Resources> {
        self.needs.get(&key).map(|x| x.as_resources().unwrap())
    }
    fn resources_by_key_mut(&mut self, key: NeedsKey) -> Option<&mut Resources> {
        self.needs
            .get_mut(&key)
            .map(|x| x.as_resources_mut().unwrap())
    }
    fn add_resources_by_key(&mut self, key: NeedsKey, resources: &Resources) {
        let old_resources = self.resources_by_key_mut(key);
        if let Some(old_resources) = old_resources {
            *old_resources -= resources
        }
    }

    pub fn require_resources(&mut self, resources: &Resources) {
        self.require_resources_by_key(NeedsKey::Resources, resources)
    }
    pub fn resources(&self) -> Option<&Resources> {
        self.resources_by_key(NeedsKey::Resources)
    }
    pub fn add_resources(&mut self, resources: &Resources) {
        self.add_resources_by_key(NeedsKey::Resources, resources)
    }
    pub fn clear_resources(&mut self) {
        self.needs.remove(&NeedsKey::Resources);
    }

    pub fn require_town_resources(&mut self, resources: &Resources) {
        self.require_resources_by_key(NeedsKey::TownResources, resources)
    }
    pub fn town_resources(&self) -> Option<&Resources> {
        self.resources_by_key(NeedsKey::TownResources)
    }
    pub fn add_town_resources(&mut self, resources: &Resources) {
        self.add_resources_by_key(NeedsKey::TownResources, resources)
    }
    pub fn clear_town_resources(&mut self) {
        self.needs.remove(&NeedsKey::TownResources);
    }
}
