use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

use opaque_vec_testing as ovt;
use crate::common;

fn create_drain_vec<T, A>(drain_value: T, length: usize, alloc: A) -> TypedProjVec<T, A>
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::with_capacity_in(length, alloc);
    for _ in 0..length {
        vec.push(drain_value.clone());
    }

    vec
}

fn run_test_typed_proj_vec_drain_partial_vec<T, A>(values: &[T], drain_value: T, count: usize, index: usize, alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let drained_expected = create_drain_vec(drain_value, count, alloc.clone());
    let expected = common::typed_proj_vec::from_slice_in(values, alloc.clone());
    let mut result = common::typed_proj_vec::shift_insert_slice(values, drained_expected.as_slice(), index, alloc.clone());
    let drained_result = {
        let mut _vec = TypedProjVec::with_capacity_in(count, alloc.clone());
        for value in result.drain(index..(index + count)) {
            _vec.push(value);
        }

        _vec
    };

    assert_eq!(result, expected);
    assert_eq!(drained_result, drained_expected);
}

fn run_test_typed_proj_vec_drain_partial_vec_values<T, A>(values: &[T], drain_value: T, max_count: usize, alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    for i in 0..values.len() {
        for count in 0..max_count {
            run_test_typed_proj_vec_drain_partial_vec(values, drain_value.clone(), count, i, alloc.clone());
        }
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_vec_size:expr, $drain_value:expr, $max_count:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_typed_proj_vec_drain_partial_vec_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_vec_size>($range_spec);
                let drain_value: $typ = $drain_value;
                let max_count = $max_count;
                let alloc = alloc::Global;
                run_test_typed_proj_vec_drain_partial_vec_values(&values, drain_value, max_count, alloc);
            }

            #[test]
            fn test_typed_proj_vec_drain_partial_vec_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_vec_size>($alt_spec);
                let drain_value: $typ = $drain_value;
                let max_count = $max_count;
                let alloc = alloc::Global;
                run_test_typed_proj_vec_drain_partial_vec_values(&values, drain_value, max_count, alloc);
            }
        }
    };
}

generate_tests!(
    u8,
    128,
    u8::MAX,
    16,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX)
);
generate_tests!(
    u16,
    128,
    u16::MAX,
    16,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX)
);
generate_tests!(
    u32,
    128,
    u32::MAX,
    16,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX)
);
generate_tests!(
    u64,
    128,
    u64::MAX,
    16,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX)
);
generate_tests!(
    usize,
    128,
    usize::MAX,
    16,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX)
);
