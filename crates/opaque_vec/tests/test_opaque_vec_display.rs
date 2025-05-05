#![feature(allocator_api)]
use opaque_vec::OpaqueVec;

use core::any;
use core::fmt;
use std::alloc;

#[test]
fn test_opaque_vec_display_empty() {
    let vec = OpaqueVec::new::<i32>();

    let expected = "[]";
    let result = format!("{}", vec);

    assert_eq!(result, expected);
}

fn run_test_opaque_vec_display<T>(values: &[T], expected: &str)
where
    T: any::Any + Clone + fmt::Display,
{
    let mut vec = OpaqueVec::new::<T>();
    for value in values.iter().cloned() {
        vec.push::<T, alloc::Global>(value);
    }

    let result = format!("{}", vec);

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_display1() {
    let values = [i32::from_ne_bytes([0x00, 0x00, 0x00, 0x00])];
    let expected = "[[0, 0, 0, 0]]";

    run_test_opaque_vec_display(&values, expected);
}

#[test]
fn test_opaque_vec_display2() {
    let values = [i32::from_ne_bytes([0x01, 0x02, 0x03, 0x04])];
    let expected = "[[1, 2, 3, 4]]";

    run_test_opaque_vec_display(&values, expected);
}

#[test]
fn test_opaque_vec_display3() {
    let values = [i32::from_ne_bytes([0x00, 0x00, 0x00, 0x00]), i32::from_ne_bytes([0x00, 0x00, 0x00, 0x01])];
    let expected = "[[0, 0, 0, 0], [0, 0, 0, 1]]";

    run_test_opaque_vec_display(&values, expected);
}

#[test]
fn test_opaque_vec_display4() {
    let values = [i32::from_ne_bytes([0x00, 0x00, 0x00, 0x04]), i32::from_ne_bytes([0x00, 0x00, 0x00, 0x05])];
    let expected = "[[0, 0, 0, 4], [0, 0, 0, 5]]";

    run_test_opaque_vec_display(&values, expected);
}

#[test]
fn test_opaque_vec_display5() {
    let values = [
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x01]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x02]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x03]),
    ];
    let expected = "[[0, 0, 0, 1], [0, 0, 0, 2], [0, 0, 0, 3]]";

    run_test_opaque_vec_display(&values, expected);
}

#[test]
fn test_opaque_vec_display6() {
    let values = [
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x01]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x02]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x03]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x04]),
    ];
    let expected = "[[0, 0, 0, 1], [0, 0, 0, 2], [0, 0, 0, 3], [0, 0, 0, 4]]";

    run_test_opaque_vec_display(&values, expected);
}

#[test]
fn test_opaque_vec_display7() {
    let values = [
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x01]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x02]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x03]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x04]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x05]),
    ];
    let expected = "[[0, 0, 0, 1], [0, 0, 0, 2], [0, 0, 0, 3], [0, 0, 0, 4], [0, 0, 0, 5]]";

    run_test_opaque_vec_display(&values, expected);
}

#[test]
fn test_opaque_vec_display8() {
    let values = [
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x01]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x02]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x03]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x04]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x05]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x06]),
    ];
    let expected = "[[0, 0, 0, 1], [0, 0, 0, 2], [0, 0, 0, 3], [0, 0, 0, 4], [0, 0, 0, 5], [0, 0, 0, 6]]";

    run_test_opaque_vec_display(&values, expected);
}

#[test]
fn test_opaque_vec_display9() {
    let values = [
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x01]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x02]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x03]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x04]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x05]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x06]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x07]),
    ];
    let expected = "[[0, 0, 0, 1], [0, 0, 0, 2], [0, 0, 0, 3], [0, 0, 0, 4], [0, 0, 0, 5], [0, 0, 0, 6], [0, 0, 0, 7]]";

    run_test_opaque_vec_display(&values, expected);
}

#[test]
fn test_opaque_vec_display10() {
    let values = [
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x01]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x02]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x03]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x04]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x05]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x06]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x07]),
        i32::from_ne_bytes([0x00, 0x00, 0x00, 0x08]),
    ];
    let expected = "[[0, 0, 0, 1], [0, 0, 0, 2], [0, 0, 0, 3], [0, 0, 0, 4], [0, 0, 0, 5], [0, 0, 0, 6], [0, 0, 0, 7], [0, 0, 0, 8]]";

    run_test_opaque_vec_display(&values, expected);
}
