use crate::map::common::projected::strategy_type_projected_index_map_max_len;
use opaque_index_map::TypeProjectedIndexMap;

use core::any;
use core::fmt;
use std::hash;
use std::vec::Vec;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn from_entries_insert_full_in<K, V, S, A>(entries: &TypeProjectedIndexMap<K, V, S, A>) -> TypeProjectedIndexMap<K, V, S, A>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map: TypeProjectedIndexMap<K, V, S, A> = TypeProjectedIndexMap::with_hasher_proj_in(
        entries.hasher().clone(),
        entries.allocator().clone(),
    );

    for (key, value) in entries
        .as_slice()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
    {
        map.insert_full(key, value);
    }

    map
}

fn prop_insert_full_as_mut_slice<K, V, S, A>(mut entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = from_entries_insert_full_in(&entries);
    let expected = entries.as_mut_slice();
    let result = map.as_mut_slice();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_insert_full_as_slice<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in(&entries);
    let expected = entries.as_slice();
    let result = map.as_slice();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_insert_full_contains_key<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash,
    V: any::Any + Clone + Eq,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = TypeProjectedIndexMap::with_hasher_proj_in(
        entries.hasher().clone(),
        entries.allocator().clone()
    );

    for key in entries.iter().map(|tuple| tuple.0) {
        prop_assert!(!map.contains_key(key));
    }

    for (key, value) in entries.iter().map(|(k, v)| (k.clone(), v.clone())) {
        map.insert(key, value);
    }

    for key in entries.iter().map(|tuple| tuple.0) {
        prop_assert!(map.contains_key(key));
    }

    Ok(())
}

fn prop_insert_full_get<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in(&entries);
    for (key, value) in entries.iter() {
        let expected = Some(value);
        let result = map.get(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_get_full<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in(&entries);
    for (index, (key, value)) in entries.iter().enumerate() {
        let expected = Some((index, key, value));
        let result = map.get_full(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_get_mut<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = from_entries_insert_full_in(&entries);
    for (key, value) in entries.iter() {
        let mut cloned_value = value.clone();
        let expected = Some(&mut cloned_value);
        let result = map.get_mut(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_get_full_mut<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = from_entries_insert_full_in(&entries);
    for (index, (key, value)) in entries.iter().enumerate() {
        let mut cloned_value = value.clone();
        let expected = Some((index, key, &mut cloned_value));
        let result = map.get_full_mut(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_get_index_of<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in(&entries);
    for (index, key) in entries.keys().enumerate() {
        let expected = Some(index);
        let result = map.get_index_of(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_get_key_value<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in(&entries);
    for (key, value) in entries.iter() {
        let expected = Some((key, value));
        let result = map.get_key_value(key);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_iter<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in(&entries);
    for (result, expected) in map.iter().zip(entries.iter()) {
        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_iter_mut<K, V, S, A>(mut entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = from_entries_insert_full_in(&entries);
    for (result, expected) in map.iter_mut().zip(entries.iter_mut()) {
        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_len<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in(&entries);
    let expected = entries.len();
    let result = map.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_insert_full_values<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = from_entries_insert_full_in(&entries);
    let expected: Vec<V> = entries
        .values()
        .cloned()
        .collect();
    let result: Vec<V> = map
        .values()
        .cloned()
        .collect();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_insert_full_values_mut<K, V, S, A>(mut entries: TypeProjectedIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = from_entries_insert_full_in(&entries);
    let expected: Vec<V> = entries
        .values_mut()
        .map(|value| value.clone())
        .collect();
    let result: Vec<V> = map
        .values_mut()
        .map(|value| value.clone())
        .collect();

    prop_assert_eq!(result, expected);

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
                fn prop_insert_full_as_mut_slice(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_as_mut_slice(entries)?
                }

                #[test]
                fn prop_insert_full_as_slice(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_as_slice(entries)?
                }

                #[test]
                fn prop_insert_full_contains_key(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_contains_key(entries)?
                }

                #[test]
                fn prop_insert_full_get(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_get(entries)?
                }

                #[test]
                fn prop_insert_full_get_full(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_get_full(entries)?
                }

                #[test]
                fn prop_insert_full_get_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_get_mut(entries)?
                }

                #[test]
                fn prop_insert_full_get_full_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_get_full_mut(entries)?
                }

                #[test]
                fn prop_insert_full_get_index_of(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_get_index_of(entries)?
                }

                #[test]
                fn prop_insert_full_get_key_value(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_get_key_value(entries)?
                }

                #[test]
                fn prop_insert_full_iter(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_iter(entries)?
                }

                #[test]
                fn prop_insert_full_iter_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_iter_mut(entries)?
                }

                #[test]
                fn prop_insert_full_len(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_len(entries)?
                }


                #[test]
                fn prop_insert_full_values(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_values(entries)?
                }

                #[test]
                fn prop_insert_full_values_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_values_mut(entries)?
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
