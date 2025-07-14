use crate::set::common::projected::strategy_type_projected_index_set_max_len;
use opaque_index_map::TypeProjectedIndexSet;

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

fn prop_sort_by_contains<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    let sorted_set = {
        let mut _sorted_set = entries.clone();
        _sorted_set.sort_by(|v1, v2| v1.cmp(v2));
        _sorted_set
    };

    for value in set.iter() {
        prop_assert!(sorted_set.contains(value));
    }

    for value in sorted_set.iter() {
        prop_assert!(set.contains(value));
    }

    Ok(())
}

fn prop_sort_by_get1<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    let sorted_set = {
        let mut _sorted_set = entries.clone();
        _sorted_set.sort_by(|v1, v2| v1.cmp(v2));
        _sorted_set
    };

    for value in set.iter() {
        let expected = set.get(value);
        let result = sorted_set.get(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sort_by_get2<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    let sorted_set = {
        let mut _sorted_set = entries.clone();
        _sorted_set.sort_by(|v1, v2| v1.cmp(v2));
        _sorted_set
    };

    for value in sorted_set.iter() {
        let expected = sorted_set.get(value);
        let result = set.get(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sort_by_len<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    let sorted_set = {
        let mut _sorted_set = entries.clone();
        _sorted_set.sort_by(|v1, v2| v1.cmp(v2));
        _sorted_set
    };

    prop_assert_eq!(sorted_set.len(), set.len());

    Ok(())
}

fn prop_sort_by_ordering<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let sorted_set = {
        let mut _sorted_set = entries.clone();
        _sorted_set.sort_by(|v1, v2| v1.cmp(v2));
        _sorted_set
    };

    let values: Vec<T> = sorted_set.iter().cloned().collect();
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
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sort_by_contains(entries)?
                }

                #[test]
                fn prop_sort_by_get1(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sort_by_get1(entries)?
                }

                #[test]
                fn prop_sort_by_get2(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sort_by_get2(entries)?
                }

                #[test]
                fn prop_sort_by_len(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sort_by_len(entries)?
                }

                #[test]
                fn prop_sort_by_ordering(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sort_by_ordering(entries)?
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
    strategy_type_projected_index_set_max_len,
);
generate_props!(
    usize,
    usize,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_set_max_len,
);
generate_props!(
    string,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_set_max_len,
);
