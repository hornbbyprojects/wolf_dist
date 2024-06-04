#![feature(stmt_expr_attributes)]
#![feature(type_alias_impl_trait)]
use std::collections::hash_map::Entry;
use std::collections::*;
use std::marker::PhantomData;

#[macro_export]
macro_rules! makeId {
    ($id_name: ident) => {
        #[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
        pub struct $id_name(pub u32);
        impl From<u32> for $id_name {
            fn from(id: u32) -> Self {
                $id_name(id)
            }
        }
        impl Into<u32> for $id_name {
            fn into(self) -> u32 {
                self.0
            }
        }
    };
}

pub struct IdMap<IdType, ItemType> {
    _phantom: PhantomData<IdType>,
    inner_map: HashMap<u32, ItemType>,
}
impl<IdType, ItemType> IdMap<IdType, ItemType> {
    pub fn new() -> Self {
        IdMap {
            inner_map: HashMap::new(),
            _phantom: PhantomData,
        }
    }
}

pub struct IntoIter<IdType, ItemType> {
    _phantom: PhantomData<IdType>,
    inner_iter: std::collections::hash_map::IntoIter<u32, ItemType>,
}
impl<IdType, ItemType> IntoIter<IdType, ItemType> {
    fn new(inner_iter: std::collections::hash_map::IntoIter<u32, ItemType>) -> Self {
        IntoIter {
            _phantom: PhantomData,
            inner_iter,
        }
    }
}
impl<IdType: From<u32>, ItemType> Iterator for IntoIter<IdType, ItemType> {
    type Item = (IdType, ItemType);
    fn next(&mut self) -> Option<(IdType, ItemType)> {
        self.inner_iter
            .next()
            .map(|(id, item)| (IdType::from(id), item))
    }
}

impl<IdType: From<u32>, ItemType> IntoIterator for IdMap<IdType, ItemType> {
    type Item = (IdType, ItemType);
    type IntoIter = IntoIter<IdType, ItemType>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self.inner_map.into_iter())
    }
}

impl<IdType: Into<u32> + From<u32> + Copy, ItemType> IdMap<IdType, ItemType> {
    pub fn len(&self) -> usize {
        self.inner_map.len()
    }
    pub fn get(&self, id: IdType) -> Option<&ItemType> {
        let id_as_u32: u32 = id.into();
        self.inner_map.get(&id_as_u32)
    }
    pub fn get_mut(&mut self, id: IdType) -> Option<&mut ItemType> {
        let id_as_u32: u32 = id.into();
        self.inner_map.get_mut(&id_as_u32)
    }
    pub fn insert(&mut self, id: IdType, item: ItemType) -> Option<ItemType> {
        let id_as_u32: u32 = id.into();
        self.inner_map.insert(id_as_u32, item)
    }
    pub fn remove(&mut self, id: IdType) -> Option<ItemType> {
        let id_as_u32: u32 = id.into();
        self.inner_map.remove(&id_as_u32)
    }
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (IdType, &'a ItemType)> + 'a {
        self.inner_map
            .iter()
            .map(|(id, item)| (IdType::from(*id), item))
    }
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = (IdType, &'a mut ItemType)> + 'a {
        self.inner_map
            .iter_mut()
            .map(|(id, item)| (IdType::from(*id), item))
    }
    pub fn is_empty(&self) -> bool {
        self.inner_map.is_empty()
    }
    pub fn get_ids(&self) -> Vec<IdType> {
        self.inner_map.keys().map(|x| (*x).into()).collect()
    }
    pub fn entry(&mut self, key: IdType) -> Entry<u32, ItemType> {
        self.inner_map.entry(key.into())
    }
    pub fn contains_key(&self, key: IdType) -> bool {
        self.inner_map.contains_key(&key.into())
    }
    pub fn values(&self) -> impl Iterator<Item = &ItemType> {
        self.inner_map.values()
    }
}
