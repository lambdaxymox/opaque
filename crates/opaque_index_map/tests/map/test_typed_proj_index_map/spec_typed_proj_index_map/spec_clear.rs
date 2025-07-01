use crate::map::common::projected::strategy_type_projected_index_map_max_len;
use opaque_index_map::TypedProjIndexMap;

use core::any;
use core::fmt;
use std::alloc;
use std::hash;

use proptest::prelude::*;

fn prop_clear_as_slice<K, V, S, A>(entries: TypedProjIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let expected = TypedProjIndexMap::with_hasher_proj_in(
        entries.hasher().clone(),
        entries.allocator().clone(),
    );
    let result = {
        let mut map = entries.clone();
        map.clear();
        map
    };

    prop_assert_eq!(result.as_slice(), expected.as_slice());

    Ok(())
}

fn prop_clear_is_empty<K, V, S, A>(entries: TypedProjIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut result = entries.clone();
    result.clear();

    prop_assert!(result.is_empty());

    Ok(())
}

fn prop_clear_len<K, V, S, A>(entries: TypedProjIndexMap<K, V, S, A>) -> Result<(), TestCaseError>
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug + Ord,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut map = entries.clone();
    map.clear();

    let expected = 0;
    let result = map.len();

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
                fn prop_clear_as_slice(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_clear_as_slice(entries)?
                }

                #[test]
                fn prop_clear_is_empty(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_clear_is_empty(entries)?
                }

                #[test]
                fn prop_clear_len(entries in super::$map_gen::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>($max_length)) {
                    let entries: super::TypedProjIndexMap<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ> = entries;
                    super::prop_clear_len(entries)?
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
