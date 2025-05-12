use opaque_index_map::TypedProjIndexMap;

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
    let proj_map = TypedProjIndexMap::<K, V, S, A>::with_hasher_in(build_hasher, alloc);
    let expected = 0;
    let result = proj_map.len();

    assert_eq!(result, expected);
}

fn run_test_opaque_index_map_empty_is_empty<K, V, S, A>(build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    let proj_map = TypedProjIndexMap::<K, V, S, A>::with_hasher_in(build_hasher, alloc);

    assert!(proj_map.is_empty());
}

fn run_test_opaque_index_map_empty_contains_no_values<K, V, S, A>(build_hasher: S, alloc: A)
where
    K: any::Any + Clone + Eq + Ord + hash::Hash + fmt::Debug + TryFrom<usize> + iter::Step,
    <K as TryFrom<usize>>::Error: fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    S: any::Any + hash::BuildHasher + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    let proj_map = TypedProjIndexMap::<K, V, S, A>::with_hasher_in(build_hasher, alloc);
    let min_key: K = TryFrom::try_from(0).unwrap();
    let max_key: K = TryFrom::try_from(127).unwrap();
    for key in min_key..max_key {
        assert!(!proj_map.contains_key(&key));
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
    let proj_map = TypedProjIndexMap::<K, V, S, A>::with_hasher_in(build_hasher, alloc);
    let min_key: K = TryFrom::try_from(0).unwrap();
    let max_key: K = TryFrom::try_from(127).unwrap();
    for key in min_key..max_key {
        let result = proj_map.get(&key);

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
    u16_i8,
    key_type = u16,
    value_type = i8
);
generate_tests!(
    u16_i16,
    key_type = u16,
    value_type = i16
);
generate_tests!(
    u16_i32,
    key_type = u16,
    value_type = i32
);
generate_tests!(
    u16_i64,
    key_type = u16,
    value_type = i64
);
generate_tests!(
    u16_i128,
    key_type = u16,
    value_type = i128
);
generate_tests!(
    u16_isize,
    key_type = u16,
    value_type = isize
);

generate_tests!(
    u32_i8,
    key_type = u32,
    value_type = i8
);
generate_tests!(
    u32_i16,
    key_type = u32,
    value_type = i16
);
generate_tests!(
    u32_i32,
    key_type = u32,
    value_type = i32
);
generate_tests!(
    u32_i64,
    key_type = u32,
    value_type = i64
);
generate_tests!(
    u32_i128,
    key_type = u32,
    value_type = i128
);
generate_tests!(
    u32_isize,
    key_type = u32,
    value_type = isize
);

generate_tests!(
    u64_i8,
    key_type = u64,
    value_type = i8
);
generate_tests!(
    u64_i16,
    key_type = u64,
    value_type = i16
);
generate_tests!(
    u64_i32,
    key_type = u64,
    value_type = i32
);
generate_tests!(
    u64_i64,
    key_type = u64,
    value_type = i64
);
generate_tests!(
    u64_i128,
    key_type = u64,
    value_type = i128
);
generate_tests!(
    u64_isize,
    key_type = u64,
    value_type = isize
);

generate_tests!(
    u128_i8,
    key_type = u128,
    value_type = i8
);
generate_tests!(
    u128_i16,
    key_type = u128,
    value_type = i16
);
generate_tests!(
    u128_i32,
    key_type = u128,
    value_type = i32
);
generate_tests!(
    u128_i64,
    key_type = u128,
    value_type = i64
);
generate_tests!(
    u128_i128,
    key_type = u128,
    value_type = i128
);
generate_tests!(
    u128_isize,
    key_type = u128,
    value_type = isize
);

generate_tests!(
    usize_i8,
    key_type = usize,
    value_type = i8
);
generate_tests!(
    usize_i16,
    key_type = usize,
    value_type = i16
);
generate_tests!(
    usize_i32,
    key_type = usize,
    value_type = i32
);
generate_tests!(
    usize_i64,
    key_type = usize,
    value_type = i64
);
generate_tests!(
    usize_i128,
    key_type = usize,
    value_type = i128
);
generate_tests!(
    usize_isize,
    key_type = usize,
    value_type = isize
);

generate_tests!(
    i16_i8,
    key_type = i16,
    value_type = i8
);
generate_tests!(
    i16_i16,
    key_type = i16,
    value_type = i16
);
generate_tests!(
    i16_i32,
    key_type = i16,
    value_type = i32
);
generate_tests!(
    i16_i64,
    key_type = i16,
    value_type = i64
);
generate_tests!(
    i16_i128,
    key_type = i16,
    value_type = i128
);
generate_tests!(
    i16_isize,
    key_type = i16,
    value_type = isize
);

generate_tests!(
    i32_i8,
    key_type = i32,
    value_type = i8
);
generate_tests!(
    i32_i16,
    key_type = i32,
    value_type = i16
);
generate_tests!(
    i32_i32,
    key_type = i32,
    value_type = i32
);
generate_tests!(
    i32_i64,
    key_type = i32,
    value_type = i64
);
generate_tests!(
    i32_i128,
    key_type = i32,
    value_type = i128
);
generate_tests!(
    i32_isize,
    key_type = i32,
    value_type = isize
);

generate_tests!(
    i64_i8,
    key_type = i64,
    value_type = i8
);
generate_tests!(
    i64_i16,
    key_type = i64,
    value_type = i16
);
generate_tests!(
    i64_i32,
    key_type = i64,
    value_type = i32
);
generate_tests!(
    i64_i64,
    key_type = i64,
    value_type = i64
);
generate_tests!(
    i64_i128,
    key_type = i64,
    value_type = i128
);
generate_tests!(
    i64_isize,
    key_type = i64,
    value_type = isize
);

generate_tests!(
    i128_i8,
    key_type = i128,
    value_type = i8
);
generate_tests!(
    i128_i16,
    key_type = i128,
    value_type = i16
);
generate_tests!(
    i128_i32,
    key_type = i128,
    value_type = i32
);
generate_tests!(
    i128_i64,
    key_type = i128,
    value_type = i64
);
generate_tests!(
    i128_i128,
    key_type = i128,
    value_type = i128
);
generate_tests!(
    i128_isize,
    key_type = i128,
    value_type = isize
);

generate_tests!(
    isize_i8,
    key_type = isize,
    value_type = i8
);
generate_tests!(
    isize_i16,
    key_type = isize,
    value_type = i16
);
generate_tests!(
    isize_i32,
    key_type = isize,
    value_type = i32
);
generate_tests!(
    isize_i64,
    key_type = isize,
    value_type = i64
);
generate_tests!(
    isize_i128,
    key_type = isize,
    value_type = i128
);
generate_tests!(
    isize_isize,
    key_type = isize,
    value_type = isize
);
