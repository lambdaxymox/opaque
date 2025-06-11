use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use core::ops;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_opaque_vec_from_array<const N: usize, T>(expected: [T; N])
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
{
    let vec = OpaqueVec::from(expected.clone());
    let result = vec.as_slice::<T, alloc::Global>();

    assert_eq!(result, expected);
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, lengths = { $($len:expr),+ }, $range_spec:expr, $alt_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_vec_from_array_range_values() {
                $(
                    {
                        let values = opaque_vec_testing::range_values::<$typ, $len>($range_spec);
                        run_test_opaque_vec_from_array(values);
                    }
                )+
            }

            #[test]
            fn test_opaque_vec_from_array_alternating_values() {
                $(
                    {
                        let values = opaque_vec_testing::alternating_values::<$typ, $len>($alt_spec);
                        run_test_opaque_vec_from_array(values);
                    }
                )+
            }
        }
    };
}

generate_tests!(
    u8,
    u8,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, 0)
);
generate_tests!(
    u16,
    u16,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, 0)
);
generate_tests!(
    u32,
    u32,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, 0)
);
generate_tests!(
    u64,
    u64,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, 0)
);
generate_tests!(
    usize,
    usize,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, 0)
);
generate_tests!(
    string,
    String,
    lengths = { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 32, 64, 128 },
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ovt::StringRangeFrom::new(0))),
    opaque_vec_testing::AlternatingValuesSpec::new(String::from("foo"), String::from("bar"))
);
