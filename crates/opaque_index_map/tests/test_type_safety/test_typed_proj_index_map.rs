use opaque_index_map::{OpaqueIndexMap, TypedProjIndexMap};

use std::{alloc, any, hash};
use std::hash::RandomState;
use std::alloc::{Global, System};


fn run_test_opaque_index_map_with_hasher_in_has_type<K, V, S, A>(build_hasher: S, alloc: A)
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send  + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_map = TypedProjIndexMap::<K, V, S, A>::with_hasher_in(build_hasher, alloc);
    let opaque_map = OpaqueIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<S>());
    assert!(opaque_map.has_allocator_type::<A>());
}

fn run_test_opaque_index_map_with_capacity_and_hasher_in_has_type<K, V, S, A>(build_hasher: S, alloc: A)
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send  + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_map = TypedProjIndexMap::<K, V, S, A>::with_capacity_and_hasher_in(1024, build_hasher, alloc);
    let opaque_map = OpaqueIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<S>());
    assert!(opaque_map.has_allocator_type::<A>());
}

fn run_test_opaque_index_map_new_in_has_type<K, V, A>(alloc: A)
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_map = TypedProjIndexMap::<K, V, _, A>::new_in(alloc);
    let opaque_map = OpaqueIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<hash::RandomState>());
    assert!(opaque_map.has_allocator_type::<A>());
}

fn run_test_opaque_index_map_with_capacity_in_has_type<K, V, A>(alloc: A)
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_map = TypedProjIndexMap::<K, V, _, A>::with_capacity_in(1024, alloc);
    let opaque_map = OpaqueIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<hash::RandomState>());
    assert!(opaque_map.has_allocator_type::<A>());
}

fn run_test_opaque_index_map_with_hasher_has_type<K, V, S>(build_hasher: S)
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send  + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    let proj_map = TypedProjIndexMap::<K, V, S, _>::with_hasher(build_hasher);
    let opaque_map = OpaqueIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<S>());
    assert!(opaque_map.has_allocator_type::<alloc::Global>());
}

fn run_test_opaque_index_map_with_capacity_and_hasher_has_type<K, V, S>(build_hasher: S)
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send  + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    let proj_map = TypedProjIndexMap::<K, V, S, _>::with_capacity_and_hasher(1024, build_hasher);
    let opaque_map = OpaqueIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<S>());
    assert!(opaque_map.has_allocator_type::<alloc::Global>());
}

fn run_test_opaque_index_map_new_has_type<K, V>()
where
    K: any::Any,
    V: any::Any,
{
    let proj_map = TypedProjIndexMap::<K, V, _, _>::new();
    let opaque_map = OpaqueIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<hash::RandomState>());
    assert!(opaque_map.has_allocator_type::<Global>());
}

fn run_test_opaque_index_map_with_capacity_has_type<K, V>()
where
    K: any::Any,
    V: any::Any,
{
    let proj_map = TypedProjIndexMap::<K, V, _, _>::with_capacity(1024);
    let opaque_map = OpaqueIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<hash::RandomState>());
    assert!(opaque_map.has_allocator_type::<Global>());
}

macro_rules! generate_tests {
    ($module_name:ident, $key_typ:ty, $value_typ:ty, $build_hasher_typ:ty, $alloc_typ:ty) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_index_map_with_hasher_in_has_type() {
                let build_hasher: $build_hasher_typ = Default::default();
                let alloc: $alloc_typ = Default::default();
                run_test_opaque_index_map_with_hasher_in_has_type::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(build_hasher, alloc);
            }

            #[test]
            fn test_opaque_index_map_with_capacity_and_hasher_in_has_type() {
                let build_hasher: $build_hasher_typ = Default::default();
                let alloc: $alloc_typ = Default::default();
                run_test_opaque_index_map_with_capacity_and_hasher_in_has_type::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(build_hasher, alloc);
            }

            #[test]
            fn test_opaque_index_map_new_in_has_type() {
                let alloc: $alloc_typ = Default::default();
                run_test_opaque_index_map_new_in_has_type::<$key_typ, $value_typ, $alloc_typ>(alloc);
            }

            #[test]
            fn test_opaque_index_map_with_capacity_in_has_type() {
                let alloc: $alloc_typ = Default::default();
                run_test_opaque_index_map_with_capacity_in_has_type::<$key_typ, $value_typ, $alloc_typ>(alloc);
            }

            #[test]
            fn test_opaque_index_map_with_hasher_has_type() {
                let build_hasher: $build_hasher_typ = Default::default();
                run_test_opaque_index_map_with_hasher_has_type::<$key_typ, $value_typ, $build_hasher_typ>(build_hasher);
            }

            #[test]
            fn test_opaque_index_map_with_capacity_and_hasher_has_type() {
                let build_hasher: $build_hasher_typ = Default::default();
                run_test_opaque_index_map_with_capacity_and_hasher_has_type::<$key_typ, $value_typ, $build_hasher_typ>(build_hasher);
            }
            #[test]
            fn test_opaque_index_map_new_has_type() {
                run_test_opaque_index_map_new_has_type::<$key_typ, $value_typ>();
            }

            #[test]
            fn test_opaque_index_map_with_capacity_has_type() {
                run_test_opaque_index_map_with_capacity_has_type::<$key_typ, $value_typ>();
            }
        }
    };
}

generate_tests!(i8_i8_random_state_global, i8, i8, RandomState, Global);
generate_tests!(i8_i8_random_state_system, i8, i8, RandomState, System);
generate_tests!(i8_i16_random_state_global, i8, i16, RandomState, Global);
generate_tests!(i8_i16_random_state_system, i8, i16, RandomState, System);
generate_tests!(i8_i32_random_state_global, i8, i32, RandomState, Global);
generate_tests!(i8_i32_random_state_system, i8, i32, RandomState, System);
generate_tests!(i8_i64_random_state_global, i8, i64, RandomState, Global);
generate_tests!(i8_i64_random_state_system, i8, i64, RandomState, System);
generate_tests!(i8_i128_random_state_global, i8, i128, RandomState, Global);
generate_tests!(i8_i128_random_state_system, i8, i128, RandomState, System);
generate_tests!(i8_isize_random_state_global, i8, isize, RandomState, Global);
generate_tests!(i8_isize_random_state_system, i8, isize, RandomState, System);
generate_tests!(i8_u8_random_state_global, i8, u8, RandomState, Global);
generate_tests!(i8_u8_random_state_system, i8, u8, RandomState, System);
generate_tests!(i8_u16_random_state_global, i8, u16, RandomState, Global);
generate_tests!(i8_u16_random_state_system, i8, u16, RandomState, System);
generate_tests!(i8_u32_random_state_global, i8, u32, RandomState, Global);
generate_tests!(i8_u32_random_state_system, i8, u32, RandomState, System);
generate_tests!(i8_u64_random_state_global, i8, u64, RandomState, Global);
generate_tests!(i8_u64_random_state_system, i8, u64, RandomState, System);
generate_tests!(i8_u128_random_state_global, i8, u128, RandomState, Global);
generate_tests!(i8_u128_random_state_system, i8, u128, RandomState, System);
generate_tests!(i8_usize_random_state_global, i8, usize, RandomState, Global);
generate_tests!(i8_usize_random_state_system, i8, usize, RandomState, System);
generate_tests!(i8_f32_random_state_global, i8, f32, RandomState, Global);
generate_tests!(i8_f32_random_state_system, i8, f32, RandomState, System);
generate_tests!(i8_f64_random_state_global, i8, f64, RandomState, Global);
generate_tests!(i8_f64_random_state_system, i8, f64, RandomState, System);
generate_tests!(i8_bool_random_state_global, i8, bool, RandomState, Global);
generate_tests!(i8_bool_random_state_system, i8, bool, RandomState, System);
generate_tests!(i8_char_random_state_global, i8, char, RandomState, Global);
generate_tests!(i8_char_random_state_system, i8, char, RandomState, System);
generate_tests!(i8_string_random_state_global, i8, String, RandomState, Global);
generate_tests!(i8_string_random_state_system, i8, String, RandomState, System);
generate_tests!(i8_box_any_random_state_global, i8, Box<dyn any::Any>, RandomState, Global);
generate_tests!(i8_box_any_random_state_system, i8, Box<dyn any::Any>, RandomState, System);

