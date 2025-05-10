#![feature(allocator_api)]
use opaque_vec::OpaqueVec;

use core::any;
use std::alloc;

fn run_test_opaque_vec_pop_empty1<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    assert!(vec.pop::<T, A>().is_none());
}

fn run_test_opaque_vec_pop_empty2<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    for _ in 0..65536 {
        assert!(vec.pop::<T, A>().is_none());
    }
}

fn run_test_opaque_vec_pop_empty_is_empty1<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    assert!(vec.is_empty::<T, A>());

    vec.pop::<T, A>();

    assert!(vec.is_empty::<T, A>());
}

fn run_test_opaque_vec_pop_empty_is_empty2<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    assert!(vec.is_empty::<T, A>());

    for _ in 0..65536 {
        vec.pop::<T, A>();
    }

    assert!(vec.is_empty::<T, A>());
}

macro_rules! generate_tests {
    ($($typ:ident),*) => {
        $(
            mod $typ {
                use super::*;

                #[test]
                fn test_opaque_vec_pop_empty1() {
                    let alloc = alloc::Global;
                    run_test_opaque_vec_pop_empty1::<$typ, alloc::Global>(alloc);
                }

                #[test]
                fn test_opaque_vec_pop_empty2() {
                    let alloc = alloc::Global;
                    run_test_opaque_vec_pop_empty1::<$typ, alloc::Global>(alloc);
                }

                #[test]
                fn test_opaque_vec_pop_empty_is_empty1() {
                    let alloc = alloc::Global;
                    run_test_opaque_vec_pop_empty_is_empty1::<$typ, alloc::Global>(alloc);
                }

                #[test]
                fn test_opaque_vec_pop_is_empty_is_empty2() {
                    let alloc = alloc::Global;
                    run_test_opaque_vec_pop_empty_is_empty2::<$typ, alloc::Global>(alloc);
                }
            }
        )*
    };
}

generate_tests!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);
