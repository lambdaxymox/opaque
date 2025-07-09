use crate::map::common::erased::strategy_type_erased_index_map_max_len;
use opaque_index_map::TypeErasedIndexMap;

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

fn prop_insert_get_full_equiv_get_index_of_get_key_value<K, V, S, A>(entries: TypeErasedIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    for key in map.keys::<K, V, S, A>() {
        let expected = {
            let maybe_index = map.get_index_of::<_, K, V, S, A>(key);

            prop_assert!(maybe_index.is_some());

            let maybe_key_value = map.get_key_value::<_, K, V, S, A>(key);

            prop_assert!(maybe_key_value.is_some());

            let index = maybe_index.unwrap();
            let key_value = maybe_key_value.unwrap();

            Some((index, key_value.0, key_value.1))
        };
        let result = map.get_full::<_, K, V, S, A>(key);

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
        $map_gen:ident,
    ) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_insert_get_full_equiv_get_index_of_get_key_value(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeErasedIndexMap = entries;
                    super::prop_insert_get_full_equiv_get_index_of_get_key_value::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
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
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_erased_index_map_max_len,
);
