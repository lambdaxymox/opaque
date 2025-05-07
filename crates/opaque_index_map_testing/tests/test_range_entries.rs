use core::{
    any,
    fmt,
    hash,
    ops,
};

use opaque_index_map_testing as oimt;

fn run_test_range_entries<K, V>(spec: oimt::RangeEntriesSpec<K, V>, expected: Vec<(K, V)>)
where
    K: any::Any + Clone + Eq + hash::Hash + fmt::Debug,
    V: any::Any + Clone + Eq + fmt::Debug,
    ops::RangeInclusive<K>: DoubleEndedIterator<Item = K>,
    ops::RangeInclusive<V>: DoubleEndedIterator<Item = V>,
{
    let result = oimt::range_entries(spec);

    assert_eq!(result, expected);
}

macro_rules! generate_tests {
    (($key_typ:ident, $value_typ:ident), $( $test_name:ident :: $range_spec:expr => $expected:expr ),* $(,)?) => {
        mod $key_typ {
            use super::*;
            $(
                #[test]
                fn $test_name() {
                    let spec: oimt::RangeEntriesSpec<$key_typ, $value_typ> = $range_spec;
                    let expected: Vec<($key_typ, $value_typ)> = $expected;
                    run_test_range_entries::<$key_typ, $value_typ>(spec, expected);
                }
            )*
        }
    };
}

generate_tests!(
    (u32, i32),
    test_u32_i32_0  :: oimt::RangeEntriesSpec::new(1..=0, 1..=0)   => vec![],
    test_u32_i32_1  :: oimt::RangeEntriesSpec::new(0..=0, 1..=1)   => vec![(0, 1)],
    test_u32_i32_2  :: oimt::RangeEntriesSpec::new(0..=1, 1..=2)   => vec![(0, 1), (1, 2)],
    test_u32_i32_3  :: oimt::RangeEntriesSpec::new(0..=2, 1..=3)   => vec![(0, 1), (1, 2), (2, 3)],
    test_u32_i32_4  :: oimt::RangeEntriesSpec::new(0..=3, 1..=4)   => vec![(0, 1), (1, 2), (2, 3), (3, 4)],
    test_u32_i32_5  :: oimt::RangeEntriesSpec::new(0..=4, 1..=5)   => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5)],
    test_u32_i32_6  :: oimt::RangeEntriesSpec::new(0..=5, 1..=6)   => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6)],
    test_u32_i32_7  :: oimt::RangeEntriesSpec::new(0..=6, 1..=7)   => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7)],
    test_u32_i32_8  :: oimt::RangeEntriesSpec::new(0..=7, 1..=8)   => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8)],
    test_u32_i32_9  :: oimt::RangeEntriesSpec::new(0..=8, 1..=9)   => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9)],
    test_u32_i32_10 :: oimt::RangeEntriesSpec::new(0..=9, 1..=10)  => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9), (9, 10)],
    test_u32_i32_11 :: oimt::RangeEntriesSpec::new(0..=10, 1..=11) => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9), (9, 10), (10, 11)],
    test_u32_i32_12 :: oimt::RangeEntriesSpec::new(0..=11, 1..=12) => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9), (9, 10), (10, 11), (11, 12)],
    test_u32_i32_13 :: oimt::RangeEntriesSpec::new(0..=12, 1..=13) => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9), (9, 10), (10, 11), (11, 12), (12, 13)],
    test_u32_i32_14 :: oimt::RangeEntriesSpec::new(0..=13, 1..=14) => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9), (9, 10), (10, 11), (11, 12), (12, 13), (13, 14)],
    test_u32_i32_15 :: oimt::RangeEntriesSpec::new(0..=14, 1..=15) => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9), (9, 10), (10, 11), (11, 12), (12, 13), (13, 14), (14, 15)],
    test_u32_i32_16 :: oimt::RangeEntriesSpec::new(0..=15, 1..=16) => vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 6), (6, 7), (7, 8), (8, 9), (9, 10), (10, 11), (11, 12), (12, 13), (13, 14), (14, 15), (15, 16)],
);
