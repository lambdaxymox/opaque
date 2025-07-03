use crate::set::common::projected::strategy_type_projected_index_set_max_len;
use opaque_index_map::TypedProjIndexSet;

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

fn prop_clear_as_slice<T, S, A>(entries: TypedProjIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let expected = TypedProjIndexSet::new();
    let result = {
        let mut set = entries.clone();
        set.clear();
        set
    };

    prop_assert_eq!(result.as_slice(), expected.as_slice());

    Ok(())
}

fn prop_clear_is_empty<T, S, A>(entries: TypedProjIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    set.clear();

    prop_assert!(set.is_empty());

    Ok(())
}

fn prop_clear_len<T, S, A>(entries: TypedProjIndexSet<T, S, A>) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut set = entries.clone();
    set.clear();

    prop_assert_eq!(set.len(), 0);

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
                fn prop_clear_as_slice(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_clear_as_slice(entries)?
                }

                #[test]
                fn prop_clear_is_empty(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_clear_is_empty(entries)?
                }

                #[test]
                fn prop_clear_len(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexSet<$value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_clear_len(entries)?
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
