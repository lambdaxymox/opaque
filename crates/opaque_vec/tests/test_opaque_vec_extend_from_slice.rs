mod common;

use opaque_vec::OpaqueVec;

use core::fmt;

use common::array_generators as ag;

fn expected<T>(values: &[T], extension_values: &[T]) -> OpaqueVec
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::from(values);
    for extension_value in extension_values.iter() {
        vec.push::<T>(extension_value.clone());
    }

    vec
}

fn result<T>(values: &[T], extension_values: &[T]) -> OpaqueVec
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::from(values);
    vec.extend_from_slice::<T>(extension_values);

    vec
}

fn run_test_opaque_vec_extend_from_slice<T>(values: &[T], extension_values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let expected = expected(values, extension_values);
    let result = result(values, extension_values);

    assert_eq!(result.as_slice::<T>(), expected.as_slice::<T>());
}

fn run_test_opaque_vec_extend_from_slice_values<T>(values: &[T], extension_values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let expected = expected(values, extension_values);
    let result = result(values, extension_values);
    for len in 0..values.len() {
        let prefix_values = &values[0..len];
        let prefix_extension_values = &extension_values[0..len];
        run_test_opaque_vec_extend_from_slice(prefix_values, prefix_extension_values);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr, $const_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_clone_len_range_values() {
                let values = ag::range_values::<$typ, $max_array_size>($range_spec);
                let extension_values = ag::constant_values::<$typ, $max_array_size>($const_spec);
                run_test_opaque_vec_extend_from_slice_values(&values, &extension_values);
            }

            #[test]
            fn test_opaque_vec_clone_len_alternating_values() {
                let values = ag::alternating_values::<$typ, $max_array_size>($alt_spec);
                let extension_values = ag::constant_values::<$typ, $max_array_size>($const_spec);
                run_test_opaque_vec_extend_from_slice_values(&values, &extension_values);
            }
        }
    };
}

generate_tests!(i8,    128,  ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i8::MIN,    0), ag::ConstantValuesSpec::new(i8::MAX));
generate_tests!(i16,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i16::MIN,   0), ag::ConstantValuesSpec::new(i16::MAX));
generate_tests!(i32,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i32::MIN,   0), ag::ConstantValuesSpec::new(i32::MAX));
generate_tests!(i64,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i64::MIN,   0), ag::ConstantValuesSpec::new(i64::MAX));
generate_tests!(i128,  1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i128::MIN,  0), ag::ConstantValuesSpec::new(i128::MAX));
generate_tests!(isize, 1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(isize::MIN, 0), ag::ConstantValuesSpec::new(isize::MAX));

generate_tests!(u8,    128,  ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u8::MIN,    u8::MAX),    ag::ConstantValuesSpec::new(u8::MAX));
generate_tests!(u16,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u16::MIN,   u16::MAX),   ag::ConstantValuesSpec::new(u16::MAX));
generate_tests!(u32,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u32::MIN,   u32::MAX),   ag::ConstantValuesSpec::new(u32::MAX));
generate_tests!(u64,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u64::MIN,   u64::MAX),   ag::ConstantValuesSpec::new(u64::MAX));
generate_tests!(u128,  1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u128::MIN,  u128::MAX),  ag::ConstantValuesSpec::new(u128::MAX));
generate_tests!(usize, 1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(usize::MIN, usize::MAX), ag::ConstantValuesSpec::new(usize::MAX));

/*
#[test]
fn test_opaque_vec_extend_from_slice1() {
    let values = nonnegative_integer_values::<1>();
    let extension_values = OpaqueVec::from_iter((0..1).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice2() {
    let values = nonnegative_integer_values::<2>();
    let extension_values = OpaqueVec::from_iter((0..2).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice3() {
    let values = nonnegative_integer_values::<3>();
    let extension_values = OpaqueVec::from_iter((0..3).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice4() {
    let values = nonnegative_integer_values::<4>();
    let extension_values = OpaqueVec::from_iter((0..4).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice5() {
    let values = nonnegative_integer_values::<5>();
    let extension_values = OpaqueVec::from_iter((0..5).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice6() {
    let values = nonnegative_integer_values::<6>();
    let extension_values = OpaqueVec::from_iter((0..6).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice7() {
    let values = nonnegative_integer_values::<7>();
    let extension_values = OpaqueVec::from_iter((0..7).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice8() {
    let values = nonnegative_integer_values::<8>();
    let extension_values = OpaqueVec::from_iter((0..8).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice9() {
    let values = nonnegative_integer_values::<9>();
    let extension_values = OpaqueVec::from_iter((0..9).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice10() {
    let values = nonnegative_integer_values::<10>();
    let extension_values = OpaqueVec::from_iter((0..10).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice11() {
    let values = nonnegative_integer_values::<11>();
    let extension_values = OpaqueVec::from_iter((0..11).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice12() {
    let values = nonnegative_integer_values::<12>();
    let extension_values = OpaqueVec::from_iter((0..12).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice13() {
    let values = nonnegative_integer_values::<13>();
    let extension_values = OpaqueVec::from_iter((0..13).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice14() {
    let values = nonnegative_integer_values::<14>();
    let extension_values = OpaqueVec::from_iter((0..14).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice15() {
    let values = nonnegative_integer_values::<15>();
    let extension_values = OpaqueVec::from_iter((0..15).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice16() {
    let values = nonnegative_integer_values::<16>();
    let extension_values = OpaqueVec::from_iter((0..16).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice32() {
    let values = nonnegative_integer_values::<32>();
    let extension_values = OpaqueVec::from_iter((0..32).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}

#[test]
fn test_opaque_vec_extend_from_slice64() {
    let values = nonnegative_integer_values::<64>();
    let extension_values = OpaqueVec::from_iter((0..64).map(|i| i32::MAX));

    run_test_opaque_vec_extend_from_slice(&values, extension_values.as_slice::<i32>());
}
*/