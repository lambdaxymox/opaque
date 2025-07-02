use crate::common::projected::strategy_type_projected_vec_max_len;
use opaque_vec::TypedProjVec;
use opaque_alloc::TypedProjAlloc;

use core::any;
use core::fmt;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(feature = "nightly")]
use std::boxed::Box;

#[cfg(not(feature = "nightly"))]
use allocator_api2::alloc;

#[cfg(not(feature = "nightly"))]
use allocator_api2::boxed::Box;

use proptest::prelude::*;

fn prop_from_boxed_slice<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn expected<T, A>(values: &TypedProjVec<T, A>) -> Box<[T], TypedProjAlloc<A>>
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut result = TypedProjVec::with_capacity_proj_in(values.len(), values.allocator().clone());
        result.extend(values.iter().cloned());

        result.into_boxed_slice()
    }

    let boxed_values = expected(&values);
    let vec = TypedProjVec::from(boxed_values.clone());

    let expected = boxed_values.as_ref();
    let result = vec.as_slice();

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_from_boxed_slice(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_from_boxed_slice(values)?
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
