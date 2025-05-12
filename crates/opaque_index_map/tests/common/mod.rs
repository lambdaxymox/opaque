pub mod opaque_index_map {
    use opaque_index_map::OpaqueIndexMap;

    use core::any;
    use std::hash;
    use std::alloc;

    pub fn from_entries<K, V>(entries: &[(K, V)]) -> OpaqueIndexMap
    where
        K: any::Any + Clone + Eq + hash::Hash,
        V: any::Any + Clone + Eq,
    {
        let mut map = OpaqueIndexMap::new::<K, V>();
        for (key, value) in entries.iter().cloned() {
            map.insert::<K, V, hash::RandomState, alloc::Global>(key, value);
        }

        map
    }

    pub fn from_entries_full<K, V>(entries: &[(K, V)]) -> OpaqueIndexMap
    where
        K: any::Any + Clone + Eq + hash::Hash,
        V: any::Any + Clone + Eq,
    {
        let mut map = OpaqueIndexMap::new::<K, V>();
        for (key, value) in entries.iter().cloned() {
            map.insert_full::<K, V, hash::RandomState, alloc::Global>(key, value);
        }

        map
    }

    pub fn clone<K, V, S, A>(opaque_map: &OpaqueIndexMap) -> OpaqueIndexMap
    where
        K: any::Any + Clone + Eq + hash::Hash,
        V: any::Any + Clone + Eq,
        S: any::Any + hash::BuildHasher + Clone,
        A: any::Any + alloc::Allocator + Clone,
    {
        let proj_map = opaque_map.as_proj::<K, V, S, A>();
        let cloned_proj_map = proj_map.clone();

        OpaqueIndexMap::from_proj(cloned_proj_map)
    }
}

pub mod typed_proj_index_map {
    use opaque_index_map::TypedProjIndexMap;

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
}
