use crate::set::common::projected::strategy_type_projected_index_set_max_len;
use opaque_index_map::TypeProjectedIndexSet;

use core::any;
use core::fmt;
use std::format;
use std::hash;
use std::string::String;
use std::vec::Vec;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn prop_truncate_len_length_less_than_or_equal_to<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<T, S, A>(set: &TypeProjectedIndexSet<T, S, A>, len: usize) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let vec: Vec<T> = set.iter().cloned().take(len).collect();

        vec
    }

    fn result<T, S, A>(set: &TypeProjectedIndexSet<T, S, A>, len: usize) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut cloned_set = set.clone();
        cloned_set.truncate(len);

        let vec: Vec<T> = cloned_set.iter().cloned().collect();

        vec
    }

    for len in 0..entries.len() {
        let set = entries.clone();
        let expected_entries = expected(&entries, len);
        let result_entries = result(&set, len);
        let expected = expected_entries.len();
        let result = result_entries.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_truncate_length_less_than_or_equal_to<T, S, A>(entries: TypeProjectedIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<T, S, A>(set: &TypeProjectedIndexSet<T, S, A>, len: usize) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let vec: Vec<T> = set.iter().cloned().take(len).collect();

        vec
    }

    fn result<T, S, A>(set: &TypeProjectedIndexSet<T, S, A>, len: usize) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut cloned_set = set.clone();
        cloned_set.truncate(len);

        let vec: Vec<T> = cloned_set.iter().cloned().collect();

        vec
    }

    for len in 0..entries.len() {
        let set = entries.clone();
        let expected = expected(&entries, len);
        let result = result(&set, len);

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
                fn prop_truncate_len_length_less_than_or_equal_to(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_truncate_len_length_less_than_or_equal_to(entries)?
                }

                #[test]
                fn prop_truncate_length_less_than_or_equal_to(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_truncate_length_less_than_or_equal_to(entries)?
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
