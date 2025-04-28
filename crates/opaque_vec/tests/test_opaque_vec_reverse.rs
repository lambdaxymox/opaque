use opaque_vec::OpaqueVec;

use core::fmt;

fn expected<T>(values: &[T]) -> OpaqueVec
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::new::<T>();
    for value in values.iter().rev().cloned() {
        vec.push::<T>(value);
    }

    vec
}

fn result<T>(values: &[T]) -> OpaqueVec
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::from(values);
    vec.reverse::<T>();

    vec
}

fn run_test_opaque_vec_reverse<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let expected = expected(values);
    let result = result(values);

    assert_eq!(result, expected);
}

fn run_test_opaque_vec_reverse_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    for len in 0..values.len() {
        let prefix_values = &values[0..len];
        run_test_opaque_vec_reverse(prefix_values);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_reverse_range_values() {
                let values = opaque_vec_testing::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_vec_reverse_values(&values);
            }

            #[test]
            fn test_opaque_vec_reverse_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_vec_reverse_values(&values);
            }
        }
    };
}

generate_tests!(i8, 128, opaque_vec_testing::RangeValuesSpec::new(0), opaque_vec_testing::AlternatingValuesSpec::new(i8::MIN, 0));
generate_tests!(i16, 1024, opaque_vec_testing::RangeValuesSpec::new(0), opaque_vec_testing::AlternatingValuesSpec::new(i16::MIN, 0));
generate_tests!(i32, 1024, opaque_vec_testing::RangeValuesSpec::new(0), opaque_vec_testing::AlternatingValuesSpec::new(i32::MIN, 0));
generate_tests!(i64, 1024, opaque_vec_testing::RangeValuesSpec::new(0), opaque_vec_testing::AlternatingValuesSpec::new(i64::MIN, 0));
generate_tests!(
    i128,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(i128::MIN, 0)
);
generate_tests!(
    isize,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(isize::MIN, 0)
);

generate_tests!(
    u8,
    128,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX)
);
generate_tests!(
    u16,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX)
);
generate_tests!(
    u32,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX)
);
generate_tests!(
    u64,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX)
);
generate_tests!(
    u128,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u128::MIN, u128::MAX)
);
generate_tests!(
    usize,
    1024,
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX)
);
