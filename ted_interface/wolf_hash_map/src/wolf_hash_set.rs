use std::collections::hash_set;
use std::collections::HashSet;

type UnderlyingSetType<K> = HashSet<K, std::hash::BuildHasherDefault<seahash::SeaHasher>>;

#[derive(Clone)]
pub struct WolfHashSet<K>(UnderlyingSetType<K>);

impl<K: std::cmp::Eq + std::hash::Hash> WolfHashSet<K> {
    pub fn new() -> WolfHashSet<K> {
        WolfHashSet(HashSet::with_hasher(Default::default()))
    }
    pub fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
    pub fn iter(&self) -> hash_set::Iter<K> {
        self.0.iter()
    }
    pub fn difference<'a>(
        &'a self,
        other: &'a Self,
    ) -> hash_set::Difference<'a, K, std::hash::BuildHasherDefault<::seahash::SeaHasher>> {
        self.0.difference(&other.0)
    }
    pub fn intersection<'a>(
        &'a self,
        other: &'a Self,
    ) -> hash_set::Intersection<'a, K, std::hash::BuildHasherDefault<::seahash::SeaHasher>> {
        self.0.intersection(&other.0)
    }
    pub fn insert(&mut self, item: K) -> bool {
        self.0.insert(item)
    }
    pub fn remove(&mut self, item: &K) -> bool {
        self.0.remove(item)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn contains(&self, item: &K) -> bool {
        self.0.contains(item)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn drain(&mut self) -> hash_set::Drain<K> {
        self.0.drain()
    }
    pub fn extend(&mut self, extend_with: impl Iterator<Item = K>) {
        self.0.extend(extend_with)
    }
}

impl<K> IntoIterator for WolfHashSet<K> {
    type Item = K;
    type IntoIter = <UnderlyingSetType<K> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K: std::hash::Hash + Eq> std::iter::FromIterator<K> for WolfHashSet<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        WolfHashSet(iter.into_iter().collect())
    }
}
