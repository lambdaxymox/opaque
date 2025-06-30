use crate::common::erased::strategy_type_erased_vec_max_len;
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_push_contains<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());

    for value in values.iter::<T, A>() {
        prop_assert!(!vec.contains::<T, A>(value));
    }

    for value in values.iter::<T, A>().cloned() {
        vec.push::<T, A>(value);
    }

    for value in values.iter::<T, A>() {
        prop_assert!(vec.contains::<T, A>(value));
    }

    Ok(())
}

fn prop_push_get<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    for value in values.iter::<T, A>().cloned() {
        vec.push::<T, A>(value);
    }

    for i in 0..vec.len() {
        let expected = Some(values.as_slice::<T, A>()[i].clone());
        let result = vec.get::<_, T, A>(i).cloned();

        assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_push_len<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
    for value in values.iter::<T, A>().cloned() {
        vec.push::<T, A>(value);
    }

    let expected = values.len();
    let result = vec.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_push_contains(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_push_contains::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_push_get(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_push_get::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_push_len(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_push_len::<$typ, $alloc_typ>(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 128, strategy_type_erased_vec_max_len);
generate_props!(u8, u8, alloc::Global, 128, strategy_type_erased_vec_max_len);
generate_props!(u16, u16, alloc::Global, 128, strategy_type_erased_vec_max_len);
generate_props!(u32, u32, alloc::Global, 128, strategy_type_erased_vec_max_len);
generate_props!(u64, u64, alloc::Global, 128, strategy_type_erased_vec_max_len);
generate_props!(usize, usize, alloc::Global, 128, strategy_type_erased_vec_max_len);
generate_props!(string, String, alloc::Global, 128, strategy_type_erased_vec_max_len);
