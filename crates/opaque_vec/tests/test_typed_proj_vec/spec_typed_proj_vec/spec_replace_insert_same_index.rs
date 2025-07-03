use crate::common::projected::strategy_alloc;
use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn strategy_single_value<T>() -> impl Strategy<Value = T>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
{
    any::<T>()
}

fn prop_replace_insert_contains_same_index1<T, A>(value: T, alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_in(alloc);

    prop_assert!(!vec.contains(&value));

    vec.replace_insert(0, value.clone());

    prop_assert!(vec.contains(&value));

    Ok(())
}

fn prop_replace_insert_contains_same_index2<T, A>(initial_value: T, value: T, alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_in(alloc);
    vec.replace_insert(0, initial_value.clone());

    prop_assert!(vec.contains(&initial_value));

    for _ in 0..65536 {
        vec.replace_insert(0, value.clone());

        prop_assert!(vec.contains(&value));
    }

    Ok(())
}

fn prop_replace_insert_get_same_index1<T, A>(value: T, alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_in(alloc);
    vec.replace_insert(0, value.clone());

    let expected = Some(value.clone());
    let result = vec.get(0).cloned();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_replace_insert_get_same_index2<T, A>(initial_value: T, value: T, alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_in(alloc);
    vec.replace_insert(0, initial_value.clone());

    let expected_initial = Some(initial_value.clone());
    let result_initial = vec.get(0).cloned();
    prop_assert_eq!(result_initial, expected_initial);

    for _ in 0..65536 {
        vec.replace_insert(0, value.clone());
        let expected = Some(value.clone());
        let result = vec.get(0).cloned();

        prop_assert_eq!(result, expected);
    }

    Ok(())
}
fn prop_replace_insert_len_same_index<T, A>(value: T, alloc: A) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut vec = TypedProjVec::new_in(alloc);

    prop_assert!(vec.is_empty());

    for _ in 0..65536 {
        vec.replace_insert(0, value.clone());
    }

    prop_assert_eq!(vec.len(), 1);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $initial_value_gen:ident, $value_gen:ident, $alloc_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_replace_insert_contains_same_index1(
                    value in super::$value_gen::<$typ>(),
                    alloc in super::$alloc_gen::<$alloc_typ>(),
                ) {
                    let value: $typ = value;
                    let alloc: $alloc_typ = alloc;
                    super::prop_replace_insert_contains_same_index1(value, alloc)?
                }

                #[test]
                fn prop_replace_insert_contains_same_index2(
                    initial_value in super::$initial_value_gen::<$typ>(),
                    value in super::$value_gen::<$typ>(),
                    alloc in super::$alloc_gen::<$alloc_typ>(),
                ) {
                    let initial_value: $typ = initial_value;
                    let value: $typ = value;
                    let alloc: $alloc_typ = alloc;
                    super::prop_replace_insert_contains_same_index2(initial_value, value, alloc)?
                }

                #[test]
                fn prop_replace_insert_get_same_index1(
                    value in super::$value_gen::<$typ>(),
                    alloc in super::$alloc_gen::<$alloc_typ>(),
                ) {
                    let value: $typ = value;
                    let alloc: $alloc_typ = alloc;
                    super::prop_replace_insert_get_same_index1(value, alloc)?
                }

                #[test]
                fn prop_replace_insert_get_same_index2(
                    initial_value in super::$initial_value_gen::<$typ>(),
                    value in super::$value_gen::<$typ>(),
                    alloc in super::$alloc_gen::<$alloc_typ>(),
                ) {
                    let initial_value: $typ = initial_value;
                    let value: $typ = value;
                    let alloc: $alloc_typ = alloc;
                    super::prop_replace_insert_get_same_index2(initial_value, value, alloc)?
                }

                #[test]
                fn prop_replace_insert_len_same_index(
                    value in super::$value_gen::<$typ>(),
                    alloc in super::$alloc_gen::<$alloc_typ>(),
                ) {
                    let value: $typ = value;
                    let alloc: $alloc_typ = alloc;
                    super::prop_replace_insert_len_same_index(value, alloc)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, strategy_single_value, strategy_single_value, strategy_alloc);
generate_props!(u8, u8, alloc::Global, strategy_single_value, strategy_single_value, strategy_alloc);
generate_props!(u16, u16, alloc::Global, strategy_single_value, strategy_single_value, strategy_alloc);
generate_props!(u32, u32, alloc::Global, strategy_single_value, strategy_single_value, strategy_alloc);
generate_props!(u64, u64, alloc::Global, strategy_single_value, strategy_single_value, strategy_alloc);
generate_props!(usize, usize, alloc::Global, strategy_single_value, strategy_single_value, strategy_alloc);
generate_props!(string, String, alloc::Global, strategy_single_value, strategy_single_value, strategy_alloc);