generate_tests!(i16_i8_random_state_global, i16, i8, RandomState, Global);
generate_tests!(i16_i8_random_state_system, i16, i8, RandomState, System);
generate_tests!(i16_i16_random_state_global, i16, i16, RandomState, Global);
generate_tests!(i16_i16_random_state_system, i16, i16, RandomState, System);
generate_tests!(i16_i32_random_state_global, i16, i32, RandomState, Global);
generate_tests!(i16_i32_random_state_system, i16, i32, RandomState, System);
generate_tests!(i16_i64_random_state_global, i16, i64, RandomState, Global);
generate_tests!(i16_i64_random_state_system, i16, i64, RandomState, System);
generate_tests!(i16_i128_random_state_global, i16, i128, RandomState, Global);
generate_tests!(i16_i128_random_state_system, i16, i128, RandomState, System);
generate_tests!(i16_isize_random_state_global, i16, isize, RandomState, Global);
generate_tests!(i16_isize_random_state_system, i16, isize, RandomState, System);
generate_tests!(i16_u8_random_state_global, i16, u8, RandomState, Global);
generate_tests!(i16_u8_random_state_system, i16, u8, RandomState, System);
generate_tests!(i16_u16_random_state_global, i16, u16, RandomState, Global);
generate_tests!(i16_u16_random_state_system, i16, u16, RandomState, System);
generate_tests!(i16_u32_random_state_global, i16, u32, RandomState, Global);
generate_tests!(i16_u32_random_state_system, i16, u32, RandomState, System);
generate_tests!(i16_u64_random_state_global, i16, u64, RandomState, Global);
generate_tests!(i16_u64_random_state_system, i16, u64, RandomState, System);
generate_tests!(i16_u128_random_state_global, i16, u128, RandomState, Global);
generate_tests!(i16_u128_random_state_system, i16, u128, RandomState, System);
generate_tests!(i16_usize_random_state_global, i16, usize, RandomState, Global);
generate_tests!(i16_usize_random_state_system, i16, usize, RandomState, System);
generate_tests!(i16_f32_random_state_global, i16, f32, RandomState, Global);
generate_tests!(i16_f32_random_state_system, i16, f32, RandomState, System);
generate_tests!(i16_f64_random_state_global, i16, f64, RandomState, Global);
generate_tests!(i16_f64_random_state_system, i16, f64, RandomState, System);
generate_tests!(i16_bool_random_state_global, i16, bool, RandomState, Global);
generate_tests!(i16_bool_random_state_system, i16, bool, RandomState, System);
generate_tests!(i16_char_random_state_global, i16, char, RandomState, Global);
generate_tests!(i16_char_random_state_system, i16, char, RandomState, System);
generate_tests!(i16_string_random_state_global, i16, String, RandomState, Global);
generate_tests!(i16_string_random_state_system, i16, String, RandomState, System);
generate_tests!(i16_box_any_random_state_global, i16, Box<dyn any::Any>, RandomState, Global);
generate_tests!(i16_box_any_random_state_system, i16, Box<dyn any::Any>, RandomState, System);

generate_tests!(i32_i8_random_state_global, i32, i8, RandomState, Global);
generate_tests!(i32_i8_random_state_system, i32, i8, RandomState, System);
generate_tests!(i32_i16_random_state_global, i32, i16, RandomState, Global);
generate_tests!(i32_i16_random_state_system, i32, i16, RandomState, System);
generate_tests!(i32_i32_random_state_global, i32, i32, RandomState, Global);
generate_tests!(i32_i32_random_state_system, i32, i32, RandomState, System);
generate_tests!(i32_i64_random_state_global, i32, i64, RandomState, Global);
generate_tests!(i32_i64_random_state_system, i32, i64, RandomState, System);
generate_tests!(i32_i128_random_state_global, i32, i128, RandomState, Global);
generate_tests!(i32_i128_random_state_system, i32, i128, RandomState, System);
generate_tests!(i32_isize_random_state_global, i32, isize, RandomState, Global);
generate_tests!(i32_isize_random_state_system, i32, isize, RandomState, System);
generate_tests!(i32_u8_random_state_global, i32, u8, RandomState, Global);
generate_tests!(i32_u8_random_state_system, i32, u8, RandomState, System);
generate_tests!(i32_u16_random_state_global, i32, u16, RandomState, Global);
generate_tests!(i32_u16_random_state_system, i32, u16, RandomState, System);
generate_tests!(i32_u32_random_state_global, i32, u32, RandomState, Global);
generate_tests!(i32_u32_random_state_system, i32, u32, RandomState, System);
generate_tests!(i32_u64_random_state_global, i32, u64, RandomState, Global);
generate_tests!(i32_u64_random_state_system, i32, u64, RandomState, System);
generate_tests!(i32_u128_random_state_global, i32, u128, RandomState, Global);
generate_tests!(i32_u128_random_state_system, i32, u128, RandomState, System);
generate_tests!(i32_usize_random_state_global, i32, usize, RandomState, Global);
generate_tests!(i32_usize_random_state_system, i32, usize, RandomState, System);
generate_tests!(i32_f32_random_state_global, i32, f32, RandomState, Global);
generate_tests!(i32_f32_random_state_system, i32, f32, RandomState, System);
generate_tests!(i32_f64_random_state_global, i32, f64, RandomState, Global);
generate_tests!(i32_f64_random_state_system, i32, f64, RandomState, System);
generate_tests!(i32_bool_random_state_global, i32, bool, RandomState, Global);
generate_tests!(i32_bool_random_state_system, i32, bool, RandomState, System);
generate_tests!(i32_char_random_state_global, i32, char, RandomState, Global);
generate_tests!(i32_char_random_state_system, i32, char, RandomState, System);
generate_tests!(i32_string_random_state_global, i32, String, RandomState, Global);
generate_tests!(i32_string_random_state_system, i32, String, RandomState, System);
generate_tests!(i32_box_any_random_state_global, i32, Box<dyn any::Any>, RandomState, Global);
generate_tests!(i32_box_any_random_state_system, i32, Box<dyn any::Any>, RandomState, System);

