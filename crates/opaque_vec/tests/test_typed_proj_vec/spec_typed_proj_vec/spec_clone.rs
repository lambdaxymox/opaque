use crate::common::projected::strategy_type_projected_vec_max_len;
use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use core::mem;
use std::alloc;

use proptest::prelude::*;

fn prop_clone_as_slice<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let cloned_values = values.clone();

    let expected = values.as_slice();
    let result = cloned_values.as_slice();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_clone_occupy_disjoint_memory_locations<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let cloned_values = values.clone();

    let disjoint_if_non_empty_and_non_zst = if (values.len() != 0) && (mem::size_of::<T>() != 0) {
        values.as_ptr() != cloned_values.as_ptr()
    } else {
        true
    };

    prop_assert!(disjoint_if_non_empty_and_non_zst);

    Ok(())
}

fn prop_clone_occupy_disjoint_memory_regions<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let cloned_values = values.clone();

    let ptr_start1 = values.as_ptr() as usize;
    let ptr_start2 = cloned_values.as_ptr() as usize;
    let ptr_end1 = {
        let len1 = values.len() * mem::size_of::<T>();
        ptr_start1 + len1
    };
    let ptr_end2 = {
        let len2 = cloned_values.len() * mem::size_of::<T>();
        ptr_start2 + len2
    };

    prop_assert!(ptr_end1 <= ptr_start2 || ptr_end2 <= ptr_start1);

    Ok(())
}

fn prop_clone_len<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let cloned_values = values.clone();

    let expected = values.len();
    let result = cloned_values.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_clone_as_slice(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::TypedProjVec<$typ, alloc::Global> = values;
                    super::prop_clone_as_slice(values)?
                }

                #[test]
                fn prop_clone_occupy_disjoint_memory_locations(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::TypedProjVec<$typ, alloc::Global> = values;
                    super::prop_clone_occupy_disjoint_memory_locations(values)?
                }

                #[test]
                fn prop_clone_occupy_disjoint_memory_regions(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::TypedProjVec<$typ, alloc::Global> = values;
                    super::prop_clone_occupy_disjoint_memory_regions(values)?
                }

                #[test]
                fn prop_clone_len(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::TypedProjVec<$typ, alloc::Global> = values;
                    super::prop_clone_len(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), 128, strategy_type_projected_vec_max_len);
generate_props!(u8, u8, 128, strategy_type_projected_vec_max_len);
generate_props!(u16, u16, 128, strategy_type_projected_vec_max_len);
generate_props!(u32, u32, 128, strategy_type_projected_vec_max_len);
generate_props!(u64, u64, 128, strategy_type_projected_vec_max_len);
generate_props!(usize, usize, 128, strategy_type_projected_vec_max_len);
generate_props!(string, String, 128, strategy_type_projected_vec_max_len);
