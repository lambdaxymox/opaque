use crate::common::projected::strategy_type_projected_vec_max_len;
use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_extend_from_slice_values<T, A>(values: TypedProjVec<T, A>, extension_values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let result = {
        let mut _result = values.clone();
        _result.extend_from_slice(extension_values.as_slice());
        _result
    };

    for i in 0..values.len() {
        prop_assert_eq!(&result[i], &values[i]);
    }

    for j in 0..extension_values.len() {
        prop_assert_eq!(&result[values.len() + j], &extension_values[j]);
    }

    Ok(())
}

fn prop_extend_from_slice_len<T, A>(values: TypedProjVec<T, A>, extension_values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let result = {
        let mut _result = values.clone();
        _result.extend_from_slice(&extension_values);
        _result
    };

    prop_assert_eq!(result.len(), values.len() + extension_values.len());

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_extend_from_slice_values(
                    values1 in super::$vec_gen::<$typ, alloc::Global>($max_length),
                    values2 in super::$vec_gen::<$typ, alloc::Global>($max_length),
                ) {
                    let values1: super::TypedProjVec<$typ, alloc::Global> = values1;
                    let values2: super::TypedProjVec<$typ, alloc::Global> = values2;
                    super::prop_extend_from_slice_values(values1, values2)?
                }

                #[test]
                fn prop_extend_from_slice_len(
                    values1 in super::$vec_gen::<$typ, alloc::Global>($max_length),
                    values2 in super::$vec_gen::<$typ, alloc::Global>($max_length),
                ) {
                    let values1: super::TypedProjVec<$typ, alloc::Global> = values1;
                    let values2: super::TypedProjVec<$typ, alloc::Global> = values2;
                    super::prop_extend_from_slice_len(values1, values2)?
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
