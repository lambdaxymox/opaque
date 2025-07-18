use crate::common::erased::strategy_type_erased_vec_max_len;
use opaque_vec::TypeErasedVec;

use core::any;
use core::fmt;
use std::format;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

fn prop_append_as_slice_source<T, A>(values1: TypeErasedVec, values2: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut source = values1.clone::<T, A>();
    let mut destination = values2.clone::<T, A>();

    source.append::<T, A>(&mut destination);

    for i in 0..values1.len() {
        prop_assert_eq!(&source.as_slice::<T, A>()[i], &values1.as_slice::<T, A>()[i]);
    }

    for j in 0..values2.len() {
        prop_assert_eq!(&source.as_slice::<T, A>()[values1.len() + j], &values2.as_slice::<T, A>()[j]);
    }

    Ok(())
}

fn prop_append_as_slice_destination<T, A>(values1: TypeErasedVec, values2: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut source = values1.clone::<T, A>();
    let mut destination = values2.clone::<T, A>();

    source.append::<T, A>(&mut destination);

    prop_assert!(destination.is_empty());

    Ok(())
}

fn prop_append_len_source<T, A>(values1: TypeErasedVec, values2: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut source = values1.clone::<T, A>();
    let mut destination = values2.clone::<T, A>();

    source.append::<T, A>(&mut destination);

    let expected = values1.len() + values2.len();
    let result = source.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_append_len_destination<T, A>(values1: TypeErasedVec, values2: TypeErasedVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut source = values1.clone::<T, A>();
    let mut destination = values2.clone::<T, A>();

    source.append::<T, A>(&mut destination);

    let expected = 0;
    let result = destination.len();

    prop_assert_eq!(result, expected);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_append_as_slice_source(
                    values1 in super::$vec_gen::<$typ, $alloc_typ>($max_length),
                    values2 in super::$vec_gen::<$typ, $alloc_typ>($max_length),
                ) {
                    let values1: super::TypeErasedVec = values1;
                    let values2: super::TypeErasedVec = values2;
                    super::prop_append_as_slice_source::<$typ, $alloc_typ>(values1, values2)?
                }

                #[test]
                fn prop_append_as_slice_destination(
                    values1 in super::$vec_gen::<$typ, $alloc_typ>($max_length),
                    values2 in super::$vec_gen::<$typ, $alloc_typ>($max_length),
                ) {
                    let values1: super::TypeErasedVec = values1;
                    let values2: super::TypeErasedVec = values2;
                    super::prop_append_as_slice_destination::<$typ, $alloc_typ>(values1, values2)?
                }

                #[test]
                fn prop_append_len_source(
                    values1 in super::$vec_gen::<$typ, $alloc_typ>($max_length),
                    values2 in super::$vec_gen::<$typ, $alloc_typ>($max_length),
                ) {
                    let values1: super::TypeErasedVec = values1;
                    let values2: super::TypeErasedVec = values2;
                    super::prop_append_len_source::<$typ, $alloc_typ>(values1, values2)?
                }

                #[test]
                fn prop_append_len_destination(
                    values1 in super::$vec_gen::<$typ, $alloc_typ>($max_length),
                    values2 in super::$vec_gen::<$typ, $alloc_typ>($max_length),
                ) {
                    let values1: super::TypeErasedVec = values1;
                    let values2: super::TypeErasedVec = values2;
                    super::prop_append_len_destination::<$typ, $alloc_typ>(values1, values2)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 32, strategy_type_erased_vec_max_len);
generate_props!(u8, u8, alloc::Global, 32, strategy_type_erased_vec_max_len);
generate_props!(u16, u16, alloc::Global, 32, strategy_type_erased_vec_max_len);
generate_props!(u32, u32, alloc::Global, 32, strategy_type_erased_vec_max_len);
generate_props!(u64, u64, alloc::Global, 32, strategy_type_erased_vec_max_len);
generate_props!(usize, usize, alloc::Global, 32, strategy_type_erased_vec_max_len);
generate_props!(string, String, alloc::Global, 32, strategy_type_erased_vec_max_len);
