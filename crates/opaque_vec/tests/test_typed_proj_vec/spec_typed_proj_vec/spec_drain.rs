use crate::common;
use crate::common::projected::{
    SingleBoundedValue,
    strategy_type_projected_vec_len,
    strategy_type_projected_vec_max_len,
};
use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::format;
use std::string::{String, ToString};

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

use proptest::prelude::*;

trait SingleDrainValue {
    fn drain_value() -> Self;
}

impl SingleDrainValue for () { fn drain_value() -> Self { () } }
impl SingleDrainValue for u8 { fn drain_value() -> Self { u8::MAX } }
impl SingleDrainValue for u16 { fn drain_value() -> Self { u16::MAX } }
impl SingleDrainValue for u32 { fn drain_value() -> Self { u32::MAX } }
impl SingleDrainValue for u64 { fn drain_value() -> Self { u64::MAX } }
impl SingleDrainValue for usize { fn drain_value() -> Self { usize::MAX } }
impl SingleDrainValue for String { fn drain_value() -> Self { usize::MAX.to_string() } }

fn strategy_single_drain_value<T>() -> impl Strategy<Value = T>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleBoundedValue + SingleDrainValue,
{
    Just(<T as SingleDrainValue>::drain_value())
}

fn strategy_prop_drain_partial_vec<T, A>(max_length: usize, max_count: usize) -> impl Strategy<Value = (TypedProjVec<T, A>, T, usize, usize)>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleBoundedValue + SingleDrainValue,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (1..=max_length).prop_flat_map(move |length|
        (strategy_type_projected_vec_len(length), strategy_single_drain_value(), 0..=max_count, 0..length)
    )
}

fn prop_drain_entire_vec<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let expected = values.clone();
    let result = {
        let mut _vec = values.clone();
        let mut _result = TypedProjVec::new_proj_in(values.allocator().clone());
        for value in _vec.drain(..) {
            _result.push(value);
        }

        _result
    };

    prop_assert_eq!(result, expected);

    Ok(())
}

fn prop_drain_nothing_vec<T, A>(values: TypedProjVec<T, A>) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    for start in 0..values.len() {
        let mut result = {
            let mut _result = TypedProjVec::with_capacity_proj_in(values.len(), values.allocator().clone());
            _result.extend(values.iter().cloned());
            _result
        };
        let expected = result.clone();

        let mut drained_result = TypedProjVec::new_proj_in(values.allocator().clone());
        drained_result.extend(result.drain(start..start));

        prop_assert!(drained_result.is_empty());
        prop_assert_eq!(result, expected);
    }

    Ok(())
}

fn prop_drain_partial_vec<T, A>((values, drain_value, count, index): (TypedProjVec<T, A>, T, usize, usize)) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn drained_expected<T, A>(drain_value: T, length: usize, alloc: A) -> TypedProjVec<T, A>
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut vec = TypedProjVec::with_capacity_in(length, alloc);
        for _ in 0..length {
            vec.push(drain_value.clone());
        }

        vec
    }

    let drained_expected = drained_expected(drain_value, count, values.allocator().allocator().clone());
    let expected = values.clone();

    let mut result = common::projected::shift_insert_slice(
        values.as_slice(),
        drained_expected.as_slice(),
        index,
        values.allocator().allocator().clone(),
    );
    let drained_result = {
        let mut _vec = TypedProjVec::with_capacity_proj_in(count, values.allocator().clone());
        for value in result.drain(index..(index + count)) {
            _vec.push(value);
        }

        _vec
    };

    prop_assert_eq!(result, expected);
    prop_assert_eq!(drained_result, drained_expected);

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $alloc_typ:ty, $max_length:expr, $max_count:expr, $vec_gen:ident) => {
        mod $module_name {
            use super::*;
            proptest! {
                #[test]
                fn prop_drain_entire_vec(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_drain_entire_vec(values)?
                }

                #[test]
                fn prop_drain_nothing_vec(values in super::$vec_gen::<$typ, $alloc_typ>($max_length)) {
                    let values: super::TypedProjVec<$typ, $alloc_typ> = values;
                    super::prop_drain_nothing_vec(values)?
                }

                #[test]
                fn prop_drain_partial_vec(values in super::strategy_prop_drain_partial_vec::<$typ, $alloc_typ>($max_length, $max_count)) {
                    let values: (super::TypedProjVec<$typ, $alloc_typ>, $typ, usize, usize) = values;
                    super::prop_drain_partial_vec(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), alloc::Global, 32, 16, strategy_type_projected_vec_max_len);
generate_props!(u8, u8, alloc::Global, 32, 16, strategy_type_projected_vec_max_len);
generate_props!(u16, u16, alloc::Global, 32, 16, strategy_type_projected_vec_max_len);
generate_props!(u32, u32, alloc::Global, 32, 16, strategy_type_projected_vec_max_len);
generate_props!(u64, u64, alloc::Global, 32, 16, strategy_type_projected_vec_max_len);
generate_props!(usize, usize, alloc::Global, 32, 16, strategy_type_projected_vec_max_len);
generate_props!(string, String, alloc::Global, 32, 16, strategy_type_projected_vec_max_len);