generate_tests!(i64_i8_random_state_global, i64, i8, RandomState, Global);
generate_tests!(i64_i8_random_state_system, i64, i8, RandomState, System);
generate_tests!(i64_i16_random_state_global, i64, i16, RandomState, Global);
generate_tests!(i64_i16_random_state_system, i64, i16, RandomState, System);
generate_tests!(i64_i32_random_state_global, i64, i32, RandomState, Global);
generate_tests!(i64_i32_random_state_system, i64, i32, RandomState, System);
generate_tests!(i64_i64_random_state_global, i64, i64, RandomState, Global);
generate_tests!(i64_i64_random_state_system, i64, i64, RandomState, System);
generate_tests!(i64_i128_random_state_global, i64, i128, RandomState, Global);
generate_tests!(i64_i128_random_state_system, i64, i128, RandomState, System);
generate_tests!(i64_isize_random_state_global, i64, isize, RandomState, Global);
generate_tests!(i64_isize_random_state_system, i64, isize, RandomState, System);
generate_tests!(i64_u8_random_state_global, i64, u8, RandomState, Global);
generate_tests!(i64_u8_random_state_system, i64, u8, RandomState, System);
generate_tests!(i64_u16_random_state_global, i64, u16, RandomState, Global);
generate_tests!(i64_u16_random_state_system, i64, u16, RandomState, System);
generate_tests!(i64_u32_random_state_global, i64, u32, RandomState, Global);
generate_tests!(i64_u32_random_state_system, i64, u32, RandomState, System);
generate_tests!(i64_u64_random_state_global, i64, u64, RandomState, Global);
generate_tests!(i64_u64_random_state_system, i64, u64, RandomState, System);
generate_tests!(i64_u128_random_state_global, i64, u128, RandomState, Global);
generate_tests!(i64_u128_random_state_system, i64, u128, RandomState, System);
generate_tests!(i64_usize_random_state_global, i64, usize, RandomState, Global);
generate_tests!(i64_usize_random_state_system, i64, usize, RandomState, System);
generate_tests!(i64_f32_random_state_global, i64, f32, RandomState, Global);
generate_tests!(i64_f32_random_state_system, i64, f32, RandomState, System);
generate_tests!(i64_f64_random_state_global, i64, f64, RandomState, Global);
generate_tests!(i64_f64_random_state_system, i64, f64, RandomState, System);
generate_tests!(i64_bool_random_state_global, i64, bool, RandomState, Global);
generate_tests!(i64_bool_random_state_system, i64, bool, RandomState, System);
generate_tests!(i64_char_random_state_global, i64, char, RandomState, Global);
generate_tests!(i64_char_random_state_system, i64, char, RandomState, System);
generate_tests!(i64_string_random_state_global, i64, String, RandomState, Global);
generate_tests!(i64_string_random_state_system, i64, String, RandomState, System);
generate_tests!(i64_box_any_random_state_global, i64, Box<dyn any::Any>, RandomState, Global);
generate_tests!(i64_box_any_random_state_system, i64, Box<dyn any::Any>, RandomState, System);

generate_tests!(i128_i8_random_state_global, i128, i8, RandomState, Global);
generate_tests!(i128_i8_random_state_system, i128, i8, RandomState, System);
generate_tests!(i128_i16_random_state_global, i128, i16, RandomState, Global);
generate_tests!(i128_i16_random_state_system, i128, i16, RandomState, System);
generate_tests!(i128_i32_random_state_global, i128, i32, RandomState, Global);
generate_tests!(i128_i32_random_state_system, i128, i32, RandomState, System);
generate_tests!(i128_i64_random_state_global, i128, i64, RandomState, Global);
generate_tests!(i128_i64_random_state_system, i128, i64, RandomState, System);
generate_tests!(i128_i128_random_state_global, i128, i128, RandomState, Global);
generate_tests!(i128_i128_random_state_system, i128, i128, RandomState, System);
generate_tests!(i128_isize_random_state_global, i128, isize, RandomState, Global);
generate_tests!(i128_isize_random_state_system, i128, isize, RandomState, System);
generate_tests!(i128_u8_random_state_global, i128, u8, RandomState, Global);
generate_tests!(i128_u8_random_state_system, i128, u8, RandomState, System);
generate_tests!(i128_u16_random_state_global, i128, u16, RandomState, Global);
generate_tests!(i128_u16_random_state_system, i128, u16, RandomState, System);
generate_tests!(i128_u32_random_state_global, i128, u32, RandomState, Global);
generate_tests!(i128_u32_random_state_system, i128, u32, RandomState, System);
generate_tests!(i128_u64_random_state_global, i128, u64, RandomState, Global);
generate_tests!(i128_u64_random_state_system, i128, u64, RandomState, System);
generate_tests!(i128_u128_random_state_global, i128, u128, RandomState, Global);
generate_tests!(i128_u128_random_state_system, i128, u128, RandomState, System);
generate_tests!(i128_usize_random_state_global, i128, usize, RandomState, Global);
generate_tests!(i128_usize_random_state_system, i128, usize, RandomState, System);
generate_tests!(i128_f32_random_state_global, i128, f32, RandomState, Global);
generate_tests!(i128_f32_random_state_system, i128, f32, RandomState, System);
generate_tests!(i128_f64_random_state_global, i128, f64, RandomState, Global);
generate_tests!(i128_f64_random_state_system, i128, f64, RandomState, System);
generate_tests!(i128_bool_random_state_global, i128, bool, RandomState, Global);
generate_tests!(i128_bool_random_state_system, i128, bool, RandomState, System);
generate_tests!(i128_char_random_state_global, i128, char, RandomState, Global);
generate_tests!(i128_char_random_state_system, i128, char, RandomState, System);
generate_tests!(i128_string_random_state_global, i128, String, RandomState, Global);
generate_tests!(i128_string_random_state_system, i128, String, RandomState, System);
generate_tests!(i128_box_any_random_state_global, i128, Box<dyn any::Any>, RandomState, Global);
generate_tests!(i128_box_any_random_state_system, i128, Box<dyn any::Any>, RandomState, System);

