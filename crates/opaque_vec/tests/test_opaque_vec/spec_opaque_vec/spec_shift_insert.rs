use crate::common::erased::strategy_type_erased_vec_max_len;
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use allocator_api2::alloc;

use proptest::prelude::*;

fn prop_shift_insert_start<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn expected<T, A>(values: &OpaqueVec) -> OpaqueVec
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
        for value in values.iter::<T, A>().rev().cloned() {
            vec.push::<T, A>(value);
        }

        vec
    }

    fn result<T, A>(values: &OpaqueVec) -> OpaqueVec
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
        for value in values.iter::<T, A>().cloned() {
            vec.shift_insert::<T, A>(0, value);
        }

        vec
    }
    
    let expected_vec = expected::<T, A>(&values);
    let result_vec = result::<T, A>(&values);

    let expected = expected_vec.as_slice::<T, A>();
    let result = result_vec.as_slice::<T, A>();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_shift_insert_len<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    for (i, value) in values.iter::<T, A>().cloned().enumerate() {
        vec.shift_insert::<T, A>(i, value);
    }

    let expected = values.len();
    let result = vec.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_shift_insert_get<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    for (i, value) in values.iter::<T, A>().cloned().enumerate() {
        vec.shift_insert::<T, A>(i, value);
    }

    for i in 0..vec.len() {
        let expected = Some(values.as_slice::<T, A>()[i].clone());
        let result = vec.get::<_, T, A>(i).cloned();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_shift_insert_end<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());

    for (i, value) in values.iter::<T, A>().cloned().enumerate() {
        vec.shift_insert::<T, A>(i, value);
    }

    let expected = values.as_slice::<T, A>();
    let result = vec.as_slice::<T, A>();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_shift_insert_contains<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());

    for value in values.iter::<T, A>() {
        prop_assert!(!vec.contains::<T, A>(value));
    }

    for (i, value) in values.iter::<T, A>().cloned().enumerate() {
        vec.shift_insert::<T, A>(i, value);
    }

    for value in values.iter::<T, A>() {
        prop_assert!(vec.contains::<T, A>(value));
    }

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_shift_insert_start(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_shift_insert_start::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_shift_insert_len(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_shift_insert_len::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_shift_insert_get(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_shift_insert_get::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_shift_insert_end(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_shift_insert_end::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_shift_insert_contains(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_shift_insert_contains::<$typ, $alloc_typ>(values)?
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
