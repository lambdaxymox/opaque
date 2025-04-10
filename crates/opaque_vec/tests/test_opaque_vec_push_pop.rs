mod common;

use opaque_vec::OpaqueVec;

use core::fmt;

use common::array_generators as ag;

/*
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
*/

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

fn run_test_opaque_vec_push_pop<T>(values: &[T])
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

fn run_test_opaque_vec_push_pop_exists_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    for len in 0..values.len() {
        let prefix_values = &values[0..len];
        run_test_opaque_vec_push_pop_exists(prefix_values);
    }
}

fn run_test_opaque_vec_push_pop_values<T>(values: &[T])
where
    T: PartialEq + Clone + fmt::Debug + 'static,
{
    for len in 0..values.len() {
        let prefix_values = &values[0..len];
        run_test_opaque_vec_push_pop(prefix_values);
    }
}

macro_rules! generate_tests {
    ($typ:ident, $max_array_size:expr, $range_spec:expr, $alt_spec:expr) => {
        mod $typ {
            use super::*;

            #[test]
            fn test_opaque_vec_push_pop_exists_range_values() {
                let values = ag::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_vec_push_pop_exists_values(&values);
            }

            #[test]
            fn test_opaque_vec_push_pop_exists_alternating_values() {
                let values = ag::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_vec_push_pop_exists_values(&values);
            }

            #[test]
            fn test_opaque_vec_push_pop_range_values() {
                let values = ag::range_values::<$typ, $max_array_size>($range_spec);
                run_test_opaque_vec_push_pop_values(&values);
            }

            #[test]
            fn test_opaque_vec_push_pop_alternating_values() {
                let values = ag::alternating_values::<$typ, $max_array_size>($alt_spec);
                run_test_opaque_vec_push_pop_values(&values);
            }
        }
    };
}

generate_tests!(i8,    128,  ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i8::MIN,    0));
generate_tests!(i16,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i16::MIN,   0));
generate_tests!(i32,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i32::MIN,   0));
generate_tests!(i64,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i64::MIN,   0));
generate_tests!(i128,  1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(i128::MIN,  0));
generate_tests!(isize, 1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(isize::MIN, 0));

generate_tests!(u8,    128,  ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u8::MIN,    u8::MAX));
generate_tests!(u16,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u16::MIN,   u16::MAX));
generate_tests!(u32,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u32::MIN,   u32::MAX));
generate_tests!(u64,   1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u64::MIN,   u64::MAX));
generate_tests!(u128,  1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(u128::MIN,  u128::MAX));
generate_tests!(usize, 1024, ag::RangeValuesSpec::new(0), ag::AlternatingValuesSpec::new(usize::MIN, usize::MAX));

/*
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
*/
