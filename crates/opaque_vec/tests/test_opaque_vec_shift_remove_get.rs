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

fn run_test_opaque_vec_shift_remove_get_from_end<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::from(values);

    for _ in 0..vec.len() {
        let last_index = vec.len() - 1;
        let expected = vec
            .get::<T>(last_index)
            .cloned()
            .unwrap();
        let result = vec.shift_remove::<T>(last_index);

        assert_eq!(result, expected);
    }
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}

#[test]
fn test_opaque_vec_shift_remove_get_from_end64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_shift_remove_get_from_end(&values);
}
