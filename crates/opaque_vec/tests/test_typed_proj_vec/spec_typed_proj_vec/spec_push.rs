use crate::common::projected::strategy_type_projected_vec_max_len;
use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

use proptest::prelude::*;

fn prop_push_contains<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());

    for value in values.iter() {
        prop_assert!(!vec.contains(value));
    }

    for value in values.iter().cloned() {
        vec.push(value);
    }

    for value in values.iter() {
        prop_assert!(vec.contains(value));
    }

    Ok(())
}

fn prop_push_get<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());
    for value in values.iter().cloned() {
        vec.push(value);
    }

    for i in 0..vec.len() {
        let expected = Some(values[i].clone());
        let result = vec.get(i).cloned();

        assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_push_len<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_proj_in(values.allocator().clone());
    for value in values.iter().cloned() {
        vec.push(value);
    }

    let expected = values.len();
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
                fn prop_push_contains(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::TypedProjVec<$typ, alloc::Global> = values;
                    super::prop_push_contains(values)?
                }

                #[test]
                fn prop_push_get(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::TypedProjVec<$typ, alloc::Global> = values;
                    super::prop_push_get(values)?
                }

                #[test]
                fn prop_push_len(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::TypedProjVec<$typ, alloc::Global> = values;
                    super::prop_push_len(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), 128, strategy_type_projected_vec_max_len);
generate_props!(u8, u8, 128, strategy_type_projected_vec_max_len);
generate_props!(u16, u16, 128, strategy_type_projected_vec_max_len);
generate_props!(u32, u32, 128, strategy_type_projected_vec_max_len);
generate_props!(u64, u64, 128, strategy_type_projected_vec_max_len);
generate_props!(usize, usize, 128, strategy_type_projected_vec_max_len);
generate_props!(string, String, 128, strategy_type_projected_vec_max_len);
