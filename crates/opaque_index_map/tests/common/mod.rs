use opaque_index_map::OpaqueIndexMap;

use core::hash;

pub fn from_entries<K, V>(entries: &[(K, V)]) -> OpaqueIndexMap
where
    K: Clone + Eq + hash::Hash + 'static,
    V: Clone + Eq + 'static,
{
    let mut map = OpaqueIndexMap::new::<K, V>();
    for (key, value) in entries.iter().cloned() {
        map.insert(key, value);
    }

    map
}

pub fn from_entries_full<K, V>(entries: &[(K, V)]) -> OpaqueIndexMap
where
    K: Clone + Eq + hash::Hash + 'static,
    V: Clone + Eq + 'static,
{
    let mut map = OpaqueIndexMap::new::<K, V>();
    for (key, value) in entries.iter().cloned() {
        map.insert_full(key, value);
    }

    map
}
