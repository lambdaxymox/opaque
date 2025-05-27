use opaque_index_map::map::TypedProjIndexMap;

use core::any;
use std::hash;
use std::alloc;

pub fn from_entries<K, V>(entries: &[(K, V)]) -> TypedProjIndexMap<K, V, hash::RandomState, alloc::Global>
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
{
    let mut map = TypedProjIndexMap::new();
    for (key, value) in entries.iter().cloned() {
        map.insert(key, value);
    }

    map
}

pub fn from_entries_full<K, V>(entries: &[(K, V)]) -> TypedProjIndexMap<K, V, hash::RandomState, alloc::Global>
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
{
    let mut map = TypedProjIndexMap::new();
    for (key, value) in entries.iter().cloned() {
        map.insert_full(key, value);
    }

    map
}
