use crate::map::common::projected::strategy_type_projected_index_map_max_len;
use opaque_index_map::TypeProjectedIndexMap;

use core::any;
use core::cmp;
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

fn from_sorted_unstable_by_in<F, K, V, S, A>(
    entries: &TypeProjectedIndexMap<K, V, S, A>,
    cmp: F,
) -> TypeProjectedIndexMap<K, V, S, A>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
    F: FnMut(&K, &V, &K, &V) -> cmp::Ordering,
{
    let mut map = TypeProjectedIndexMap::with_capacity_and_hasher_proj_in(
        entries.len(),
        entries.hasher().clone(),
        entries.allocator().clone(),
    );

    for (key, value) in entries.clone().sorted_unstable_by(cmp) {
        map.insert(key, value);
    }

    map
}

fn prop_sorted_unstable_by_contains_key<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    let sorted_map: TypeProjectedIndexMap<K, V, S, A> = from_sorted_unstable_by_in(&entries, |_k1, v1, _k2, v2| v1.cmp(v2));

    for key in map.keys() {
        prop_assert!(sorted_map.contains_key(key));
    }

    for key in sorted_map.keys() {
        prop_assert!(map.contains_key(key));
    }

    Ok(())
}

fn prop_sorted_unstable_by_get1<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    let sorted_map: TypeProjectedIndexMap<K, V, S, A> = from_sorted_unstable_by_in(&entries, |_k1, v1, _k2, v2| v1.cmp(v2));

    for key in map.keys() {
        let expected = map.get(key);
        let result = sorted_map.get(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sorted_unstable_by_get2<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    let sorted_map: TypeProjectedIndexMap<K, V, S, A> = from_sorted_unstable_by_in(&entries, |_k1, v1, _k2, v2| v1.cmp(v2));

    for key in sorted_map.keys() {
        let expected = sorted_map.get(key);
        let result = map.get(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sorted_unstable_by_get_key_value1<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    let sorted_map: TypeProjectedIndexMap<K, V, S, A> = from_sorted_unstable_by_in(&entries, |_k1, v1, _k2, v2| v1.cmp(v2));

    for key in map.keys() {
        let expected = map.get_key_value(key);
        let result = sorted_map.get_key_value(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sorted_unstable_by_get_key_value2<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    let sorted_map: TypeProjectedIndexMap<K, V, S, A> = from_sorted_unstable_by_in(&entries, |_k1, v1, _k2, v2| v1.cmp(v2));

    for key in sorted_map.keys() {
        let expected = sorted_map.get_key_value(key);
        let result = map.get_key_value(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sorted_unstable_by_len<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone();
    let sorted_map: TypeProjectedIndexMap<K, V, S, A> = from_sorted_unstable_by_in(&entries, |_k1, v1, _k2, v2| v1.cmp(v2));

    prop_assert_eq!(sorted_map.len(), map.len());

    Ok(())
}

fn prop_sorted_unstable_by_ordering<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + Ord + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let sorted_map: TypeProjectedIndexMap<K, V, S, A> = from_sorted_unstable_by_in(&entries, |_k1, v1, _k2, v2| v1.cmp(v2));

    let values: Vec<V> = sorted_map.values().cloned().collect();
    for i in 1..values.len() {
        prop_assert!(values[i - 1] <= values[i]);
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
                fn prop_sorted_unstable_by_contains_key(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sorted_unstable_by_contains_key(entries)?
                }

                #[test]
                fn prop_sorted_unstable_by_get1(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sorted_unstable_by_get1(entries)?
                }

                #[test]
                fn prop_sorted_unstable_by_get2(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sorted_unstable_by_get2(entries)?
                }

                #[test]
                fn prop_sorted_unstable_by_get_key_value1(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sorted_unstable_by_get_key_value1(entries)?
                }

                #[test]
                fn prop_sorted_unstable_by_get_key_value2(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sorted_unstable_by_get_key_value2(entries)?
                }

                #[test]
                fn prop_sorted_unstable_by_len(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sorted_unstable_by_len(entries)?
                }

                #[test]
                fn prop_sorted_unstable_by_ordering(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sorted_unstable_by_ordering(entries)?
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
