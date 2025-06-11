use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

fn run_test_typed_proj_vec_replace_insert_contains_same_index1<T, A>(value: T, alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::new_in(alloc);

    assert!(!vec.contains(&value));

    vec.replace_insert(0, value.clone());

    assert!(vec.contains(&value));
}

fn run_test_typed_proj_vec_replace_insert_contains_same_index2<T, A>(initial_value: T, value: T, alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::new_in(alloc);
    vec.replace_insert(0, initial_value.clone());

    assert!(vec.contains(&initial_value));

    for _ in 0..65536 {
        vec.replace_insert(0, value.clone());

        assert!(vec.contains(&value));
    }
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, $initial_value:expr, $value:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn test_typed_proj_vec_replace_insert_contains_same_index1() {
                let value: $typ = $value;
                let alloc = alloc::Global;
                run_test_typed_proj_vec_replace_insert_contains_same_index1(value, alloc);
            }

            #[test]
            fn test_typed_proj_vec_replace_insert_contains_same_index2() {
                let initial_value: $typ = $initial_value;
                let value: $typ = $value;
                let alloc = alloc::Global;
                run_test_typed_proj_vec_replace_insert_contains_same_index2(initial_value, value, alloc);
            }
        }
    };
}

generate_tests!(unit, (), (), ());
generate_tests!(u8, u8, u8::MIN, u8::MAX);
generate_tests!(u16, u16, u16::MIN, u16::MAX);
generate_tests!(u32, u32, u32::MIN, u32::MAX);
generate_tests!(u64, u64, u64::MIN, u64::MAX);
generate_tests!(usize, usize, usize::MIN, usize::MAX);
generate_tests!(string, String, String::from("foo"), String::from("bar"));
