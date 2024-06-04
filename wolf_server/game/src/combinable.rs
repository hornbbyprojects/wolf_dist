use std::hash::Hash;
pub struct CombinedVecs<T>(pub Vec<T>);
impl<T> CombinedVecs<T> {
    pub fn combine_result(mut self, mut other: Self) -> Self {
        self.0.append(&mut other.0);
        self
    }
    pub fn extract(self) -> Vec<T> {
        self.0
    }
}

pub struct CombinedHashMaps<K, V>(pub wolf_hash_map::WolfHashMap<K, V>);

impl<K: Eq + Hash + Clone, V: Clone> CombinedHashMaps<K, V> {
    pub fn combine_result(mut self, other: Self) -> Self {
        for (k, v) in other.0.iter() {
            self.0.insert(k.clone(), v.clone());
        }
        self
    }
}

pub struct OrBool(pub bool);

impl OrBool {
    pub fn combine_result(self, other: Self) -> Self {
        OrBool(self.0 || other.0)
    }
    pub fn extract(self) -> bool {
        self.0
    }
}

pub struct CantCombine<T>(pub T);
impl<T> CantCombine<T> {
    pub fn combine_result(self, _other: Self) -> Self {
        self
    }
    pub fn extract(self) -> T {
        self.0
    }
}

pub struct Added<T>(pub T);
impl<T: std::ops::Add<Output = T>> Added<T> {
    pub fn combine_result(self, other: Self) -> Self {
        Added(self.0 + other.0)
    }
    pub fn extract(self) -> T {
        self.0
    }
}
