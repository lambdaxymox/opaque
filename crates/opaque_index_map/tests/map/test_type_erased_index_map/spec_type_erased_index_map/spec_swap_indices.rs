use crate::map::common::erased::{
    SingleBoundedValue,
    strategy_type_erased_index_map_len,
};
use opaque_index_map::TypeErasedIndexMap;

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

fn strategy_prop_swap_indices<K, V, S, A>(max_length: usize) -> impl Strategy<Value = (TypeErasedIndexMap, usize, usize)>
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
        let map = strategy_type_erased_index_map_len::<K, V, S, A>(length);
        (map, 0..length, 0..length)
    })
}

fn prop_swap_indices_eq<K, V, S, A>(entries: TypeErasedIndexMap, a: usize, b: usize) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone::<K, V, S, A>();
    map.swap_indices::<K, V, S, A>(a, b);

    prop_assert_eq!(map.as_proj::<K, V, S, A>(), entries.as_proj::<K, V, S, A>());

    Ok(())
}

fn prop_swap_indices_swap_indices<K, V, S, A>(entries: TypeErasedIndexMap, a: usize, b: usize) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone::<K, V, S, A>();
    map.swap_indices::<K, V, S, A>(a, b);
    map.swap_indices::<K, V, S, A>(a, b);

    prop_assert_eq!(map.as_slice::<K, V, S, A>(), entries.as_slice::<K, V, S, A>());

    Ok(())
}

fn prop_swap_indices_values<K, V, S, A>(entries: TypeErasedIndexMap, a: usize, b: usize) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone::<K, V, S, A>();
    map.swap_indices::<K, V, S, A>(a, b);


    prop_assert_eq!(map.get_index::<K, V, S, A>(a), entries.get_index::<K, V, S, A>(b));
    prop_assert_eq!(map.get_index::<K, V, S, A>(b), entries.get_index::<K, V, S, A>(a));

    Ok(())
}

fn prop_swap_indices_len<K, V, S, A>(entries: TypeErasedIndexMap, a: usize, b: usize) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone::<K, V, S, A>();
    map.swap_indices::<K, V, S, A>(a, b);

    prop_assert_eq!(map.len(), entries.len());

    Ok(())
}

fn prop_swap_indices_get_index<K, V, S, A>(entries: TypeErasedIndexMap, a: usize, b: usize) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone::<K, V, S, A>();
    map.swap_indices::<K, V, S, A>(a, b);

    for i in (0..map.len()).filter(|j| j != &a && j != &b) {
        let expected = entries.get_index::<K, V, S, A>(i);
        let result = map.get_index::<K, V, S, A>(i);

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
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_swap_indices_eq::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries, a, b)?
                }

                #[test]
                fn prop_swap_indices_swap_indices((entries, a, b) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_swap_indices_swap_indices::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries, a, b)?
                }

                #[test]
                fn prop_swap_indices_values((entries, a, b) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_swap_indices_values::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries, a, b)?
                }

                #[test]
                fn prop_swap_indices_len((entries, a, b) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_swap_indices_len::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries, a, b)?
                }

                #[test]
                fn prop_swap_indices_get_index((entries, a, b) in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_swap_indices_get_index::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries, a, b)?
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
