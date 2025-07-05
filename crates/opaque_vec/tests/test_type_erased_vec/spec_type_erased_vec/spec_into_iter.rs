use crate::common::erased::{
    SingleBoundedValue,
    strategy_type_erased_vec_len,
    strategy_type_erased_vec_max_len,
};
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

fn strategy_prop_into_iter_take<T, A>(max_length: usize) -> impl Strategy<Value = (TypeErasedVec, usize)>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (1..=max_length).prop_flat_map(move |length| (strategy_type_erased_vec_len::<T, A>(length), 0..=length))
}

fn prop_into_iter_back_to_vec<T, A>(values: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let vec = values.clone::<T, A>();
    let mut result = TypeErasedVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    for value in vec.into_iter::<T, A>() {
        result.push::<T, A>(value);
    }

    let expected = values.as_slice::<T, A>();
    let result = result.as_slice::<T, A>();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_into_iter_take<T, A>((values, count): (TypeErasedVec, usize)) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypeErasedVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    vec.extend::<_, T, A>(values.iter::<T, A>().cloned());

    let mut expected = TypeErasedVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    for value in values.iter::<T, A>().cloned().take(count) {
        expected.push::<T, A>(value);
    }

    let mut result = TypeErasedVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    result.extend::<_, T, A>(values.into_iter::<T, A>().take(count));

    prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());

    Ok(())
}

fn prop_into_iter_take_none<T, A>(values: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypeErasedVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    vec.extend::<_, T, A>(values.iter::<T, A>().cloned());

    let mut result = TypeErasedVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    result.extend::<_, T, A>(values.into_iter::<T, A>().take(0));

    prop_assert!(result.is_empty());

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_into_iter_back_to_vec(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeErasedVec = values;
                    super::prop_into_iter_back_to_vec::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_into_iter_take(values in super::strategy_prop_into_iter_take::<$typ, $alloc_typ>($max_length)) {
                    let values: (super::TypeErasedVec, usize) = values;
                    super::prop_into_iter_take::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_into_iter_take_none(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeErasedVec = values;
                    super::prop_into_iter_take_none::<$typ, $alloc_typ>(values)?
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
