use crate::map::common::projected::strategy_type_projected_index_map_max_len;
use opaque_index_map::TypeProjectedIndexMap;
use opaque_vec::TypeProjectedVec;

use core::any;
use core::fmt;
use std::format;
use std::hash;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn from_keys_in<K, V, S, A>(map: &TypeProjectedIndexMap<K, V, S, A>) -> TypeProjectedVec<K, A>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut keys = TypeProjectedVec::new_proj_in(map.allocator().clone());
    for key in map.keys().cloned() {
        keys.push(key);
    }

    keys
}

fn prop_insert_get_full_mut_equiv_get_index_of_get_key_value_mut<K, V, S, A>(
    entries: TypeProjectedIndexMap<K, V, S, A>,
) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    let keys = from_keys_in(&map);
    for key in keys.iter() {
        let expected = {
            let maybe_index = map.get_index_of(key);

            prop_assert!(maybe_index.is_some());

            let maybe_key_value = map.get_key_value_mut(key).map(|(k, v)| (k.clone(), v.clone()));

            prop_assert!(maybe_key_value.is_some());

            let index = maybe_index.unwrap();
            let key_value = maybe_key_value.unwrap();

            Some((index, key_value.0, key_value.1))
        };
        let result = map.get_full_mut(key).map(|(i, k, v)| (i, k.clone(), v.clone()));

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
                fn prop_insert_get_full_mut_equiv_get_index_of_get_key_value_mut(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypeProjectedIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_insert_get_full_mut_equiv_get_index_of_get_key_value_mut(entries)?
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
    strategy_type_projected_index_map_max_len,
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_map_max_len,
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_map_max_len,
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    32,
    strategy_type_projected_index_map_max_len,
);
