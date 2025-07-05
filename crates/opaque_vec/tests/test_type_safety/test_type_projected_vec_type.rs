use opaque_vec::{TypeErasedVec, TypeProjectedVec};

use core::any;
use core::ptr::NonNull;
use std::boxed::Box;
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
        Self { alloc, }
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
        unsafe {
            self.alloc.deallocate(ptr, layout)
        }
    }
}

fn run_test_type_projected_vec_new_in_has_type<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_vec = TypeProjectedVec::<T, A>::new_in(alloc);
    let opaque_vec = TypeErasedVec::from_proj(proj_vec);

    assert!(opaque_vec.has_element_type::<T>());
    assert!(opaque_vec.has_allocator_type::<A>());
}

fn run_test_type_projected_vec_with_capacity_in_has_type<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_vec = TypeProjectedVec::<T, A>::with_capacity_in(1024, alloc);
    let opaque_vec = TypeErasedVec::from_proj(proj_vec);

    assert!(opaque_vec.has_element_type::<T>());
    assert!(opaque_vec.has_allocator_type::<A>());
}

fn run_test_type_projected_vec_new_has_type<T>()
where
    T: any::Any,
{
    let proj_vec = TypeProjectedVec::<T, Global>::new();
    let opaque_vec = TypeErasedVec::from_proj(proj_vec);

    assert!(opaque_vec.has_element_type::<T>());
    assert!(opaque_vec.has_allocator_type::<Global>());
}

fn run_test_type_projected_vec_with_capacity_has_type<T>()
where
    T: any::Any,
{
    let proj_vec = TypeProjectedVec::<T, Global>::with_capacity(1024);
    let opaque_vec = TypeErasedVec::from_proj(proj_vec);

    assert!(opaque_vec.has_element_type::<T>());
    assert!(opaque_vec.has_allocator_type::<Global>());
}

macro_rules! generate_tests {
    ($module_name:ident, $element_typ:ty, $alloc_typ:ty) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_type_projected_vec_new_in_has_type() {
                let alloc: $alloc_typ = Default::default();
                run_test_type_projected_vec_new_in_has_type::<$element_typ, $alloc_typ>(alloc);
            }

            #[test]
            fn test_type_projected_vec_with_capacity_in_has_type() {
                let alloc: $alloc_typ = Default::default();
                run_test_type_projected_vec_with_capacity_in_has_type::<$element_typ, $alloc_typ>(alloc);
            }

            #[test]
            fn test_type_projected_vec_new_has_type() {
                run_test_type_projected_vec_new_has_type::<$element_typ>();
            }

            #[test]
            fn test_type_projected_vec_with_capacity_has_type() {
                run_test_type_projected_vec_with_capacity_has_type::<$element_typ>();
            }
        }
    };
}

generate_tests!(i8_global, i8, Global);
generate_tests!(i8_wrapping, i8, WrappingAlloc<Global>);
generate_tests!(i16_global, i16, Global);
generate_tests!(i16_wrapping, i16, WrappingAlloc<Global>);
generate_tests!(i32_global, i32, Global);
generate_tests!(i32_wrapping, i32, WrappingAlloc<Global>);
generate_tests!(i64_global, i64, Global);
generate_tests!(i64_wrapping, i64, WrappingAlloc<Global>);
generate_tests!(i128_global, i128, Global);
generate_tests!(i128_wrapping, i128, WrappingAlloc<Global>);
generate_tests!(isize_global, isize, Global);
generate_tests!(isize_wrapping, isize, WrappingAlloc<Global>);

generate_tests!(u8_global, u8, Global);
generate_tests!(u8_wrapping, u8, WrappingAlloc<Global>);
generate_tests!(u16_global, u16, Global);
generate_tests!(u16_wrapping, u16, WrappingAlloc<Global>);
generate_tests!(u32_global, u32, Global);
generate_tests!(u32_wrapping, u32, WrappingAlloc<Global>);
generate_tests!(u64_global, u64, Global);
generate_tests!(u64_wrapping, u64, WrappingAlloc<Global>);
generate_tests!(u128_global, u128, Global);
generate_tests!(u128_wrapping, u128, WrappingAlloc<Global>);
generate_tests!(usize_global, usize, Global);
generate_tests!(usize_wrapping, usize, WrappingAlloc<Global>);

generate_tests!(f32_global, f32, Global);
generate_tests!(f32_wrapping, f32, WrappingAlloc<Global>);
generate_tests!(f64_global, f64, Global);
generate_tests!(f64_wrapping, f64, WrappingAlloc<Global>);

generate_tests!(bool_global, bool, Global);
generate_tests!(bool_wrapping, bool, WrappingAlloc<Global>);

generate_tests!(char_global, char, Global);
generate_tests!(char_wrapping, char, WrappingAlloc<Global>);

generate_tests!(string_global, String, Global);
generate_tests!(string_wrapping, String, WrappingAlloc<Global>);

generate_tests!(box_any_global, Box<dyn any::Any>, Global);
generate_tests!(box_any_wrapping, Box<dyn any::Any>, WrappingAlloc<Global>);
