use crate::common;

use core::any;
use core::fmt;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_opaque_blob_vec_shift_remove_forget_unchecked_end<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut opaque_blob_vec = common::from_typed_slice_in(values, alloc);

    let last_index = opaque_blob_vec.len() - 1;
    let expected = &values[0..last_index];
    let _ = unsafe {
        let ptr = opaque_blob_vec.shift_remove_forget_unchecked(last_index).cast::<T>();
        ptr.read()
    };
    let result = common::as_slice::<T>(&opaque_blob_vec);

    assert_eq!(result, expected);
}

fn run_test_opaque_blob_vec_shift_remove_forget_unchecked_end_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let iter = ovt::PrefixGenerator::new_only_nonempty(values);
    for slice in iter {
        run_test_opaque_blob_vec_shift_remove_forget_unchecked_end(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_blob_vec_shift_remove_forget_unchecked_end_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_opaque_blob_vec_shift_remove_forget_unchecked_end_values(&values, alloc);
            }
        }
    };
}

generate_tests!(u8, 128, opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX));
generate_tests!(u16, 128, opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX));
generate_tests!(u32, 128, opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX));
generate_tests!(u64, 128, opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX));
generate_tests!(u128, 128, opaque_vec_testing::AlternatingValuesSpec::new(u128::MIN, u128::MAX));
generate_tests!(usize, 128, opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX));
