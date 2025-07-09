use crate::set::common::erased::strategy_type_erased_index_set_max_len;
use opaque_index_map::TypeErasedIndexSet;

use core::any;
use core::fmt;
use std::hash;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn prop_insert_get_full_equiv_get_index_of_get<T, S, A>(entries: TypeErasedIndexSet) -> Result<(), TestCaseError>
where
    T: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<T, S, A>();
    for ref_value in map.iter::<T, S, A>() {
        let expected = {
            let maybe_index = map.get_index_of::<_, T, S, A>(ref_value);

            prop_assert!(maybe_index.is_some());

            let maybe_value = map.get::<_, T, S, A>(ref_value);

            prop_assert!(maybe_value.is_some());

            let index = maybe_index.unwrap();
            let value = maybe_value.unwrap();

            Some((index, value))
        };
        let result = map.get_full::<_, T, S, A>(ref_value);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

macro_rules! generate_props {
    (
        $module_name:ident,
        $key_typ:ty,
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
                fn prop_insert_get_full_equiv_get_index_of_get(entries in super::$set_gen::<$value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexSet = entries;
                    super::prop_insert_get_full_equiv_get_index_of_get::<$value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }
            }
        }
    };
}

generate_props!(
    u64_i64,
    u64,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_set_max_len,
);
