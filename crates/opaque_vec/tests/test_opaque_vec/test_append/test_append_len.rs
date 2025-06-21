use crate::common;

use core::any;
use core::fmt;
use core::iter;
use core::ops;
use std::alloc;

use opaque_vec_testing as ovt;

fn run_test_opaque_vec_append_len_source<T, A>(values1: &[T], values2: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = common::opaque_vec::from_slice_in(values1, alloc.clone());
    let mut destination = common::opaque_vec::from_slice_in(values2, alloc.clone());

    source.append::<T, A>(&mut destination);

    let expected = values1.len() + values2.len();
    let result = source.len();

    assert_eq!(result, expected);
}

fn run_test_opaque_vec_append_len_destination<T, A>(values1: &[T], values2: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let mut source = common::opaque_vec::from_slice_in(values1, alloc.clone());
    let mut destination = common::opaque_vec::from_slice_in(values2, alloc.clone());

    source.append::<T, A>(&mut destination);

    let expected = 0;
    let result = destination.len();

    assert_eq!(result, expected);
}

fn run_test_opaque_vec_append_len_source_values<T, A>(values1: &[T], values2: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator1 = ovt::PrefixGenerator::new(values1);
    for source in iterator1 {
        let iterator2 = ovt::PrefixGenerator::new(values2);
        for destination in iterator2 {
            run_test_opaque_vec_append_len_source::<T, A>(source, destination, alloc.clone());
        }
    }
}

fn run_test_opaque_vec_append_len_destination_values<T, A>(values1: &[T], values2: &[T], alloc: A)
where
    T: any::Any + PartialEq + Clone + Default + fmt::Debug,
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    let iterator1 = ovt::PrefixGenerator::new(values1);
    for source in iterator1 {
        let iterator2 = ovt::PrefixGenerator::new(values2);
        for destination in iterator2 {
            run_test_opaque_vec_append_len_destination::<T, A>(source, destination, alloc.clone());
        }
    }
}

macro_rules! generate_tests {
    ($module_name:ident, $typ:ty, $max_array_size:expr, $src_range_spec:expr, $src_alt_spec:expr, $dst_range_spec:expr, $dst_alt_spec:expr) => {
        mod $module_name {
            use super::*;

            #[test]
            fn run_test_opaque_vec_append_len_source_empty() {
                let values1: [$typ; 0] = [];
                let values2: [$typ; 0] = [];
                let alloc = alloc::Global;
                run_test_opaque_vec_append_len_source_values(&values1, &values2, alloc);
            }

            #[test]
            fn test_opaque_vec_append_len_source_range_values() {
                let values1 = opaque_vec_testing::range_values::<$typ, $max_array_size>($src_range_spec);
                let values2 = opaque_vec_testing::range_values::<$typ, $max_array_size>($dst_range_spec);
                let alloc = alloc::Global;
                run_test_opaque_vec_append_len_source_values(&values1, &values2, alloc);
            }

            #[test]
            fn test_opaque_vec_append_len_source_alternating_values() {
                let values1 = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($src_alt_spec);
                let values2 = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($dst_alt_spec);
                let alloc = alloc::Global;
                run_test_opaque_vec_append_len_source_values(&values1, &values2, alloc);
            }

            #[test]
            fn run_test_opaque_vec_append_len_destination_empty() {
                let values1: [$typ; 0] = [];
                let values2: [$typ; 0] = [];
                let alloc = alloc::Global;
                run_test_opaque_vec_append_len_destination_values(&values1, &values2, alloc);
            }

            #[test]
            fn test_opaque_vec_append_len_destination_range_values() {
                let values1 = opaque_vec_testing::range_values::<$typ, $max_array_size>($src_range_spec);
                let values2 = opaque_vec_testing::range_values::<$typ, $max_array_size>($dst_range_spec);
                let alloc = alloc::Global;
                run_test_opaque_vec_append_len_destination_values(&values1, &values2, alloc);
            }

            #[test]
            fn test_opaque_vec_append_len_destination_alternating_values() {
                let values1 = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($src_alt_spec);
                let values2 = opaque_vec_testing::alternating_values::<$typ, $max_array_size>($dst_alt_spec);
                let alloc = alloc::Global;
                run_test_opaque_vec_append_len_destination_values(&values1, &values2, alloc);
            }
        }
    };
}

generate_tests!(
    unit,
    (),
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(iter::repeat(()))),
    opaque_vec_testing::AlternatingValuesSpec::new((), ()),
    opaque_vec_testing::RangeValuesSpec::new(Box::new(iter::repeat(()))),
    opaque_vec_testing::AlternatingValuesSpec::new((), ())
);
generate_tests!(
    u8,
    u8,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX),
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u8::MIN, u8::MAX)
);
generate_tests!(
    u16,
    u16,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX),
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u16::MIN, u16::MAX)
);
generate_tests!(
    u32,
    u32,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX),
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u32::MIN, u32::MAX)
);
generate_tests!(
    u64,
    u64,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX),
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(u64::MIN, u64::MAX)
);
generate_tests!(
    usize,
    usize,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX),
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ops::RangeFrom { start: 0 })),
    opaque_vec_testing::AlternatingValuesSpec::new(usize::MIN, usize::MAX)
);
generate_tests!(
    string,
    String,
    128,
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ovt::StringRangeFrom::new(0))),
    opaque_vec_testing::AlternatingValuesSpec::new(String::from("foo"), String::from("bar")),
    opaque_vec_testing::RangeValuesSpec::new(Box::new(ovt::StringRangeFrom::new(0))),
    opaque_vec_testing::AlternatingValuesSpec::new(String::from("foo"), String::from("bar"))
);
