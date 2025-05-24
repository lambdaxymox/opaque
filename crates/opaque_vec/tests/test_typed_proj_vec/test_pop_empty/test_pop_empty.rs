use crate::common;

use opaque_vec::TypedProjVec;

use core::any;
use std::alloc;

fn run_test_typed_proj_vec_pop_empty1<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::<T, A>::new_in(alloc);

    assert!(vec.pop().is_none());
}

fn run_test_typed_proj_vec_pop_empty2<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::<T, A>::new_in(alloc);

    for _ in 0..65536 {
        assert!(vec.pop().is_none());
    }
}

fn run_test_typed_proj_vec_pop_empty_is_empty1<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::<T, A>::new_in(alloc);

    assert!(vec.is_empty());

    vec.pop();

    assert!(vec.is_empty());
}

fn run_test_typed_proj_vec_pop_empty_is_empty2<T, A>(alloc: A)
where
    T: any::Any,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::<T, A>::new_in(alloc);

    assert!(vec.is_empty());

    for _ in 0..65536 {
        vec.pop();
    }

    assert!(vec.is_empty());
}

macro_rules! generate_tests {
    ($($typ:ident),*) => {
        $(
            mod $typ {
                use super::*;

                #[test]
                fn test_typed_proj_vec_pop_empty1() {
                    let alloc = alloc::Global;
                    run_test_typed_proj_vec_pop_empty1::<$typ, alloc::Global>(alloc);
                }

                #[test]
                fn test_typed_proj_vec_pop_empty2() {
                    let alloc = alloc::Global;
                    run_test_typed_proj_vec_pop_empty1::<$typ, alloc::Global>(alloc);
                }

                #[test]
                fn test_typed_proj_vec_pop_empty_is_empty1() {
                    let alloc = alloc::Global;
                    run_test_typed_proj_vec_pop_empty_is_empty1::<$typ, alloc::Global>(alloc);
                }

                #[test]
                fn test_typed_proj_vec_pop_is_empty_is_empty2() {
                    let alloc = alloc::Global;
                    run_test_typed_proj_vec_pop_empty_is_empty2::<$typ, alloc::Global>(alloc);
                }
            }
        )*
    };
}

generate_tests!(u8, u16, u32, u64, usize);
