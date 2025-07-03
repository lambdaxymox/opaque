use crate::common::erased::strategy_type_erased_vec_max_len;
use opaque_vec::OpaqueVec;
use opaque_alloc::TypedProjAlloc;

use core::any;
use core::fmt;
use alloc_crate::boxed::Box;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

#[cfg(feature = "nightly")]
fn prop_from_boxed_slice<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn expected<T, A>(values: &OpaqueVec) -> Box<[T], TypedProjAlloc<A>>
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut result = OpaqueVec::with_capacity_proj_in::<T, A>(values.len(), values.allocator::<T, A>().clone());
        result.extend::<_, T, A>(values.iter::<T, A>().cloned());

        result.into_boxed_slice::<T, A>()
    }

    let boxed_values = expected::<T, A>(&values);
    let vec = OpaqueVec::from(boxed_values.clone());

    let expected = boxed_values.as_ref();
    let result = vec.as_slice::<T, A>();

    prop_assert_eq!(result, expected);

    Ok(())
}

#[cfg(feature = "nightly")]
macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_from_boxed_slice(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_from_boxed_slice::<$typ, $alloc_typ>(values)?
                }
            }
        }
    };
}

#[cfg(not(feature = "nightly"))]
macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
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
