use crate::common::erased::strategy_type_erased_vec_max_len;
use opaque_vec::TypeErasedVec;

use core::any;
use core::fmt;
use core::mem;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn prop_clone_as_slice<T, A>(values: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let cloned_values = values.clone::<T, A>();

    let expected = values.as_slice::<T, A>();
    let result = cloned_values.as_slice::<T, A>();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_clone_occupy_disjoint_memory_locations<T, A>(vec: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let cloned_values = vec.clone::<T, A>();

    let disjoint_if_non_empty_and_non_zst = if (vec.len() != 0) && (mem::size_of::<T>() != 0) {
        vec.as_ptr::<T, A>() != cloned_values.as_ptr::<T, A>()
    } else {
        true
    };

    prop_assert!(disjoint_if_non_empty_and_non_zst);

    Ok(())
}

fn prop_clone_occupy_disjoint_memory_regions<T, A>(vec: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let cloned_values = vec.clone::<T, A>();

    let ptr_start1 = vec.as_ptr::<T, A>() as usize;
    let ptr_start2 = cloned_values.as_ptr::<T, A>() as usize;
    let ptr_end1 = {
        let len1 = vec.len() * mem::size_of::<T>();
        ptr_start1 + len1
    };
    let ptr_end2 = {
        let len2 = cloned_values.len() * mem::size_of::<T>();
        ptr_start2 + len2
    };

    prop_assert!(ptr_end1 <= ptr_start2 || ptr_end2 <= ptr_start1);

    Ok(())
}

fn prop_clone_len<T, A>(vec: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let cloned_values = vec.clone::<T, A>();

    let expected = vec.len();
    let result = cloned_values.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_clone_as_slice(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeErasedVec = values;
                    super::prop_clone_as_slice::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_clone_occupy_disjoint_memory_locations(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeErasedVec = values;
                    super::prop_clone_occupy_disjoint_memory_locations::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_clone_occupy_disjoint_memory_regions(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeErasedVec = values;
                    super::prop_clone_occupy_disjoint_memory_regions::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_clone_len(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeErasedVec = values;
                    super::prop_clone_len::<$typ, $alloc_typ>(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 32, strategy_type_erased_vec_max_len);
generate_props!(u8, u8, alloc::Global, 32, strategy_type_erased_vec_max_len);
generate_props!(u16, u16, alloc::Global, 32, strategy_type_erased_vec_max_len);
generate_props!(u32, u32, alloc::Global, 32, strategy_type_erased_vec_max_len);
generate_props!(u64, u64, alloc::Global, 32, strategy_type_erased_vec_max_len);
generate_props!(usize, usize, alloc::Global, 32, strategy_type_erased_vec_max_len);
generate_props!(string, String, alloc::Global, 32, strategy_type_erased_vec_max_len);
