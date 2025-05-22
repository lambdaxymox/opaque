use crate::common;

use opaque_blob_vec::OpaqueBlobVec;

use core::any;
use core::fmt;
use core::ptr::NonNull;
use std::alloc;

use opaque_vec_testing as ovt;

fn expected<T, A>(values: &[T], alloc: A) -> OpaqueBlobVec
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut opaque_blob_vec = common::opaque_blob_vec::new_in::<T, A>(alloc);
    for value in values.iter().skip(1) {
        let value_cloned = value.clone();
        let value_ptr = NonNull::from(&value_cloned).cast::<u8>();
        opaque_blob_vec.push::<A>(value_ptr);
    }

    opaque_blob_vec
}

fn run_test_opaque_blob_vec_shift_remove_forget_unchecked_start<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut opaque_blob_vec = common::opaque_blob_vec::from_slice_in(values, alloc.clone());

    for i in 0..values.len() {
        let expected_opaque_blob_vec = expected(&values[i..], alloc.clone());
        let _ = unsafe {
            let ptr = opaque_blob_vec.shift_remove_forget_unchecked::<A>(0).cast::<T>();
            ptr.read()
        };
        let expected = common::opaque_blob_vec::as_slice::<T>(&expected_opaque_blob_vec);
        let result = common::opaque_blob_vec::as_slice::<T>(&opaque_blob_vec);

        assert_eq!(result, expected);
    }
}

fn run_test_opaque_blob_vec_shift_remove_forget_unchecked_start_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_opaque_blob_vec_shift_remove_forget_unchecked_start(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_blob_vec_shift_remove_forget_unchecked_start_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_opaque_blob_vec_shift_remove_forget_unchecked_start_values(&values, alloc);
            }
        }
    };
}

generate_tests!(u8, 128, opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX));
generate_tests!(u16, 128, opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX));
generate_tests!(u32, 128, opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX));
generate_tests!(u64, 128, opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX));
generate_tests!(usize, 128, opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX));
