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

fn prop_shift_remove_index_contains_key<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    let keys: Vec<K> = map.keys().cloned().collect();
    for key in keys.iter() {
        prop_assert!(map.contains_key(key));

        let index = map.get_index_of(key).unwrap();
        map.shift_remove_index(index);

        prop_assert!(!map.contains_key(key));
    }

    Ok(())
}

fn prop_shift_remove_index_get<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    let keys: Vec<K> = map.keys().cloned().collect();
    for key in keys.iter() {
        let expected = map.get(key).cloned();
        let index = map.get_index_of(key).unwrap();
        let result = map.shift_remove_index(index).map(|(_k, v)| v);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_shift_remove_index_get_key_value<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    let keys: Vec<K> = map.keys().cloned().collect();
    for key in keys.iter() {
        let expected = map.get_key_value(key).map(|(k, v)| (k.clone(), v.clone()));
        let index = map.get_index_of(key).unwrap();
        let result = map.shift_remove_index(index);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_shift_remove_index_get_mut<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    let keys: Vec<K> = map.keys().cloned().collect();
    for key in keys.iter() {
        let expected = map.get_mut(key).cloned();
        let index = map.get_index_of(key).unwrap();
        let result = map.shift_remove_index(index).map(|(_k, v)| v);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_shift_remove_index_len<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    let keys: Vec<K> = map.keys().cloned().collect();
    for (i, key_i) in keys.iter().enumerate() {
        let index = map.get_index_of(key_i).unwrap();
        map.shift_remove_index(index);

        let expected = keys.len() - i - 1;
        let result = map.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_shift_remove_index_preserves_order<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<K, V, S, A>(map: &TypeProjectedIndexMap<K, V, S, A>, index: usize) -> Vec<(K, V)>
    where
        K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        V: any::Any + Clone + Eq + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut entries = Vec::new();
        for entry in map.as_slice()[0..index].iter().map(|(k, v)| (k.clone(), v.clone())) {
            entries.push(entry);
        }

        for entry in map.as_slice()[(index + 1)..map.len()]
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
        {
            entries.push(entry);
        }

        entries
    }

    fn result<K, V, S, A>(map: &TypeProjectedIndexMap<K, V, S, A>, key: &K) -> Vec<(K, V)>
    where
        K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        V: any::Any + Clone + Eq + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut new_map = map.clone();
        let index = new_map.get_index_of(key).unwrap();
        new_map.shift_remove_index(index);

        let entries: Vec<(K, V)> = new_map.iter().map(|(key, value)| (key.clone(), value.clone())).collect();

        entries
    }

    let base_map = entries.clone();
    let base_keys: Vec<K> = base_map.keys().cloned().collect();
    for (index, key) in base_keys.iter().enumerate() {
        let expected = expected(&entries, index);
        let result = result(&base_map, key);

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
                fn prop_shift_remove_index_contains_key(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_shift_remove_index_contains_key(entries)?
                }

                #[test]
                fn prop_shift_remove_index_get(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_shift_remove_index_get(entries)?
                }

                #[test]
                fn prop_shift_remove_index_get_key_value(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_shift_remove_index_get_key_value(entries)?
                }

                #[test]
                fn prop_shift_remove_index_get_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_shift_remove_index_get_mut(entries)?
                }

                #[test]
                fn prop_shift_remove_index_len(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_shift_remove_index_len(entries)?
                }

                #[test]
                fn prop_shift_remove_index_preserves_order(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_shift_remove_index_preserves_order(entries)?
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