generate_tests!(isize_i8_random_state_global, isize, i8, RandomState, Global);
generate_tests!(isize_i8_random_state_system, isize, i8, RandomState, System);
generate_tests!(isize_i16_random_state_global, isize, i16, RandomState, Global);
generate_tests!(isize_i16_random_state_system, isize, i16, RandomState, System);
generate_tests!(isize_i32_random_state_global, isize, i32, RandomState, Global);
generate_tests!(isize_i32_random_state_system, isize, i32, RandomState, System);
generate_tests!(isize_i64_random_state_global, isize, i64, RandomState, Global);
generate_tests!(isize_i64_random_state_system, isize, i64, RandomState, System);
generate_tests!(isize_i128_random_state_global, isize, i128, RandomState, Global);
generate_tests!(isize_i128_random_state_system, isize, i128, RandomState, System);
generate_tests!(isize_isize_random_state_global, isize, isize, RandomState, Global);
generate_tests!(isize_isize_random_state_system, isize, isize, RandomState, System);
generate_tests!(isize_u8_random_state_global, isize, u8, RandomState, Global);
generate_tests!(isize_u8_random_state_system, isize, u8, RandomState, System);
generate_tests!(isize_u16_random_state_global, isize, u16, RandomState, Global);
generate_tests!(isize_u16_random_state_system, isize, u16, RandomState, System);
generate_tests!(isize_u32_random_state_global, isize, u32, RandomState, Global);
generate_tests!(isize_u32_random_state_system, isize, u32, RandomState, System);
generate_tests!(isize_u64_random_state_global, isize, u64, RandomState, Global);
generate_tests!(isize_u64_random_state_system, isize, u64, RandomState, System);
generate_tests!(isize_u128_random_state_global, isize, u128, RandomState, Global);
generate_tests!(isize_u128_random_state_system, isize, u128, RandomState, System);
generate_tests!(isize_usize_random_state_global, isize, usize, RandomState, Global);
generate_tests!(isize_usize_random_state_system, isize, usize, RandomState, System);
generate_tests!(isize_f32_random_state_global, isize, f32, RandomState, Global);
generate_tests!(isize_f32_random_state_system, isize, f32, RandomState, System);
generate_tests!(isize_f64_random_state_global, isize, f64, RandomState, Global);
generate_tests!(isize_f64_random_state_system, isize, f64, RandomState, System);
generate_tests!(isize_bool_random_state_global, isize, bool, RandomState, Global);
generate_tests!(isize_bool_random_state_system, isize, bool, RandomState, System);
generate_tests!(isize_char_random_state_global, isize, char, RandomState, Global);
generate_tests!(isize_char_random_state_system, isize, char, RandomState, System);
generate_tests!(isize_string_random_state_global, isize, String, RandomState, Global);
generate_tests!(isize_string_random_state_system, isize, String, RandomState, System);
generate_tests!(isize_box_any_random_state_global, isize, Box<dyn any::Any>, RandomState, Global);
generate_tests!(isize_box_any_random_state_system, isize, Box<dyn any::Any>, RandomState, System);

generate_tests!(u8_i8_random_state_global, u8, i8, RandomState, Global);
generate_tests!(u8_i8_random_state_system, u8, i8, RandomState, System);
generate_tests!(u8_i16_random_state_global, u8, i16, RandomState, Global);
generate_tests!(u8_i16_random_state_system, u8, i16, RandomState, System);
generate_tests!(u8_i32_random_state_global, u8, i32, RandomState, Global);
generate_tests!(u8_i32_random_state_system, u8, i32, RandomState, System);
generate_tests!(u8_i64_random_state_global, u8, i64, RandomState, Global);
generate_tests!(u8_i64_random_state_system, u8, i64, RandomState, System);
generate_tests!(u8_i128_random_state_global, u8, i128, RandomState, Global);
generate_tests!(u8_i128_random_state_system, u8, i128, RandomState, System);
generate_tests!(u8_isize_random_state_global, u8, isize, RandomState, Global);
generate_tests!(u8_isize_random_state_system, u8, isize, RandomState, System);
generate_tests!(u8_u8_random_state_global, u8, u8, RandomState, Global);
generate_tests!(u8_u8_random_state_system, u8, u8, RandomState, System);
generate_tests!(u8_u16_random_state_global, u8, u16, RandomState, Global);
generate_tests!(u8_u16_random_state_system, u8, u16, RandomState, System);
generate_tests!(u8_u32_random_state_global, u8, u32, RandomState, Global);
generate_tests!(u8_u32_random_state_system, u8, u32, RandomState, System);
generate_tests!(u8_u64_random_state_global, u8, u64, RandomState, Global);
generate_tests!(u8_u64_random_state_system, u8, u64, RandomState, System);
generate_tests!(u8_u128_random_state_global, u8, u128, RandomState, Global);
generate_tests!(u8_u128_random_state_system, u8, u128, RandomState, System);
generate_tests!(u8_usize_random_state_global, u8, usize, RandomState, Global);
generate_tests!(u8_usize_random_state_system, u8, usize, RandomState, System);
generate_tests!(u8_f32_random_state_global, u8, f32, RandomState, Global);
generate_tests!(u8_f32_random_state_system, u8, f32, RandomState, System);
generate_tests!(u8_f64_random_state_global, u8, f64, RandomState, Global);
generate_tests!(u8_f64_random_state_system, u8, f64, RandomState, System);
generate_tests!(u8_bool_random_state_global, u8, bool, RandomState, Global);
generate_tests!(u8_bool_random_state_system, u8, bool, RandomState, System);
generate_tests!(u8_char_random_state_global, u8, char, RandomState, Global);
generate_tests!(u8_char_random_state_system, u8, char, RandomState, System);
generate_tests!(u8_string_random_state_global, u8, String, RandomState, Global);
generate_tests!(u8_string_random_state_system, u8, String, RandomState, System);
generate_tests!(u8_box_any_random_state_global, u8, Box<dyn any::Any>, RandomState, Global);
generate_tests!(u8_box_any_random_state_system, u8, Box<dyn any::Any>, RandomState, System);

generate_tests!(u16_i8_random_state_global, u16, i8, RandomState, Global);
generate_tests!(u16_i8_random_state_system, u16, i8, RandomState, System);
generate_tests!(u16_i16_random_state_global, u16, i16, RandomState, Global);
generate_tests!(u16_i16_random_state_system, u16, i16, RandomState, System);
generate_tests!(u16_i32_random_state_global, u16, i32, RandomState, Global);
generate_tests!(u16_i32_random_state_system, u16, i32, RandomState, System);
generate_tests!(u16_i64_random_state_global, u16, i64, RandomState, Global);
generate_tests!(u16_i64_random_state_system, u16, i64, RandomState, System);
generate_tests!(u16_i128_random_state_global, u16, i128, RandomState, Global);
generate_tests!(u16_i128_random_state_system, u16, i128, RandomState, System);
generate_tests!(u16_isize_random_state_global, u16, isize, RandomState, Global);
generate_tests!(u16_isize_random_state_system, u16, isize, RandomState, System);
generate_tests!(u16_u8_random_state_global, u16, u8, RandomState, Global);
generate_tests!(u16_u8_random_state_system, u16, u8, RandomState, System);
generate_tests!(u16_u16_random_state_global, u16, u16, RandomState, Global);
generate_tests!(u16_u16_random_state_system, u16, u16, RandomState, System);
generate_tests!(u16_u32_random_state_global, u16, u32, RandomState, Global);
generate_tests!(u16_u32_random_state_system, u16, u32, RandomState, System);
generate_tests!(u16_u64_random_state_global, u16, u64, RandomState, Global);
generate_tests!(u16_u64_random_state_system, u16, u64, RandomState, System);
generate_tests!(u16_u128_random_state_global, u16, u128, RandomState, Global);
generate_tests!(u16_u128_random_state_system, u16, u128, RandomState, System);
generate_tests!(u16_usize_random_state_global, u16, usize, RandomState, Global);
generate_tests!(u16_usize_random_state_system, u16, usize, RandomState, System);
generate_tests!(u16_f32_random_state_global, u16, f32, RandomState, Global);
generate_tests!(u16_f32_random_state_system, u16, f32, RandomState, System);
generate_tests!(u16_f64_random_state_global, u16, f64, RandomState, Global);
generate_tests!(u16_f64_random_state_system, u16, f64, RandomState, System);
generate_tests!(u16_bool_random_state_global, u16, bool, RandomState, Global);
generate_tests!(u16_bool_random_state_system, u16, bool, RandomState, System);
generate_tests!(u16_char_random_state_global, u16, char, RandomState, Global);
generate_tests!(u16_char_random_state_system, u16, char, RandomState, System);
generate_tests!(u16_string_random_state_global, u16, String, RandomState, Global);
generate_tests!(u16_string_random_state_system, u16, String, RandomState, System);
generate_tests!(u16_box_any_random_state_global, u16, Box<dyn any::Any>, RandomState, Global);
generate_tests!(u16_box_any_random_state_system, u16, Box<dyn any::Any>, RandomState, System);

