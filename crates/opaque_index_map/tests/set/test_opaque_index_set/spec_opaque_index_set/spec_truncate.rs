use crate::set::common::erased::strategy_type_erased_index_set_max_len;
use opaque_index_map::OpaqueIndexSet;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use proptest::prelude::*;

fn prop_truncate_len_length_less_than_or_equal_to<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<T, S, A>(set: &OpaqueIndexSet, len: usize) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let vec: Vec<T> = set
            .iter::<T, S, A>()
            .cloned()
            .take(len)
            .collect();

        vec
    }

    fn result<T, S, A>(set: &OpaqueIndexSet, len: usize) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut cloned_set = set.clone::<T, S, A>();
        cloned_set.truncate::<T, S, A>(len);

        let vec: Vec<T> = cloned_set
            .iter::<T, S, A>()
            .cloned()
            .collect();

        vec
    }
    
    for len in 0..entries.len() {
        let set = entries.clone::<T, S, A>();
        let expected_entries = expected::<T, S, A>(&entries, len);
        let result_entries = result::<T, S, A>(&set, len);
        let expected = expected_entries.len();
        let result = result_entries.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_truncate_length_less_than_or_equal_to<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn expected<T, S, A>(set: &OpaqueIndexSet, len: usize) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let vec: Vec<T> = set
            .iter::<T, S, A>()
            .cloned()
            .take(len)
            .collect();

        vec
    }

    fn result<T, S, A>(set: &OpaqueIndexSet, len: usize) -> Vec<T>
    where
        T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
        S: any::Any + hash::BuildHasher + Send + Sync + Clone,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut cloned_set = set.clone::<T, S, A>();
        cloned_set.truncate::<T, S, A>(len);

        let vec: Vec<T> = cloned_set
            .iter::<T, S, A>()
            .cloned()
            .collect();

        vec
    }
    
    for len in 0..entries.len() {
        let set = entries.clone::<T, S, A>();
        let expected = expected::<T, S, A>(&entries, len);
        let result = result::<T, S, A>(&set, len);

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
            use proptest::prelude::*;
            use std::hash;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_truncate_len_length_less_than_or_equal_to(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_truncate_len_length_less_than_or_equal_to::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_truncate_length_less_than_or_equal_to(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_truncate_length_less_than_or_equal_to::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
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
