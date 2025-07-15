use crate::set::common::projected::{
    SingleBoundedValue,
    strategy_type_projected_index_set_len,
};
use opaque_index_map::TypeProjectedIndexSet;

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

fn strategy_prop_move_index<T, S, A>(max_length: usize) -> impl Strategy<Value = (TypeProjectedIndexSet<T, S, A>, usize, usize)>
where
    T: any::Any + Clone + Eq + hash::Hash + Ord + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
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
        let set = strategy_type_projected_index_set_len(length);
        (set, 0..length, 0..length)
    })
}

fn prop_move_index_eq<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>, from: usize, to: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    set.move_index(from, to);

    prop_assert_eq!(set, entries);

    Ok(())
}

fn prop_move_index_move_index<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>, from: usize, to: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    set.move_index(from, to);
    set.move_index(to, from);

    prop_assert_eq!(set.as_slice(), entries.as_slice());

    Ok(())
}

fn prop_move_index_values<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>, from: usize, to: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    set.move_index(from, to);


    prop_assert_eq!(set.get_index(to), entries.get_index(from));

    Ok(())
}

fn prop_move_index_len<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>, from: usize, to: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    set.move_index(from, to);

    prop_assert_eq!(set.len(), entries.len());

    Ok(())
}

fn prop_move_index_get_index<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>, from: usize, to: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    set.move_index(from, to);

    let entries_iter = (0..entries.len()).filter(|i| i != &from);
    let set_iter = (0..set.len()).filter(|j| j != &to);
    for (i, j) in entries_iter.zip(set_iter) {
        let expected = entries.get_index(i);
        let result = set.get_index(j);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_move_index_ordering_min_to_max<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>, from: usize, to: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    let min = usize::min(from, to);
    let max = usize::max(from, to);
    set.move_index(min, max);

    // Every entry in `[0, min)` is untouched.
    for i in 0..min {
        let expected = entries.get_index(i);
        let result = set.get_index(i);

        prop_assert_eq!(result, expected);
    }

    // Every entry in `[min + 1, max)` shifts down one unit in the map's storage.
    for i in (min + 1)..max {
        let expected = entries.get_index(i);
        let result = set.get_index(i - 1);

        prop_assert_eq!(result, expected);
    }

    // The entry at `min` moves to `max`.
    prop_assert_eq!(set.get_index(max), entries.get_index(min));

    // Every entry in `[max + 1, set.len())` is untouched.
    for i in (max + 1)..set.len() {
        let expected = entries.get_index(i);
        let result = set.get_index(i);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_move_index_ordering_max_to_min<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>, from: usize, to: usize) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    let min = usize::min(from, to);
    let max = usize::max(from, to);
    set.move_index(max, min);

    // Every entry in `[0, min)` is untouched.
    for i in 0..min {
        let expected = entries.get_index(i);
        let result = set.get_index(i);

        prop_assert_eq!(result, expected);
    }

    // The entry at `max` moves to `min`.
    prop_assert_eq!(set.get_index(min), entries.get_index(max));

    // Every entry in `[min, max - 1)` shifts up one unit.
    for i in (min + 1)..max {
        let expected = entries.get_index(i - 1);
        let result = set.get_index(i);

        prop_assert_eq!(result, expected);
    }

    // Every entry in `[max + 1, set.len())` is untouched.
    for i in (max + 1)..set.len() {
        let expected = entries.get_index(i);
        let result = set.get_index(i);

        prop_assert_eq!(result, expected);
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
                fn prop_move_index_eq((entries, from, to) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_eq(entries, from, to)?
                }

                #[test]
                fn prop_move_index_move_index((entries, from, to) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_move_index(entries, from, to)?
                }

                #[test]
                fn prop_move_index_values((entries, from, to) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_values(entries, from, to)?
                }

                #[test]
                fn prop_move_index_len((entries, from, to) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_len(entries, from, to)?
                }

                #[test]
                fn prop_move_index_get_index((entries, from, to) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_get_index(entries, from, to)?
                }

                #[test]
                fn prop_move_index_ordering_min_to_max((entries, from, to) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_ordering_min_to_max(entries, from, to)?
                }

                #[test]
                fn prop_move_index_ordering_max_to_min((entries, from, to) in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_move_index_ordering_max_to_min(entries, from, to)?
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
    strategy_prop_move_index,
);
generate_props!(
    usize,
    usize,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_move_index,
);
generate_props!(
    string,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_prop_move_index,
);
