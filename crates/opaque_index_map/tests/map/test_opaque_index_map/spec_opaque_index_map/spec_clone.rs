use crate::map::common::erased::strategy_type_erased_index_map_max_len;
use opaque_index_map::OpaqueIndexMap;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use proptest::prelude::*;

fn prop_clone<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let cloned_map = map.clone::<K, V, S, A>();

    let expected = map.as_slice::<K, V, S, A>();
    let result = cloned_map.as_slice::<K, V, S, A>();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_clone_len<K, V, S, A>(entries: OpaqueIndexMap) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let map = entries.clone::<K, V, S, A>();
    let cloned_map = map.clone::<K, V, S, A>();

    let expected = map.len();
    let result = cloned_map.len();

    prop_assert_eq!(result, expected);

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
            use proptest::prelude::*;
            use std::hash;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_clone(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    super::prop_clone::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
                }

                #[test]
                fn prop_clone_len(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    super::prop_clone_len::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(entries)?
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
    128,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    usize_i64,
    usize,
    i64,
    hash::RandomState,
    alloc::Global,
    128,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    string_i64,
    String,
    i64,
    hash::RandomState,
    alloc::Global,
    128,
    strategy_type_erased_index_map_max_len,
);
generate_props!(
    string_string,
    String,
    String,
    hash::RandomState,
    alloc::Global,
    128,
    strategy_type_erased_index_map_max_len,
);
