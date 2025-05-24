use crate::common;

use core::any;
use core::fmt;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_opaque_blob_vec_swap_remove_forget_unchecked_get_from_end<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut opaque_blob_vec = common::opaque_blob_vec::from_slice_in(values, alloc);

    for _ in 0..opaque_blob_vec.len() {
        let last_index = opaque_blob_vec.len() - 1;
        let expected = unsafe {
            let ptr = opaque_blob_vec.get_unchecked::<A>(last_index).cast::<T>();
            ptr.read()
        };
        let result = unsafe {
            let ptr = opaque_blob_vec.swap_remove_forget_unchecked::<A>(last_index).cast::<T>();
            ptr.read()
        };

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_blob_vec_swap_remove_forget_unchecked_get_from_end_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_opaque_blob_vec_swap_remove_forget_unchecked_get_from_end(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_blob_vec_swap_remove_forget_unchecked_get_from_end_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_opaque_blob_vec_swap_remove_forget_unchecked_get_from_end_values(&values, alloc);
            }
        }
    };
}

generate_tests!(u8, 128, opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX));
generate_tests!(u16, 128, opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX));
generate_tests!(u32, 128, opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX));
generate_tests!(u64, 128, opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX));
generate_tests!(usize, 128, opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX));
