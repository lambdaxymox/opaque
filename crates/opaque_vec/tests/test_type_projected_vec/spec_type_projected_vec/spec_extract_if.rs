use crate::common::projected::strategy_type_projected_vec_max_len;
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

fn prop_extract_if_len<F, T, A>(values: TypeProjectedVec<T, A>, filter: F) -> Result<(), TestCaseError>
where
    F: Fn(&T) -> bool,
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut remaining = values.clone();
    let mut extracted = TypeProjectedVec::new_proj_in(values.allocator().clone());
    for value in remaining.extract_if(.., |v| filter(v)) {
        extracted.push(value);
    }

    prop_assert_eq!(extracted.len() + remaining.len(), values.len());

    Ok(())
}

fn prop_extract_if_extracted<F, T, A>(values: TypeProjectedVec<T, A>, filter: F) -> Result<(), TestCaseError>
where
    F: Fn(&T) -> bool,
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut remaining = values.clone();
    let mut extracted = TypeProjectedVec::new_proj_in(values.allocator().clone());
    for value in remaining.extract_if(.., |v| filter(v)) {
        extracted.push(value);
    }

    for value in extracted.iter() {
        prop_assert!(filter(value));
    }

    Ok(())
}

fn prop_extract_if_remaining<F, T, A>(values: TypeProjectedVec<T, A>, filter: F) -> Result<(), TestCaseError>
where
    F: Fn(&T) -> bool,
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    let mut remaining = values.clone();
    let mut extracted = TypeProjectedVec::new_proj_in(values.allocator().clone());
    for value in remaining.extract_if(.., |v| filter(v)) {
        extracted.push(value);
    }

    for value in remaining.iter() {
        prop_assert!(!filter(value));
    }

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $vec_gen:ident, $filter:expr) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_extract_if_len(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    super::prop_extract_if_len(values, $filter)?
                }

                #[test]
                fn prop_extract_if_extracted(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    super::prop_extract_if_extracted(values, $filter)?
                }

                #[test]
                fn prop_extract_if_remaining(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypeProjectedVec<$typ, $alloc_typ> = values;
                    super::prop_extract_if_remaining(values, $filter)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 32, strategy_type_projected_vec_max_len, |v| { true });
generate_props!(u8, u8, alloc::Global, 32, strategy_type_projected_vec_max_len, |v| { v % 2 == 0 });
generate_props!(u16, u16, alloc::Global, 32, strategy_type_projected_vec_max_len, |v| { v % 2 == 0 });
generate_props!(u32, u32, alloc::Global, 32, strategy_type_projected_vec_max_len, |v| { v % 2 == 0 });
generate_props!(u64, u64, alloc::Global, 32, strategy_type_projected_vec_max_len, |v| { v % 2 == 0 });
generate_props!(usize, usize, alloc::Global, 32, strategy_type_projected_vec_max_len, |v| { v % 2 == 0 });
generate_props!(string, String, alloc::Global, 32, strategy_type_projected_vec_max_len, |v| { v.len() % 2 == 0 });
