use opaque_vec::OpaqueVec;

use std::fmt;

fn nonnegative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = (i as i32) + 1;
    }

    prefix
}

fn negative_integer_values<const N: usize>() -> [i32; N] {
    let mut prefix = [0_i32; N];
    for i in 0..N {
        prefix[i] = -((i as i32) + 1)
    }

    prefix
}

fn run_test_opaque_vec_shift_insert_end<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::new::<T>();

    for (i, value) in values.iter().cloned().enumerate() {
        vec.shift_insert::<T>(i, value);
    }

    let expected = values;
    let result = vec.as_slice::<T>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_shift_insert_end1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_shift_insert_end(&values);
}

#[test]
fn test_opaque_vec_shift_insert_end64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_shift_insert_end(&values);
}