generate_tests!(u32_i8_random_state_global, u32, i8, RandomState, Global);
generate_tests!(u32_i8_random_state_system, u32, i8, RandomState, System);
generate_tests!(u32_i16_random_state_global, u32, i16, RandomState, Global);
generate_tests!(u32_i16_random_state_system, u32, i16, RandomState, System);
generate_tests!(u32_i32_random_state_global, u32, i32, RandomState, Global);
generate_tests!(u32_i32_random_state_system, u32, i32, RandomState, System);
generate_tests!(u32_i64_random_state_global, u32, i64, RandomState, Global);
generate_tests!(u32_i64_random_state_system, u32, i64, RandomState, System);
generate_tests!(u32_i128_random_state_global, u32, i128, RandomState, Global);
generate_tests!(u32_i128_random_state_system, u32, i128, RandomState, System);
generate_tests!(u32_isize_random_state_global, u32, isize, RandomState, Global);
generate_tests!(u32_isize_random_state_system, u32, isize, RandomState, System);
generate_tests!(u32_u8_random_state_global, u32, u8, RandomState, Global);
generate_tests!(u32_u8_random_state_system, u32, u8, RandomState, System);
generate_tests!(u32_u16_random_state_global, u32, u16, RandomState, Global);
generate_tests!(u32_u16_random_state_system, u32, u16, RandomState, System);
generate_tests!(u32_u32_random_state_global, u32, u32, RandomState, Global);
generate_tests!(u32_u32_random_state_system, u32, u32, RandomState, System);
generate_tests!(u32_u64_random_state_global, u32, u64, RandomState, Global);
generate_tests!(u32_u64_random_state_system, u32, u64, RandomState, System);
generate_tests!(u32_u128_random_state_global, u32, u128, RandomState, Global);
generate_tests!(u32_u128_random_state_system, u32, u128, RandomState, System);
generate_tests!(u32_usize_random_state_global, u32, usize, RandomState, Global);
generate_tests!(u32_usize_random_state_system, u32, usize, RandomState, System);
generate_tests!(u32_f32_random_state_global, u32, f32, RandomState, Global);
generate_tests!(u32_f32_random_state_system, u32, f32, RandomState, System);
generate_tests!(u32_f64_random_state_global, u32, f64, RandomState, Global);
generate_tests!(u32_f64_random_state_system, u32, f64, RandomState, System);
generate_tests!(u32_bool_random_state_global, u32, bool, RandomState, Global);
generate_tests!(u32_bool_random_state_system, u32, bool, RandomState, System);
generate_tests!(u32_char_random_state_global, u32, char, RandomState, Global);
generate_tests!(u32_char_random_state_system, u32, char, RandomState, System);
generate_tests!(u32_string_random_state_global, u32, String, RandomState, Global);
generate_tests!(u32_string_random_state_system, u32, String, RandomState, System);
generate_tests!(u32_box_any_random_state_global, u32, Box<dyn any::Any>, RandomState, Global);
generate_tests!(u32_box_any_random_state_system, u32, Box<dyn any::Any>, RandomState, System);

generate_tests!(u64_i8_random_state_global, u64, i8, RandomState, Global);
generate_tests!(u64_i8_random_state_system, u64, i8, RandomState, System);
generate_tests!(u64_i16_random_state_global, u64, i16, RandomState, Global);
generate_tests!(u64_i16_random_state_system, u64, i16, RandomState, System);
generate_tests!(u64_i32_random_state_global, u64, i32, RandomState, Global);
generate_tests!(u64_i32_random_state_system, u64, i32, RandomState, System);
generate_tests!(u64_i64_random_state_global, u64, i64, RandomState, Global);
generate_tests!(u64_i64_random_state_system, u64, i64, RandomState, System);
generate_tests!(u64_i128_random_state_global, u64, i128, RandomState, Global);
generate_tests!(u64_i128_random_state_system, u64, i128, RandomState, System);
generate_tests!(u64_isize_random_state_global, u64, isize, RandomState, Global);
generate_tests!(u64_isize_random_state_system, u64, isize, RandomState, System);
generate_tests!(u64_u8_random_state_global, u64, u8, RandomState, Global);
generate_tests!(u64_u8_random_state_system, u64, u8, RandomState, System);
generate_tests!(u64_u16_random_state_global, u64, u16, RandomState, Global);
generate_tests!(u64_u16_random_state_system, u64, u16, RandomState, System);
generate_tests!(u64_u32_random_state_global, u64, u32, RandomState, Global);
generate_tests!(u64_u32_random_state_system, u64, u32, RandomState, System);
generate_tests!(u64_u64_random_state_global, u64, u64, RandomState, Global);
generate_tests!(u64_u64_random_state_system, u64, u64, RandomState, System);
generate_tests!(u64_u128_random_state_global, u64, u128, RandomState, Global);
generate_tests!(u64_u128_random_state_system, u64, u128, RandomState, System);
generate_tests!(u64_usize_random_state_global, u64, usize, RandomState, Global);
generate_tests!(u64_usize_random_state_system, u64, usize, RandomState, System);
generate_tests!(u64_f32_random_state_global, u64, f32, RandomState, Global);
generate_tests!(u64_f32_random_state_system, u64, f32, RandomState, System);
generate_tests!(u64_f64_random_state_global, u64, f64, RandomState, Global);
generate_tests!(u64_f64_random_state_system, u64, f64, RandomState, System);
generate_tests!(u64_bool_random_state_global, u64, bool, RandomState, Global);
generate_tests!(u64_bool_random_state_system, u64, bool, RandomState, System);
generate_tests!(u64_char_random_state_global, u64, char, RandomState, Global);
generate_tests!(u64_char_random_state_system, u64, char, RandomState, System);
generate_tests!(u64_string_random_state_global, u64, String, RandomState, Global);
generate_tests!(u64_string_random_state_system, u64, String, RandomState, System);
generate_tests!(u64_box_any_random_state_global, u64, Box<dyn any::Any>, RandomState, Global);
generate_tests!(u64_box_any_random_state_system, u64, Box<dyn any::Any>, RandomState, System);

