use std::collections::hash_map::Entry;
use std::hash::Hash;

use wolf_hash_map::WolfHashSet;

pub fn insert_lazy<K: Eq + Hash, V: Hash + Eq>(
    map: &mut wolf_hash_map::WolfHashMap<K, WolfHashSet<V>>,
    key: K,
    value: V,
) {
    map.entry(key).or_insert(WolfHashSet::new()).insert(value);
}
pub fn remove_lazy<K: Hash + Eq, V: Hash + Eq>(
    map: &mut wolf_hash_map::WolfHashMap<K, WolfHashSet<V>>,
    key: K,
    value: &V,
) -> bool {
    match map.entry(key) {
        Entry::Occupied(mut entry) => {
            let items = entry.get_mut();
            let ret = items.remove(value);
            if items.is_empty() {
                entry.remove();
            }
            ret
        }
        _ => false,
    }
}
