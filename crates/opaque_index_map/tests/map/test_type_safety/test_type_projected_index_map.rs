use opaque_index_map::map::{
    TypeErasedIndexMap,
    TypeProjectedIndexMap,
};

use core::any;
use core::ptr::NonNull;
use std::boxed::Box;
use std::hash;
use std::hash::RandomState;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(feature = "nightly")]
use std::alloc::Global;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc::Global;

#[derive(Clone, Default)]
struct WrappingAlloc<A> {
    alloc: A,
}

impl<A> WrappingAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn new(alloc: A) -> Self {
        Self { alloc }
    }
}

unsafe impl<A> alloc::Allocator for WrappingAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
        self.alloc.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
        unsafe { self.alloc.deallocate(ptr, layout) }
    }
}

fn run_test_type_erased_index_map_with_hasher_in_has_type<K, V, S, A>(build_hasher: S, alloc: A)
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_map = TypeProjectedIndexMap::<K, V, S, A>::with_hasher_in(build_hasher, alloc);
    let opaque_map = TypeErasedIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<S>());
    assert!(opaque_map.has_allocator_type::<A>());
}

fn run_test_type_erased_index_map_with_capacity_and_hasher_in_has_type<K, V, S, A>(build_hasher: S, alloc: A)
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_map = TypeProjectedIndexMap::<K, V, S, A>::with_capacity_and_hasher_in(1024, build_hasher, alloc);
    let opaque_map = TypeErasedIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<S>());
    assert!(opaque_map.has_allocator_type::<A>());
}

fn run_test_type_erased_index_map_new_in_has_type<K, V, A>(alloc: A)
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_map = TypeProjectedIndexMap::<K, V, _, A>::new_in(alloc);
    let opaque_map = TypeErasedIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<hash::RandomState>());
    assert!(opaque_map.has_allocator_type::<A>());
}

fn run_test_type_erased_index_map_with_capacity_in_has_type<K, V, A>(alloc: A)
where
    K: any::Any,
    V: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_map = TypeProjectedIndexMap::<K, V, _, A>::with_capacity_in(1024, alloc);
    let opaque_map = TypeErasedIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<hash::RandomState>());
    assert!(opaque_map.has_allocator_type::<A>());
}

fn run_test_type_erased_index_map_with_hasher_has_type<K, V, S>(build_hasher: S)
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    let proj_map = TypeProjectedIndexMap::<K, V, S, _>::with_hasher(build_hasher);
    let opaque_map = TypeErasedIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<S>());
    assert!(opaque_map.has_allocator_type::<alloc::Global>());
}

fn run_test_type_erased_index_map_with_capacity_and_hasher_has_type<K, V, S>(build_hasher: S)
where
    K: any::Any,
    V: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    let proj_map = TypeProjectedIndexMap::<K, V, S, _>::with_capacity_and_hasher(1024, build_hasher);
    let opaque_map = TypeErasedIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<S>());
    assert!(opaque_map.has_allocator_type::<alloc::Global>());
}

fn run_test_type_erased_index_map_new_has_type<K, V>()
where
    K: any::Any,
    V: any::Any,
{
    let proj_map = TypeProjectedIndexMap::<K, V, _, _>::new();
    let opaque_map = TypeErasedIndexMap::from_proj(proj_map);

    assert!(opaque_map.has_key_type::<K>());
    assert!(opaque_map.has_value_type::<V>());
    assert!(opaque_map.has_build_hasher_type::<hash::RandomState>());
    assert!(opaque_map.has_allocator_type::<Global>());
}

fn run_test_type_erased_index_map_with_capacity_has_type<K, V>()
where
    K: any::Any,
    V: any::Any,
{
    let proj_map = TypeProjectedIndexMap::<K, V, _, _>::with_capacity(1024);
    let opaque_map = TypeErasedIndexMap::from_proj(proj_map);

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
            fn test_type_erased_index_map_with_hasher_in_has_type() {
                let build_hasher: $build_hasher_typ = Default::default();
                let alloc: $alloc_typ = Default::default();
                run_test_type_erased_index_map_with_hasher_in_has_type::<$key_typ, $value_typ, $build_hasher_typ, $alloc_typ>(
                    build_hasher,
                    alloc,
                );
            }

            #[test]
            fn test_type_erased_index_map_with_capacity_and_hasher_in_has_type() {
                let build_hasher: $build_hasher_typ = Default::default();
                let alloc: $alloc_typ = Default::default();
                run_test_type_erased_index_map_with_capacity_and_hasher_in_has_type::<
                    $key_typ,
                    $value_typ,
                    $build_hasher_typ,
                    $alloc_typ,
                >(build_hasher, alloc);
            }

            #[test]
            fn test_type_erased_index_map_new_in_has_type() {
                let alloc: $alloc_typ = Default::default();
                run_test_type_erased_index_map_new_in_has_type::<$key_typ, $value_typ, $alloc_typ>(alloc);
            }

            #[test]
            fn test_type_erased_index_map_with_capacity_in_has_type() {
                let alloc: $alloc_typ = Default::default();
                run_test_type_erased_index_map_with_capacity_in_has_type::<$key_typ, $value_typ, $alloc_typ>(alloc);
            }

            #[test]
            fn test_type_erased_index_map_with_hasher_has_type() {
                let build_hasher: $build_hasher_typ = Default::default();
                run_test_type_erased_index_map_with_hasher_has_type::<$key_typ, $value_typ, $build_hasher_typ>(build_hasher);
            }

            #[test]
            fn test_type_erased_index_map_with_capacity_and_hasher_has_type() {
                let build_hasher: $build_hasher_typ = Default::default();
                run_test_type_erased_index_map_with_capacity_and_hasher_has_type::<$key_typ, $value_typ, $build_hasher_typ>(
                    build_hasher,
                );
            }
            #[test]
            fn test_type_erased_index_map_new_has_type() {
                run_test_type_erased_index_map_new_has_type::<$key_typ, $value_typ>();
            }

            #[test]
            fn test_type_erased_index_map_with_capacity_has_type() {
                run_test_type_erased_index_map_with_capacity_has_type::<$key_typ, $value_typ>();
            }
        }
    };
}

