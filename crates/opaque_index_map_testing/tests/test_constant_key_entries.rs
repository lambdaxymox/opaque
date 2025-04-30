use core::{
    fmt,
    hash,
    ops,
};

use opaque_index_map_testing as oimt;

fn run_test_constant_entries<K, V>(spec: oimt::ConstantKeyEntriesSpec<K, V>, expected: Vec<(K, V)>)
where
    K: Clone + Eq + hash::Hash + fmt::Debug + 'static,
    V: Clone + Eq + fmt::Debug + 'static,
    ops::RangeInclusive<V>: DoubleEndedIterator<Item = V>,
{
    let result = oimt::constant_key_entries(spec);

    assert_eq!(result, expected);
}

macro_rules! generate_tests {
    (($key_typ:ident, $value_typ:ident), $( $test_name:ident :: $const_spec:expr => $expected:expr ),* $(,)?) => {
        mod $key_typ {
            use super::*;
            $(
                #[test]
                fn $test_name() {
                    let spec: oimt::ConstantKeyEntriesSpec<$key_typ, $value_typ> = $const_spec;
                    let expected: Vec<($key_typ, $value_typ)> = $expected;
                    run_test_constant_entries::<$key_typ, $value_typ>(spec, expected);
                }
            )*
        }
    };
}

generate_tests!(
    (u32, i32),
    test_u32_i32_0  :: oimt::ConstantKeyEntriesSpec::new(126, 1..=0)  => vec![],
    test_u32_i32_1  :: oimt::ConstantKeyEntriesSpec::new(126, 0..=0)  => vec![(126, 0)],
    test_u32_i32_2  :: oimt::ConstantKeyEntriesSpec::new(126, 0..=1)  => vec![(126, 0), (126, 1)],
    test_u32_i32_3  :: oimt::ConstantKeyEntriesSpec::new(126, 0..=2)  => vec![(126, 0), (126, 1), (126, 2)],
    test_u32_i32_4  :: oimt::ConstantKeyEntriesSpec::new(126, 0..=3)  => vec![(126, 0), (126, 1), (126, 2), (126, 3)],
    test_u32_i32_5  :: oimt::ConstantKeyEntriesSpec::new(126, 0..=4)  => vec![(126, 0), (126, 1), (126, 2), (126, 3), (126, 4)],
    test_u32_i32_6  :: oimt::ConstantKeyEntriesSpec::new(126, 0..=5)  => vec![(126, 0), (126, 1), (126, 2), (126, 3), (126, 4), (126, 5)],
    test_u32_i32_7  :: oimt::ConstantKeyEntriesSpec::new(126, 0..=6)  => vec![(126, 0), (126, 1), (126, 2), (126, 3), (126, 4), (126, 5), (126, 6)],
    test_u32_i32_8  :: oimt::ConstantKeyEntriesSpec::new(126, 0..=7)  => vec![(126, 0), (126, 1), (126, 2), (126, 3), (126, 4), (126, 5), (126, 6), (126, 7)],
    test_u32_i32_9  :: oimt::ConstantKeyEntriesSpec::new(126, 0..=8)  => vec![(126, 0), (126, 1), (126, 2), (126, 3), (126, 4), (126, 5), (126, 6), (126, 7), (126, 8)],
    test_u32_i32_10 :: oimt::ConstantKeyEntriesSpec::new(126, 0..=9) => vec![(126, 0), (126, 1), (126, 2), (126, 3), (126, 4), (126, 5), (126, 6), (126, 7), (126, 8), (126, 9)],
    test_u32_i32_11 :: oimt::ConstantKeyEntriesSpec::new(126, 0..=10) => vec![(126, 0), (126, 1), (126, 2), (126, 3), (126, 4), (126, 5), (126, 6), (126, 7), (126, 8), (126, 9), (126, 10)],
    test_u32_i32_12 :: oimt::ConstantKeyEntriesSpec::new(126, 0..=11) => vec![(126, 0), (126, 1), (126, 2), (126, 3), (126, 4), (126, 5), (126, 6), (126, 7), (126, 8), (126, 9), (126, 10), (126, 11)],
    test_u32_i32_13 :: oimt::ConstantKeyEntriesSpec::new(126, 0..=12) => vec![(126, 0), (126, 1), (126, 2), (126, 3), (126, 4), (126, 5), (126, 6), (126, 7), (126, 8), (126, 9), (126, 10), (126, 11), (126, 12)],
    test_u32_i32_14 :: oimt::ConstantKeyEntriesSpec::new(126, 0..=13) => vec![(126, 0), (126, 1), (126, 2), (126, 3), (126, 4), (126, 5), (126, 6), (126, 7), (126, 8), (126, 9), (126, 10), (126, 11), (126, 12), (126, 13)],
    test_u32_i32_15 :: oimt::ConstantKeyEntriesSpec::new(126, 0..=14) => vec![(126, 0), (126, 1), (126, 2), (126, 3), (126, 4), (126, 5), (126, 6), (126, 7), (126, 8), (126, 9), (126, 10), (126, 11), (126, 12), (126, 13), (126, 14)],
    test_u32_i32_16 :: oimt::ConstantKeyEntriesSpec::new(126, 0..=15) => vec![(126, 0), (126, 1), (126, 2), (126, 3), (126, 4), (126, 5), (126, 6), (126, 7), (126, 8), (126, 9), (126, 10), (126, 11), (126, 12), (126, 13), (126, 14), (126, 15)],
);
