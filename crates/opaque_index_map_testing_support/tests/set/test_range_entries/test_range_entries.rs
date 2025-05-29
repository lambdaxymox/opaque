use core::{
    any,
    fmt,
    hash,
    ops,
};

use opaque_index_map_testing as oimt;

fn run_test_range_entries<T>(spec: oimt::set::RangeEntriesSpec<T>, expected: Vec<T>)
where
    T: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    ops::RangeInclusive<T>: DoubleEndedIterator<Item = T>,
{
    let result = oimt::set::range_entries(spec);

    assert_eq!(result, expected);
}

macro_rules! generate_tests {
    ($value_typ:ident, $( $test_name:ident :: $range_spec:expr => $expected:expr ),* $(,)?) => {
        mod $value_typ {
            use super::*;
            $(
                #[test]
                fn $test_name() {
                    let spec: oimt::set::RangeEntriesSpec<$value_typ> = $range_spec;
                    let expected: Vec<$value_typ> = $expected;
                    run_test_range_entries::<$value_typ>(spec, expected);
                }
            )*
        }
    };
}

generate_tests!(
    u32,
    test_u32_0  :: oimt::set::RangeEntriesSpec::new(1..=0)  => Vec::from_iter(1..=0),
    test_u32_1  :: oimt::set::RangeEntriesSpec::new(0..=0)  => Vec::from_iter(0..=0),
    test_u32_2  :: oimt::set::RangeEntriesSpec::new(0..=1)  => Vec::from_iter(0..=1),
    test_u32_3  :: oimt::set::RangeEntriesSpec::new(0..=2)  => Vec::from_iter(0..=2),
    test_u32_4  :: oimt::set::RangeEntriesSpec::new(0..=3)  => Vec::from_iter(0..=3),
    test_u32_5  :: oimt::set::RangeEntriesSpec::new(0..=4)  => Vec::from_iter(0..=4),
    test_u32_6  :: oimt::set::RangeEntriesSpec::new(0..=5)  => Vec::from_iter(0..=5),
    test_u32_7  :: oimt::set::RangeEntriesSpec::new(0..=6)  => Vec::from_iter(0..=6),
    test_u32_8  :: oimt::set::RangeEntriesSpec::new(0..=7)  => Vec::from_iter(0..=7),
    test_u32_9  :: oimt::set::RangeEntriesSpec::new(0..=8)  => Vec::from_iter(0..=8),
    test_u32_10 :: oimt::set::RangeEntriesSpec::new(0..=9)  => Vec::from_iter(0..=9),
    test_u32_11 :: oimt::set::RangeEntriesSpec::new(0..=10) => Vec::from_iter(0..=10),
    test_u32_12 :: oimt::set::RangeEntriesSpec::new(0..=11) => Vec::from_iter(0..=11),
    test_u32_13 :: oimt::set::RangeEntriesSpec::new(0..=12) => Vec::from_iter(0..=12),
    test_u32_14 :: oimt::set::RangeEntriesSpec::new(0..=13) => Vec::from_iter(0..=13),
    test_u32_15 :: oimt::set::RangeEntriesSpec::new(0..=14) => Vec::from_iter(0..=14),
    test_u32_16 :: oimt::set::RangeEntriesSpec::new(0..=15) => Vec::from_iter(0..=15),
);
