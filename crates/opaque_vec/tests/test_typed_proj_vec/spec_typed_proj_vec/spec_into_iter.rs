use crate::common::projected::{
    SingleBoundedValue,
    strategy_type_projected_vec_len,
    strategy_type_projected_vec_max_len,
};
use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn strategy_prop_into_iter_take<T, A>(max_length: usize) -> impl Strategy<Value = (TypedProjVec<T, A>, usize)>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleBoundedValue,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (1..=max_length).prop_flat_map(move |length| (strategy_type_projected_vec_len(length), 0..=length))
}

fn prop_into_iter_back_to_vec<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let vec = values.clone();
    let mut result = TypedProjVec::new_proj_in(values.allocator().clone());
    for value in vec.into_iter() {
        result.push(value);
    }

    let expected = values.as_slice();
    let result = result.as_slice();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_into_iter_take<T, A>((values, count): (TypedProjVec<T, A>, usize)) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());
    vec.extend(values.iter().cloned());

    let mut expected = TypedProjVec::new_proj_in(values.allocator().clone());
    for value in values.iter().cloned().take(count) {
        expected.push(value);
    }

    let mut result = TypedProjVec::new_proj_in(values.allocator().clone());
    result.extend(values.into_iter().take(count));

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_into_iter_take_none<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());
    vec.extend(values.iter().cloned());

    let mut result = TypedProjVec::new_proj_in(values.allocator().clone());
    result.extend(values.into_iter().take(0));

    prop_assert!(result.is_empty());

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_into_iter_back_to_vec(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_into_iter_back_to_vec(values)?
                }

                #[test]
                fn prop_into_iter_take(values in super::strategy_prop_into_iter_take::<$typ, $alloc_typ>($max_length)) {
                    let values: (super::TypedProjVec<$typ, $alloc_typ>, usize) = values;
                    super::prop_into_iter_take(values)?
                }

                #[test]
                fn prop_into_iter_take_none(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_into_iter_take_none(values)?
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
