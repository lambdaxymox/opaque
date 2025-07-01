use crate::map::common::erased::strategy_type_erased_index_map_max_len;
use opaque_index_map::OpaqueIndexMap;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use proptest::prelude::*;

fn prop_truncate_len_length_less_than_or_equal_to<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<K, V, S, A>(entries: &OpaqueIndexMap, len: usize) -> Vec<(K, V)>
    where
        K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        V: any::Any + Clone + Eq + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let vec: Vec<(K, V)> = entries
            .iter::<K, V, S, A>()
            .map(|(k, v)| (k.clone(), v.clone()))
            .take(len)
            .collect();

        vec
    }

    fn result<K, V, S, A>(map: &OpaqueIndexMap, len: usize) -> Vec<(K, V)>
    where
        K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        V: any::Any + Clone + Eq + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut cloned_map = map.clone::<K, V, S, A>();
        cloned_map.truncate::<K, V, S, A>(len);

        let vec: Vec<(K, V)> = cloned_map
            .iter::<K, V, S, A>()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        vec
    }
    
    for len in 0..entries.len() {
        let map = entries.clone::<K, V, S, A>();
        let expected_entries = expected::<K, V, S, A>(&entries, len);
        let result_entries = result::<K, V, S, A>(&map, len);
        let expected = expected_entries.len();
        let result = result_entries.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_truncate_length_less_than_or_equal_to<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<K, V, S, A>(entries: &OpaqueIndexMap, len: usize) -> Vec<(K, V)>
    where
        K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        V: any::Any + Clone + Eq + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let vec: Vec<(K, V)> = entries
            .iter::<K, V, S, A>()
            .map(|(k, v)| (k.clone(), v.clone()))
            .take(len)
            .collect();

        vec
    }

    fn result<K, V, S, A>(map: &OpaqueIndexMap, len: usize) -> Vec<(K, V)>
    where
        K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        V: any::Any + Clone + Eq + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut cloned_map = map.clone::<K, V, S, A>();
        cloned_map.truncate::<K, V, S, A>(len);

        let vec: Vec<(K, V)> = cloned_map
            .iter::<K, V, S, A>()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        vec
    }
    
    for len in 0..entries.len() {
        let map = entries.clone::<K, V, S, A>();
        let expected = expected::<K, V, S, A>(&entries, len);
        let result = result::<K, V, S, A>(&map, len);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

macro_rules! generate_props {
    (
        $module_name:ident,
        $key_typ:ty,
        $value_typ:ty,
        $build_hasher_typ:ty,
        $alloc_typ:ty,
        $max_length:expr,
        $map_gen:ident,
    ) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::hash;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_truncate_len_length_less_than_or_equal_to(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_truncate_len_length_less_than_or_equal_to::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_truncate_length_less_than_or_equal_to(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexMap = entries;
                    super::prop_truncate_length_less_than_or_equal_to::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }
            }
        }
    };
}

generate_props!(
    u64_i64,
    u64,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
);
