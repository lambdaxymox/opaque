use crate::common::projected::strategy_type_projected_vec_max_len;
use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_shift_insert_start<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn expected<T, A>(values: &TypedProjVec<T, A>) -> TypedProjVec<T, A>
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());
        for value in values.iter().rev().cloned() {
            vec.push(value);
        }

        vec
    }

    fn result<T, A>(values: &TypedProjVec<T, A>) -> TypedProjVec<T, A>
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());
        for value in values.iter().cloned() {
            vec.shift_insert(0, value);
        }

        vec
    }
    
    let expected_vec = expected(&values);
    let result_vec = result(&values);

    let expected = expected_vec.as_slice();
    let result = result_vec.as_slice();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_shift_insert_len<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());
    for (i, value) in values.iter().cloned().enumerate() {
        vec.shift_insert(i, value);
    }

    let expected = values.len();
    let result = vec.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_shift_insert_get<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());
    for (i, value) in values.iter().cloned().enumerate() {
        vec.shift_insert(i, value);
    }

    for i in 0..vec.len() {
        let expected = Some(values[i].clone());
        let result = vec.get(i).cloned();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_shift_insert_end<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());

    for (i, value) in values.iter().cloned().enumerate() {
        vec.shift_insert(i, value);
    }

    let expected = values.as_slice();
    let result = vec.as_slice();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_shift_insert_contains<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());

    for value in values.iter() {
        prop_assert!(!vec.contains(value));
    }

    for (i, value) in values.iter().cloned().enumerate() {
        vec.shift_insert(i, value);
    }

    for value in values.iter() {
        prop_assert!(vec.contains(value));
    }

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_shift_insert_start(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_shift_insert_start(values)?
                }

                #[test]
                fn prop_shift_insert_len(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_shift_insert_len(values)?
                }

                #[test]
                fn prop_shift_insert_get(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_shift_insert_get(values)?
                }

                #[test]
                fn prop_shift_insert_end(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_shift_insert_end(values)?
                }

                #[test]
                fn prop_shift_insert_contains(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_shift_insert_contains(values)?
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
