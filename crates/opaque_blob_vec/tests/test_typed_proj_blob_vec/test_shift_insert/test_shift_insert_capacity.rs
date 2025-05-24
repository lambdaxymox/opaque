use crate::common;

use opaque_blob_vec::TypedProjBlobVec;

use core::any;
use core::fmt;
use core::ptr::NonNull;
use std::alloc;

use opaque_vec_testing as ovt;

pub fn from_slice_in<T, A>(values: &[T], alloc: A) -> TypedProjBlobVec<A>
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = common::typed_proj_blob_vec::new_in::<T, A>(alloc);
    for i in 0..values.len() {
        let value: T = values[i].clone();
        let value_ptr: NonNull<u8> = NonNull::from(&value).cast::<u8>();
        vec.shift_insert(i, value_ptr);
    }

    vec
}

fn run_test_typed_proj_blob_vec_shift_insert_capacity<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let proj_blob_vec = from_slice_in(values, alloc);

    assert!(proj_blob_vec.len() <= proj_blob_vec.capacity());
}

fn run_test_typed_proj_blob_vec_shift_insert_capacity_values<T, A>(values: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iter = ovt::PrefixGenerator::new(values);
    for slice in iter {
        run_test_typed_proj_blob_vec_shift_insert_capacity(slice, alloc.clone());
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_typed_proj_blob_vec_shift_insert_capacity_alternating_values() {
                let values = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($alt_spec);
                let alloc = alloc::Global;
                run_test_typed_proj_blob_vec_shift_insert_capacity_values(&values, alloc);
            }
        }
    };
}

generate_tests!(u8, 128, opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX));
generate_tests!(u16, 128, opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX));
generate_tests!(u32, 128, opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX));
generate_tests!(u64, 128, opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX));
generate_tests!(usize, 128, opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX));
