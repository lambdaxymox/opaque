use crate::common::erased::{
    strategy_alloc,
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

fn strategy_single_value<T>() -> impl Strategy<Value = T>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
{
    any::<T>()
}

fn prop_shift_insert_contains_same_index1<T, A>(value: T, alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypeErasedVec::new_in::<T, A>(alloc);

    prop_assert!(!vec.contains::<T, A>(&value));

    vec.shift_insert::<T, A>(0, value.clone());

    prop_assert!(vec.contains::<T, A>(&value));

    Ok(())
}

fn prop_shift_insert_contains_same_index2<T, A>(values: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypeErasedVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    for value in values.iter::<T, A>() {
        prop_assert!(!vec.contains::<T, A>(&value));
    }

    for value in values.iter::<T, A>().cloned() {
        vec.shift_insert::<T, A>(0, value);
    }

    for value in values.iter::<T, A>() {
        prop_assert!(vec.contains::<T, A>(&value));
    }

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident, $value_gen:ident, $alloc_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_shift_insert_contains_same_index1(
                    value in super::$value_gen::<$typ>(),
                    alloc in super::$alloc_gen::<$alloc_typ>(),
                ) {
                    let value: $typ = value;
                    let alloc: $alloc_typ = alloc;
                    super::prop_shift_insert_contains_same_index1::<$typ, $alloc_typ>(value, alloc)?
                }

                #[test]
                fn prop_shift_insert_contains_same_index2(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeErasedVec = values;
                    super::prop_shift_insert_contains_same_index2::<$typ, $alloc_typ>(values)?
                }
            }
        }
    };
}

generate_props!(
    unit,
    (),
    alloc::Global,
    32,
    strategy_type_erased_vec_max_len,
    strategy_single_value,
    strategy_alloc
);
generate_props!(
    u8,
    u8,
    alloc::Global,
    32,
    strategy_type_erased_vec_max_len,
    strategy_single_value,
    strategy_alloc
);
generate_props!(
    u16,
    u16,
    alloc::Global,
    32,
    strategy_type_erased_vec_max_len,
    strategy_single_value,
    strategy_alloc
);
generate_props!(
    u32,
    u32,
    alloc::Global,
    32,
    strategy_type_erased_vec_max_len,
    strategy_single_value,
    strategy_alloc
);
generate_props!(
    u64,
    u64,
    alloc::Global,
    32,
    strategy_type_erased_vec_max_len,
    strategy_single_value,
    strategy_alloc
);
generate_props!(
    usize,
    usize,
    alloc::Global,
    32,
    strategy_type_erased_vec_max_len,
    strategy_single_value,
    strategy_alloc
);
generate_props!(
    string,
    String,
    alloc::Global,
    32,
    strategy_type_erased_vec_max_len,
    strategy_single_value,
    strategy_alloc
);
