use byteorder::*;
use std::collections::hash_map::*;
use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use wolf_serialise::WolfSerialise;

//ironically named wrapper so we can set hasher globally easily
#[derive(Clone, Debug)]
pub struct WolfHashMap<K, V>(HashMap<K, V, std::hash::BuildHasherDefault<seahash::SeaHasher>>);

impl<K: Eq + Hash, V: PartialEq> PartialEq for WolfHashMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<K: Eq + Hash, V: Eq> Eq for WolfHashMap<K, V> {}

impl<K: std::cmp::Eq + std::hash::Hash, V> WolfHashMap<K, V> {
    pub fn new() -> WolfHashMap<K, V> {
        WolfHashMap(HashMap::with_hasher(Default::default()))
    }
    pub fn with_capacity(capacity: usize) -> WolfHashMap<K, V> {
        WolfHashMap(HashMap::with_capacity_and_hasher(
            capacity,
            Default::default(),
        ))
    }
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.0.insert(key, value)
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<K, V> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> std::collections::hash_map::IterMut<K, V> {
        self.0.iter_mut()
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        self.0.get(key)
    }
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        self.0.get_mut(key)
    }
    pub fn remove<Q: ?Sized + Hash + Eq>(&mut self, key: &Q) -> Option<V>
    where
        K: std::borrow::Borrow<Q>,
    {
        self.0.remove(key)
    }
    pub fn contains_key(&self, key: &K) -> bool {
        self.0.contains_key(key)
    }
    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        self.0.entry(key)
    }
    pub fn raw_entry_mut(
        &mut self,
    ) -> RawEntryBuilderMut<K, V, std::hash::BuildHasherDefault<seahash::SeaHasher>> {
        self.0.raw_entry_mut()
    }
    pub fn reserve(&mut self, amount: usize) {
        self.0.reserve(amount)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn drain(&mut self) -> std::collections::hash_map::Drain<K, V> {
        self.0.drain()
    }
}

impl<K: Hash + Eq, V> FromIterator<(K, V)> for WolfHashMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut ret = WolfHashMap::new();
        ret.0.extend(iter);
        ret
    }
}
impl<K, V> IntoIterator for WolfHashMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
impl<K: WolfSerialise + Eq + std::hash::Hash, V: WolfSerialise> WolfSerialise
    for WolfHashMap<K, V>
{
    fn wolf_serialise<W: std::io::Write>(&self, out_stream: &mut W) -> std::io::Result<()> {
        out_stream.write_u32::<BigEndian>(self.len() as u32)?;
        for (key, item) in self.iter() {
            key.wolf_serialise(out_stream)?;
            item.wolf_serialise(out_stream)?;
        }
        Ok(())
    }
    fn wolf_deserialise<R: std::io::Read>(in_stream: &mut R) -> std::io::Result<Self> {
        let length = in_stream.read_u32::<BigEndian>()?;
        let mut ret = WolfHashMap::with_capacity(length as usize);
        for _ in 0..length {
            let key = K::wolf_deserialise(in_stream)?;
            let item = V::wolf_deserialise(in_stream)?;
            ret.insert(key, item);
        }
        Ok(ret)
    }
}
