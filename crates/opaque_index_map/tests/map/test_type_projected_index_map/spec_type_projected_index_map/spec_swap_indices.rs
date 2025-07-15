use crate::map::common::projected::{
    SingleBoundedValue,
    strategy_type_projected_index_map_len,
};
use opaque_index_map::TypeProjectedIndexMap;

use core::any;
use core::fmt;
use std::{hash, ops};
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn strategy_prop_swap_indices<K, V, S, A>(max_length: usize) -> impl Strategy<Value = (TypeProjectedIndexMap<K, V, S, A>, usize, usize)>
where
    K: any::Any + Clone + Eq + hash::Hash + Ord + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    V: any::Any + Clone + Eq + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone + Default + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn clamped_interval(max_length: usize) -> ops::RangeInclusive<usize> {
        if max_length == 0 {
            1..=1
        } else {
            1..=max_length
        }
    }

    clamped_interval(max_length).prop_flat_map(move |length| {
        let map = strategy_type_projected_index_map_len(length);
        (map, 0..length, 0..length)
    })
}

fn prop_swap_indices_eq<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>, a: usize, b: usize) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    map.swap_indices(a, b);

    prop_assert_eq!(map, entries);

    Ok(())
}

fn prop_swap_indices_swap_indices<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>, a: usize, b: usize) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    map.swap_indices(a, b);
    map.swap_indices(a, b);

    prop_assert_eq!(map.as_slice(), entries.as_slice());

    Ok(())
}

fn prop_swap_indices_values<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>, a: usize, b: usize) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    map.swap_indices(a, b);


    prop_assert_eq!(map.get_index(a), entries.get_index(b));
    prop_assert_eq!(map.get_index(b), entries.get_index(a));

    Ok(())
}

fn prop_swap_indices_len<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>, a: usize, b: usize) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    map.swap_indices(a, b);

    prop_assert_eq!(map.len(), entries.len());

    Ok(())
}

fn prop_swap_indices_get_index<K, V, S, A>(entries: TypeProjectedIndexMap<K, V, S, A>, a: usize, b: usize) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    map.swap_indices(a, b);

    for i in (0..map.len()).filter(|j| j != &a && j != &b) {
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
                fn prop_swap_indices_eq((entries, a, b) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_indices_eq(entries, a, b)?
                }

                #[test]
                fn prop_swap_indices_swap_indices((entries, a, b) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_indices_swap_indices(entries, a, b)?
                }

                #[test]
                fn prop_swap_indices_values((entries, a, b) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_indices_values(entries, a, b)?
                }

                #[test]
                fn prop_swap_indices_len((entries, a, b) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_indices_len(entries, a, b)?
                }

                #[test]
                fn prop_swap_indices_get_index((entries, a, b) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_swap_indices_get_index(entries, a, b)?
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
    strategy_prop_swap_indices,
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_swap_indices,
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_swap_indices,
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_swap_indices,
);
