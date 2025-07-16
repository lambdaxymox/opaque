use crate::set::common::projected::strategy_type_projected_index_set_max_len;
use opaque_index_map::TypeProjectedIndexSet;

use core::any;
use core::fmt;
use std::format;
use std::string::String;
use std::vec::Vec;
use std::{
    cmp,
    hash,
};

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn from_sorted_unstable_by_in<F, T, S, A>(entries: &TypeProjectedIndexSet<T, S, A>, cmp: F) -> TypeProjectedIndexSet<T, S, A>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
    F: FnMut(&T, &T) -> cmp::Ordering,
{
    let mut set = TypeProjectedIndexSet::with_capacity_and_hasher_proj_in(
        entries.len(),
        entries.hasher().clone(),
        entries.allocator().clone(),
    );

    for value in entries.clone().sorted_unstable_by(cmp) {
        set.insert(value);
    }

    set
}

fn prop_sorted_unstable_by_contains<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    let sorted_set = from_sorted_unstable_by_in(&entries, |v1, v2| v1.cmp(v2));

    for value in set.iter() {
        prop_assert!(sorted_set.contains(value));
    }

    for value in sorted_set.iter() {
        prop_assert!(set.contains(value));
    }

    Ok(())
}

fn prop_sorted_unstable_by_get1<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    let sorted_set = from_sorted_unstable_by_in(&entries, |v1, v2| v1.cmp(v2));

    for value in set.iter() {
        let expected = set.get(value);
        let result = sorted_set.get(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sorted_unstable_by_get2<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    let sorted_set = from_sorted_unstable_by_in(&entries, |v1, v2| v1.cmp(v2));

    for value in sorted_set.iter() {
        let expected = sorted_set.get(value);
        let result = set.get(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_sorted_unstable_by_len<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone();
    let sorted_set = from_sorted_unstable_by_in(&entries, |v1, v2| v1.cmp(v2));

    prop_assert_eq!(sorted_set.len(), set.len());

    Ok(())
}

fn prop_sorted_unstable_by_ordering<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let sorted_set = from_sorted_unstable_by_in(&entries, |v1, v2| v1.cmp(v2));

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
                fn prop_sorted_unstable_by_contains(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sorted_unstable_by_contains(entries)?
                }

                #[test]
                fn prop_sorted_unstable_by_get1(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sorted_unstable_by_get1(entries)?
                }

                #[test]
                fn prop_sorted_unstable_by_get2(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sorted_unstable_by_get2(entries)?
                }

                #[test]
                fn prop_sorted_unstable_by_len(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sorted_unstable_by_len(entries)?
                }

                #[test]
                fn prop_sorted_unstable_by_ordering(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_sorted_unstable_by_ordering(entries)?
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
