use crate::common::erased::strategy_type_erased_vec_max_len;
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_clear_as_slice<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let expected = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    let result = {
        let mut _vec = values.clone::<T, A>();
        _vec.clear::<T, A>();
        _vec
    };

    prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());

    Ok(())
}

fn prop_clear_is_empty<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut result = values.clone::<T, A>();
    result.clear::<T, A>();

    prop_assert!(result.is_empty());

    Ok(())
}

fn prop_clear_len<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut result = values.clone::<T, A>();
    result.clear::<T, A>();

    assert_eq!(result.len(), 0);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_clear_as_slice(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_clear_as_slice::<$typ, alloc::Global>(values)?
                }

                #[test]
                fn prop_clear_is_empty(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_clear_is_empty::<$typ, alloc::Global>(values)?
                }

                #[test]
                fn prop_clear_len(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_clear_len::<$typ, alloc::Global>(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), 128, strategy_type_erased_vec_max_len);
generate_props!(u8, u8, 128, strategy_type_erased_vec_max_len);
generate_props!(u16, u16, 128, strategy_type_erased_vec_max_len);
generate_props!(u32, u32, 128, strategy_type_erased_vec_max_len);
generate_props!(u64, u64, 128, strategy_type_erased_vec_max_len);
generate_props!(usize, usize, 128, strategy_type_erased_vec_max_len);
generate_props!(string, String, 128, strategy_type_erased_vec_max_len);
