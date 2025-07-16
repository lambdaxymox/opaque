use crate::map::common::projected::{
    SingleBoundedValue,
    strategy_type_projected_index_map_len,
};
use opaque_index_map::TypeProjectedIndexMap;

use core::any;
use core::fmt;
use std::format;
use std::string::String;
use std::{
    hash,
    ops,
};

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn strategy_prop_move_index<K, V, S, A>(
    max_length: usize,
) -> impl Strategy<Value = (TypeProjectedIndexMap<K, V, S, A>, usize, usize)>
where
    K: any::Any + Clone + Eq + hash::Hash + Ord + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    V: any::Any + Clone + Eq + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn clamped_interval(max_length: usize) -> ops::RangeInclusive<usize> {
        if max_length == 0 { 1..=1 } else { 1..=max_length }
    }

    clamped_interval(max_length).prop_flat_map(move |length| {
        let map = strategy_type_projected_index_map_len(length);
        (map, 0..length, 0..length)
    })
}

fn prop_move_index_eq<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>, from: usize, to: usize) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    map.move_index(from, to);

    prop_assert_eq!(map, entries);

    Ok(())
}

fn prop_move_index_move_index<K, V, S, A>(
    entries: TypeProjectedIndexMap<K, V, S, A>,
    from: usize,
    to: usize,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    map.move_index(from, to);
    map.move_index(to, from);

    prop_assert_eq!(map.as_slice(), entries.as_slice());

    Ok(())
}

fn prop_move_index_values<K, V, S, A>(
    entries: TypeProjectedIndexMap<K, V, S, A>,
    from: usize,
    to: usize,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    map.move_index(from, to);


    prop_assert_eq!(map.get_index(to), entries.get_index(from));

    Ok(())
}

fn prop_move_index_len<K, V, S, A>(
    entries: TypeProjectedIndexMap<K, V, S, A>,
    from: usize,
    to: usize,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    map.move_index(from, to);

    prop_assert_eq!(map.len(), entries.len());

    Ok(())
}

fn prop_move_index_get_index<K, V, S, A>(
    entries: TypeProjectedIndexMap<K, V, S, A>,
    from: usize,
    to: usize,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    map.move_index(from, to);

    let entries_iter = (0..entries.len()).filter(|i| i != &from);
    let map_iter = (0..map.len()).filter(|j| j != &to);
    for (i, j) in entries_iter.zip(map_iter) {
        let expected = entries.get_index(i);
        let result = map.get_index(j);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_move_index_ordering_min_to_max<K, V, S, A>(
    entries: TypeProjectedIndexMap<K, V, S, A>,
    from: usize,
    to: usize,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    let min = usize::min(from, to);
    let max = usize::max(from, to);
    map.move_index(min, max);

    // Every entry in `[0, min)` is untouched.
    for i in 0..min {
        let expected = entries.get_index(i);
        let result = map.get_index(i);

        prop_assert_eq!(result, expected);
    }

    // Every entry in `[min + 1, max)` shifts down one unit in the map's storage.
    for i in (min + 1)..max {
        let expected = entries.get_index(i);
        let result = map.get_index(i - 1);

        prop_assert_eq!(result, expected);
    }

    // The entry at `min` moves to `max`.
    prop_assert_eq!(map.get_index(max), entries.get_index(min));

    // Every entry in `[max + 1, map.len())` is untouched.
    for i in (max + 1)..map.len() {
        let expected = entries.get_index(i);
        let result = map.get_index(i);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_move_index_ordering_max_to_min<K, V, S, A>(
    entries: TypeProjectedIndexMap<K, V, S, A>,
    from: usize,
    to: usize,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    let min = usize::min(from, to);
    let max = usize::max(from, to);
    map.move_index(max, min);

    // Every entry in `[0, min)` is untouched.
    for i in 0..min {
        let expected = entries.get_index(i);
        let result = map.get_index(i);

        prop_assert_eq!(result, expected);
    }

    // The entry at `max` moves to `min`.
    prop_assert_eq!(map.get_index(min), entries.get_index(max));

    // Every entry in `[min, max - 1)` shifts up one unit.
    for i in (min + 1)..max {
        let expected = entries.get_index(i - 1);
        let result = map.get_index(i);

        prop_assert_eq!(result, expected);
    }

    // Every entry in `[max + 1, map.len())` is untouched.
    for i in (max + 1)..map.len() {
        let expected = entries.get_index(i);
        let result = map.get_index(i);

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
                fn prop_move_index_eq((entries, from, to) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_eq(entries, from, to)?
                }

                #[test]
                fn prop_move_index_move_index((entries, from, to) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_move_index(entries, from, to)?
                }

                #[test]
                fn prop_move_index_values((entries, from, to) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_values(entries, from, to)?
                }

                #[test]
                fn prop_move_index_len((entries, from, to) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_len(entries, from, to)?
                }

                #[test]
                fn prop_move_index_get_index((entries, from, to) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_get_index(entries, from, to)?
                }

                #[test]
                fn prop_move_index_ordering_min_to_max((entries, from, to) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_ordering_min_to_max(entries, from, to)?
                }

                #[test]
                fn prop_move_index_ordering_max_to_min((entries, from, to) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_ordering_max_to_min(entries, from, to)?
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
    strategy_prop_move_index,
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_move_index,
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_move_index,
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_move_index,
);