generate_tests!(u128_i8_random_state_global, u128, i8, RandomState, Global);
generate_tests!(u128_i8_random_state_system, u128, i8, RandomState, System);
generate_tests!(u128_i16_random_state_global, u128, i16, RandomState, Global);
generate_tests!(u128_i16_random_state_system, u128, i16, RandomState, System);
generate_tests!(u128_i32_random_state_global, u128, i32, RandomState, Global);
generate_tests!(u128_i32_random_state_system, u128, i32, RandomState, System);
generate_tests!(u128_i64_random_state_global, u128, i64, RandomState, Global);
generate_tests!(u128_i64_random_state_system, u128, i64, RandomState, System);
generate_tests!(u128_i128_random_state_global, u128, i128, RandomState, Global);
generate_tests!(u128_i128_random_state_system, u128, i128, RandomState, System);
generate_tests!(u128_isize_random_state_global, u128, isize, RandomState, Global);
generate_tests!(u128_isize_random_state_system, u128, isize, RandomState, System);
generate_tests!(u128_u8_random_state_global, u128, u8, RandomState, Global);
generate_tests!(u128_u8_random_state_system, u128, u8, RandomState, System);
generate_tests!(u128_u16_random_state_global, u128, u16, RandomState, Global);
generate_tests!(u128_u16_random_state_system, u128, u16, RandomState, System);
generate_tests!(u128_u32_random_state_global, u128, u32, RandomState, Global);
generate_tests!(u128_u32_random_state_system, u128, u32, RandomState, System);
generate_tests!(u128_u64_random_state_global, u128, u64, RandomState, Global);
generate_tests!(u128_u64_random_state_system, u128, u64, RandomState, System);
generate_tests!(u128_u128_random_state_global, u128, u128, RandomState, Global);
generate_tests!(u128_u128_random_state_system, u128, u128, RandomState, System);
generate_tests!(u128_usize_random_state_global, u128, usize, RandomState, Global);
generate_tests!(u128_usize_random_state_system, u128, usize, RandomState, System);
generate_tests!(u128_f32_random_state_global, u128, f32, RandomState, Global);
generate_tests!(u128_f32_random_state_system, u128, f32, RandomState, System);
generate_tests!(u128_f64_random_state_global, u128, f64, RandomState, Global);
generate_tests!(u128_f64_random_state_system, u128, f64, RandomState, System);
generate_tests!(u128_bool_random_state_global, u128, bool, RandomState, Global);
generate_tests!(u128_bool_random_state_system, u128, bool, RandomState, System);
generate_tests!(u128_char_random_state_global, u128, char, RandomState, Global);
generate_tests!(u128_char_random_state_system, u128, char, RandomState, System);
generate_tests!(u128_string_random_state_global, u128, String, RandomState, Global);
generate_tests!(u128_string_random_state_system, u128, String, RandomState, System);
generate_tests!(u128_box_any_random_state_global, u128, Box<dyn any::Any>, RandomState, Global);
generate_tests!(u128_box_any_random_state_system, u128, Box<dyn any::Any>, RandomState, System);

generate_tests!(usize_i8_random_state_global, usize, i8, RandomState, Global);
generate_tests!(usize_i8_random_state_system, usize, i8, RandomState, System);
generate_tests!(usize_i16_random_state_global, usize, i16, RandomState, Global);
generate_tests!(usize_i16_random_state_system, usize, i16, RandomState, System);
generate_tests!(usize_i32_random_state_global, usize, i32, RandomState, Global);
generate_tests!(usize_i32_random_state_system, usize, i32, RandomState, System);
generate_tests!(usize_i64_random_state_global, usize, i64, RandomState, Global);
generate_tests!(usize_i64_random_state_system, usize, i64, RandomState, System);
generate_tests!(usize_i128_random_state_global, usize, i128, RandomState, Global);
generate_tests!(usize_i128_random_state_system, usize, i128, RandomState, System);
generate_tests!(usize_isize_random_state_global, usize, isize, RandomState, Global);
generate_tests!(usize_isize_random_state_system, usize, isize, RandomState, System);
generate_tests!(usize_u8_random_state_global, usize, u8, RandomState, Global);
generate_tests!(usize_u8_random_state_system, usize, u8, RandomState, System);
generate_tests!(usize_u16_random_state_global, usize, u16, RandomState, Global);
generate_tests!(usize_u16_random_state_system, usize, u16, RandomState, System);
generate_tests!(usize_u32_random_state_global, usize, u32, RandomState, Global);
generate_tests!(usize_u32_random_state_system, usize, u32, RandomState, System);
generate_tests!(usize_u64_random_state_global, usize, u64, RandomState, Global);
generate_tests!(usize_u64_random_state_system, usize, u64, RandomState, System);
generate_tests!(usize_u128_random_state_global, usize, u128, RandomState, Global);
generate_tests!(usize_u128_random_state_system, usize, u128, RandomState, System);
generate_tests!(usize_usize_random_state_global, usize, usize, RandomState, Global);
generate_tests!(usize_usize_random_state_system, usize, usize, RandomState, System);
generate_tests!(usize_f32_random_state_global, usize, f32, RandomState, Global);
generate_tests!(usize_f32_random_state_system, usize, f32, RandomState, System);
generate_tests!(usize_f64_random_state_global, usize, f64, RandomState, Global);
generate_tests!(usize_f64_random_state_system, usize, f64, RandomState, System);
generate_tests!(usize_bool_random_state_global, usize, bool, RandomState, Global);
generate_tests!(usize_bool_random_state_system, usize, bool, RandomState, System);
generate_tests!(usize_char_random_state_global, usize, char, RandomState, Global);
generate_tests!(usize_char_random_state_system, usize, char, RandomState, System);
generate_tests!(usize_string_random_state_global, usize, String, RandomState, Global);
generate_tests!(usize_string_random_state_system, usize, String, RandomState, System);
generate_tests!(usize_box_any_random_state_global, usize, Box<dyn any::Any>, RandomState, Global);
generate_tests!(usize_box_any_random_state_system, usize, Box<dyn any::Any>, RandomState, System);

