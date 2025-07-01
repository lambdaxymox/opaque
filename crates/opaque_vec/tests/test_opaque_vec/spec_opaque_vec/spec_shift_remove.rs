use crate::common::erased::{
    strategy_type_erased_vec_max_len,
    strategy_type_erased_vec_max_len_nonempty,
};
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_shift_remove_end<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = values.clone::<T, A>();

    let last_index = vec.len() - 1;
    let expected = &values.as_slice::<T, A>()[0..last_index];
    let _ = vec.shift_remove::<T, A>(last_index);
    let result = vec.as_slice::<T, A>();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_shift_remove_start<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn expected<T, A>(values: &OpaqueVec, start: usize) -> OpaqueVec
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut vec = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
        for value in values.iter::<T, A>().skip(start).skip(1).cloned() {
            vec.push::<T, A>(value);
        }

        vec
    }
    
    let mut vec = values.clone::<T, A>();

    for i in 0..values.len() {
        let new_vec = expected::<T, A>(&values, i);
        let _ = vec.shift_remove::<T, A>(0);
        let expected = new_vec.as_slice::<T, A>();
        let result = vec.as_slice::<T, A>();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_shift_remove_get_from_end<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = values.clone::<T, A>();

    for _ in 0..vec.len() {
        let last_index = vec.len() - 1;
        let expected = vec.get::<_, T, A>(last_index).cloned().unwrap();
        let result = vec.shift_remove::<T, A>(last_index);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_shift_remove_len<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let values_vec = values.clone::<T, A>();

    for i in 0..values.len() {
        let result_vec = {
            let mut vec = values_vec.clone::<T, A>();
            vec.shift_remove::<T, A>(i);
            vec
        };

        let expected = values.len() - 1;
        let result = result_vec.len();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident, $nonempty_vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_shift_remove_end(values in super::$nonempty_vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_shift_remove_end::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_shift_remove_start(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_shift_remove_start::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_shift_remove_get_from_end(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_shift_remove_get_from_end::<$typ, $alloc_typ>(values)?
                }

                #[test]
                fn prop_shift_remove_len(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_shift_remove_len::<$typ, $alloc_typ>(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_type_erased_vec_max_len_nonempty);
generate_props!(u8, u8, alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_type_erased_vec_max_len_nonempty);
generate_props!(u16, u16, alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_type_erased_vec_max_len_nonempty);
generate_props!(u32, u32, alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_type_erased_vec_max_len_nonempty);
generate_props!(u64, u64, alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_type_erased_vec_max_len_nonempty);
generate_props!(usize, usize, alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_type_erased_vec_max_len_nonempty);
generate_props!(string, String, alloc::Global, 32, strategy_type_erased_vec_max_len, strategy_type_erased_vec_max_len_nonempty);
