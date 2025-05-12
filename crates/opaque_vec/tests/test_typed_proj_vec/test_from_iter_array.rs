use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

fn run_test_typed_proj_vec_from_iter_array<const N: usize, T>(expected: [T; N])
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
{
    let vec = TypedProjVec::from_iter(expected.iter().cloned());
    let result = vec.as_slice();

    assert_eq!(result, expected);
}

#[test]
fn test_typed_proj_vec_from_iter_array_empty() {
    let values: [i32; 0] = [];
    let mut vec = TypedProjVec::from_iter(values);

    let expected = values.as_slice();
    let result = vec.as_slice();

    assert_eq!(result, expected);
}

macro_rules! generate_tests {
    ($typ:ident, lengths = { $($len:expr),+ }, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_typed_proj_vec_from_array_range_values() {
                $(
                    {
                        let values = opaque_vec_testing::range_values::<$typ, $len>($range_spec);
                        run_test_typed_proj_vec_from_iter_array(values);
                    }
                )+
            }

            #[test]
            fn test_typed_proj_vec_from_array_alternating_values() {
                $(
                    {
                        let values = opaque_vec_testing::alternating_values::<$typ, $len>($alt_spec);
                        run_test_typed_proj_vec_from_iter_array(values);
                    }
                )+
            }
        }
    };
}

generate_tests!(
    u8,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, 0)
);
generate_tests!(
    u16,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, 0)
);
generate_tests!(
    u32,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, 0)
);
generate_tests!(
    u64,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, 0)
);
generate_tests!(
    usize,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    opaque_vec_testing::RangeValuesSpec::new(0),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, 0)
);
