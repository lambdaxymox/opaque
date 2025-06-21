use crate::common;

use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use core::iter;
use core::ops;
use std::alloc;

use opaque_vec_testing as ovt;

fn create_drain_vec<T, A>(drain_value: T, length: usize, alloc: A) -> OpaqueVec
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = OpaqueVec::with_capacity_in::<T, A>(length, alloc);
    for _ in 0..length {
        vec.push::<T, A>(drain_value.clone());
    }

    vec
}

fn run_test_opaque_vec_drain_partial_vec<T, A>(values: &[T], drain_value: T, count: usize, index: usize, alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let drained_expected = create_drain_vec(drain_value, count, alloc.clone());
    let expected = common::opaque_vec::from_slice_in(values, alloc.clone());
    let mut result = common::opaque_vec::shift_insert_slice(values, drained_expected.as_slice::<T ,A>(), index, alloc.clone());
    let drained_result = {
        let mut _vec = OpaqueVec::with_capacity_in::<T, A>(count, alloc.clone());
        for value in result.drain::<_, T, A>(index..(index + count)) {
            _vec.push::<T, A>(value);
        }

        _vec
    };

    assert_eq!(result.as_slice::<T, A>(), expected.as_slice::<T, A>());
    assert_eq!(drained_result.as_slice::<T, A>(), drained_expected.as_slice::<T, A>());
}

fn run_test_opaque_vec_drain_partial_vec_values<T, A>(values: &[T], drain_value: T, max_count: usize, alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    for i in 0..values.len() {
        for count in 0..max_count {
            run_test_opaque_vec_drain_partial_vec(values, drain_value.clone(), count, i, alloc.clone());
        }
    }
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, $max_vec_size:expr, $drain_value:expr, $max_count:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_vec_drain_partial_vec_range_values() {
                let values = ovt::range_values::<$typ, $max_vec_size>($range_spec);
                let drain_value: $typ = $drain_value;
                let max_count = $max_count;
                let alloc = alloc::Global;
                run_test_opaque_vec_drain_partial_vec_values(&values, drain_value, max_count, alloc);
            }

            #[test]
            fn test_opaque_vec_drain_partial_vec_alternating_values() {
                let values = ovt::alternating_values::<$typ, $max_vec_size>($alt_spec);
                let drain_value: $typ = $drain_value;
                let max_count = $max_count;
                let alloc = alloc::Global;
                run_test_opaque_vec_drain_partial_vec_values(&values, drain_value, max_count, alloc);
            }
        }
    };
}

generate_tests!(
    unit,
    (),
    128,
    (),
    16,
    ovt::RangeValuesSpec::new(Box::new(iter::repeat(()))),
    ovt::AlternatingValuesSpec::new((), ())
);
generate_tests!(
    u8,
    u8,
    128,
    u8::MAX,
    16,
    ovt::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    ovt::AlternatingValuesSpec::new(u8::MIN, u8::MAX)
);
generate_tests!(
    u16,
    u16,
    128,
    u16::MAX,
    16,
    ovt::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    ovt::AlternatingValuesSpec::new(u16::MIN, u16::MAX)
);
generate_tests!(
    u32,
    u32,
    128,
    u32::MAX,
    16,
    ovt::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    ovt::AlternatingValuesSpec::new(u32::MIN, u32::MAX)
);
generate_tests!(
    u64,
    u64,
    128,
    u64::MAX,
    16,
    ovt::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    ovt::AlternatingValuesSpec::new(u64::MIN, u64::MAX)
);
generate_tests!(
    usize,
    usize,
    128,
    usize::MAX,
    16,
    ovt::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    ovt::AlternatingValuesSpec::new(usize::MIN, usize::MAX)
);
generate_tests!(
    string,
    String,
    128,
    usize::MAX.to_string(),
    16,
    ovt::RangeValuesSpec::new(Box::new(ovt::StringRangeFrom::new(0))),
    ovt::AlternatingValuesSpec::new(String::from("foo"), String::from("bar"))
);