generate_tests!(i8_i8_random_state_global, i8, i8, RandomState, Global);
generate_tests!(i8_i8_random_state_wrapping, i8, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_i16_random_state_global, i8, i16, RandomState, Global);
generate_tests!(i8_i16_random_state_wrapping, i8, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_i32_random_state_global, i8, i32, RandomState, Global);
generate_tests!(i8_i32_random_state_wrapping, i8, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_i64_random_state_global, i8, i64, RandomState, Global);
generate_tests!(i8_i64_random_state_wrapping, i8, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_i128_random_state_global, i8, i128, RandomState, Global);
generate_tests!(i8_i128_random_state_wrapping, i8, i128, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_isize_random_state_global, i8, isize, RandomState, Global);
generate_tests!(i8_isize_random_state_wrapping, i8, isize, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_u8_random_state_global, i8, u8, RandomState, Global);
generate_tests!(i8_u8_random_state_wrapping, i8, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_u16_random_state_global, i8, u16, RandomState, Global);
generate_tests!(i8_u16_random_state_wrapping, i8, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_u32_random_state_global, i8, u32, RandomState, Global);
generate_tests!(i8_u32_random_state_wrapping, i8, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_u64_random_state_global, i8, u64, RandomState, Global);
generate_tests!(i8_u64_random_state_wrapping, i8, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_u128_random_state_global, i8, u128, RandomState, Global);
generate_tests!(i8_u128_random_state_wrapping, i8, u128, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_usize_random_state_global, i8, usize, RandomState, Global);
generate_tests!(i8_usize_random_state_wrapping, i8, usize, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_f32_random_state_global, i8, f32, RandomState, Global);
generate_tests!(i8_f32_random_state_wrapping, i8, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_f64_random_state_global, i8, f64, RandomState, Global);
generate_tests!(i8_f64_random_state_wrapping, i8, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_bool_random_state_global, i8, bool, RandomState, Global);
generate_tests!(i8_bool_random_state_wrapping, i8, bool, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_char_random_state_global, i8, char, RandomState, Global);
generate_tests!(i8_char_random_state_wrapping, i8, char, RandomState, WrappingAlloc<Global>);
generate_tests!(i8_string_random_state_global, i8, String, RandomState, Global);
generate_tests!(
    i8_string_random_state_wrapping,
    i8,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i8_box_any_random_state_global, i8, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    i8_box_any_random_state_wrapping,
    i8,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(i16_i8_random_state_global, i16, i8, RandomState, Global);
generate_tests!(i16_i8_random_state_wrapping, i16, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_i16_random_state_global, i16, i16, RandomState, Global);
generate_tests!(i16_i16_random_state_wrapping, i16, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_i32_random_state_global, i16, i32, RandomState, Global);
generate_tests!(i16_i32_random_state_wrapping, i16, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_i64_random_state_global, i16, i64, RandomState, Global);
generate_tests!(i16_i64_random_state_wrapping, i16, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_i128_random_state_global, i16, i128, RandomState, Global);
generate_tests!(i16_i128_random_state_wrapping, i16, i128, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_isize_random_state_global, i16, isize, RandomState, Global);
generate_tests!(
    i16_isize_random_state_wrapping,
    i16,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i16_u8_random_state_global, i16, u8, RandomState, Global);
generate_tests!(i16_u8_random_state_wrapping, i16, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_u16_random_state_global, i16, u16, RandomState, Global);
generate_tests!(i16_u16_random_state_wrapping, i16, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_u32_random_state_global, i16, u32, RandomState, Global);
generate_tests!(i16_u32_random_state_wrapping, i16, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_u64_random_state_global, i16, u64, RandomState, Global);
generate_tests!(i16_u64_random_state_wrapping, i16, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_u128_random_state_global, i16, u128, RandomState, Global);
generate_tests!(i16_u128_random_state_wrapping, i16, u128, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_usize_random_state_global, i16, usize, RandomState, Global);
generate_tests!(
    i16_usize_random_state_wrapping,
    i16,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i16_f32_random_state_global, i16, f32, RandomState, Global);
generate_tests!(i16_f32_random_state_wrapping, i16, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_f64_random_state_global, i16, f64, RandomState, Global);
generate_tests!(i16_f64_random_state_wrapping, i16, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_bool_random_state_global, i16, bool, RandomState, Global);
generate_tests!(i16_bool_random_state_wrapping, i16, bool, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_char_random_state_global, i16, char, RandomState, Global);
generate_tests!(i16_char_random_state_wrapping, i16, char, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_string_random_state_global, i16, String, RandomState, Global);
generate_tests!(
    i16_string_random_state_wrapping,
    i16,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i16_box_any_random_state_global, i16, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    i16_box_any_random_state_wrapping,
    i16,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(i32_i8_random_state_global, i32, i8, RandomState, Global);
generate_tests!(i32_i8_random_state_wrapping, i32, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_i16_random_state_global, i32, i16, RandomState, Global);
generate_tests!(i32_i16_random_state_wrapping, i32, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_i32_random_state_global, i32, i32, RandomState, Global);
generate_tests!(i32_i32_random_state_wrapping, i32, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_i64_random_state_global, i32, i64, RandomState, Global);
generate_tests!(i32_i64_random_state_wrapping, i32, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_i128_random_state_global, i32, i128, RandomState, Global);
generate_tests!(i32_i128_random_state_wrapping, i32, i128, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_isize_random_state_global, i32, isize, RandomState, Global);
generate_tests!(
    i32_isize_random_state_wrapping,
    i32,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i32_u8_random_state_global, i32, u8, RandomState, Global);
generate_tests!(i32_u8_random_state_wrapping, i32, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_u16_random_state_global, i32, u16, RandomState, Global);
generate_tests!(i32_u16_random_state_wrapping, i32, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_u32_random_state_global, i32, u32, RandomState, Global);
generate_tests!(i32_u32_random_state_wrapping, i32, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_u64_random_state_global, i32, u64, RandomState, Global);
generate_tests!(i32_u64_random_state_wrapping, i32, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_u128_random_state_global, i32, u128, RandomState, Global);
generate_tests!(i32_u128_random_state_wrapping, i32, u128, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_usize_random_state_global, i32, usize, RandomState, Global);
generate_tests!(
    i32_usize_random_state_wrapping,
    i32,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i32_f32_random_state_global, i32, f32, RandomState, Global);
generate_tests!(i32_f32_random_state_wrapping, i32, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_f64_random_state_global, i32, f64, RandomState, Global);
generate_tests!(i32_f64_random_state_wrapping, i32, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_bool_random_state_global, i32, bool, RandomState, Global);
generate_tests!(i32_bool_random_state_wrapping, i32, bool, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_char_random_state_global, i32, char, RandomState, Global);
generate_tests!(i32_char_random_state_wrapping, i32, char, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_string_random_state_global, i32, String, RandomState, Global);
generate_tests!(
    i32_string_random_state_wrapping,
    i32,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i32_box_any_random_state_global, i32, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    i32_box_any_random_state_wrapping,
    i32,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(i64_i8_random_state_global, i64, i8, RandomState, Global);
generate_tests!(i64_i8_random_state_wrapping, i64, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_i16_random_state_global, i64, i16, RandomState, Global);
generate_tests!(i64_i16_random_state_wrapping, i64, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_i32_random_state_global, i64, i32, RandomState, Global);
generate_tests!(i64_i32_random_state_wrapping, i64, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_i64_random_state_global, i64, i64, RandomState, Global);
generate_tests!(i64_i64_random_state_wrapping, i64, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_i128_random_state_global, i64, i128, RandomState, Global);
generate_tests!(i64_i128_random_state_wrapping, i64, i128, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_isize_random_state_global, i64, isize, RandomState, Global);
generate_tests!(
    i64_isize_random_state_wrapping,
    i64,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i64_u8_random_state_global, i64, u8, RandomState, Global);
generate_tests!(i64_u8_random_state_wrapping, i64, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_u16_random_state_global, i64, u16, RandomState, Global);
generate_tests!(i64_u16_random_state_wrapping, i64, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_u32_random_state_global, i64, u32, RandomState, Global);
generate_tests!(i64_u32_random_state_wrapping, i64, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_u64_random_state_global, i64, u64, RandomState, Global);
generate_tests!(i64_u64_random_state_wrapping, i64, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_u128_random_state_global, i64, u128, RandomState, Global);
generate_tests!(i64_u128_random_state_wrapping, i64, u128, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_usize_random_state_global, i64, usize, RandomState, Global);
generate_tests!(
    i64_usize_random_state_wrapping,
    i64,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i64_f32_random_state_global, i64, f32, RandomState, Global);
generate_tests!(i64_f32_random_state_wrapping, i64, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_f64_random_state_global, i64, f64, RandomState, Global);
generate_tests!(i64_f64_random_state_wrapping, i64, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_bool_random_state_global, i64, bool, RandomState, Global);
generate_tests!(i64_bool_random_state_wrapping, i64, bool, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_char_random_state_global, i64, char, RandomState, Global);
generate_tests!(i64_char_random_state_wrapping, i64, char, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_string_random_state_global, i64, String, RandomState, Global);
generate_tests!(
    i64_string_random_state_wrapping,
    i64,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i64_box_any_random_state_global, i64, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    i64_box_any_random_state_wrapping,
    i64,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(i128_i8_random_state_global, i128, i8, RandomState, Global);
generate_tests!(i128_i8_random_state_wrapping, i128, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(i128_i16_random_state_global, i128, i16, RandomState, Global);
generate_tests!(i128_i16_random_state_wrapping, i128, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(i128_i32_random_state_global, i128, i32, RandomState, Global);
generate_tests!(i128_i32_random_state_wrapping, i128, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(i128_i64_random_state_global, i128, i64, RandomState, Global);
generate_tests!(i128_i64_random_state_wrapping, i128, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(i128_i128_random_state_global, i128, i128, RandomState, Global);
generate_tests!(
    i128_i128_random_state_wrapping,
    i128,
    i128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i128_isize_random_state_global, i128, isize, RandomState, Global);
generate_tests!(
    i128_isize_random_state_wrapping,
    i128,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i128_u8_random_state_global, i128, u8, RandomState, Global);
generate_tests!(i128_u8_random_state_wrapping, i128, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(i128_u16_random_state_global, i128, u16, RandomState, Global);
generate_tests!(i128_u16_random_state_wrapping, i128, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(i128_u32_random_state_global, i128, u32, RandomState, Global);
generate_tests!(i128_u32_random_state_wrapping, i128, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(i128_u64_random_state_global, i128, u64, RandomState, Global);
generate_tests!(i128_u64_random_state_wrapping, i128, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(i128_u128_random_state_global, i128, u128, RandomState, Global);
generate_tests!(
    i128_u128_random_state_wrapping,
    i128,
    u128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i128_usize_random_state_global, i128, usize, RandomState, Global);
generate_tests!(
    i128_usize_random_state_wrapping,
    i128,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i128_f32_random_state_global, i128, f32, RandomState, Global);
generate_tests!(i128_f32_random_state_wrapping, i128, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(i128_f64_random_state_global, i128, f64, RandomState, Global);
generate_tests!(i128_f64_random_state_wrapping, i128, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(i128_bool_random_state_global, i128, bool, RandomState, Global);
generate_tests!(
    i128_bool_random_state_wrapping,
    i128,
    bool,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i128_char_random_state_global, i128, char, RandomState, Global);
generate_tests!(
    i128_char_random_state_wrapping,
    i128,
    char,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i128_string_random_state_global, i128, String, RandomState, Global);
generate_tests!(
    i128_string_random_state_wrapping,
    i128,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(i128_box_any_random_state_global, i128, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    i128_box_any_random_state_wrapping,
    i128,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(isize_i8_random_state_global, isize, i8, RandomState, Global);
generate_tests!(isize_i8_random_state_wrapping, isize, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(isize_i16_random_state_global, isize, i16, RandomState, Global);
generate_tests!(
    isize_i16_random_state_wrapping,
    isize,
    i16,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_i32_random_state_global, isize, i32, RandomState, Global);
generate_tests!(
    isize_i32_random_state_wrapping,
    isize,
    i32,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_i64_random_state_global, isize, i64, RandomState, Global);
generate_tests!(
    isize_i64_random_state_wrapping,
    isize,
    i64,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_i128_random_state_global, isize, i128, RandomState, Global);
generate_tests!(
    isize_i128_random_state_wrapping,
    isize,
    i128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_isize_random_state_global, isize, isize, RandomState, Global);
generate_tests!(
    isize_isize_random_state_wrapping,
    isize,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_u8_random_state_global, isize, u8, RandomState, Global);
generate_tests!(isize_u8_random_state_wrapping, isize, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(isize_u16_random_state_global, isize, u16, RandomState, Global);
generate_tests!(
    isize_u16_random_state_wrapping,
    isize,
    u16,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_u32_random_state_global, isize, u32, RandomState, Global);
generate_tests!(
    isize_u32_random_state_wrapping,
    isize,
    u32,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_u64_random_state_global, isize, u64, RandomState, Global);
generate_tests!(
    isize_u64_random_state_wrapping,
    isize,
    u64,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_u128_random_state_global, isize, u128, RandomState, Global);
generate_tests!(
    isize_u128_random_state_wrapping,
    isize,
    u128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_usize_random_state_global, isize, usize, RandomState, Global);
generate_tests!(
    isize_usize_random_state_wrapping,
    isize,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_f32_random_state_global, isize, f32, RandomState, Global);
generate_tests!(
    isize_f32_random_state_wrapping,
    isize,
    f32,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_f64_random_state_global, isize, f64, RandomState, Global);
generate_tests!(
    isize_f64_random_state_wrapping,
    isize,
    f64,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_bool_random_state_global, isize, bool, RandomState, Global);
generate_tests!(
    isize_bool_random_state_wrapping,
    isize,
    bool,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_char_random_state_global, isize, char, RandomState, Global);
generate_tests!(
    isize_char_random_state_wrapping,
    isize,
    char,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(isize_string_random_state_global, isize, String, RandomState, Global);
generate_tests!(
    isize_string_random_state_wrapping,
    isize,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(
    isize_box_any_random_state_global,
    isize,
    Box<dyn any::Any>,
    RandomState,
    Global
);
generate_tests!(
    isize_box_any_random_state_wrapping,
    isize,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(u8_i8_random_state_global, u8, i8, RandomState, Global);
generate_tests!(u8_i8_random_state_wrapping, u8, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_i16_random_state_global, u8, i16, RandomState, Global);
generate_tests!(u8_i16_random_state_wrapping, u8, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_i32_random_state_global, u8, i32, RandomState, Global);
generate_tests!(u8_i32_random_state_wrapping, u8, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_i64_random_state_global, u8, i64, RandomState, Global);
generate_tests!(u8_i64_random_state_wrapping, u8, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_i128_random_state_global, u8, i128, RandomState, Global);
generate_tests!(u8_i128_random_state_wrapping, u8, i128, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_isize_random_state_global, u8, isize, RandomState, Global);
generate_tests!(u8_isize_random_state_wrapping, u8, isize, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_u8_random_state_global, u8, u8, RandomState, Global);
generate_tests!(u8_u8_random_state_wrapping, u8, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_u16_random_state_global, u8, u16, RandomState, Global);
generate_tests!(u8_u16_random_state_wrapping, u8, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_u32_random_state_global, u8, u32, RandomState, Global);
generate_tests!(u8_u32_random_state_wrapping, u8, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_u64_random_state_global, u8, u64, RandomState, Global);
generate_tests!(u8_u64_random_state_wrapping, u8, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_u128_random_state_global, u8, u128, RandomState, Global);
generate_tests!(u8_u128_random_state_wrapping, u8, u128, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_usize_random_state_global, u8, usize, RandomState, Global);
generate_tests!(u8_usize_random_state_wrapping, u8, usize, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_f32_random_state_global, u8, f32, RandomState, Global);
generate_tests!(u8_f32_random_state_wrapping, u8, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_f64_random_state_global, u8, f64, RandomState, Global);
generate_tests!(u8_f64_random_state_wrapping, u8, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_bool_random_state_global, u8, bool, RandomState, Global);
generate_tests!(u8_bool_random_state_wrapping, u8, bool, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_char_random_state_global, u8, char, RandomState, Global);
generate_tests!(u8_char_random_state_wrapping, u8, char, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_string_random_state_global, u8, String, RandomState, Global);
generate_tests!(
    u8_string_random_state_wrapping,
    u8,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u8_box_any_random_state_global, u8, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    u8_box_any_random_state_wrapping,
    u8,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(u16_i8_random_state_global, u16, i8, RandomState, Global);
generate_tests!(u16_i8_random_state_wrapping, u16, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_i16_random_state_global, u16, i16, RandomState, Global);
generate_tests!(u16_i16_random_state_wrapping, u16, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_i32_random_state_global, u16, i32, RandomState, Global);
generate_tests!(u16_i32_random_state_wrapping, u16, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_i64_random_state_global, u16, i64, RandomState, Global);
generate_tests!(u16_i64_random_state_wrapping, u16, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_i128_random_state_global, u16, i128, RandomState, Global);
generate_tests!(u16_i128_random_state_wrapping, u16, i128, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_isize_random_state_global, u16, isize, RandomState, Global);
generate_tests!(
    u16_isize_random_state_wrapping,
    u16,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u16_u8_random_state_global, u16, u8, RandomState, Global);
generate_tests!(u16_u8_random_state_wrapping, u16, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_u16_random_state_global, u16, u16, RandomState, Global);
generate_tests!(u16_u16_random_state_wrapping, u16, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_u32_random_state_global, u16, u32, RandomState, Global);
generate_tests!(u16_u32_random_state_wrapping, u16, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_u64_random_state_global, u16, u64, RandomState, Global);
generate_tests!(u16_u64_random_state_wrapping, u16, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_u128_random_state_global, u16, u128, RandomState, Global);
generate_tests!(u16_u128_random_state_wrapping, u16, u128, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_usize_random_state_global, u16, usize, RandomState, Global);
generate_tests!(
    u16_usize_random_state_wrapping,
    u16,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u16_f32_random_state_global, u16, f32, RandomState, Global);
generate_tests!(u16_f32_random_state_wrapping, u16, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_f64_random_state_global, u16, f64, RandomState, Global);
generate_tests!(u16_f64_random_state_wrapping, u16, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_bool_random_state_global, u16, bool, RandomState, Global);
generate_tests!(u16_bool_random_state_wrapping, u16, bool, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_char_random_state_global, u16, char, RandomState, Global);
generate_tests!(u16_char_random_state_wrapping, u16, char, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_string_random_state_global, u16, String, RandomState, Global);
generate_tests!(
    u16_string_random_state_wrapping,
    u16,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u16_box_any_random_state_global, u16, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    u16_box_any_random_state_wrapping,
    u16,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(u32_i8_random_state_global, u32, i8, RandomState, Global);
generate_tests!(u32_i8_random_state_wrapping, u32, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_i16_random_state_global, u32, i16, RandomState, Global);
generate_tests!(u32_i16_random_state_wrapping, u32, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_i32_random_state_global, u32, i32, RandomState, Global);
generate_tests!(u32_i32_random_state_wrapping, u32, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_i64_random_state_global, u32, i64, RandomState, Global);
generate_tests!(u32_i64_random_state_wrapping, u32, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_i128_random_state_global, u32, i128, RandomState, Global);
generate_tests!(u32_i128_random_state_wrapping, u32, i128, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_isize_random_state_global, u32, isize, RandomState, Global);
generate_tests!(
    u32_isize_random_state_wrapping,
    u32,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u32_u8_random_state_global, u32, u8, RandomState, Global);
generate_tests!(u32_u8_random_state_wrapping, u32, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_u16_random_state_global, u32, u16, RandomState, Global);
generate_tests!(u32_u16_random_state_wrapping, u32, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_u32_random_state_global, u32, u32, RandomState, Global);
generate_tests!(u32_u32_random_state_wrapping, u32, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_u64_random_state_global, u32, u64, RandomState, Global);
generate_tests!(u32_u64_random_state_wrapping, u32, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_u128_random_state_global, u32, u128, RandomState, Global);
generate_tests!(u32_u128_random_state_wrapping, u32, u128, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_usize_random_state_global, u32, usize, RandomState, Global);
generate_tests!(
    u32_usize_random_state_wrapping,
    u32,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u32_f32_random_state_global, u32, f32, RandomState, Global);
generate_tests!(u32_f32_random_state_wrapping, u32, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_f64_random_state_global, u32, f64, RandomState, Global);
generate_tests!(u32_f64_random_state_wrapping, u32, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_bool_random_state_global, u32, bool, RandomState, Global);
generate_tests!(u32_bool_random_state_wrapping, u32, bool, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_char_random_state_global, u32, char, RandomState, Global);
generate_tests!(u32_char_random_state_wrapping, u32, char, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_string_random_state_global, u32, String, RandomState, Global);
generate_tests!(
    u32_string_random_state_wrapping,
    u32,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u32_box_any_random_state_global, u32, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    u32_box_any_random_state_wrapping,
    u32,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(u64_i8_random_state_global, u64, i8, RandomState, Global);
generate_tests!(u64_i8_random_state_wrapping, u64, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_i16_random_state_global, u64, i16, RandomState, Global);
generate_tests!(u64_i16_random_state_wrapping, u64, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_i32_random_state_global, u64, i32, RandomState, Global);
generate_tests!(u64_i32_random_state_wrapping, u64, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_i64_random_state_global, u64, i64, RandomState, Global);
generate_tests!(u64_i64_random_state_wrapping, u64, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_i128_random_state_global, u64, i128, RandomState, Global);
generate_tests!(u64_i128_random_state_wrapping, u64, i128, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_isize_random_state_global, u64, isize, RandomState, Global);
generate_tests!(
    u64_isize_random_state_wrapping,
    u64,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u64_u8_random_state_global, u64, u8, RandomState, Global);
generate_tests!(u64_u8_random_state_wrapping, u64, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_u16_random_state_global, u64, u16, RandomState, Global);
generate_tests!(u64_u16_random_state_wrapping, u64, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_u32_random_state_global, u64, u32, RandomState, Global);
generate_tests!(u64_u32_random_state_wrapping, u64, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_u64_random_state_global, u64, u64, RandomState, Global);
generate_tests!(u64_u64_random_state_wrapping, u64, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_u128_random_state_global, u64, u128, RandomState, Global);
generate_tests!(u64_u128_random_state_wrapping, u64, u128, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_usize_random_state_global, u64, usize, RandomState, Global);
generate_tests!(
    u64_usize_random_state_wrapping,
    u64,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u64_f32_random_state_global, u64, f32, RandomState, Global);
generate_tests!(u64_f32_random_state_wrapping, u64, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_f64_random_state_global, u64, f64, RandomState, Global);
generate_tests!(u64_f64_random_state_wrapping, u64, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_bool_random_state_global, u64, bool, RandomState, Global);
generate_tests!(u64_bool_random_state_wrapping, u64, bool, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_char_random_state_global, u64, char, RandomState, Global);
generate_tests!(u64_char_random_state_wrapping, u64, char, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_string_random_state_global, u64, String, RandomState, Global);
generate_tests!(
    u64_string_random_state_wrapping,
    u64,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u64_box_any_random_state_global, u64, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    u64_box_any_random_state_wrapping,
    u64,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(u128_i8_random_state_global, u128, i8, RandomState, Global);
generate_tests!(u128_i8_random_state_wrapping, u128, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(u128_i16_random_state_global, u128, i16, RandomState, Global);
generate_tests!(u128_i16_random_state_wrapping, u128, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(u128_i32_random_state_global, u128, i32, RandomState, Global);
generate_tests!(u128_i32_random_state_wrapping, u128, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(u128_i64_random_state_global, u128, i64, RandomState, Global);
generate_tests!(u128_i64_random_state_wrapping, u128, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(u128_i128_random_state_global, u128, i128, RandomState, Global);
generate_tests!(
    u128_i128_random_state_wrapping,
    u128,
    i128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u128_isize_random_state_global, u128, isize, RandomState, Global);
generate_tests!(
    u128_isize_random_state_wrapping,
    u128,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u128_u8_random_state_global, u128, u8, RandomState, Global);
generate_tests!(u128_u8_random_state_wrapping, u128, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(u128_u16_random_state_global, u128, u16, RandomState, Global);
generate_tests!(u128_u16_random_state_wrapping, u128, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(u128_u32_random_state_global, u128, u32, RandomState, Global);
generate_tests!(u128_u32_random_state_wrapping, u128, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(u128_u64_random_state_global, u128, u64, RandomState, Global);
generate_tests!(u128_u64_random_state_wrapping, u128, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(u128_u128_random_state_global, u128, u128, RandomState, Global);
generate_tests!(
    u128_u128_random_state_wrapping,
    u128,
    u128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u128_usize_random_state_global, u128, usize, RandomState, Global);
generate_tests!(
    u128_usize_random_state_wrapping,
    u128,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u128_f32_random_state_global, u128, f32, RandomState, Global);
generate_tests!(u128_f32_random_state_wrapping, u128, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(u128_f64_random_state_global, u128, f64, RandomState, Global);
generate_tests!(u128_f64_random_state_wrapping, u128, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(u128_bool_random_state_global, u128, bool, RandomState, Global);
generate_tests!(
    u128_bool_random_state_wrapping,
    u128,
    bool,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u128_char_random_state_global, u128, char, RandomState, Global);
generate_tests!(
    u128_char_random_state_wrapping,
    u128,
    char,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u128_string_random_state_global, u128, String, RandomState, Global);
generate_tests!(
    u128_string_random_state_wrapping,
    u128,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(u128_box_any_random_state_global, u128, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    u128_box_any_random_state_wrapping,
    u128,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(usize_i8_random_state_global, usize, i8, RandomState, Global);
generate_tests!(usize_i8_random_state_wrapping, usize, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(usize_i16_random_state_global, usize, i16, RandomState, Global);
generate_tests!(
    usize_i16_random_state_wrapping,
    usize,
    i16,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_i32_random_state_global, usize, i32, RandomState, Global);
generate_tests!(
    usize_i32_random_state_wrapping,
    usize,
    i32,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_i64_random_state_global, usize, i64, RandomState, Global);
generate_tests!(
    usize_i64_random_state_wrapping,
    usize,
    i64,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_i128_random_state_global, usize, i128, RandomState, Global);
generate_tests!(
    usize_i128_random_state_wrapping,
    usize,
    i128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_isize_random_state_global, usize, isize, RandomState, Global);
generate_tests!(
    usize_isize_random_state_wrapping,
    usize,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_u8_random_state_global, usize, u8, RandomState, Global);
generate_tests!(usize_u8_random_state_wrapping, usize, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(usize_u16_random_state_global, usize, u16, RandomState, Global);
generate_tests!(
    usize_u16_random_state_wrapping,
    usize,
    u16,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_u32_random_state_global, usize, u32, RandomState, Global);
generate_tests!(
    usize_u32_random_state_wrapping,
    usize,
    u32,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_u64_random_state_global, usize, u64, RandomState, Global);
generate_tests!(
    usize_u64_random_state_wrapping,
    usize,
    u64,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_u128_random_state_global, usize, u128, RandomState, Global);
generate_tests!(
    usize_u128_random_state_wrapping,
    usize,
    u128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_usize_random_state_global, usize, usize, RandomState, Global);
generate_tests!(
    usize_usize_random_state_wrapping,
    usize,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_f32_random_state_global, usize, f32, RandomState, Global);
generate_tests!(
    usize_f32_random_state_wrapping,
    usize,
    f32,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_f64_random_state_global, usize, f64, RandomState, Global);
generate_tests!(
    usize_f64_random_state_wrapping,
    usize,
    f64,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_bool_random_state_global, usize, bool, RandomState, Global);
generate_tests!(
    usize_bool_random_state_wrapping,
    usize,
    bool,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_char_random_state_global, usize, char, RandomState, Global);
generate_tests!(
    usize_char_random_state_wrapping,
    usize,
    char,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(usize_string_random_state_global, usize, String, RandomState, Global);
generate_tests!(
    usize_string_random_state_wrapping,
    usize,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(
    usize_box_any_random_state_global,
    usize,
    Box<dyn any::Any>,
    RandomState,
    Global
);
generate_tests!(
    usize_box_any_random_state_wrapping,
    usize,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(f32_i8_random_state_global, f32, i8, RandomState, Global);
generate_tests!(f32_i8_random_state_wrapping, f32, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_i16_random_state_global, f32, i16, RandomState, Global);
generate_tests!(f32_i16_random_state_wrapping, f32, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_i32_random_state_global, f32, i32, RandomState, Global);
generate_tests!(f32_i32_random_state_wrapping, f32, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_i64_random_state_global, f32, i64, RandomState, Global);
generate_tests!(f32_i64_random_state_wrapping, f32, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_i128_random_state_global, f32, i128, RandomState, Global);
generate_tests!(f32_i128_random_state_wrapping, f32, i128, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_isize_random_state_global, f32, isize, RandomState, Global);
generate_tests!(
    f32_isize_random_state_wrapping,
    f32,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(f32_u8_random_state_global, f32, u8, RandomState, Global);
generate_tests!(f32_u8_random_state_wrapping, f32, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_u16_random_state_global, f32, u16, RandomState, Global);
generate_tests!(f32_u16_random_state_wrapping, f32, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_u32_random_state_global, f32, u32, RandomState, Global);
generate_tests!(f32_u32_random_state_wrapping, f32, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_u64_random_state_global, f32, u64, RandomState, Global);
generate_tests!(f32_u64_random_state_wrapping, f32, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_u128_random_state_global, f32, u128, RandomState, Global);
generate_tests!(f32_u128_random_state_wrapping, f32, u128, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_usize_random_state_global, f32, usize, RandomState, Global);
generate_tests!(
    f32_usize_random_state_wrapping,
    f32,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(f32_f32_random_state_global, f32, f32, RandomState, Global);
generate_tests!(f32_f32_random_state_wrapping, f32, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_f64_random_state_global, f32, f64, RandomState, Global);
generate_tests!(f32_f64_random_state_wrapping, f32, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_bool_random_state_global, f32, bool, RandomState, Global);
generate_tests!(f32_bool_random_state_wrapping, f32, bool, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_char_random_state_global, f32, char, RandomState, Global);
generate_tests!(f32_char_random_state_wrapping, f32, char, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_string_random_state_global, f32, String, RandomState, Global);
generate_tests!(
    f32_string_random_state_wrapping,
    f32,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(f32_box_any_random_state_global, f32, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    f32_box_any_random_state_wrapping,
    f32,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(f64_i8_random_state_global, f64, i8, RandomState, Global);
generate_tests!(f64_i8_random_state_wrapping, f64, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_i16_random_state_global, f64, i16, RandomState, Global);
generate_tests!(f64_i16_random_state_wrapping, f64, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_i32_random_state_global, f64, i32, RandomState, Global);
generate_tests!(f64_i32_random_state_wrapping, f64, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_i64_random_state_global, f64, i64, RandomState, Global);
generate_tests!(f64_i64_random_state_wrapping, f64, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_i128_random_state_global, f64, i128, RandomState, Global);
generate_tests!(f64_i128_random_state_wrapping, f64, i128, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_isize_random_state_global, f64, isize, RandomState, Global);
generate_tests!(
    f64_isize_random_state_wrapping,
    f64,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(f64_u8_random_state_global, f64, u8, RandomState, Global);
generate_tests!(f64_u8_random_state_wrapping, f64, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_u16_random_state_global, f64, u16, RandomState, Global);
generate_tests!(f64_u16_random_state_wrapping, f64, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_u32_random_state_global, f64, u32, RandomState, Global);
generate_tests!(f64_u32_random_state_wrapping, f64, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_u64_random_state_global, f64, u64, RandomState, Global);
generate_tests!(f64_u64_random_state_wrapping, f64, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_u128_random_state_global, f64, u128, RandomState, Global);
generate_tests!(f64_u128_random_state_wrapping, f64, u128, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_usize_random_state_global, f64, usize, RandomState, Global);
generate_tests!(
    f64_usize_random_state_wrapping,
    f64,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(f64_f32_random_state_global, f64, f32, RandomState, Global);
generate_tests!(f64_f32_random_state_wrapping, f64, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_f64_random_state_global, f64, f64, RandomState, Global);
generate_tests!(f64_f64_random_state_wrapping, f64, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_bool_random_state_global, f64, bool, RandomState, Global);
generate_tests!(f64_bool_random_state_wrapping, f64, bool, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_char_random_state_global, f64, char, RandomState, Global);
generate_tests!(f64_char_random_state_wrapping, f64, char, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_string_random_state_global, f64, String, RandomState, Global);
generate_tests!(
    f64_string_random_state_wrapping,
    f64,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(f64_box_any_random_state_global, f64, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    f64_box_any_random_state_wrapping,
    f64,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(bool_i8_random_state_global, bool, i8, RandomState, Global);
generate_tests!(bool_i8_random_state_wrapping, bool, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(bool_i16_random_state_global, bool, i16, RandomState, Global);
generate_tests!(bool_i16_random_state_wrapping, bool, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(bool_i32_random_state_global, bool, i32, RandomState, Global);
generate_tests!(bool_i32_random_state_wrapping, bool, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(bool_i64_random_state_global, bool, i64, RandomState, Global);
generate_tests!(bool_i64_random_state_wrapping, bool, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(bool_i128_random_state_global, bool, i128, RandomState, Global);
generate_tests!(
    bool_i128_random_state_wrapping,
    bool,
    i128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(bool_isize_random_state_global, bool, isize, RandomState, Global);
generate_tests!(
    bool_isize_random_state_wrapping,
    bool,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(bool_u8_random_state_global, bool, u8, RandomState, Global);
generate_tests!(bool_u8_random_state_wrapping, bool, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(bool_u16_random_state_global, bool, u16, RandomState, Global);
generate_tests!(bool_u16_random_state_wrapping, bool, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(bool_u32_random_state_global, bool, u32, RandomState, Global);
generate_tests!(bool_u32_random_state_wrapping, bool, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(bool_u64_random_state_global, bool, u64, RandomState, Global);
generate_tests!(bool_u64_random_state_wrapping, bool, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(bool_u128_random_state_global, bool, u128, RandomState, Global);
generate_tests!(
    bool_u128_random_state_wrapping,
    bool,
    u128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(bool_usize_random_state_global, bool, usize, RandomState, Global);
generate_tests!(
    bool_usize_random_state_wrapping,
    bool,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(bool_f32_random_state_global, bool, f32, RandomState, Global);
generate_tests!(bool_f32_random_state_wrapping, bool, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(bool_f64_random_state_global, bool, f64, RandomState, Global);
generate_tests!(bool_f64_random_state_wrapping, bool, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(bool_bool_random_state_global, bool, bool, RandomState, Global);
generate_tests!(
    bool_bool_random_state_wrapping,
    bool,
    bool,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(bool_char_random_state_global, bool, char, RandomState, Global);
generate_tests!(
    bool_char_random_state_wrapping,
    bool,
    char,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(bool_string_random_state_global, bool, String, RandomState, Global);
generate_tests!(
    bool_string_random_state_wrapping,
    bool,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(bool_box_any_random_state_global, bool, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    bool_box_any_random_state_wrapping,
    bool,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(char_i8_random_state_global, char, i8, RandomState, Global);
generate_tests!(char_i8_random_state_wrapping, char, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(char_i16_random_state_global, char, i16, RandomState, Global);
generate_tests!(char_i16_random_state_wrapping, char, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(char_i32_random_state_global, char, i32, RandomState, Global);
generate_tests!(char_i32_random_state_wrapping, char, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(char_i64_random_state_global, char, i64, RandomState, Global);
generate_tests!(char_i64_random_state_wrapping, char, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(char_i128_random_state_global, char, i128, RandomState, Global);
generate_tests!(
    char_i128_random_state_wrapping,
    char,
    i128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(char_isize_random_state_global, char, isize, RandomState, Global);
generate_tests!(
    char_isize_random_state_wrapping,
    char,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(char_u8_random_state_global, char, u8, RandomState, Global);
generate_tests!(char_u8_random_state_wrapping, char, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(char_u16_random_state_global, char, u16, RandomState, Global);
generate_tests!(char_u16_random_state_wrapping, char, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(char_u32_random_state_global, char, u32, RandomState, Global);
generate_tests!(char_u32_random_state_wrapping, char, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(char_u64_random_state_global, char, u64, RandomState, Global);
generate_tests!(char_u64_random_state_wrapping, char, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(char_u128_random_state_global, char, u128, RandomState, Global);
generate_tests!(
    char_u128_random_state_wrapping,
    char,
    u128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(char_usize_random_state_global, char, usize, RandomState, Global);
generate_tests!(
    char_usize_random_state_wrapping,
    char,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(char_f32_random_state_global, char, f32, RandomState, Global);
generate_tests!(char_f32_random_state_wrapping, char, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(char_f64_random_state_global, char, f64, RandomState, Global);
generate_tests!(char_f64_random_state_wrapping, char, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(char_bool_random_state_global, char, bool, RandomState, Global);
generate_tests!(
    char_bool_random_state_wrapping,
    char,
    bool,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(char_char_random_state_global, char, char, RandomState, Global);
generate_tests!(
    char_char_random_state_wrapping,
    char,
    char,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(char_string_random_state_global, char, String, RandomState, Global);
generate_tests!(
    char_string_random_state_wrapping,
    char,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(char_box_any_random_state_global, char, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    char_box_any_random_state_wrapping,
    char,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);

generate_tests!(string_i8_random_state_global, String, i8, RandomState, Global);
generate_tests!(
    string_i8_random_state_wrapping,
    String,
    i8,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_i16_random_state_global, String, i16, RandomState, Global);
generate_tests!(
    string_i16_random_state_wrapping,
    String,
    i16,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_i32_random_state_global, String, i32, RandomState, Global);
generate_tests!(
    string_i32_random_state_wrapping,
    String,
    i32,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_i64_random_state_global, String, i64, RandomState, Global);
generate_tests!(
    string_i64_random_state_wrapping,
    String,
    i64,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_i128_random_state_global, String, i128, RandomState, Global);
generate_tests!(
    string_i128_random_state_wrapping,
    String,
    i128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_isize_random_state_global, String, isize, RandomState, Global);
generate_tests!(
    string_isize_random_state_wrapping,
    String,
    isize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_u8_random_state_global, String, u8, RandomState, Global);
generate_tests!(
    string_u8_random_state_wrapping,
    String,
    u8,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_u16_random_state_global, String, u16, RandomState, Global);
generate_tests!(
    string_u16_random_state_wrapping,
    String,
    u16,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_u32_random_state_global, String, u32, RandomState, Global);
generate_tests!(
    string_u32_random_state_wrapping,
    String,
    u32,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_u64_random_state_global, String, u64, RandomState, Global);
generate_tests!(
    string_u64_random_state_wrapping,
    String,
    u64,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_u128_random_state_global, String, u128, RandomState, Global);
generate_tests!(
    string_u128_random_state_wrapping,
    String,
    u128,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_usize_random_state_global, String, usize, RandomState, Global);
generate_tests!(
    string_usize_random_state_wrapping,
    String,
    usize,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_f32_random_state_global, String, f32, RandomState, Global);
generate_tests!(
    string_f32_random_state_wrapping,
    String,
    f32,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_f64_random_state_global, String, f64, RandomState, Global);
generate_tests!(
    string_f64_random_state_wrapping,
    String,
    f64,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_bool_random_state_global, String, bool, RandomState, Global);
generate_tests!(
    string_bool_random_state_wrapping,
    String,
    bool,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_char_random_state_global, String, char, RandomState, Global);
generate_tests!(
    string_char_random_state_wrapping,
    String,
    char,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(string_string_random_state_global, String, String, RandomState, Global);
generate_tests!(
    string_string_random_state_wrapping,
    String,
    String,
    RandomState,
    WrappingAlloc<Global>
);
generate_tests!(
    string_box_any_random_state_global,
    String,
    Box<dyn any::Any>,
    RandomState,
    Global
);
generate_tests!(
    string_box_any_random_state_wrapping,
    String,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);
