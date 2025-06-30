use crate::common::erased::strategy_type_erased_vec_max_len;
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_contains<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    for value in values.iter::<T, A>() {
        prop_assert!(values.contains::<T, A>(value));
    }

    Ok(())
}

fn prop_contains_empty<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let empty_values = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    for value in values.iter::<T, A>() {
        prop_assert!(!empty_values.contains::<T, A>(value));
    }

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_contains(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_contains::<$typ, alloc::Global>(values)?
                }

                #[test]
                fn prop_contains_empty(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_contains_empty::<$typ, alloc::Global>(values)?
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
