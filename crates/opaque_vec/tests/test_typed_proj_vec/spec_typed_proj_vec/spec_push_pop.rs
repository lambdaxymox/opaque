use crate::common::projected::strategy_type_projected_vec_max_len;
use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_push_pop<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn expected<T, A>(values: &TypedProjVec<T, A>) -> TypedProjVec<T, A>
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut expected_vec = TypedProjVec::new_proj_in(values.allocator().clone());
        for value in values.iter().rev().cloned() {
            expected_vec.push(value);
        }

        expected_vec
    }

    fn result<T, A>(values: &TypedProjVec<T, A>) -> TypedProjVec<T, A>
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug,
        A: any::Any + alloc::Allocator + Send + Sync + Clone,
    {
        let mut vec = values.clone();
        let mut result_vec = TypedProjVec::new_proj_in(values.allocator().clone());
        for _ in 0..vec.len() {
            let popped = vec.pop();

            result_vec.push(popped.unwrap());
        }

        result_vec
    }

    let expected = expected(&values);
    let result = result(&values);

    prop_assert_eq!(result.as_slice(), expected.as_slice());

    Ok(())
}

fn prop_push_pop_exists<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());
    for value in values.iter().cloned() {
        vec.push(value);
    }

    for _ in 0..vec.len() {
        let result = vec.pop();

        prop_assert!(result.is_some());
    }

    let result = vec.pop();

    prop_assert!(result.is_none());

    Ok(())
}

fn prop_push_pop_len<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());
    for value in values.iter().cloned() {
        vec.push(value);
    }

    let _ = vec.pop();

    let expected = if values.len() > 0 { values.len() - 1 } else { 0 };
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
                fn prop_push_pop(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_push_pop(values)?
                }

                #[test]
                fn prop_push_pop_exists(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_push_pop_exists(values)?
                }

                #[test]
                fn prop_push_pop_len(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_push_pop_len(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 128, strategy_type_projected_vec_max_len);
generate_props!(u8, u8, alloc::Global, 128, strategy_type_projected_vec_max_len);
generate_props!(u16, u16, alloc::Global, 128, strategy_type_projected_vec_max_len);
generate_props!(u32, u32, alloc::Global, 128, strategy_type_projected_vec_max_len);
generate_props!(u64, u64, alloc::Global, 128, strategy_type_projected_vec_max_len);
generate_props!(usize, usize, alloc::Global, 128, strategy_type_projected_vec_max_len);
generate_props!(string, String, alloc::Global, 128, strategy_type_projected_vec_max_len);
