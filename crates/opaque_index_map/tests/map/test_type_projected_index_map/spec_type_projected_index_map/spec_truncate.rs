use crate::map::common::projected::strategy_type_projected_index_map_max_len;
use opaque_index_map::TypeProjectedIndexMap;

use core::any;
use core::fmt;
use std::format;
use std::hash;
use std::string::String;
use std::vec::Vec;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn prop_truncate_len_length_less_than_or_equal_to<K, V, S, A>(
    entries: TypeProjectedIndexMap<K, V, S, A>,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<K, V, S, A>(entries: &TypeProjectedIndexMap<K, V, S, A>, len: usize) -> Vec<(K, V)>
    where
        K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        V: any::Any + Clone + Eq + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let vec: Vec<(K, V)> = entries.iter().map(|(k, v)| (k.clone(), v.clone())).take(len).collect();

        vec
    }

    fn result<K, V, S, A>(map: &TypeProjectedIndexMap<K, V, S, A>, len: usize) -> Vec<(K, V)>
    where
        K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        V: any::Any + Clone + Eq + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut cloned_map = map.clone();
        cloned_map.truncate(len);

        let vec: Vec<(K, V)> = cloned_map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        vec
    }

    for len in 0..entries.len() {
        let map = entries.clone();
        let expected_entries = expected(&entries, len);
        let result_entries = result(&map, len);
        let expected = expected_entries.len();
        let result = result_entries.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_truncate_length_less_than_or_equal_to<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<K, V, S, A>(entries: &TypeProjectedIndexMap<K, V, S, A>, len: usize) -> Vec<(K, V)>
    where
        K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        V: any::Any + Clone + Eq + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let vec: Vec<(K, V)> = entries.iter().map(|(k, v)| (k.clone(), v.clone())).take(len).collect();

        vec
    }

    fn result<K, V, S, A>(map: &TypeProjectedIndexMap<K, V, S, A>, len: usize) -> Vec<(K, V)>
    where
        K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        V: any::Any + Clone + Eq + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut cloned_map = map.clone();
        cloned_map.truncate(len);

        let vec: Vec<(K, V)> = cloned_map.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

        vec
    }

    for len in 0..entries.len() {
        let map = entries.clone();
        let expected = expected(&entries, len);
        let result = result(&map, len);

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
            use super::*;
            proptest! {
                #[test]
                fn prop_truncate_len_length_less_than_or_equal_to(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_truncate_len_length_less_than_or_equal_to(entries)?
                }

                #[test]
                fn prop_truncate_length_less_than_or_equal_to(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_truncate_length_less_than_or_equal_to(entries)?
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
    strategy_type_projected_index_map_max_len,
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_map_max_len,
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_map_max_len,
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_map_max_len,
);
