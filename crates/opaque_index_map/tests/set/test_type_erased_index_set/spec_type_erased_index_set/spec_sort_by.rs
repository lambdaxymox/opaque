use crate::set::common::erased::strategy_type_erased_index_set_max_len;
use opaque_index_map::TypeErasedIndexSet;

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

fn prop_sort_by_contains<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    let sorted_set = {
        let mut _sorted_set = entries.clone::<T, S, A>();
        _sorted_set.sort_by::<_, T, S, A>(|v1, v2| v1.cmp(v2));
        _sorted_set
    };

    for value in set.iter::<T, S, A>() {
        prop_assert!(sorted_set.contains::<_, T, S, A>(value));
    }

    for value in sorted_set.iter::<T, S, A>() {
        prop_assert!(set.contains::<_, T, S, A>(value));
    }

    Ok(())
}

fn prop_sort_by_get1<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    let sorted_set = {
        let mut _sorted_set = entries.clone::<T, S, A>();
        _sorted_set.sort_by::<_, T, S, A>(|v1, v2| v1.cmp(v2));
        _sorted_set
    };

    for value in set.iter::<T, S, A>() {
        let expected = set.get::<_, T, S, A>(value);
        let result = sorted_set.get::<_, T, S, A>(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sort_by_get2<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    let sorted_set = {
        let mut _sorted_set = entries.clone::<T, S, A>();
        _sorted_set.sort_by::<_, T, S, A>(|v1, v2| v1.cmp(v2));
        _sorted_set
    };

    for value in sorted_set.iter::<T, S, A>() {
        let expected = sorted_set.get::<_, T, S, A>(value);
        let result = set.get::<_, T, S, A>(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sort_by_len<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    let sorted_set = {
        let mut _sorted_set = entries.clone::<T, S, A>();
        _sorted_set.sort_by::<_, T, S, A>(|v1, v2| v1.cmp(v2));
        _sorted_set
    };

    prop_assert_eq!(sorted_set.len(), set.len());

    Ok(())
}

fn prop_sort_by_ordering<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let sorted_set = {
        let mut _sorted_set = entries.clone::<T, S, A>();
        _sorted_set.sort_by::<_, T, S, A>(|v1, v2| v1.cmp(v2));
        _sorted_set
    };

    let values: Vec<T> = sorted_set.iter::<T, S, A>().cloned().collect();
    for i in 1..values.len() {
        prop_assert!(values[i - 1] <= values[i]);
    }

    Ok(())
}

macro_rules! generate_props {
    (
        $module_name:ident,
        $value_typ:ty,
        $build_hasher_typ:ty,
        $alloc_typ:ty,
        $max_length:expr,
        $set_gen:ident,
    ) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_sort_by_contains(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_sort_by_contains::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sort_by_get1(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_sort_by_get1::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sort_by_get2(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_sort_by_get2::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sort_by_len(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_sort_by_len::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_sort_by_ordering(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_sort_by_ordering::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }
            }
        }
    };
}

generate_props!(
    u64,
    u64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
);
generate_props!(
    usize,
    usize,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
);
generate_props!(
    string,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
);
