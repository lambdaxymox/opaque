use opaque_index_map::set::{
    TypeErasedIndexSet,
    TypeProjectedIndexSet,
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

fn run_test_type_erased_index_set_with_hasher_in_has_type<T, S, A>(build_hasher: S, alloc: A)
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_set = TypeProjectedIndexSet::<T, S, A>::with_hasher_in(build_hasher, alloc);
    let opaque_set = TypeErasedIndexSet::from_proj(proj_set);

    assert!(opaque_set.has_value_type::<T>());
    assert!(opaque_set.has_build_hasher_type::<S>());
    assert!(opaque_set.has_allocator_type::<A>());
}

fn run_test_type_erased_index_set_with_capacity_and_hasher_in_has_type<T, S, A>(build_hasher: S, alloc: A)
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_set = TypeProjectedIndexSet::<T, S, A>::with_capacity_and_hasher_in(1024, build_hasher, alloc);
    let opaque_set = TypeErasedIndexSet::from_proj(proj_set);

    assert!(opaque_set.has_value_type::<T>());
    assert!(opaque_set.has_build_hasher_type::<S>());
    assert!(opaque_set.has_allocator_type::<A>());
}

fn run_test_type_erased_index_set_new_in_has_type<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_set = TypeProjectedIndexSet::<T, _, A>::new_in(alloc);
    let opaque_set = TypeErasedIndexSet::from_proj(proj_set);

    assert!(opaque_set.has_value_type::<T>());
    assert!(opaque_set.has_build_hasher_type::<hash::RandomState>());
    assert!(opaque_set.has_allocator_type::<A>());
}

fn run_test_type_erased_index_set_with_capacity_in_has_type<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_set = TypeProjectedIndexSet::<T, _, A>::with_capacity_in(1024, alloc);
    let opaque_set = TypeErasedIndexSet::from_proj(proj_set);

    assert!(opaque_set.has_value_type::<T>());
    assert!(opaque_set.has_build_hasher_type::<hash::RandomState>());
    assert!(opaque_set.has_allocator_type::<A>());
}

fn run_test_type_erased_index_set_with_hasher_has_type<T, S>(build_hasher: S)
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    let proj_set = TypeProjectedIndexSet::<T, S, _>::with_hasher(build_hasher);
    let opaque_set = TypeErasedIndexSet::from_proj(proj_set);

    assert!(opaque_set.has_value_type::<T>());
    assert!(opaque_set.has_build_hasher_type::<S>());
    assert!(opaque_set.has_allocator_type::<alloc::Global>());
}

fn run_test_type_erased_index_set_with_capacity_and_hasher_has_type<T, S>(build_hasher: S)
where
    T: any::Any,
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    let proj_set = TypeProjectedIndexSet::<T, S, _>::with_capacity_and_hasher(1024, build_hasher);
    let opaque_set = TypeErasedIndexSet::from_proj(proj_set);

    assert!(opaque_set.has_value_type::<T>());
    assert!(opaque_set.has_build_hasher_type::<S>());
    assert!(opaque_set.has_allocator_type::<alloc::Global>());
}

fn run_test_type_erased_index_set_new_has_type<T>()
where
    T: any::Any,
{
    let proj_set = TypeProjectedIndexSet::<T, _, _>::new();
    let opaque_set = TypeErasedIndexSet::from_proj(proj_set);

    assert!(opaque_set.has_value_type::<T>());
    assert!(opaque_set.has_build_hasher_type::<hash::RandomState>());
    assert!(opaque_set.has_allocator_type::<Global>());
}

fn run_test_type_erased_index_set_with_capacity_has_type<T>()
where
    T: any::Any,
{
    let proj_set = TypeProjectedIndexSet::<T, _, _>::with_capacity(1024);
    let opaque_set = TypeErasedIndexSet::from_proj(proj_set);

    assert!(opaque_set.has_value_type::<T>());
    assert!(opaque_set.has_build_hasher_type::<hash::RandomState>());
    assert!(opaque_set.has_allocator_type::<Global>());
}

