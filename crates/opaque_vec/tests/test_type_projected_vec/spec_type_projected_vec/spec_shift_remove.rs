use crate::common::projected::{
    strategy_type_projected_vec_max_len,
    strategy_type_projected_vec_max_len_nonempty,
};
use opaque_vec::TypeProjectedVec;

use core::any;
use core::fmt;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn prop_shift_remove_end<T, A>(values: TypeProjectedVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = values.clone();

    let last_index = vec.len() - 1;
    let expected = &values[0..last_index];
    let _ = vec.shift_remove(last_index);
    let result = vec.as_slice();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_shift_remove_start<T, A>(values: TypeProjectedVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn expected<T, A>(values: &TypeProjectedVec<T, A>, start: usize) -> TypeProjectedVec<T, A>
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut vec = TypeProjectedVec::new_proj_in(values.allocator().clone());
        for value in values.iter().skip(start).skip(1).cloned() {
            vec.push(value);
        }

        vec
    }
    
    let mut vec = values.clone();
    for i in 0..values.len() {
        let new_vec = expected(&values, i);
        let _ = vec.shift_remove(0);
        let expected = new_vec.as_slice();
        let result = vec.as_slice();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_shift_remove_get_from_end<T, A>(values: TypeProjectedVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = values.clone();

    for _ in 0..vec.len() {
        let last_index = vec.len() - 1;
        let expected = vec.get(last_index).cloned().unwrap();
        let result = vec.shift_remove(last_index);

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_shift_remove_len<T, A>(values: TypeProjectedVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let values_vec = values.clone();

    for i in 0..values.len() {
        let result_vec = {
            let mut vec = values_vec.clone();
            vec.shift_remove(i);
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
            use super::*;
            proptest! {
                #[test]
                fn prop_shift_remove_end(values in super::$nonempty_vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    super::prop_shift_remove_end(values)?
                }

                #[test]
                fn prop_shift_remove_start(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    super::prop_shift_remove_start(values)?
                }

                #[test]
                fn prop_shift_remove_get_from_end(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    super::prop_shift_remove_get_from_end(values)?
                }

                #[test]
                fn prop_shift_remove_len(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    super::prop_shift_remove_len(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_type_projected_vec_max_len_nonempty);
generate_props!(u8, u8, alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_type_projected_vec_max_len_nonempty);
generate_props!(u16, u16, alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_type_projected_vec_max_len_nonempty);
generate_props!(u32, u32, alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_type_projected_vec_max_len_nonempty);
generate_props!(u64, u64, alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_type_projected_vec_max_len_nonempty);
generate_props!(usize, usize, alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_type_projected_vec_max_len_nonempty);
generate_props!(string, String, alloc::Global, 32, strategy_type_projected_vec_max_len, strategy_type_projected_vec_max_len_nonempty);
