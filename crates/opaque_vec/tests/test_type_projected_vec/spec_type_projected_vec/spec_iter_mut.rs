use crate::common::projected::{
    SingleBoundedValue,
    strategy_type_projected_vec_len,
    strategy_type_projected_vec_max_len,
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

fn strategy_prop_iter_mut_take<T, A>(max_length: usize) -> impl Strategy<Value = (TypeProjectedVec<T, A>, usize)>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (1..=max_length).prop_flat_map(move |length| (strategy_type_projected_vec_len(length), 0..=length))
}

fn prop_iter_mut_back_to_vec<T, A>(values: TypeProjectedVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = values.clone();
    let mut result = TypeProjectedVec::new_proj_in(values.allocator().clone());
    for value in vec.iter_mut() {
        result.push(value.clone());
    }

    let expected = values.as_slice();
    let result = result.as_slice();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_iter_mut_take<T, A>((values, count): (TypeProjectedVec<T, A>, usize)) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypeProjectedVec::new_proj_in(values.allocator().clone());
    vec.extend(values.iter().cloned());

    let mut expected = TypeProjectedVec::new_proj_in(values.allocator().clone());
    for value in values.iter().cloned().take(count) {
        expected.push(value);
    }

    let mut result = TypeProjectedVec::new_proj_in(values.allocator().clone());
    result.extend(vec.iter_mut().map(|v| v.clone()).take(count));

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_iter_mut_take_none<T, A>(values: TypeProjectedVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypeProjectedVec::new_proj_in(values.allocator().clone());
    vec.extend(values.iter().cloned());

    let mut result = TypeProjectedVec::new_proj_in(values.allocator().clone());
    result.extend(vec.iter_mut().map(|v| v.clone()).take(0));

    prop_assert!(result.is_empty());

    Ok(())
}

fn prop_iter_mut_ordering<T, A>(values: TypeProjectedVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = values.clone();
    let mut iter = vec.iter_mut();
    for i in 0..values.len() {
        let expected = values.get(i).cloned();
        let result = iter.next().cloned();

        prop_assert_eq!(result, expected);
    }

    prop_assert_eq!(iter.next(), None);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_iter_mut_back_to_vec(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    super::prop_iter_mut_back_to_vec(values)?
                }

                #[test]
                fn prop_iter_mut_take(values in super::strategy_prop_iter_mut_take::<$typ, $alloc_typ>($max_length)) {
                    let values: (super::TypeProjectedVec<$typ, $alloc_typ>, usize) = values;
                    super::prop_iter_mut_take(values)?
                }

                #[test]
                fn prop_iter_mut_take_none(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    super::prop_iter_mut_take_none(values)?
                }

                #[test]
                fn prop_iter_mut_ordering(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    super::prop_iter_mut_ordering(values)?
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
