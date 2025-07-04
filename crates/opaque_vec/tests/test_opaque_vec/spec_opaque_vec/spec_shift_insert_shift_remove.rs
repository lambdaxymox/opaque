use crate::common::erased::strategy_type_erased_vec_max_len;
use opaque_vec::TypeErasedVec;

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

fn prop_shift_insert_shift_remove<T, A>(values: TypeErasedVec, new_value: T) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let expected = values.clone::<T, A>();
    let mut result = values.clone::<T, A>();

    prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());

    for i in 0..values.len() {
        result.shift_insert::<T, A>(i, new_value.clone());
        result.shift_remove::<T, A>(i);

        prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());
    }

    Ok(())
}

fn prop_shift_remove_shift_insert<T, A>(values: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let expected = values.clone::<T, A>();
    let mut result = values.clone::<T, A>();

    prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());

    for i in 0..values.len() {
        let removed_value = result.shift_remove::<T, A>(i);
        result.shift_insert::<T, A>(i, removed_value);

        prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());
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
                    let values: super::TypeErasedVec = values;
                    let new_value: $typ = new_value;
                    super::prop_shift_insert_shift_remove::<$typ, $alloc_typ>(values, new_value)?
                }

                #[test]
                fn prop_shift_remove_shift_insert(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeErasedVec = values;
                    super::prop_shift_remove_shift_insert::<$typ, $alloc_typ>(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_single_value);
generate_props!(u8, u8, alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_single_value);
generate_props!(u16, u16, alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_single_value);
generate_props!(u32, u32, alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_single_value);
generate_props!(u64, u64, alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_single_value);
generate_props!(usize, usize, alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_single_value);
generate_props!(string, String, alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_single_value);
