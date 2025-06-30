use crate::common::erased::strategy_type_erased_vec_max_len;
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_push_pop<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn expected<T, A>(values: &OpaqueVec) -> OpaqueVec
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut expected_vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
        for value in values.iter::<T, A>().rev().cloned() {
            expected_vec.push::<T, A>(value);
        }

        expected_vec
    }

    fn result<T, A>(values: &OpaqueVec) -> OpaqueVec
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut vec = values.clone::<T, A>();
        let mut result_vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
        for _ in 0..vec.len() {
            let popped = vec.pop::<T, A>();

            result_vec.push::<T, A>(popped.unwrap());
        }

        result_vec
    }

    let expected = expected::<T, A>(&values);
    let result = result::<T, A>(&values);

    prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());

    Ok(())
}

fn prop_push_pop_exists<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    for value in values.iter::<T, A>().cloned() {
        vec.push::<T, A>(value);
    }

    for _ in 0..vec.len() {
        let result = vec.pop::<T, A>();

        prop_assert!(result.is_some());
    }

    let result = vec.pop::<T, A>();

    prop_assert!(result.is_none());

    Ok(())
}

fn prop_push_pop_len<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    for value in values.iter::<T, A>().cloned() {
        vec.push::<T, A>(value);
    }

    let _ = vec.pop::<T, A>();

    let expected = if values.len() > 0 { values.len() - 1 } else { 0 };
    let result = vec.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_push_pop(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_push_pop::<$typ, alloc::Global>(values)?
                }

                #[test]
                fn prop_push_pop_exists(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_push_pop_exists::<$typ, alloc::Global>(values)?
                }

                #[test]
                fn prop_push_pop_len(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_push_pop_len::<$typ, alloc::Global>(values)?
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
