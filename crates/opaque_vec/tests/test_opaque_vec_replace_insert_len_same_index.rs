#![feature(allocator_api)]
use opaque_vec::OpaqueVec;

use core::any;
use std::alloc;

fn run_test_opaque_vec_replace_insert_len_same_index<T>(value: T)
where
    T: any::Any + PartialEq + Clone,
{
    let mut vec = OpaqueVec::new::<T>();

    assert!(vec.is_empty());

    for _ in 0..65536 {
        vec.replace_insert::<T, alloc::Global>(0, value.clone());
    }

    assert_eq!(vec.len(), 1);
}

macro_rules! generate_tests {
    ($typ:ident, $value:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_replace_insert_len_same_index() {
                run_test_opaque_vec_replace_insert_len_same_index($value);
            }
        }
    };
}

generate_tests!(i8, i8::MAX);
generate_tests!(i16, i16::MAX);
generate_tests!(i32, i32::MAX);
generate_tests!(i64, i64::MAX);
generate_tests!(i128, i128::MAX);
generate_tests!(isize, isize::MAX);

generate_tests!(u8, u8::MAX);
generate_tests!(u16, u16::MAX);
generate_tests!(u32, u32::MAX);
generate_tests!(u64, u64::MAX);
generate_tests!(u128, u128::MAX);
generate_tests!(usize, usize::MAX);
