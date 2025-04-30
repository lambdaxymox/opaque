#![feature(allocator_api)]
mod common;

use core::fmt;

use opaque_vec_testing as ovt;

fn run_test_opaque_blob_vec_clear_is_empty<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut opaque_blob_vec = common::from_typed_slice(values);
    opaque_blob_vec.clear();

    assert!(opaque_blob_vec.is_empty());
}

fn run_test_opaque_blob_vec_clear_is_empty_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_opaque_blob_vec_clear_is_empty(slice);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_blob_vec_clear_is_empty_empty() {
                let values: [$typ; 0] = [];

                run_test_opaque_blob_vec_clear_is_empty(&values);
            }

            #[test]
            fn test_opaque_blob_vec_clear_is_empty_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_blob_vec_clear_is_empty_values(&values);
            }
        }
    };
}

generate_tests!(u8, 128, opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX));
generate_tests!(u16, 1024, opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX));
generate_tests!(u32, 1024, opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX));
generate_tests!(u64, 1024, opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX));
generate_tests!(u128, 1024, opaque_vec_testing::AlternatingValuesSpec::new(u128::MIN, u128::MAX));
generate_tests!(usize, 1024, opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX));
