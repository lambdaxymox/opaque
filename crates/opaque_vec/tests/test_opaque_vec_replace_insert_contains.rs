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

fn run_test_opaque_vec_replace_insert_contains<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::new::<T>();

    for value in values.iter() {
        assert!(!vec.contains::<T>(value));
    }

    for (i, value) in values.iter().cloned().enumerate() {
        vec.replace_insert::<T>(i, value);
    }

    for value in values.iter() {
        assert!(vec.contains::<T>(value));
    }
}

#[test]
fn test_opaque_vec_replace_insert_contains_same_index1() {
    let mut vec = OpaqueVec::new::<u32>();
    let value: u32 = 1;
    
    assert!(!vec.contains::<u32>(&value));
    
    vec.replace_insert::<u32>(0, value);

    assert!(vec.contains::<u32>(&value));
}

#[test]
fn test_opaque_vec_replace_insert_contains_same_index2() {
    let mut vec = OpaqueVec::new::<u32>();
    for value in 0..65536 {
        assert!(!vec.contains::<u32>(&value));

        vec.replace_insert::<u32>(0, value);

        assert!(vec.contains::<u32>(&value));
    }
}

#[test]
fn test_opaque_vec_replace_insert_contains1() {
    let values = nonnegative_integer_values::<1>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains4() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains8() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_replace_insert_contains(&values);
}

#[test]
fn test_opaque_vec_replace_insert_contains64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_replace_insert_contains(&values);
}
