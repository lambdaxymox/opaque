use crate::common::erased::strategy_type_erased_vec_max_len;
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_truncate_as_slice_length_greater_than_or_equal_to<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let base_proj_vec = values.clone::<T, A>();
    let min_len = values.len();
    let max_len = 10 * values.len();
    for len in min_len..max_len {
        let mut opaque_vec = base_proj_vec.clone::<T, A>();

        opaque_vec.truncate::<T, A>(len);

        let expected = &values.as_slice::<T, A>()[..];
        let result = opaque_vec.as_slice::<T, A>();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_truncate_as_slice_length_less_than_or_equal_to<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let base_proj_vec = values.clone::<T, A>();
    for len in 0..values.len() {
        let mut opaque_vec = base_proj_vec.clone::<T, A>();

        opaque_vec.truncate::<T, A>(len);

        let expected = &values.as_slice::<T, A>()[..len];
        let result = opaque_vec.as_slice::<T, A>();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_truncate_len_length_greater_than_or_equal_to<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let base_proj_vec = values.clone::<T, A>();
    let min_len = values.len();
    let max_len = 10 * values.len();
    for len in min_len..max_len {
        let mut opaque_vec = base_proj_vec.clone::<T, A>();

        opaque_vec.truncate::<T, A>(len);

        let expected = values.len();
        let result = opaque_vec.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_truncate_len_length_less_than_or_equal_to<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let base_proj_vec = values.clone::<T, A>();
    for len in 0..values.len() {
        let mut opaque_vec = base_proj_vec.clone::<T, A>();

        opaque_vec.truncate::<T, A>(len);

        let expected = len;
        let result = opaque_vec.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_truncate_as_slice_length_greater_than_or_equal_to(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_truncate_as_slice_length_greater_than_or_equal_to::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_truncate_as_slice_length_less_than_or_equal_to(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_truncate_as_slice_length_less_than_or_equal_to::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_truncate_len_length_greater_than_or_equal_to(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_truncate_len_length_greater_than_or_equal_to::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_truncate_len_length_less_than_or_equal_to(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_truncate_len_length_less_than_or_equal_to::<$typ, $alloc_typ>(values)?
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
