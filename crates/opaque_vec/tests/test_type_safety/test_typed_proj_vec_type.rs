use opaque_vec::{OpaqueVec, TypedProjVec};

use std::{alloc, any};
use std::alloc::{Global, System};

fn run_test_typed_proj_vec_new_in_has_type<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_vec = TypedProjVec::<T, A>::new_in(alloc);
    let opaque_vec = OpaqueVec::from_proj(proj_vec);

    assert!(opaque_vec.has_element_type::<T>());
    assert!(opaque_vec.has_allocator_type::<A>());
}

fn run_test_typed_proj_vec_with_capacity_in_has_type<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync,
{
    let proj_vec = TypedProjVec::<T, A>::with_capacity_in(1024, alloc);
    let opaque_vec = OpaqueVec::from_proj(proj_vec);

    assert!(opaque_vec.has_element_type::<T>());
    assert!(opaque_vec.has_allocator_type::<A>());
}

fn run_test_typed_proj_vec_new_has_type<T>()
where
    T: any::Any,
{
    let proj_vec = TypedProjVec::<T, Global>::new();
    let opaque_vec = OpaqueVec::from_proj(proj_vec);

    assert!(opaque_vec.has_element_type::<T>());
    assert!(opaque_vec.has_allocator_type::<Global>());
}

fn run_test_typed_proj_vec_with_capacity_has_type<T>()
where
    T: any::Any,
{
    let proj_vec = TypedProjVec::<T, Global>::with_capacity(1024);
    let opaque_vec = OpaqueVec::from_proj(proj_vec);

    assert!(opaque_vec.has_element_type::<T>());
    assert!(opaque_vec.has_allocator_type::<Global>());
}

macro_rules! generate_tests {
    ($module_name:ident, $element_typ:ty, $alloc_typ:ty) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_vec_new_in_has_type() {
                let alloc: $alloc_typ = Default::default();
                run_test_typed_proj_vec_new_in_has_type::<$element_typ, $alloc_typ>(alloc);
            }

            #[test]
            fn test_typed_proj_vec_with_capacity_in_has_type() {
                let alloc: $alloc_typ = Default::default();
                run_test_typed_proj_vec_with_capacity_in_has_type::<$element_typ, $alloc_typ>(alloc);
            }

            #[test]
            fn test_typed_proj_vec_new_has_type() {
                run_test_typed_proj_vec_new_has_type::<$element_typ>();
            }

            #[test]
            fn test_typed_proj_vec_with_capacity_has_type() {
                run_test_typed_proj_vec_with_capacity_has_type::<$element_typ>();
            }
        }
    };
}

generate_tests!(i8_global, i8, Global);
generate_tests!(i8_system, i8, System);
generate_tests!(i16_global, i16, Global);
generate_tests!(i16_system, i16, System);
generate_tests!(i32_global, i32, Global);
generate_tests!(i32_system, i32, System);
generate_tests!(i64_global, i64, Global);
generate_tests!(i64_system, i64, System);
generate_tests!(i128_global, i128, Global);
generate_tests!(i128_system, i128, System);
generate_tests!(isize_global, isize, Global);
generate_tests!(isize_system, isize, System);

generate_tests!(u8_global, u8, Global);
generate_tests!(u8_system, u8, System);
generate_tests!(u16_global, u16, Global);
generate_tests!(u16_system, u16, System);
generate_tests!(u32_global, u32, Global);
generate_tests!(u32_system, u32, System);
generate_tests!(u64_global, u64, Global);
generate_tests!(u64_system, u64, System);
generate_tests!(u128_global, u128, Global);
generate_tests!(u128_system, u128, System);
generate_tests!(usize_global, usize, Global);
generate_tests!(usize_system, usize, System);

generate_tests!(f32_global, f32, Global);
generate_tests!(f32_system, f32, System);
generate_tests!(f64_global, f64, Global);
generate_tests!(f64_system, f64, System);

generate_tests!(bool_global, bool, Global);
generate_tests!(bool_system, bool, System);

generate_tests!(char_global, char, Global);
generate_tests!(char_system, char, System);

generate_tests!(string_global, String, Global);
generate_tests!(string_system, String, System);

generate_tests!(box_any_global, Box<dyn any::Any>, Global);
generate_tests!(box_any_system, Box<dyn any::Any>, System);
