use crate::common::projected::strategy_type_projected_vec_max_len;
use opaque_vec::TypeProjectedVec;

use core::any;
use core::fmt;
use alloc_crate::vec::Vec;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

#[cfg(feature = "nightly")]
fn prop_from_vec<T, A>(values: TypeProjectedVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn expected<T, A>(values: &[T], alloc: A) -> Vec<T, A>
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut vec = Vec::with_capacity_in(values.len(), alloc);
        vec.extend(values.iter().cloned());

        vec
    }

    let expected_values = expected(values.as_slice(), values.allocator().allocator().clone());
    let vec = TypeProjectedVec::from(expected_values.clone());

    let expected = expected_values.as_slice();
    let result = vec.as_slice();

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
                fn prop_from_vec(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    super::prop_from_vec(values)?
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

generate_props!(unit, (), alloc::Global, 32, strategy_type_projected_vec_max_len);
generate_props!(u8, u8, alloc::Global, 32, strategy_type_projected_vec_max_len);
generate_props!(u16, u16, alloc::Global, 32, strategy_type_projected_vec_max_len);
generate_props!(u32, u32, alloc::Global, 32, strategy_type_projected_vec_max_len);
generate_props!(u64, u64, alloc::Global, 32, strategy_type_projected_vec_max_len);
generate_props!(usize, usize, alloc::Global, 32, strategy_type_projected_vec_max_len);
generate_props!(string, String, alloc::Global, 32, strategy_type_projected_vec_max_len);
