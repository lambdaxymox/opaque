use crate::common::projected::strategy_type_projected_vec_max_len;
use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use allocator_api2::alloc;

use proptest::prelude::*;

fn prop_truncate_as_slice_length_greater_than_or_equal_to<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let base_proj_vec = values.clone();
    let min_len = values.len();
    let max_len = 10 * values.len();
    for len in min_len..max_len {
        let mut proj_vec = base_proj_vec.clone();

        proj_vec.truncate(len);

        let expected = &values[..];
        let result = proj_vec.as_slice();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_truncate_as_slice_length_less_than_or_equal_to<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let base_proj_vec = values.clone();
    for len in 0..values.len() {
        let mut proj_vec = base_proj_vec.clone();

        proj_vec.truncate(len);

        let expected = &values[..len];
        let result = proj_vec.as_slice();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_truncate_len_length_greater_than_or_equal_to<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let base_proj_vec = values.clone();
    let min_len = values.len();
    let max_len = 10 * values.len();
    for len in min_len..max_len {
        let mut proj_vec = base_proj_vec.clone();

        proj_vec.truncate(len);

        let expected = values.len();
        let result = proj_vec.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_truncate_len_length_less_than_or_equal_to<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let base_proj_vec = values.clone();
    for len in 0..values.len() {
        let mut proj_vec = base_proj_vec.clone();

        proj_vec.truncate(len);

        let expected = len;
        let result = proj_vec.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_truncate_as_slice_length_greater_than_or_equal_to(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_truncate_as_slice_length_greater_than_or_equal_to(values)?
                }

                #[test]
                fn prop_truncate_as_slice_length_less_than_or_equal_to(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_truncate_as_slice_length_less_than_or_equal_to(values)?
                }

                #[test]
                fn prop_truncate_len_length_greater_than_or_equal_to(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_truncate_len_length_greater_than_or_equal_to(values)?
                }

                #[test]
                fn prop_truncate_len_length_less_than_or_equal_to(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_truncate_len_length_less_than_or_equal_to(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 32, strategy_type_projected_vec_max_len);
generate_props!(u8, u8, alloc::Global, 32, strategy_type_projected_vec_max_len);
generate_props!(u16, u16, alloc::Global, 32, strategy_type_projected_vec_max_len);
generate_props!(u32, u32, alloc::Global, 32, strategy_type_projected_vec_max_len);
generate_props!(u64, u64, alloc::Global, 32, strategy_type_projected_vec_max_len);
generate_props!(usize, usize, alloc::Global, 32, strategy_type_projected_vec_max_len);
generate_props!(string, String, alloc::Global, 32, strategy_type_projected_vec_max_len);
