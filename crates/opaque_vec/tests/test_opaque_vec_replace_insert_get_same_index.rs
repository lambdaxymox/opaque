mod common;

use opaque_vec::OpaqueVec;

use core::fmt;

fn run_test_opaque_vec_replace_insert_get_same_index1<T>(value: T)
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::new::<T>();
    vec.replace_insert::<T>(0, value.clone());

    let expected = Some(value.clone());
    let result = vec.get::<T>(0).cloned();

    assert_eq!(result, expected);
}

fn run_test_opaque_vec_replace_insert_get_same_index2<T>(initial_value: T, value: T)
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::new::<T>();
    vec.replace_insert::<T>(0, initial_value.clone());

    let expected_initial = Some(initial_value.clone());
    let result_initial = vec.get::<T>(0).cloned();
    assert_eq!(result_initial, expected_initial);

    for _ in 0..65536 {
        vec.replace_insert::<T>(0, value.clone());
        let expected = Some(value.clone());
        let result = vec.get::<T>(0).cloned();

        assert_eq!(result, expected);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $initial_value:expr, $value:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_replace_insert_get_same_index1() {
                run_test_opaque_vec_replace_insert_get_same_index1($value);
            }

            #[test]
            fn test_opaque_vec_replace_insert_get_same_index2() {
                run_test_opaque_vec_replace_insert_get_same_index2($initial_value, $value);
            }
        }
    };
}

generate_tests!(i8,    i8::MIN,    i8::MAX);
generate_tests!(i16,   i16::MIN,   i16::MAX);
generate_tests!(i32,   i32::MIN,   i32::MAX);
generate_tests!(i64,   i64::MIN,   i64::MAX);
generate_tests!(i128,  i128::MIN,  i128::MAX);
generate_tests!(isize, isize::MIN, isize::MAX);

generate_tests!(u8,    u8::MIN,    u8::MAX);
generate_tests!(u16,   u16::MIN,   u16::MAX);
generate_tests!(u32,   u32::MIN,   u32::MAX);
generate_tests!(u64,   u64::MIN,   u64::MAX);
generate_tests!(u128,  u128::MIN,  u128::MAX);
generate_tests!(usize, usize::MIN, usize::MAX);
