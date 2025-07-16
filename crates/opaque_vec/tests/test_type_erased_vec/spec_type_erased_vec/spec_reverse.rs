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

fn prop_reverse<T, A>(values: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn expected<T, A>(values: &TypeErasedVec) -> TypeErasedVec
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut vec = TypeErasedVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
        for value in values.iter::<T, A>().rev().cloned() {
            vec.push::<T, A>(value);
        }

        vec
    }

    fn result<T, A>(values: &TypeErasedVec) -> TypeErasedVec
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut vec = values.clone::<T, A>();
        vec.reverse::<T, A>();

        vec
    }

    let expected = expected::<T, A>(&values);
    let result = result::<T, A>(&values);

    prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());

    Ok(())
}

fn prop_reverse_len<T, A>(values: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut reversed = values.clone::<T, A>();
    reversed.reverse::<T, A>();

    let expected = values.len();
    let result = reversed.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_reverse_reverse<T, A>(values: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let expected = values.clone::<T, A>();
    let mut result = values.clone::<T, A>();
    result.reverse::<T, A>();
    result.reverse::<T, A>();

    prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_reverse(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeErasedVec = values;
                    super::prop_reverse::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_reverse_len(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeErasedVec = values;
                    super::prop_reverse_len::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_reverse_reverse(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeErasedVec = values;
                    super::prop_reverse_reverse::<$typ, $alloc_typ>(values)?
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
