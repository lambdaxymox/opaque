use crate::common;

use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_opaque_vec_truncate_len_length_greater_than_or_equal_to<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let base_opaque_vec = common::opaque_vec::from_slice_in(values, alloc);
    let min_len = values.len();
    let max_len = 10 * values.len();
    for len in min_len..max_len {
        let mut opaque_vec = base_opaque_vec.clone::<T, A>();

        opaque_vec.truncate::<T, A>(len);

        let expected = values.len();
        let result = opaque_vec.len();

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_vec_truncate_len_length_greater_than_or_equal_to_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug + TryFrom<usize>,
    <T as TryFrom<usize>>::Error: fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_opaque_vec_truncate_len_length_greater_than_or_equal_to(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_truncate_len_length_greater_than_or_equal_to_empty() {
                let values: [$typ; 0] = [];
                let alloc = alloc::Global;
                run_test_opaque_vec_truncate_len_length_greater_than_or_equal_to(&values, alloc);
            }

            #[test]
            fn test_opaque_vec_truncate_len_length_greater_than_or_equal_to_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                let alloc = alloc::Global;
                run_test_opaque_vec_truncate_len_length_greater_than_or_equal_to_values(&values, alloc);
            }

            #[test]
            fn test_opaque_vec_truncate_len_length_greater_than_or_equal_to_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_opaque_vec_truncate_len_length_greater_than_or_equal_to_values(&values, alloc);
            }
        }
    };
}

generate_tests!(
    u8,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX)
);
generate_tests!(
    u16,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX)
);
generate_tests!(
    u32,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX)
);
generate_tests!(
    u64,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX)
);
generate_tests!(
    usize,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX)
);
