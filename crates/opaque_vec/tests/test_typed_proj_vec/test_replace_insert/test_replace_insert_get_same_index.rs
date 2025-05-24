use opaque_vec::TypedProjVec;

use core::any;
use core::fmt;
use std::alloc;

fn run_test_typed_proj_vec_replace_insert_get_same_index1<T, A>(value: T, alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::new_in(alloc);
    vec.replace_insert(0, value.clone());

    let expected = Some(value.clone());
    let result = vec.get(0).cloned();

    assert_eq!(result, expected);
}

fn run_test_typed_proj_vec_replace_insert_get_same_index2<T, A>(initial_value: T, value: T, alloc: A)
where
    T: any::Any + PartialEq + Clone + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut vec = TypedProjVec::new_in(alloc);
    vec.replace_insert(0, initial_value.clone());

    let expected_initial = Some(initial_value.clone());
    let result_initial = vec.get(0).cloned();
    assert_eq!(result_initial, expected_initial);

    for _ in 0..65536 {
        vec.replace_insert(0, value.clone());
        let expected = Some(value.clone());
        let result = vec.get(0).cloned();

        assert_eq!(result, expected);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $initial_value:expr, $value:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_typed_proj_vec_replace_insert_get_same_index1() {
                let value: $typ = $value;
                let alloc = alloc::Global;
                run_test_typed_proj_vec_replace_insert_get_same_index1(value, alloc);
            }

            #[test]
            fn test_typed_proj_vec_replace_insert_get_same_index2() {
                let initial_value: $typ = $initial_value;
                let value: $typ = $value;
                let alloc = alloc::Global;
                run_test_typed_proj_vec_replace_insert_get_same_index2(initial_value, value, alloc);
            }
        }
    };
}

generate_tests!(u8, u8::MIN, u8::MAX);
generate_tests!(u16, u16::MIN, u16::MAX);
generate_tests!(u32, u32::MIN, u32::MAX);
generate_tests!(u64, u64::MIN, u64::MAX);
generate_tests!(usize, usize::MIN, usize::MAX);
