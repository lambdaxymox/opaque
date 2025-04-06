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

fn run_test_opaque_vec_push_pop_exists<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::new::<T>();
    for value in values.iter().cloned() {
        vec.push::<T>(value);
    }

    for _ in 0..vec.len() {
        let result = vec.pop::<T>();

        assert!(result.is_some());
    }

    let result = vec.pop::<T>();

    assert!(result.is_none());
}

fn run_test_opaque_vec_push_pop_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    let mut vec = OpaqueVec::new::<T>();
    for value in values.iter().cloned() {
        vec.push::<T>(value);
    }

    let expected = {
        let mut _expected = OpaqueVec::new::<T>();
        for value in values.iter().rev().cloned() {
            _expected.push::<T>(value);
        }

        _expected
    };
    let result = {
        let mut _result = OpaqueVec::new::<T>();
        for _ in 0..vec.len() {
            let popped = vec.pop::<T>();

            _result.push::<T>(popped.unwrap());
        }

        _result
    };

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_push_pop_is_empty1() {
    let mut vec = OpaqueVec::new::<i32>();

    assert!(vec.is_empty());

    vec.push::<i32>(1);

    assert!(!vec.is_empty());

    vec.pop::<i32>();

    assert!(vec.is_empty());
}

#[test]
fn test_opaque_vec_push_pop_exists1() {
    let values = nonnegative_integer_values::<1>();
    
    run_test_opaque_vec_push_pop_exists(&values);
}

#[test]
fn test_opaque_vec_push_pop_exists2() {
    let values = nonnegative_integer_values::<2>();

    run_test_opaque_vec_push_pop_exists(&values);
}

#[test]
fn test_opaque_vec_push_exists3() {
    let values = nonnegative_integer_values::<4>();

    run_test_opaque_vec_push_pop_exists(&values);
}

#[test]
fn test_opaque_vec_push_pop_exists4() {
    let values = nonnegative_integer_values::<8>();

    run_test_opaque_vec_push_pop_exists(&values);
}

#[test]
fn test_opaque_vec_push_pop_values1() {
    let values = nonnegative_integer_values::<1>();
    
    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values2() {
    let values = nonnegative_integer_values::<2>();
    
    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values3() {
    let values = nonnegative_integer_values::<3>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values4() {
    let values = nonnegative_integer_values::<4>();
    
    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values5() {
    let values = nonnegative_integer_values::<5>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values6() {
    let values = nonnegative_integer_values::<6>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values7() {
    let values = nonnegative_integer_values::<7>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values8() {
    let values = nonnegative_integer_values::<8>();
    
    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values9() {
    let values = nonnegative_integer_values::<9>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values10() {
    let values = nonnegative_integer_values::<10>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values11() {
    let values = nonnegative_integer_values::<11>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values12() {
    let values = nonnegative_integer_values::<12>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values13() {
    let values = nonnegative_integer_values::<13>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values14() {
    let values = nonnegative_integer_values::<14>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values15() {
    let values = nonnegative_integer_values::<15>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values16() {
    let values = nonnegative_integer_values::<16>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values32() {
    let values = nonnegative_integer_values::<32>();

    run_test_opaque_vec_push_pop_values(&values);
}

#[test]
fn test_opaque_vec_push_pop_values64() {
    let values = nonnegative_integer_values::<64>();

    run_test_opaque_vec_push_pop_values(&values);
}
