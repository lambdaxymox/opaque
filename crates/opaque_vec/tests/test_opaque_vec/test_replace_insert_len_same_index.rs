use opaque_vec::OpaqueVec;

use core::any;
use std::alloc;

fn run_test_opaque_vec_replace_insert_len_same_index<T, A>(value: T, alloc: A)
where
    T: any::Any + PartialEq + Clone,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    assert!(vec.is_empty::<T, A>());

    for _ in 0..65536 {
        vec.replace_insert::<T, A>(0, value.clone());
    }

    assert_eq!(vec.len::<T, A>(), 1);
}

macro_rules! generate_tests {
    ($typ:ident, $value:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_replace_insert_len_same_index() {
                let value: $typ = $value;
                let alloc = alloc::Global;
                run_test_opaque_vec_replace_insert_len_same_index(value, alloc);
            }
        }
    };
}

generate_tests!(u8, u8::MAX);
generate_tests!(u16, u16::MAX);
generate_tests!(u32, u32::MAX);
generate_tests!(u64, u64::MAX);
generate_tests!(usize, usize::MAX);
