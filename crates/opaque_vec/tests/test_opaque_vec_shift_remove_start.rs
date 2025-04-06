use imgui_vulkan_renderer_opaque_vec::OpaqueVec;

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

fn run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::from(values);

    let last_index = vec.len() - 1;
    let expected = &values[0..last_index];
    let _ = vec.shift_remove::<T>(last_index);
    let result = vec.as_slice::<T>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}

#[test]
fn test_opaque_vec_shift_remove_from_end_does_not_change_remainder64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_shift_remove_from_end_does_not_change_remainder(&values);
}