generate_tests!(f32_i8_random_state_global, f32, i8, RandomState, Global);
generate_tests!(f32_i8_random_state_system, f32, i8, RandomState, System);
generate_tests!(f32_i16_random_state_global, f32, i16, RandomState, Global);
generate_tests!(f32_i16_random_state_system, f32, i16, RandomState, System);
generate_tests!(f32_i32_random_state_global, f32, i32, RandomState, Global);
generate_tests!(f32_i32_random_state_system, f32, i32, RandomState, System);
generate_tests!(f32_i64_random_state_global, f32, i64, RandomState, Global);
generate_tests!(f32_i64_random_state_system, f32, i64, RandomState, System);
generate_tests!(f32_i128_random_state_global, f32, i128, RandomState, Global);
generate_tests!(f32_i128_random_state_system, f32, i128, RandomState, System);
generate_tests!(f32_isize_random_state_global, f32, isize, RandomState, Global);
generate_tests!(f32_isize_random_state_system, f32, isize, RandomState, System);
generate_tests!(f32_u8_random_state_global, f32, u8, RandomState, Global);
generate_tests!(f32_u8_random_state_system, f32, u8, RandomState, System);
generate_tests!(f32_u16_random_state_global, f32, u16, RandomState, Global);
generate_tests!(f32_u16_random_state_system, f32, u16, RandomState, System);
generate_tests!(f32_u32_random_state_global, f32, u32, RandomState, Global);
generate_tests!(f32_u32_random_state_system, f32, u32, RandomState, System);
generate_tests!(f32_u64_random_state_global, f32, u64, RandomState, Global);
generate_tests!(f32_u64_random_state_system, f32, u64, RandomState, System);
generate_tests!(f32_u128_random_state_global, f32, u128, RandomState, Global);
generate_tests!(f32_u128_random_state_system, f32, u128, RandomState, System);
generate_tests!(f32_usize_random_state_global, f32, usize, RandomState, Global);
generate_tests!(f32_usize_random_state_system, f32, usize, RandomState, System);
generate_tests!(f32_f32_random_state_global, f32, f32, RandomState, Global);
generate_tests!(f32_f32_random_state_system, f32, f32, RandomState, System);
generate_tests!(f32_f64_random_state_global, f32, f64, RandomState, Global);
generate_tests!(f32_f64_random_state_system, f32, f64, RandomState, System);
generate_tests!(f32_bool_random_state_global, f32, bool, RandomState, Global);
generate_tests!(f32_bool_random_state_system, f32, bool, RandomState, System);
generate_tests!(f32_char_random_state_global, f32, char, RandomState, Global);
generate_tests!(f32_char_random_state_system, f32, char, RandomState, System);
generate_tests!(f32_string_random_state_global, f32, String, RandomState, Global);
generate_tests!(f32_string_random_state_system, f32, String, RandomState, System);
generate_tests!(f32_box_any_random_state_global, f32, Box<dyn any::Any>, RandomState, Global);
generate_tests!(f32_box_any_random_state_system, f32, Box<dyn any::Any>, RandomState, System);

generate_tests!(f64_i8_random_state_global, f64, i8, RandomState, Global);
generate_tests!(f64_i8_random_state_system, f64, i8, RandomState, System);
generate_tests!(f64_i16_random_state_global, f64, i16, RandomState, Global);
generate_tests!(f64_i16_random_state_system, f64, i16, RandomState, System);
generate_tests!(f64_i32_random_state_global, f64, i32, RandomState, Global);
generate_tests!(f64_i32_random_state_system, f64, i32, RandomState, System);
generate_tests!(f64_i64_random_state_global, f64, i64, RandomState, Global);
generate_tests!(f64_i64_random_state_system, f64, i64, RandomState, System);
generate_tests!(f64_i128_random_state_global, f64, i128, RandomState, Global);
generate_tests!(f64_i128_random_state_system, f64, i128, RandomState, System);
generate_tests!(f64_isize_random_state_global, f64, isize, RandomState, Global);
generate_tests!(f64_isize_random_state_system, f64, isize, RandomState, System);
generate_tests!(f64_u8_random_state_global, f64, u8, RandomState, Global);
generate_tests!(f64_u8_random_state_system, f64, u8, RandomState, System);
generate_tests!(f64_u16_random_state_global, f64, u16, RandomState, Global);
generate_tests!(f64_u16_random_state_system, f64, u16, RandomState, System);
generate_tests!(f64_u32_random_state_global, f64, u32, RandomState, Global);
generate_tests!(f64_u32_random_state_system, f64, u32, RandomState, System);
generate_tests!(f64_u64_random_state_global, f64, u64, RandomState, Global);
generate_tests!(f64_u64_random_state_system, f64, u64, RandomState, System);
generate_tests!(f64_u128_random_state_global, f64, u128, RandomState, Global);
generate_tests!(f64_u128_random_state_system, f64, u128, RandomState, System);
generate_tests!(f64_usize_random_state_global, f64, usize, RandomState, Global);
generate_tests!(f64_usize_random_state_system, f64, usize, RandomState, System);
generate_tests!(f64_f32_random_state_global, f64, f32, RandomState, Global);
generate_tests!(f64_f32_random_state_system, f64, f32, RandomState, System);
generate_tests!(f64_f64_random_state_global, f64, f64, RandomState, Global);
generate_tests!(f64_f64_random_state_system, f64, f64, RandomState, System);
generate_tests!(f64_bool_random_state_global, f64, bool, RandomState, Global);
generate_tests!(f64_bool_random_state_system, f64, bool, RandomState, System);
generate_tests!(f64_char_random_state_global, f64, char, RandomState, Global);
generate_tests!(f64_char_random_state_system, f64, char, RandomState, System);
generate_tests!(f64_string_random_state_global, f64, String, RandomState, Global);
generate_tests!(f64_string_random_state_system, f64, String, RandomState, System);
generate_tests!(f64_box_any_random_state_global, f64, Box<dyn any::Any>, RandomState, Global);
generate_tests!(f64_box_any_random_state_system, f64, Box<dyn any::Any>, RandomState, System);

generate_tests!(bool_i8_random_state_global, bool, i8, RandomState, Global);
generate_tests!(bool_i8_random_state_system, bool, i8, RandomState, System);
generate_tests!(bool_i16_random_state_global, bool, i16, RandomState, Global);
generate_tests!(bool_i16_random_state_system, bool, i16, RandomState, System);
generate_tests!(bool_i32_random_state_global, bool, i32, RandomState, Global);
generate_tests!(bool_i32_random_state_system, bool, i32, RandomState, System);
generate_tests!(bool_i64_random_state_global, bool, i64, RandomState, Global);
generate_tests!(bool_i64_random_state_system, bool, i64, RandomState, System);
generate_tests!(bool_i128_random_state_global, bool, i128, RandomState, Global);
generate_tests!(bool_i128_random_state_system, bool, i128, RandomState, System);
generate_tests!(bool_isize_random_state_global, bool, isize, RandomState, Global);
generate_tests!(bool_isize_random_state_system, bool, isize, RandomState, System);
generate_tests!(bool_u8_random_state_global, bool, u8, RandomState, Global);
generate_tests!(bool_u8_random_state_system, bool, u8, RandomState, System);
generate_tests!(bool_u16_random_state_global, bool, u16, RandomState, Global);
generate_tests!(bool_u16_random_state_system, bool, u16, RandomState, System);
generate_tests!(bool_u32_random_state_global, bool, u32, RandomState, Global);
generate_tests!(bool_u32_random_state_system, bool, u32, RandomState, System);
generate_tests!(bool_u64_random_state_global, bool, u64, RandomState, Global);
generate_tests!(bool_u64_random_state_system, bool, u64, RandomState, System);
generate_tests!(bool_u128_random_state_global, bool, u128, RandomState, Global);
generate_tests!(bool_u128_random_state_system, bool, u128, RandomState, System);
generate_tests!(bool_usize_random_state_global, bool, usize, RandomState, Global);
generate_tests!(bool_usize_random_state_system, bool, usize, RandomState, System);
generate_tests!(bool_f32_random_state_global, bool, f32, RandomState, Global);
generate_tests!(bool_f32_random_state_system, bool, f32, RandomState, System);
generate_tests!(bool_f64_random_state_global, bool, f64, RandomState, Global);
generate_tests!(bool_f64_random_state_system, bool, f64, RandomState, System);
generate_tests!(bool_bool_random_state_global, bool, bool, RandomState, Global);
generate_tests!(bool_bool_random_state_system, bool, bool, RandomState, System);
generate_tests!(bool_char_random_state_global, bool, char, RandomState, Global);
generate_tests!(bool_char_random_state_system, bool, char, RandomState, System);
generate_tests!(bool_string_random_state_global, bool, String, RandomState, Global);
generate_tests!(bool_string_random_state_system, bool, String, RandomState, System);
generate_tests!(bool_box_any_random_state_global, bool, Box<dyn any::Any>, RandomState, Global);
generate_tests!(bool_box_any_random_state_system, bool, Box<dyn any::Any>, RandomState, System);

