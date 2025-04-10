mod common;

use opaque_vec::OpaqueVec;

use core::fmt;

use common::array_generators as ag;

fn run_test_opaque_vec_from_iter_array<const N: usize, T>(expected: [T; N])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let vec = OpaqueVec::from_iter(expected.iter().cloned());
    let result = vec.as_slice::<T>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_from_iter_array_empty() {
    let values: [i32; 0] = [];
    let mut vec = OpaqueVec::from_iter(values);

    let expected = values.as_slice();
    let result = vec.as_slice::<i32>();

    assert_eq!(result, expected);
}

macro_rules! generate_tests {
    ($typ:ident, lengths = { $($len:expr),+ }, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_from_array_range_values() {
                $(
                    {
                        let values = ag::range_values::<$typ, $len>($range_spec);
                        run_test_opaque_vec_from_iter_array(values);
                    }
                )+
            }

            #[test]
            fn test_opaque_vec_from_array_alternating_values() {
                $(
                    {
                        let values = ag::alternating_values::<$typ, $len>($alt_spec);
                        run_test_opaque_vec_from_iter_array(values);
                    }
                )+
            }
        }
    };
}

generate_tests!(
    i8,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    ag::RangeValuesSpec::new(0),
    ag::AlternatingValuesSpec::new(i8::MIN, 0)
);
generate_tests!(
    i16,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512, 1024 },
    ag::RangeValuesSpec::new(0),
    ag::AlternatingValuesSpec::new(i16::MIN, 0)
);
generate_tests!(
    i32,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512, 1024 },
    ag::RangeValuesSpec::new(0),
    ag::AlternatingValuesSpec::new(i32::MIN, 0)
);
generate_tests!(
    i64,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512, 1024 },
    ag::RangeValuesSpec::new(0),
    ag::AlternatingValuesSpec::new(i64::MIN, 0)
);
generate_tests!(
    i128,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512, 1024 },
    ag::RangeValuesSpec::new(0),
    ag::AlternatingValuesSpec::new(i128::MIN, 0)
);
generate_tests!(
    isize,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512, 1024 },
    ag::RangeValuesSpec::new(0),
    ag::AlternatingValuesSpec::new(isize::MIN, 0)
);

generate_tests!(
    u8,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    ag::RangeValuesSpec::new(0),
    ag::AlternatingValuesSpec::new(u8::MIN, 0)
);
generate_tests!(
    u16,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512, 1024 },
    ag::RangeValuesSpec::new(0),
    ag::AlternatingValuesSpec::new(u16::MIN, 0)
);
generate_tests!(
    u32,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512, 1024 },
    ag::RangeValuesSpec::new(0),
    ag::AlternatingValuesSpec::new(u32::MIN, 0)
);
generate_tests!(
    u64,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512, 1024 },
    ag::RangeValuesSpec::new(0),
    ag::AlternatingValuesSpec::new(u64::MIN, 0)
);
generate_tests!(
    u128,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512, 1024 },
    ag::RangeValuesSpec::new(0),
    ag::AlternatingValuesSpec::new(u128::MIN, 0)
);
generate_tests!(
    usize,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128, 256, 512, 1024 },
    ag::RangeValuesSpec::new(0),
    ag::AlternatingValuesSpec::new(usize::MIN, 0)
);
