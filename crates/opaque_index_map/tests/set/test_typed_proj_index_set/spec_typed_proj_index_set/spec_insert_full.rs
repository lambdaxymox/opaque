use crate::set::common::projected::strategy_type_projected_index_set_max_len;
use opaque_index_map::TypedProjIndexSet;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use proptest::prelude::*;

fn from_entries_insert_full_in<T, S, A>(entries: &TypedProjIndexSet<T, S, A>) -> TypedProjIndexSet<T, S, A>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set: TypedProjIndexSet<T, S, A> = TypedProjIndexSet::with_hasher_proj_in(
        entries.hasher().clone(),
        entries.allocator().clone(),
    );

    for value in entries
        .as_slice()
        .iter()
        .cloned()
    {
        set.insert_full(value);
    }

    set
}

fn prop_insert_full_as_slice<T, S, A>(entries: TypedProjIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<T, S, A>(set: &TypedProjIndexSet<T, S, A>) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let expected: Vec<T> = set.iter().cloned().collect();

        expected
    }

    fn result<T, S, A>(set: &TypedProjIndexSet<T, S, A>) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let result: Vec<T> = set.as_slice().iter().cloned().collect();

        result
    }

    let set = from_entries_insert_full_in(&entries);
    let expected = expected(&entries);
    let result = result::<T, S, A>(&set);

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_insert_full_contains<T, S, A>(entries: TypedProjIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = TypedProjIndexSet::with_hasher_proj_in(
        entries.hasher().clone(),
        entries.allocator().clone(),
    );

    for value in entries.iter() {
        prop_assert!(!set.contains(value));
    }

    for value in entries.iter().cloned() {
        set.insert_full(value);
    }

    for value in entries.iter() {
        prop_assert!(set.contains(value));
    }

    Ok(())
}

fn prop_insert_full_get<T, S, A>(entries: TypedProjIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_entries_insert_full_in(&entries);
    for value in entries.iter() {
        let expected = Some(value);
        let result = set.get(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_get_full<T, S, A>(entries: TypedProjIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_entries_insert_full_in(&entries);
    for (index, value) in entries.iter().enumerate() {
        let expected = Some((index, value));
        let result = set.get_full(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_get_index_of<T, S, A>(entries: TypedProjIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_entries_insert_full_in(&entries);
    for (index, value) in entries.iter().enumerate() {
        let expected = Some(index);
        let result = set.get_index_of(value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_iter<T, S, A>(entries: TypedProjIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = from_entries_insert_full_in(&entries);
    for (result, expected) in set.iter().zip(entries.iter()) {
        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_insert_full_len<T, S, A>(entries: TypedProjIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<T, S, A>(set: &TypedProjIndexSet<T, S, A>) -> usize
    where
        T: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let unique_values = {
            let mut values = Vec::from_iter(set.iter().cloned());
            values.sort();
            values.dedup();
            values
        };

        unique_values.len()
    }

    let set = from_entries_insert_full_in(&entries);
    let expected = expected(&entries);
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
                fn prop_insert_full_as_slice(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_as_slice(entries)?
                }

                #[test]
                fn prop_insert_full_contains(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_contains(entries)?
                }

                #[test]
                fn prop_insert_full_get(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_get(entries)?
                }

                #[test]
                fn prop_insert_full_get_full(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_get_full(entries)?
                }

                #[test]
                fn prop_insert_full_get_index_of(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_get_index_of(entries)?
                }

                #[test]
                fn prop_insert_full_iter(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_iter(entries)?
                }

                #[test]
                fn prop_insert_full_len(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_full_len(entries)?
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
    strategy_type_projected_index_set_max_len,
);
generate_props!(
    usize,
    usize,
    hash::RandomState,
    alloc::Global,
    128,
    strategy_type_projected_index_set_max_len,
);
generate_props!(
    string,
    String,
    hash::RandomState,
    alloc::Global,
    128,
    strategy_type_projected_index_set_max_len,
);