generate_tests!(char_i8_random_state_global, char, i8, RandomState, Global);
generate_tests!(char_i8_random_state_system, char, i8, RandomState, System);
generate_tests!(char_i16_random_state_global, char, i16, RandomState, Global);
generate_tests!(char_i16_random_state_system, char, i16, RandomState, System);
generate_tests!(char_i32_random_state_global, char, i32, RandomState, Global);
generate_tests!(char_i32_random_state_system, char, i32, RandomState, System);
generate_tests!(char_i64_random_state_global, char, i64, RandomState, Global);
generate_tests!(char_i64_random_state_system, char, i64, RandomState, System);
generate_tests!(char_i128_random_state_global, char, i128, RandomState, Global);
generate_tests!(char_i128_random_state_system, char, i128, RandomState, System);
generate_tests!(char_isize_random_state_global, char, isize, RandomState, Global);
generate_tests!(char_isize_random_state_system, char, isize, RandomState, System);
generate_tests!(char_u8_random_state_global, char, u8, RandomState, Global);
generate_tests!(char_u8_random_state_system, char, u8, RandomState, System);
generate_tests!(char_u16_random_state_global, char, u16, RandomState, Global);
generate_tests!(char_u16_random_state_system, char, u16, RandomState, System);
generate_tests!(char_u32_random_state_global, char, u32, RandomState, Global);
generate_tests!(char_u32_random_state_system, char, u32, RandomState, System);
generate_tests!(char_u64_random_state_global, char, u64, RandomState, Global);
generate_tests!(char_u64_random_state_system, char, u64, RandomState, System);
generate_tests!(char_u128_random_state_global, char, u128, RandomState, Global);
generate_tests!(char_u128_random_state_system, char, u128, RandomState, System);
generate_tests!(char_usize_random_state_global, char, usize, RandomState, Global);
generate_tests!(char_usize_random_state_system, char, usize, RandomState, System);
generate_tests!(char_f32_random_state_global, char, f32, RandomState, Global);
generate_tests!(char_f32_random_state_system, char, f32, RandomState, System);
generate_tests!(char_f64_random_state_global, char, f64, RandomState, Global);
generate_tests!(char_f64_random_state_system, char, f64, RandomState, System);
generate_tests!(char_bool_random_state_global, char, bool, RandomState, Global);
generate_tests!(char_bool_random_state_system, char, bool, RandomState, System);
generate_tests!(char_char_random_state_global, char, char, RandomState, Global);
generate_tests!(char_char_random_state_system, char, char, RandomState, System);
generate_tests!(char_string_random_state_global, char, String, RandomState, Global);
generate_tests!(char_string_random_state_system, char, String, RandomState, System);
generate_tests!(char_box_any_random_state_global, char, Box<dyn any::Any>, RandomState, Global);
generate_tests!(char_box_any_random_state_system, char, Box<dyn any::Any>, RandomState, System);

generate_tests!(string_i8_random_state_global, String, i8, RandomState, Global);
generate_tests!(string_i8_random_state_system, String, i8, RandomState, System);
generate_tests!(string_i16_random_state_global, String, i16, RandomState, Global);
generate_tests!(string_i16_random_state_system, String, i16, RandomState, System);
generate_tests!(string_i32_random_state_global, String, i32, RandomState, Global);
generate_tests!(string_i32_random_state_system, String, i32, RandomState, System);
generate_tests!(string_i64_random_state_global, String, i64, RandomState, Global);
generate_tests!(string_i64_random_state_system, String, i64, RandomState, System);
generate_tests!(string_i128_random_state_global, String, i128, RandomState, Global);
generate_tests!(string_i128_random_state_system, String, i128, RandomState, System);
generate_tests!(string_isize_random_state_global, String, isize, RandomState, Global);
generate_tests!(string_isize_random_state_system, String, isize, RandomState, System);
generate_tests!(string_u8_random_state_global, String, u8, RandomState, Global);
generate_tests!(string_u8_random_state_system, String, u8, RandomState, System);
generate_tests!(string_u16_random_state_global, String, u16, RandomState, Global);
generate_tests!(string_u16_random_state_system, String, u16, RandomState, System);
generate_tests!(string_u32_random_state_global, String, u32, RandomState, Global);
generate_tests!(string_u32_random_state_system, String, u32, RandomState, System);
generate_tests!(string_u64_random_state_global, String, u64, RandomState, Global);
generate_tests!(string_u64_random_state_system, String, u64, RandomState, System);
generate_tests!(string_u128_random_state_global, String, u128, RandomState, Global);
generate_tests!(string_u128_random_state_system, String, u128, RandomState, System);
generate_tests!(string_usize_random_state_global, String, usize, RandomState, Global);
generate_tests!(string_usize_random_state_system, String, usize, RandomState, System);
generate_tests!(string_f32_random_state_global, String, f32, RandomState, Global);
generate_tests!(string_f32_random_state_system, String, f32, RandomState, System);
generate_tests!(string_f64_random_state_global, String, f64, RandomState, Global);
generate_tests!(string_f64_random_state_system, String, f64, RandomState, System);
generate_tests!(string_bool_random_state_global, String, bool, RandomState, Global);
generate_tests!(string_bool_random_state_system, String, bool, RandomState, System);
generate_tests!(string_char_random_state_global, String, char, RandomState, Global);
generate_tests!(string_char_random_state_system, String, char, RandomState, System);
generate_tests!(string_string_random_state_global, String, String, RandomState, Global);
generate_tests!(string_string_random_state_system, String, String, RandomState, System);
generate_tests!(string_box_any_random_state_global, String, Box<dyn any::Any>, RandomState, Global);
generate_tests!(string_box_any_random_state_system, String, Box<dyn any::Any>, RandomState, System);
