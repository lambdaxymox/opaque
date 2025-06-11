use crate::common;

use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

fn run_test_opaque_vec_pop_empty1<T, A>(alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    assert!(vec.pop::<T, A>().is_none());
}

fn run_test_opaque_vec_pop_empty2<T, A>(alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    for _ in 0..65536 {
        assert!(vec.pop::<T, A>().is_none());
    }
}

fn run_test_opaque_vec_pop_empty_is_empty1<T, A>(alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    assert!(vec.is_empty());

    vec.pop::<T, A>();

    assert!(vec.is_empty());
}

fn run_test_opaque_vec_pop_empty_is_empty2<T, A>(alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = OpaqueVec::new_in::<T, A>(alloc);

    assert!(vec.is_empty());

    for _ in 0..65536 {
        vec.pop::<T, A>();
    }

    assert!(vec.is_empty());
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_opaque_vec_pop_empty1() {
                let alloc = alloc::Global;
                run_test_opaque_vec_pop_empty1::<$typ, alloc::Global>(alloc);
            }

            #[test]
            fn test_opaque_vec_pop_empty2() {
                let alloc = alloc::Global;
                run_test_opaque_vec_pop_empty2::<$typ, alloc::Global>(alloc);
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
    }
}

generate_tests!(unit, ());
generate_tests!(u8, u8);
generate_tests!(u16, u16);
generate_tests!(u32, u32);
generate_tests!(u64, u64);
generate_tests!(usize, usize);
generate_tests!(string, String);
