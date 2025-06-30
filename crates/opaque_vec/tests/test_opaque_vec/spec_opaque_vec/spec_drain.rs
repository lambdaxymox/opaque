use crate::common;
use crate::common::erased::{
    strategy_type_erased_vec_len,
    strategy_type_erased_vec_max_len,
};
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

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
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleDrainValue,
{
    Just(<T as SingleDrainValue>::drain_value())
}

fn strategy_prop_drain_partial_vec<T, A>(max_length: usize, max_count: usize) -> impl Strategy<Value = (OpaqueVec, T, usize, usize)>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug + Arbitrary + SingleDrainValue,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    (1..=max_length).prop_flat_map(move |length|
        (strategy_type_erased_vec_len::<T, A>(length), strategy_single_drain_value(), 0..=max_count, 0..length)
    )
}

fn prop_drain_entire_vec<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let expected = values.clone::<T, A>();
    let result = {
        let mut _vec = values.clone::<T, A>();
        let mut _result = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
        for value in _vec.drain::<_, T, A>(..) {
            _result.push::<T, A>(value);
        }

        _result
    };

    prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());

    Ok(())
}

fn prop_drain_nothing_vec<T, A>(values: OpaqueVec) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    for start in 0..values.len() {
        let mut result = {
            let mut _result = OpaqueVec::with_capacity_proj_in::<T, A>(values.len(), values.allocator::<T, A>().clone());
            _result.extend::<_, T, A>(values.iter::<T, A>().cloned());
            _result
        };
        let expected = result.clone::<T, A>();

        let mut drained_result = OpaqueVec::new_proj_in::<T, A>(values.allocator::<T, A>().clone());
        drained_result.extend::<_, T, A>(result.drain::<_, T, A>(start..start));

        prop_assert!(drained_result.is_empty());
        prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());
    }

    Ok(())
}

fn prop_drain_partial_vec<T, A>((values, drain_value, count, index): (OpaqueVec, T, usize, usize)) -> Result<(), TestCaseError>
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
{
    fn drained_expected<T, A>(drain_value: T, length: usize, alloc: A) -> OpaqueVec
    where
        T: any::Any + PartialEq + Clone + Default + fmt::Debug,
        A: any::Any + alloc::Allocator + Send + Sync + Clone + Default + fmt::Debug,
    {
        let mut vec = OpaqueVec::with_capacity_in::<T, A>(length, alloc);
        for _ in 0..length {
            vec.push::<T, A>(drain_value.clone());
        }

        vec
    }

    let drained_expected = drained_expected(drain_value, count, values.allocator::<T, A>().allocator().clone());
    let expected = values.clone::<T, A>();

    let mut result = common::erased::shift_insert_slice(
        values.as_slice::<T, A>(),
        drained_expected.as_slice::<T, A>(),
        index,
        values.allocator::<T, A>().allocator().clone(),
    );
    let drained_result = {
        let mut _vec = OpaqueVec::with_capacity_proj_in::<T, A>(count, values.allocator::<T, A>().clone());
        for value in result.drain::<_, T, A>(index..(index + count)) {
            _vec.push::<T, A>(value);
        }

        _vec
    };

    prop_assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());
    prop_assert_eq!(drained_result.as_slice::<T, A>(), drained_expected.as_slice::<T, A>());

    Ok(())
}

macro_rules! generate_props {
    ($module_name:ident, $typ:ty, $max_length:expr, $max_count:expr, $vec_gen:ident) => {
        mod $module_name {
            use proptest::prelude::*;
            use std::alloc;
            proptest! {
                #[test]
                fn prop_drain_entire_vec(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::OpaqueVec = values;
                    super::prop_drain_entire_vec::<$typ, alloc::Global>(values)?
                }

                #[test]
                fn prop_drain_nothing_vec(values in super::$vec_gen::<$typ, alloc::Global>($max_length)) {
                    let values: super::OpaqueVec  = values;
                    super::prop_drain_nothing_vec::<$typ, alloc::Global>(values)?
                }

                #[test]
                fn prop_drain_partial_vec(values in super::strategy_prop_drain_partial_vec::<$typ, alloc::Global>($max_length, $max_count)) {
                    let values: (super::OpaqueVec, $typ, usize, usize) = values;
                    super::prop_drain_partial_vec::<$typ, alloc::Global>(values)?
                }
            }
        }
    };
}

generate_props!(unit, (), 128, 16, strategy_type_erased_vec_max_len);
generate_props!(u8, u8, 128, 16, strategy_type_erased_vec_max_len);
generate_props!(u16, u16, 128, 16, strategy_type_erased_vec_max_len);
generate_props!(u32, u32, 128, 16, strategy_type_erased_vec_max_len);
generate_props!(u64, u64, 128, 16, strategy_type_erased_vec_max_len);
generate_props!(usize, usize, 128, 16, strategy_type_erased_vec_max_len);
generate_props!(string, String, 128, 16, strategy_type_erased_vec_max_len);