macro_rules! generate_tests {
    ($module_name:ident, $value_typ:ty, $build_hasher_typ:ty, $alloc_typ:ty) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_type_erased_index_set_with_hasher_in_has_type() {
                let build_hasher: $build_hasher_typ = Default::default();
                let alloc: $alloc_typ = Default::default();
                run_test_type_erased_index_set_with_hasher_in_has_type::<$value_typ, $build_hasher_typ, $alloc_typ>(
                    build_hasher,
                    alloc,
                );
            }

            #[test]
            fn test_type_erased_index_set_with_capacity_and_hasher_in_has_type() {
                let build_hasher: $build_hasher_typ = Default::default();
                let alloc: $alloc_typ = Default::default();
                run_test_type_erased_index_set_with_capacity_and_hasher_in_has_type::<$value_typ, $build_hasher_typ, $alloc_typ>(
                    build_hasher,
                    alloc,
                );
            }

            #[test]
            fn test_type_erased_index_set_new_in_has_type() {
                let alloc: $alloc_typ = Default::default();
                run_test_type_erased_index_set_new_in_has_type::<$value_typ, $alloc_typ>(alloc);
            }

            #[test]
            fn test_type_erased_index_set_with_capacity_in_has_type() {
                let alloc: $alloc_typ = Default::default();
                run_test_type_erased_index_set_with_capacity_in_has_type::<$value_typ, $alloc_typ>(alloc);
            }

            #[test]
            fn test_type_erased_index_set_with_hasher_has_type() {
                let build_hasher: $build_hasher_typ = Default::default();
                run_test_type_erased_index_set_with_hasher_has_type::<$value_typ, $build_hasher_typ>(build_hasher);
            }

            #[test]
            fn test_type_erased_index_set_with_capacity_and_hasher_has_type() {
                let build_hasher: $build_hasher_typ = Default::default();
                run_test_type_erased_index_set_with_capacity_and_hasher_has_type::<$value_typ, $build_hasher_typ>(build_hasher);
            }
            #[test]
            fn test_type_erased_index_set_new_has_type() {
                run_test_type_erased_index_set_new_has_type::<$value_typ>();
            }

            #[test]
            fn test_type_erased_index_set_with_capacity_has_type() {
                run_test_type_erased_index_set_with_capacity_has_type::<$value_typ>();
            }
        }
    };
}

generate_tests!(i8_random_state_global, i8, RandomState, Global);
generate_tests!(i8_random_state_wrapping, i8, RandomState, WrappingAlloc<Global>);
generate_tests!(i16_random_state_global, i16, RandomState, Global);
generate_tests!(i16_random_state_wrapping, i16, RandomState, WrappingAlloc<Global>);
generate_tests!(i32_random_state_global, i32, RandomState, Global);
generate_tests!(i32_random_state_wrapping, i32, RandomState, WrappingAlloc<Global>);
generate_tests!(i64_random_state_global, i64, RandomState, Global);
generate_tests!(i64_random_state_wrapping, i64, RandomState, WrappingAlloc<Global>);
generate_tests!(i128_random_state_global, i128, RandomState, Global);
generate_tests!(i128_random_state_wrapping, i128, RandomState, WrappingAlloc<Global>);
generate_tests!(isize_random_state_global, isize, RandomState, Global);
generate_tests!(isize_random_state_wrapping, isize, RandomState, WrappingAlloc<Global>);
generate_tests!(u8_random_state_global, u8, RandomState, Global);
generate_tests!(u8_random_state_wrapping, u8, RandomState, WrappingAlloc<Global>);
generate_tests!(u16_random_state_global, u16, RandomState, Global);
generate_tests!(u16_random_state_wrapping, u16, RandomState, WrappingAlloc<Global>);
generate_tests!(u32_random_state_global, u32, RandomState, Global);
generate_tests!(u32_random_state_wrapping, u32, RandomState, WrappingAlloc<Global>);
generate_tests!(u64_random_state_global, u64, RandomState, Global);
generate_tests!(u64_random_state_wrapping, u64, RandomState, WrappingAlloc<Global>);
generate_tests!(u128_random_state_global, u128, RandomState, Global);
generate_tests!(u128_random_state_wrapping, u128, RandomState, WrappingAlloc<Global>);
generate_tests!(usize_random_state_global, usize, RandomState, Global);
generate_tests!(usize_random_state_wrapping, usize, RandomState, WrappingAlloc<Global>);
generate_tests!(f32_random_state_global, f32, RandomState, Global);
generate_tests!(f32_random_state_wrapping, f32, RandomState, WrappingAlloc<Global>);
generate_tests!(f64_random_state_global, f64, RandomState, Global);
generate_tests!(f64_random_state_wrapping, f64, RandomState, WrappingAlloc<Global>);
generate_tests!(bool_random_state_global, bool, RandomState, Global);
generate_tests!(bool_random_state_wrapping, bool, RandomState, WrappingAlloc<Global>);
generate_tests!(char_random_state_global, char, RandomState, Global);
generate_tests!(char_random_state_wrapping, char, RandomState, WrappingAlloc<Global>);
generate_tests!(string_random_state_global, String, RandomState, Global);
generate_tests!(string_random_state_wrapping, String, RandomState, WrappingAlloc<Global>);
generate_tests!(box_any_random_state_global, Box<dyn any::Any>, RandomState, Global);
generate_tests!(
    box_any_random_state_wrapping,
    Box<dyn any::Any>,
    RandomState,
    WrappingAlloc<Global>
);
