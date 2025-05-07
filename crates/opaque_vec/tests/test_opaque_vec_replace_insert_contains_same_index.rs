#![feature(allocator_api)]
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

fn run_test_opaque_vec_replace_insert_contains_same_index1<T, A>(value: T, alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    assert!(!vec.contains::<T, A>(&value));

    vec.replace_insert::<T, A>(0, value.clone());

    assert!(vec.contains::<T, A>(&value));
}

fn run_test_opaque_vec_replace_insert_contains_same_index2<T, A>(initial_value: T, value: T, alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);
    vec.replace_insert::<T, A>(0, initial_value.clone());

    assert!(vec.contains::<T, A>(&initial_value));

    for _ in 0..65536 {
        vec.replace_insert::<T, A>(0, value.clone());

        assert!(vec.contains::<T, A>(&value));
    }
}

macro_rules! generate_tests {
    ($typ:ident, $initial_value:expr, $value:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_replace_insert_contains_same_index1() {
                let value: $typ = $value;
                let alloc = alloc::Global;
                run_test_opaque_vec_replace_insert_contains_same_index1(value, alloc);
            }

            #[test]
            fn test_opaque_vec_replace_insert_contains_same_index2() {
                let initial_value: $typ = $initial_value;
                let value: $typ = $value;
                let alloc = alloc::Global;
                run_test_opaque_vec_replace_insert_contains_same_index2(initial_value, value, alloc);
            }
        }
    };
}

generate_tests!(i8, i8::MIN, i8::MAX);
generate_tests!(i16, i16::MIN, i16::MAX);
generate_tests!(i32, i32::MIN, i32::MAX);
generate_tests!(i64, i64::MIN, i64::MAX);
generate_tests!(i128, i128::MIN, i128::MAX);
generate_tests!(isize, isize::MIN, isize::MAX);

generate_tests!(u8, u8::MIN, u8::MAX);
generate_tests!(u16, u16::MIN, u16::MAX);
generate_tests!(u32, u32::MIN, u32::MAX);
generate_tests!(u64, u64::MIN, u64::MAX);
generate_tests!(u128, u128::MIN, u128::MAX);
generate_tests!(usize, usize::MIN, usize::MAX);
