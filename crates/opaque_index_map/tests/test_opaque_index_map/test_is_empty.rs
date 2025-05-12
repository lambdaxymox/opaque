use opaque_index_map::OpaqueIndexMap;

use core::any;
use core::fmt;
use std::alloc;
use std::iter;
use std::hash;

fn run_test_opaque_index_map_empty_len<K, V, S, A>(build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    let opaque_map = OpaqueIndexMap::with_hasher_in::<K, V, S, A>(build_hasher, alloc);
    let expected = 0;
    let result = opaque_map.len::<K, V, S, A>();

    assert_eq!(result, expected);
}

fn run_test_opaque_index_map_empty_is_empty<K, V, S, A>(build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    let opaque_map = OpaqueIndexMap::with_hasher_in::<K, V, S, A>(build_hasher, alloc);

    assert!(opaque_map.is_empty::<K, V, S, A>());
}

fn run_test_opaque_index_map_empty_contains_no_values<K, V, S, A>(build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug + TryFrom<usize> + iter::Step,
    <K as TryFrom<usize>>::Error: fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    let opaque_map = OpaqueIndexMap::with_hasher_in::<K, V, S, A>(build_hasher, alloc);
    let min_key = TryFrom::try_from(0).unwrap();
    let max_key = TryFrom::try_from(127).unwrap();
    for key in min_key..max_key {
        assert!(!opaque_map.contains_key::<K, K, V, S, A>(&key));
    }
}

fn run_test_opaque_index_map_empty_get<K, V, S, A>(build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug + TryFrom<usize> + iter::Step,
    <K as TryFrom<usize>>::Error: fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    let opaque_map = OpaqueIndexMap::new::<K, V>();
    let min_key = TryFrom::try_from(0).unwrap();
    let max_key = TryFrom::try_from(127).unwrap();
    for key in min_key..max_key {
        let result = opaque_map.get::<K, K, V, S, A>(&key);

        assert!(result.is_none());
    }
}

macro_rules! generate_tests {
    ($module_name:ident, key_type = $key_typ:ty, value_type = $value_typ:ty) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_map_empty_len() {
                let build_hasher = hash::RandomState::default();
                let alloc = alloc::Global;
                run_test_opaque_index_map_empty_len::<$key_typ, $value_typ, hash::RandomState, alloc::Global>(build_hasher, alloc)
            }

            #[test]
            fn test_opaque_index_map_empty_is_empty() {
                let build_hasher = hash::RandomState::default();
                let alloc = alloc::Global;
                run_test_opaque_index_map_empty_is_empty::<$key_typ, $value_typ, hash::RandomState, alloc::Global>(build_hasher, alloc)
            }

            #[test]
            fn test_opaque_index_map_empty_contains_no_values() {
                let build_hasher = hash::RandomState::default();
                let alloc = alloc::Global;
                run_test_opaque_index_map_empty_contains_no_values::<$key_typ, $value_typ, hash::RandomState, alloc::Global>(build_hasher, alloc)
            }

            #[test]
            fn test_opaque_index_map_empty_get() {
                let build_hasher = hash::RandomState::default();
                let alloc = alloc::Global;
                run_test_opaque_index_map_empty_get::<$key_typ, $value_typ, hash::RandomState, alloc::Global>(build_hasher, alloc)
            }
        }
    };
}

generate_tests!(
    u64_i64,
    key_type = u64,
    value_type = i64
);
generate_tests!(
    usize_i64,
    key_type = usize,
    value_type = i64
);

