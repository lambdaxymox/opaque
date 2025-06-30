use crate::set::common::erased::strategy_type_erased_index_set_max_len;
use opaque_index_map::OpaqueIndexSet;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use proptest::prelude::*;

fn from_entries_insert_in<T, S, A>(entries: &OpaqueIndexSet) -> OpaqueIndexSet
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = OpaqueIndexSet::with_hasher_proj_in::<T, S, A>(
        entries.hasher::<T, S, A>().clone(),
        entries.allocator::<T, S, A>().clone(),
    );

    for value in entries
        .as_slice::<T, S, A>()
        .iter()
        .cloned()
    {
        set.insert::<T, S, A>(value);
    }

    set
}

fn prop_insert_as_slice<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<T, S, A>(set: &OpaqueIndexSet) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let expected: Vec<T> = set.iter::<T, S, A>().cloned().collect();

        expected
    }

    fn result<T, S, A>(set: &OpaqueIndexSet) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let result: Vec<T> = set.as_slice::<T, S, A>().iter().cloned().collect();

        result
    }

    let set = from_entries_insert_in::<T, S, A>(&entries);
    let expected = expected::<T, S, A>(&entries);
    let result = result::<T, S, A>(&set);

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_insert_contains<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = OpaqueIndexSet::with_hasher_proj_in::<T, S, A>(
        entries.hasher::<T, S, A>().clone(),
        entries.allocator::<T, S, A>().clone(),
    );

    for value in entries.iter::<T, S, A>() {
        prop_assert!(!set.contains::<_, T, S, A>(value));
    }

    for value in entries.iter::<T, S, A>().cloned() {
        set.insert::<T, S, A>(value);
    }

    for value in entries.iter::<T, S, A>() {
        prop_assert!(set.contains::<_, T, S, A>(value));
    }

    Ok(())
}

fn prop_insert_get<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_entries_insert_in::<T, S, A>(&entries);
    for value in entries.iter::<T, S, A>() {
        let expected = Some(value);
        let result = set.get::<_, T, S, A>(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_get_full<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_entries_insert_in::<T, S, A>(&entries);
    for (index, value) in entries.iter::<T, S, A>().enumerate() {
        let expected = Some((index, value));
        let result = set.get_full::<_, T, S, A>(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_get_index_of<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_entries_insert_in::<T, S, A>(&entries);
    for (index, value) in entries.iter::<T, S, A>().enumerate() {
        let expected = Some(index);
        let result = set.get_index_of::<_, T, S, A>(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_iter<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_entries_insert_in::<T, S, A>(&entries);
    for (result, expected) in set.iter::<T, S, A>().zip(entries.iter::<T, S, A>()) {
        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_len<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<T, S, A>(set: &OpaqueIndexSet) -> usize
    where
        T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let unique_values = {
            let mut values = Vec::from_iter(set.iter::<T, S, A>().cloned());
            values.sort();
            values.dedup();
            values
        };

        unique_values.len()
    }

    let set = from_entries_insert_in::<T, S, A>(&entries);
    let expected = expected::<T, S, A>(&entries);
    let result = set.len();

    prop_assert_eq!(result, expected);

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
            use proptest::prelude::*;
            use std::hash;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_insert_as_slice(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_insert_as_slice::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_contains(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_insert_contains::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_get(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_insert_get::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_get_full(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_insert_get_full::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_get_index_of(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_insert_get_index_of::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_iter(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_insert_iter::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_insert_len(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_insert_len::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
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
    128,
    strategy_type_erased_index_set_max_len,
);
generate_props!(
    usize,
    usize,
    hash::RandomState,
    alloc::Global,
    128,
    strategy_type_erased_index_set_max_len,
);
generate_props!(
    string,
    String,
    hash::RandomState,
    alloc::Global,
    128,
    strategy_type_erased_index_set_max_len,
);
