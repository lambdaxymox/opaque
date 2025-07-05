use crate::common::projected::strategy_type_projected_vec_max_len;
use opaque_vec::TypeProjectedVec;

use core::any;
use core::fmt;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn strategy_single_value<T>() -> impl Strategy<Value = T>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
{
    any::<T>()
}

fn prop_shift_insert_shift_remove<T, A>(values: TypeProjectedVec<T, A>, new_value: T) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let expected = values.clone();
    let mut result = values.clone();

    prop_assert_eq!(result.as_slice(), expected.as_slice());

    for i in 0..values.len() {
        result.shift_insert(i, new_value.clone());
        result.shift_remove(i);

        prop_assert_eq!(result.as_slice(), expected.as_slice());
    }

    Ok(())
}

fn prop_shift_remove_shift_insert<T, A>(values: TypeProjectedVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let expected = values.clone();
    let mut result = values.clone();

    prop_assert_eq!(result.as_slice(), expected.as_slice());

    for i in 0..values.len() {
        let removed_value = result.shift_remove(i);
        result.shift_insert(i, removed_value);

        prop_assert_eq!(result.as_slice(), expected.as_slice());
    }

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident, $value_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_shift_insert_shift_remove(
                    values in super::$vec_gen::<$typ, $alloc_typ>($max_length),
                    new_value in super::$value_gen::<$typ>(),
                ) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    let new_value: $typ = new_value;
                    super::prop_shift_insert_shift_remove(values, new_value)?
                }

                #[test]
                fn prop_shift_remove_shift_insert(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    super::prop_shift_remove_shift_insert(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_single_value);
generate_props!(u8, u8, alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_single_value);
generate_props!(u16, u16, alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_single_value);
generate_props!(u32, u32, alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_single_value);
generate_props!(u64, u64, alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_single_value);
generate_props!(usize, usize, alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_single_value);
generate_props!(string, String, alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_single_value);
