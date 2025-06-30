use crate::set::common::erased::strategy_type_erased_index_set_max_len;
use opaque_index_map::OpaqueIndexSet;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use proptest::prelude::*;

fn prop_clone<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    let cloned_set = set.clone::<T, S, A>();

    let expected = set.as_slice::<T, S, A>();
    let result = cloned_set.as_slice::<T, S, A>();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_clone_len<T, S, A>(entries: OpaqueIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let set = entries.clone::<T, S, A>();
    let cloned_set = set.clone::<T, S, A>();

    let expected = set.len();
    let result = cloned_set.len();

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
                fn prop_clone(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_clone::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_clone_len(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::OpaqueIndexSet = entries;
                    super::prop_clone_len::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
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
