use opaque_index_map::{GetDisjointMutError, TypeErasedIndexMap};
use opaque_vec::TypeProjectedVec;

use std::hash;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_empty_len1() {
    let opaque_map = TypeErasedIndexMap::new::<u64, i64>();

    assert_eq!(opaque_map.len(), 0);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_empty_is_empty1() {
    let opaque_map = TypeErasedIndexMap::new::<u64, i64>();

    assert!(opaque_map.is_empty());
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_empty_contains_no_values1() {
    let opaque_map = TypeErasedIndexMap::new::<u64, i64>();
    for key in 0..65536 {
        assert!(!opaque_map.contains_key::<_, u64, i64, hash::RandomState, alloc::Global>(&key));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_empty_get1() {
    let opaque_map = TypeErasedIndexMap::new::<u64, i64>();
    for key in 0..65536 {
        let result = opaque_map.get::<_, u64, i64, hash::RandomState, alloc::Global>(&key);

        assert!(result.is_none());
    }
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_empty_len2() {
   let opaque_map = TypeErasedIndexMap::new::<usize, i64>();

    assert_eq!(opaque_map.len(), 0);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_empty_is_empty2() {
   let opaque_map = TypeErasedIndexMap::new::<usize, i64>();

    assert!(opaque_map.is_empty());
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_empty_contains_no_values2() {
   let opaque_map = TypeErasedIndexMap::new::<usize, i64>();
    for key in 0..65536 {
        assert!(!opaque_map.contains_key::<_, usize, i64, hash::RandomState, alloc::Global>(&key));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_empty_get2() {
   let opaque_map = TypeErasedIndexMap::new::<usize, i64>();
    for key in 0..65536 {
        let result = opaque_map.get::<_, usize, i64, hash::RandomState, alloc::Global>(&key);

        assert!(result.is_none());
    }
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_index_of1() {
    let mut map = TypeErasedIndexMap::new::<&str, i32>();
    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), None);

    map.insert::<&str, i32, hash::RandomState, alloc::Global>("a", 1);
    map.insert::<&str, i32, hash::RandomState, alloc::Global>("b", 2);
    map.insert::<&str, i32, hash::RandomState, alloc::Global>("c", 3);

    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some(0));
    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some(1));
    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some(2));
    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_index_of2() {
    let mut map = TypeErasedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get_index_of::<_, usize, i32, hash::RandomState, alloc::Global>(&0_usize), Some(0));
    assert_eq!(map.get_index_of::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize), Some(1));
    assert_eq!(map.get_index_of::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize), Some(2));
    assert_eq!(map.get_index_of::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize), Some(3));
    assert_eq!(map.get_index_of::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize), Some(4));
    assert_eq!(map.get_index_of::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize), Some(5));
    assert_eq!(map.get_index_of::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_index_of3() {
    let mut map = TypeErasedIndexMap::from([("a", 1), ("b", 2), ("c", 3)]);

    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some(0));
    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some(2));
    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some(1));

    map.swap_remove::<_, &str, i32, hash::RandomState, alloc::Global>("b");

    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some(0));
    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some(1));
    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_index_of4() {
    let mut map: TypeErasedIndexMap = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_index_of::<_, char, (), hash::RandomState, alloc::Global>(&'*'), None);

    map.insert_before::<char, (), hash::RandomState, alloc::Global>(10, '*', ());
    assert_eq!(map.get_index_of::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some(10));

    map.insert_before::<char, (), hash::RandomState, alloc::Global>(10, 'a', ());
    assert_eq!(map.get_index_of::<_, char, (), hash::RandomState, alloc::Global>(&'a'), Some(9));
    assert_eq!(map.get_index_of::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some(10));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_index_of5() {
    let mut map: TypeErasedIndexMap = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_index_of::<_, char, (), hash::RandomState, alloc::Global>(&'*'), None);

    map.shift_insert::<char, (), hash::RandomState, alloc::Global>(10, '*', ());
    assert_eq!(map.get_index_of::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some(10));

    map.shift_insert::<char, (), hash::RandomState, alloc::Global>(10, 'a', ());
    assert_eq!(map.get_index_of::<_, char, (), hash::RandomState, alloc::Global>(&'a'), Some(10));
    assert_eq!(map.get_index_of::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some(9));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_index_of6() {
    let mut map = TypeErasedIndexMap::from([("a", 1), ("b", 2)]);

    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some(0));
    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some(1));
    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), None);

    map.insert::<&str, i32, hash::RandomState, alloc::Global>("c", 3);

    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some(0));
    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some(1));
    assert_eq!(map.get_index_of::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some(2));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get1() {
    let mut map = TypeErasedIndexMap::new::<&str, i32>();

    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), None);
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), None);
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), None);
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"d"), None);

    map.insert::<&str, i32, hash::RandomState, alloc::Global>("a", 1);
    map.insert::<&str, i32, hash::RandomState, alloc::Global>("b", 2);
    map.insert::<&str, i32, hash::RandomState, alloc::Global>("c", 3);

    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some(&1));
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some(&2));
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some(&3));
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get2() {
    let map = TypeErasedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get::<_, usize, i32, hash::RandomState, alloc::Global>(&0_usize), Some(&1_i32));
    assert_eq!(map.get::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize), Some(&2_i32));
    assert_eq!(map.get::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize), Some(&3_i32));
    assert_eq!(map.get::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize), Some(&4_i32));
    assert_eq!(map.get::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize), Some(&5_i32));
    assert_eq!(map.get::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize), Some(&6_i32));
    assert_eq!(map.get::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get3() {
    let mut map = TypeErasedIndexMap::from([("a", 1), ("b", 2), ("c", 3)]);

    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some(&1));
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some(&3));
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some(&2));

    map.swap_remove::<_, &str, i32, hash::RandomState, alloc::Global>("b");

    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some(&1));
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some(&3));
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get4() {
    let mut map: TypeErasedIndexMap = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get::<_, char, (), hash::RandomState, alloc::Global>(&'*'), None);

    map.insert_before::<char, (), hash::RandomState, alloc::Global>(10, '*', ());
    assert_eq!(map.get::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some(&()));

    map.insert_before::<char, (), hash::RandomState, alloc::Global>(10, 'a', ());
    assert_eq!(map.get::<_, char, (), hash::RandomState, alloc::Global>(&'a'), Some(&()));
    assert_eq!(map.get::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some(&()));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get5() {
    let mut map: TypeErasedIndexMap = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get::<_, char, (), hash::RandomState, alloc::Global>(&'*'), None);

    map.shift_insert::<char, (), hash::RandomState, alloc::Global>(10, '*', ());
    assert_eq!(map.get::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some(&()));

    map.shift_insert::<char, (), hash::RandomState, alloc::Global>(10, 'a', ());
    assert_eq!(map.get::<_, char, (), hash::RandomState, alloc::Global>(&'a'), Some(&()));
    assert_eq!(map.get::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some(&()));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get6() {
    let mut map = TypeErasedIndexMap::from([("a", 1), ("b", 2)]);

    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some(&1));
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some(&2));
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), None);

    map.insert::<&str, i32, hash::RandomState, alloc::Global>("c", 3);

    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some(&1));
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some(&2));
    assert_eq!(map.get::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some(&3));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_key_value1() {
    let mut map = TypeErasedIndexMap::new::<&str, i32>();

    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), None);
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), None);
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), None);
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"d"), None);

    map.insert::<&str, i32, hash::RandomState, alloc::Global>("a", 1);
    map.insert::<&str, i32, hash::RandomState, alloc::Global>("b", 2);
    map.insert::<&str, i32, hash::RandomState, alloc::Global>("c", 3);

    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some((&"a", &1)));
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some((&"b", &2)));
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some((&"c", &3)));
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_key_value2() {
    let map = TypeErasedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get_key_value::<_, usize, i32, hash::RandomState, alloc::Global>(&0_usize), Some((&0_usize, &1_i32)));
    assert_eq!(map.get_key_value::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize), Some((&1_usize, &2_i32)));
    assert_eq!(map.get_key_value::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize), Some((&2_usize, &3_i32)));
    assert_eq!(map.get_key_value::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize), Some((&3_usize, &4_i32)));
    assert_eq!(map.get_key_value::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize), Some((&4_usize, &5_i32)));
    assert_eq!(map.get_key_value::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize), Some((&5_usize, &6_i32)));
    assert_eq!(map.get_key_value::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_key_value3() {
    let mut map = TypeErasedIndexMap::from([("a", 1), ("b", 2), ("c", 3)]);

    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some((&"a", &1)));
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some((&"c", &3)));
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some((&"b", &2)));

    map.swap_remove::<_, &str, i32, hash::RandomState, alloc::Global>("b");

    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some((&"a", &1)));
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some((&"c", &3)));
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_key_value4() {
    let mut map: TypeErasedIndexMap = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_key_value::<_, char, (), hash::RandomState, alloc::Global>(&'*'), None);

    map.insert_before::<char, (), hash::RandomState, alloc::Global>(10, '*', ());
    assert_eq!(map.get_key_value::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some((&'*', &())));

    map.insert_before::<char, (), hash::RandomState, alloc::Global>(10, 'a', ());
    assert_eq!(map.get_key_value::<_, char, (), hash::RandomState, alloc::Global>(&'a'), Some((&'a', &())));
    assert_eq!(map.get_key_value::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some((&'*', &())));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_key_value5() {
    let mut map: TypeErasedIndexMap = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_key_value::<_, char, (), hash::RandomState, alloc::Global>(&'*'), None);

    map.shift_insert::<char, (), hash::RandomState, alloc::Global>(10, '*', ());
    assert_eq!(map.get_key_value::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some((&'*', &())));

    map.shift_insert::<char, (), hash::RandomState, alloc::Global>(10, 'a', ());
    assert_eq!(map.get_key_value::<_, char, (), hash::RandomState, alloc::Global>(&'a'), Some((&'a', &())));
    assert_eq!(map.get_key_value::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some((&'*', &())));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_key_value6() {
    let mut map = TypeErasedIndexMap::from([("a", 1), ("b", 2)]);

    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some((&"a", &1)));
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some((&"b", &2)));
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), None);

    map.insert::<&str, i32, hash::RandomState, alloc::Global>("c", 3);

    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some((&"a", &1)));
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some((&"b", &2)));
    assert_eq!(map.get_key_value::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some((&"c", &3)));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_full1() {
    let mut map = TypeErasedIndexMap::new::<&str, i32>();

    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), None);
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), None);
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), None);
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"d"), None);

    map.insert::<&str, i32, hash::RandomState, alloc::Global>("a", 1);
    map.insert::<&str, i32, hash::RandomState, alloc::Global>("b", 2);
    map.insert::<&str, i32, hash::RandomState, alloc::Global>("c", 3);

    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some((0, &"a", &1)));
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some((1, &"b", &2)));
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some((2, &"c", &3)));
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_full2() {
    let map = TypeErasedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get_full::<_, usize, i32, hash::RandomState, alloc::Global>(&0_usize), Some((0, &0_usize, &1_i32)));
    assert_eq!(map.get_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize), Some((1, &1_usize, &2_i32)));
    assert_eq!(map.get_full::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize), Some((2, &2_usize, &3_i32)));
    assert_eq!(map.get_full::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize), Some((3, &3_usize, &4_i32)));
    assert_eq!(map.get_full::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize), Some((4, &4_usize, &5_i32)));
    assert_eq!(map.get_full::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize), Some((5, &5_usize, &6_i32)));
    assert_eq!(map.get_full::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_full3() {
    let mut map = TypeErasedIndexMap::from([("a", 1), ("b", 2), ("c", 3)]);

    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some((0, &"a", &1)));
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some((2, &"c", &3)));
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some((1, &"b", &2)));

    map.swap_remove::<_, &str, i32, hash::RandomState, alloc::Global>("b");

    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some((0, &"a", &1)));
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some((1, &"c", &3)));
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_full4() {
    let mut map: TypeErasedIndexMap = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_full::<_, char, (), hash::RandomState, alloc::Global>(&'*'), None);

    map.insert_before::<char, (), hash::RandomState, alloc::Global>(10, '*', ());
    assert_eq!(map.get_full::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some((10, &'*', &())));

    map.insert_before::<char, (), hash::RandomState, alloc::Global>(10, 'a', ());
    assert_eq!(map.get_full::<_, char, (), hash::RandomState, alloc::Global>(&'a'), Some((9, &'a', &())));
    assert_eq!(map.get_full::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some((10, &'*', &())));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_full5() {
    let mut map: TypeErasedIndexMap = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_full::<_, char, (), hash::RandomState, alloc::Global>(&'*'), None);

    map.shift_insert::<char, (), hash::RandomState, alloc::Global>(10, '*', ());
    assert_eq!(map.get_full::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some((10, &'*', &())));

    map.shift_insert::<char, (), hash::RandomState, alloc::Global>(10, 'a', ());
    assert_eq!(map.get_full::<_, char, (), hash::RandomState, alloc::Global>(&'a'), Some((10, &'a', &())));
    assert_eq!(map.get_full::<_, char, (), hash::RandomState, alloc::Global>(&'*'), Some((9, &'*', &())));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_full6() {
    let mut map = TypeErasedIndexMap::from([("a", 1), ("b", 2)]);

    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some((0, &"a", &1)));
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some((1, &"b", &2)));
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), None);

    map.insert::<&str, i32, hash::RandomState, alloc::Global>("c", 3);

    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"a"), Some((0, &"a", &1)));
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"b"), Some((1, &"b", &2)));
    assert_eq!(map.get_full::<_, &str, i32, hash::RandomState, alloc::Global>(&"c"), Some((2, &"c", &3)));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_index1() {
    let mut map = TypeErasedIndexMap::new::<&str, i32>();

    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(0), None);
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(1), None);
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(2), None);
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(3), None);

    map.insert::<&str, i32, hash::RandomState, alloc::Global>("a", 1);
    map.insert::<&str, i32, hash::RandomState, alloc::Global>("b", 2);
    map.insert::<&str, i32, hash::RandomState, alloc::Global>("c", 3);

    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(0), Some((&"a", &1)));
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(1), Some((&"b", &2)));
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(2), Some((&"c", &3)));
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(3), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_index2() {
    let map = TypeErasedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get_index::<usize, i32, hash::RandomState, alloc::Global>(0), Some((&0_usize, &1_i32)));
    assert_eq!(map.get_index::<usize, i32, hash::RandomState, alloc::Global>(1), Some((&1_usize, &2_i32)));
    assert_eq!(map.get_index::<usize, i32, hash::RandomState, alloc::Global>(2), Some((&2_usize, &3_i32)));
    assert_eq!(map.get_index::<usize, i32, hash::RandomState, alloc::Global>(3), Some((&3_usize, &4_i32)));
    assert_eq!(map.get_index::<usize, i32, hash::RandomState, alloc::Global>(4), Some((&4_usize, &5_i32)));
    assert_eq!(map.get_index::<usize, i32, hash::RandomState, alloc::Global>(5), Some((&5_usize, &6_i32)));
    assert_eq!(map.get_index::<usize, i32, hash::RandomState, alloc::Global>(6), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_index3() {
    let mut map = TypeErasedIndexMap::from([("a", 1), ("b", 2), ("c", 3)]);

    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(0), Some((&"a", &1)));
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(2), Some((&"c", &3)));
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(1), Some((&"b", &2)));

    map.swap_remove::<_, &str, i32, hash::RandomState, alloc::Global>("b");

    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(0), Some((&"a", &1)));
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(2), None);
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(1), Some((&"c", &3)));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_index4() {
    let mut map: TypeErasedIndexMap = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_index::<char, (), hash::RandomState, alloc::Global>(10), Some((&'k', &())));

    map.insert_before::<char, (), hash::RandomState, alloc::Global>(10, '*', ());
    assert_eq!(map.get_index::<char, (), hash::RandomState, alloc::Global>(10), Some((&'*', &())));

    map.insert_before::<char, (), hash::RandomState, alloc::Global>(10, 'a', ());
    assert_eq!(map.get_index::<char, (), hash::RandomState, alloc::Global>(10), Some((&'*', &())));
    assert_eq!(map.get_index::<char, (), hash::RandomState, alloc::Global>(9), Some((&'a', &())));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_index5() {
    let mut map: TypeErasedIndexMap = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_index::<char, (), hash::RandomState, alloc::Global>(10), Some((&'k', &())));

    map.shift_insert::<char, (), hash::RandomState, alloc::Global>(10, '*', ());
    assert_eq!(map.get_index::<char, (), hash::RandomState, alloc::Global>(10), Some((&'*', &())));

    map.shift_insert::<char, (), hash::RandomState, alloc::Global>(10, 'a', ());
    assert_eq!(map.get_index::<char, (), hash::RandomState, alloc::Global>(0),  Some((&'b', &())));
    assert_eq!(map.get_index::<char, (), hash::RandomState, alloc::Global>(10), Some((&'a', &())));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_index6() {
    let mut map = TypeErasedIndexMap::from([("a", 1), ("b", 2)]);

    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(0), Some((&"a", &1)));
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(1), Some((&"b", &2)));
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(2), None);

    map.insert::<&str, i32, hash::RandomState, alloc::Global>("c", 3);

    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(0), Some((&"a", &1)));
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(1), Some((&"b", &2)));
    assert_eq!(map.get_index::<&str, i32, hash::RandomState, alloc::Global>(2), Some((&"c", &3)));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_mut2() {
    let mut map = TypeErasedIndexMap::from([
        ("1", 10),
        ("2", 20),
        ("3", 30),
        ("4", 40),
        ("5", 50),
        ("6", 60),
    ]);
    let expected = [
        Some(&mut 10),
        Some(&mut 20),
        Some(&mut 30),
        Some(&mut 40),
        Some(&mut 50),
        Some(&mut 60),
    ];
    let result = map.get_disjoint_mut::<_, 6, &str, i32, hash::RandomState, alloc::Global>([&"1", &"2", &"3", &"4", &"5", &"6"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_mut3() {
    let mut map = TypeErasedIndexMap::from([
        ("1", 10),
        ("2", 20),
        ("3", 30),
        ("4", 40),
        ("5", 50),
        ("6", 60),
    ]);
    let expected = [
        Some(&mut 10),
        Some(&mut 20),
        Some(&mut 30),
    ];
    let result = map.get_disjoint_mut::<_, 3, &str, i32, hash::RandomState, alloc::Global>([&"1", &"2", &"3"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_mut4() {
    let mut map = TypeErasedIndexMap::from([
        ("1", 10),
        ("2", 20),
        ("3", 30),
        ("4", 40),
        ("5", 50),
        ("6", 60),
    ]);
    let expected = [
        Some(&mut 40),
        Some(&mut 50),
        Some(&mut 60),
    ];
    let result = map.get_disjoint_mut::<_, 3, &str, i32, hash::RandomState, alloc::Global>([&"4", &"5", &"6"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_mut5() {
    let mut map = TypeErasedIndexMap::from([
        ("1", 10),
        ("2", 20),
        ("3", 30),
        ("4", 40),
        ("5", 50),
        ("6", 60),
    ]);
    let expected = [
        Some(&mut 10),
        Some(&mut 30),
        Some(&mut 50),
    ];
    let result = map.get_disjoint_mut::<_, 3, &str, i32, hash::RandomState, alloc::Global>([&"1", &"3", &"5"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_mut6() {
    let mut map = TypeErasedIndexMap::from([
        ("1", 10),
        ("2", 20),
        ("3", 30),
        ("4", 40),
        ("5", 50),
        ("6", 60),
    ]);
    let expected = [
        Some(&mut 20),
        Some(&mut 40),
        Some(&mut 60),
    ];
    let result = map.get_disjoint_mut::<_, 3, &str, i32, hash::RandomState, alloc::Global>([&"2", &"4", &"6"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_mut_partial_success1() {
    let mut map = TypeErasedIndexMap::from([
        ("1", 10),
        ("2", 20),
        ("3", 30),
        ("4", 40),
        ("5", 50),
        ("6", 60),
    ]);
    let expected = [
        Some(&mut 10),
        None,
        Some(&mut 30),
        None,
        Some(&mut 50),
        None,
    ];
    let result = map.get_disjoint_mut::<_, 6, &str, i32, hash::RandomState, alloc::Global>([&"1", &"20", &"3", &"40", &"5", &"60"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_mut_partial_success2() {
    let mut map = TypeErasedIndexMap::from([
        ("1", 10),
        ("2", 20),
        ("3", 30),
        ("4", 40),
        ("5", 50),
        ("6", 60),
    ]);
    let expected = [
        Some(&mut 10),
        Some(&mut 20),
        Some(&mut 30),
        None,
        Some(&mut 40),
        Some(&mut 50),
        Some(&mut 60),
        None,
    ];
    let result = map.get_disjoint_mut::<_, 8, &str, i32, hash::RandomState, alloc::Global>(["1", "2", "3", "200", "4", "5", "6", "100"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
#[should_panic]
fn test_type_erased_index_map_get_disjoint_mut_repeat_indices1() {
    let mut map = TypeErasedIndexMap::from([
        ("1", 10),
        ("2", 20),
        ("3", 30),
        ("4", 40),
        ("5", 50),
        ("6", 60),
    ]);
    let _ = map.get_disjoint_mut::<_, 6, &str, i32, hash::RandomState, alloc::Global>(["1", "2", "2", "4", "5", "6"]);

    assert!(true);
}

#[rustfmt::skip]
#[test]
#[should_panic]
fn test_type_erased_index_map_get_disjoint_mut_repeat_indices2() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 20_i32),
        (3_usize, 30_i32),
        (4_usize, 40_i32),
        (5_usize, 50_32),
        (6_usize, 60_i32),
    ]);
    let _ = map.get_disjoint_mut::<_, 6, usize, i32, hash::RandomState, alloc::Global>([&1, &1, &1, &2, &2, &3]);

    assert!(true);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_indices_mut1() {
    let mut map = TypeErasedIndexMap::from([
        (1_u32, 10_i32),
        (2_u32, 20_i32),
        (3_u32, 30_i32),
    ]);
    let expected = Ok([]);
    let result = map.get_disjoint_indices_mut::<0, u32, i32, hash::RandomState, alloc::Global>([]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_indices_mut2() {
    let mut map = TypeErasedIndexMap::from([
        (1_u32, 10_i32),
        (2_u32, 20_i32),
        (3_u32, 30_i32),
    ]);
    let expected = Ok([(&1_u32, &mut 10_i32)]);
    let result = map.get_disjoint_indices_mut::<1, u32, i32, hash::RandomState, alloc::Global>([0]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_indices_mut3() {
    let mut map = TypeErasedIndexMap::from([
        (1_u32, 10_i32),
        (2_u32, 20_i32),
        (3_u32, 30_i32),
    ]);
    let expected = Ok([(&2_u32, &mut 20_i32)]);
    let result = map.get_disjoint_indices_mut::<1, u32, i32, hash::RandomState, alloc::Global>([1]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_indices_mut4() {
    let mut map = TypeErasedIndexMap::from([
        (1_u32, 10_i32),
        (2_u32, 20_i32),
        (3_u32, 30_i32),
    ]);
    let expected = Ok([(&1_u32, &mut 10_i32), (&2_u32, &mut 20_i32)]);
    let result = map.get_disjoint_indices_mut::<2, u32, i32, hash::RandomState, alloc::Global>([0, 1]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_indices_mut_out_of_bounds() {
    let mut map = TypeErasedIndexMap::from([
        (1_u32, 10_i32),
        (2_u32, 20_i32),
        (3_u32, 30_i32),
    ]);
    let expected =  Err(GetDisjointMutError::IndexOutOfBounds);
    let result = map.get_disjoint_indices_mut::<2, u32, i32, hash::RandomState, alloc::Global>([1, 3]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_get_disjoint_indices_mut_fail_duplicate() {
    let mut map = TypeErasedIndexMap::from([
        (1_u32, 10_i32),
        (2_u32, 20_i32),
        (3_u32, 30_i32),
    ]);
    let expected = Err(GetDisjointMutError::OverlappingIndices);
    let result = map.get_disjoint_indices_mut::<3, u32, i32, hash::RandomState, alloc::Global>([1, 0, 1]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_keys1() {
    let map = TypeErasedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    for key in map.keys::<usize, i32, hash::RandomState, alloc::Global>() {
        assert!(map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(key));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_keys2() {
    let map = TypeErasedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    let expected = TypeProjectedVec::from([1_usize, 2_usize, 3_usize]);
    let result: TypeProjectedVec<usize> = map.keys::<usize, i32, hash::RandomState, alloc::Global>().cloned().collect();

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_keys3() {
    let map = TypeErasedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    let mut iter = map.keys::<usize, i32, hash::RandomState, alloc::Global>();

    assert_eq!(iter.next(), Some(&1_usize));
    assert_eq!(iter.next(), Some(&2_usize));
    assert_eq!(iter.next(), Some(&3_usize));
    assert_eq!(iter.next(), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_keys4() {
    let map = TypeErasedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    let mut iter = map.keys::<usize, i32, hash::RandomState, alloc::Global>();

    assert_eq!(map.get::<_, usize, i32, hash::RandomState, alloc::Global>(iter.next().unwrap()), Some(&10_i32));
    assert_eq!(map.get::<_, usize, i32, hash::RandomState, alloc::Global>(iter.next().unwrap()), Some(&24_i32));
    assert_eq!(map.get::<_, usize, i32, hash::RandomState, alloc::Global>(iter.next().unwrap()), Some(&58_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_values1() {
    let map = TypeErasedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    let expected = TypeProjectedVec::from([10_i32, 24_i32, 58_i32]);
    let result: TypeProjectedVec<i32> = map.values::<usize, i32, hash::RandomState, alloc::Global>().cloned().collect();

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_values2() {
    let map: TypeErasedIndexMap = TypeErasedIndexMap::new::<usize, i32>();
    let expected = TypeProjectedVec::new();
    let result: TypeProjectedVec<i32> = map.values::<usize, i32, hash::RandomState, alloc::Global>().cloned().collect();

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_values3() {
    let map = TypeErasedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    let mut iter = map.values::<usize, i32, hash::RandomState, alloc::Global>();

    assert_eq!(iter.next(), Some(&10_i32));
    assert_eq!(iter.next(), Some(&24_i32));
    assert_eq!(iter.next(), Some(&58_i32));
    assert_eq!(iter.next(), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_clear1() {
    let mut map = TypeErasedIndexMap::new::<usize, i32>();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    map.clear::<usize, i32, hash::RandomState, alloc::Global>();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_clear2() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert!(!map.is_empty());
    assert_eq!(map.len(), 6);

    map.clear::<usize, i32, hash::RandomState, alloc::Global>();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_clear3() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert!(map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize));
    assert!(map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize));
    assert!(map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize));
    assert!(map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize));
    assert!(map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize));
    assert!(map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize));

    map.clear::<usize, i32, hash::RandomState, alloc::Global>();

    assert!(!map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize));
    assert!(!map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize));
    assert!(!map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize));
    assert!(!map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize));
    assert!(!map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize));
    assert!(!map.contains_key::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_split_off1() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let expected1 = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);
    let expected2 = TypeErasedIndexMap::from([
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let result2 = map.split_off::<usize, i32, hash::RandomState, alloc::Global>(3);
    let result1 = map.clone::<usize, i32, hash::RandomState, alloc::Global>();

    assert_eq!(result1.len(), expected1.len());
    assert_eq!(result2.len(), expected2.len());
    assert_eq!(
        result1.as_proj::<usize, i32, hash::RandomState, alloc::Global>(),
        expected1.as_proj::<usize, i32, hash::RandomState, alloc::Global>(),
    );
    assert_eq!(
        result2.as_proj::<usize, i32, hash::RandomState, alloc::Global>(),
        expected2.as_proj::<usize, i32, hash::RandomState, alloc::Global>(),
    );

}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_split_off2() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let expected1 = map.clone::<usize, i32, hash::RandomState, alloc::Global>();
    let expected2 = TypeErasedIndexMap::new::<usize, i32>();
    let result2 = map.split_off::<usize, i32, hash::RandomState, alloc::Global>(map.len());
    let result1 = map.clone::<usize, i32, hash::RandomState, alloc::Global>();

    assert_eq!(result1.len(), expected1.len());
    assert_eq!(result2.len(), expected2.len());
    assert_eq!(
        result1.as_proj::<usize, i32, hash::RandomState, alloc::Global>(),
        expected1.as_proj::<usize, i32, hash::RandomState, alloc::Global>(),
    );
    assert_eq!(
        result2.as_proj::<usize, i32, hash::RandomState, alloc::Global>(),
        expected2.as_proj::<usize, i32, hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_split_off3() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let expected1 = TypeErasedIndexMap::new::<usize, i32>();
    let expected2 = map.clone::<usize, i32, hash::RandomState, alloc::Global>();
    let result2 = map.split_off::<usize, i32, hash::RandomState, alloc::Global>(0);
    let result1 = map.clone::<usize, i32, hash::RandomState, alloc::Global>();

    assert_eq!(result1.len(), expected1.len());
    assert_eq!(result2.len(), expected2.len());
    assert_eq!(
        result1.as_proj::<usize, i32, hash::RandomState, alloc::Global>(),
        expected1.as_proj::<usize, i32, hash::RandomState, alloc::Global>(),
    );
    assert_eq!(
        result2.as_proj::<usize, i32, hash::RandomState, alloc::Global>(),
        expected2.as_proj::<usize, i32, hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
#[should_panic]
fn test_type_erased_index_map_split_off_out_of_bounds1() {
    let mut map = TypeErasedIndexMap::new::<usize, i32>();
    let _ = map.split_off::<usize, i32, hash::RandomState, alloc::Global>(map.len() + 1);

    assert!(true);
}

#[rustfmt::skip]
#[test]
#[should_panic]
fn test_type_erased_index_map_split_off_out_of_bounds2() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let _ = map.split_off::<usize, i32, hash::RandomState, alloc::Global>(map.len() + 1);

    assert!(true);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_swap_remove1() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize), Some(20_i32));
    assert_eq!(map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize), Some(2043_i32));
    assert_eq!(map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize), Some(4904_i32));
    assert_eq!(map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize), Some(20994_i32));
    assert_eq!(map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize), Some(302_i32));
    assert_eq!(map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize), Some(5_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_swap_remove2() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
    ]);

    let _ = map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_swap_remove3() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);

    let _ = map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
    ]);

    let _ = map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
    ]);

    let _ = map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_swap_remove4() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize), Some(5_i32));
    assert_eq!(map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize), Some(302_i32));
    assert_eq!(map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize), Some(20994_i32));
    assert_eq!(map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize), Some(4904_i32));
    assert_eq!(map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize), Some(2043_i32));
    assert_eq!(map.swap_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize), Some(20_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_swap_remove_entry1() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize), Some((1_usize, 20_i32)));
    assert_eq!(map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize), Some((2_usize, 2043_i32)));
    assert_eq!(map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize), Some((3_usize, 4904_i32)));
    assert_eq!(map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize), Some((4_usize, 20994_i32)));
    assert_eq!(map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize), Some((5_usize, 302_i32)));
    assert_eq!(map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize), Some((6_usize, 5_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_swap_remove_entry2() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
    ]);

    let _ = map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_swap_remove_entry3() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);

    let _ = map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
    ]);

    let _ = map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
    ]);

    let _ = map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_swap_remove_entry4() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize), Some((6_usize, 5_i32)));
    assert_eq!(map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize), Some((5_usize, 302_i32)));
    assert_eq!(map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize), Some((4_usize, 20994_i32)));
    assert_eq!(map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize), Some((3_usize, 4904_i32)));
    assert_eq!(map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize), Some((2_usize, 2043_i32)));
    assert_eq!(map.swap_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize), Some((1_usize, 20_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_swap_remove_full1() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize), Some((0, 1_usize, 20_i32)));
    assert_eq!(map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize), Some((1, 2_usize, 2043_i32)));
    assert_eq!(map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize), Some((2, 3_usize, 4904_i32)));
    assert_eq!(map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize), Some((2, 4_usize, 20994_i32)));
    assert_eq!(map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize), Some((1, 5_usize, 302_i32)));
    assert_eq!(map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize), Some((0, 6_usize, 5_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_swap_remove_full2() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (6_usize, 5_i32),
    ]);

    let _ = map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_swap_remove_full3() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);

    let _ = map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
    ]);

    let _ = map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1_usize, 20_i32),
    ]);

    let _ = map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_swap_remove_full4() {
    let mut map = TypeErasedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&6_usize), Some((5, 6_usize, 5_i32)));
    assert_eq!(map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&5_usize), Some((4, 5_usize, 302_i32)));
    assert_eq!(map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&4_usize), Some((3, 4_usize, 20994_i32)));
    assert_eq!(map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&3_usize), Some((2, 3_usize, 4904_i32)));
    assert_eq!(map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&2_usize), Some((1, 2_usize, 2043_i32)));
    assert_eq!(map.swap_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1_usize), Some((0, 1_usize, 20_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_remove1() {
    let mut map = TypeErasedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1655_usize), Some(2427_i32));
    assert_eq!(map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1992_usize), Some(2910_i32));
    assert_eq!(map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&783_usize),  Some(603_i32));
    assert_eq!(map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&376_usize),  Some(834_i32));
    assert_eq!(map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&199_usize),  Some(1881_i32));
    assert_eq!(map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1098_usize), Some(1466_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_remove2() {
    let mut map = TypeErasedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1655_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1992_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&783_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&376_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&199_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1098_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_remove3() {
    let mut map = TypeErasedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1098_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
    ]);

    let _ = map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&199_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
    ]);

    let _ = map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&376_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
    ]);

    let _ = map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&783_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
    ]);

    let _ = map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1992_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
    ]);

    let _ = map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1655_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_remove4() {
    let mut map = TypeErasedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1098_usize), Some(1466_i32));
    assert_eq!(map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&199_usize),  Some(1881_i32));
    assert_eq!(map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&376_usize),  Some(834_i32));
    assert_eq!(map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&783_usize),  Some(603_i32));
    assert_eq!(map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1992_usize), Some(2910_i32));
    assert_eq!(map.shift_remove::<_, usize, i32, hash::RandomState, alloc::Global>(&1655_usize), Some(2427_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_remove_entry1() {
    let mut map = TypeErasedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1655_usize), Some((1655_usize, 2427_i32)));
    assert_eq!(map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1992_usize), Some((1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&783_usize),  Some((783_usize, 603_i32)));
    assert_eq!(map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&376_usize),  Some((376_usize, 834_i32)));
    assert_eq!(map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&199_usize),  Some((199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1098_usize), Some((1098_usize, 1466_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_remove_entry2() {
    let mut map = TypeErasedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1655_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1992_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&783_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&376_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&199_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1098_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_remove_entry3() {
    let mut map = TypeErasedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1098_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
    ]);

    let _ = map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&199_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
    ]);

    let _ = map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&376_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
    ]);

    let _ = map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&783_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
    ]);

    let _ = map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1992_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
    ]);

    let _ = map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1655_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_remove_entry4() {
    let mut map = TypeErasedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1098_usize), Some((1098_usize, 1466_i32)));
    assert_eq!(map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&199_usize),  Some((199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&376_usize),  Some((376_usize, 834_i32)));
    assert_eq!(map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&783_usize),  Some((783_usize, 603_i32)));
    assert_eq!(map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1992_usize), Some((1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_entry::<_, usize, i32, hash::RandomState, alloc::Global>(&1655_usize), Some((1655_usize, 2427_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_remove_full1() {
    let mut map = TypeErasedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1655_usize), Some((0, 1655_usize, 2427_i32)));
    assert_eq!(map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1992_usize), Some((0, 1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&783_usize),  Some((0, 783_usize, 603_i32)));
    assert_eq!(map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&376_usize),  Some((0, 376_usize, 834_i32)));
    assert_eq!(map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&199_usize),  Some((0, 199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1098_usize), Some((0, 1098_usize, 1466_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_remove_full2() {
    let mut map = TypeErasedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1655_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1992_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&783_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&376_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&199_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1098_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_remove_full3() {
    let mut map = TypeErasedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1098_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
    ]);

    let _ = map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&199_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
    ]);

    let _ = map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&376_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
    ]);

    let _ = map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&783_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
    ]);

    let _ = map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1992_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1655_usize, 2427_i32),
    ]);

    let _ = map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1655_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_remove_full4() {
    let mut map = TypeErasedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1098_usize), Some((5, 1098_usize, 1466_i32)));
    assert_eq!(map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&199_usize),  Some((4, 199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&376_usize),  Some((3, 376_usize, 834_i32)));
    assert_eq!(map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&783_usize),  Some((2, 783_usize, 603_i32)));
    assert_eq!(map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1992_usize), Some((1, 1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_full::<_, usize, i32, hash::RandomState, alloc::Global>(&1655_usize), Some((0, 1655_usize, 2427_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_insert1() {
    let mut map = TypeErasedIndexMap::new::<usize, i32>();

    assert_eq!(map.insert::<usize, i32, hash::RandomState, alloc::Global>(1803_usize, 1778_i32), None);
    assert_eq!(map.insert::<usize, i32, hash::RandomState, alloc::Global>(1057_usize, 2437_i32), None);
    assert_eq!(map.insert::<usize, i32, hash::RandomState, alloc::Global>(1924_usize, 185_i32),  None);
    assert_eq!(map.insert::<usize, i32, hash::RandomState, alloc::Global>(302_usize, 2457_i32),  None);
    assert_eq!(map.insert::<usize, i32, hash::RandomState, alloc::Global>(949_usize, 2176_i32),  None);
    assert_eq!(map.insert::<usize, i32, hash::RandomState, alloc::Global>(2968_usize, 1398_i32), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_insert2() {
    let mut map = TypeErasedIndexMap::new::<usize, i32>();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    let _ = map.insert::<usize, i32, hash::RandomState, alloc::Global>(1803_usize, 1778_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1803_usize, 1778_i32),
    ]);

    let _ = map.insert::<usize, i32, hash::RandomState, alloc::Global>(1057_usize, 2437_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
    ]);

    let _ = map.insert::<usize, i32, hash::RandomState, alloc::Global>(1924_usize, 185_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
    ]);

    let _ = map.insert::<usize, i32, hash::RandomState, alloc::Global>(302_usize, 2457_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize,  2457_i32),
    ]);

    let _ = map.insert::<usize, i32, hash::RandomState, alloc::Global>(949_usize, 2176_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize,  2457_i32),
        (949_usize,  2176_i32),
    ]);

    let _ = map.insert::<usize, i32, hash::RandomState, alloc::Global>(2968_usize, 1398_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize,  2457_i32),
        (949_usize,  2176_i32),
        (2968_usize, 1398_i32),
    ]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_insert_full1() {
    let mut map = TypeErasedIndexMap::new::<usize, i32>();

    assert_eq!(map.insert_full::<usize, i32, hash::RandomState, alloc::Global>(1803_usize, 1778_i32), (0, None));
    assert_eq!(map.insert_full::<usize, i32, hash::RandomState, alloc::Global>(1057_usize, 2437_i32), (1, None));
    assert_eq!(map.insert_full::<usize, i32, hash::RandomState, alloc::Global>(1924_usize, 185_i32),  (2, None));
    assert_eq!(map.insert_full::<usize, i32, hash::RandomState, alloc::Global>(302_usize, 2457_i32),  (3, None));
    assert_eq!(map.insert_full::<usize, i32, hash::RandomState, alloc::Global>(949_usize, 2176_i32),  (4, None));
    assert_eq!(map.insert_full::<usize, i32, hash::RandomState, alloc::Global>(2968_usize, 1398_i32), (5, None));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_insert_full2() {
    let mut map = TypeErasedIndexMap::new::<usize, i32>();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    let _ = map.insert_full::<usize, i32, hash::RandomState, alloc::Global>(1803_usize, 1778_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1803_usize, 1778_i32),
    ]);

    let _ = map.insert_full::<usize, i32, hash::RandomState, alloc::Global>(1057_usize, 2437_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
    ]);

    let _ = map.insert_full::<usize, i32, hash::RandomState, alloc::Global>(1924_usize, 185_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
    ]);

    let _ = map.insert_full::<usize, i32, hash::RandomState, alloc::Global>(302_usize, 2457_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize,  2457_i32),
    ]);

    let _ = map.insert_full::<usize, i32, hash::RandomState, alloc::Global>(949_usize, 2176_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize,  2457_i32),
        (949_usize,  2176_i32),
    ]);

    let _ = map.insert_full::<usize, i32, hash::RandomState, alloc::Global>(2968_usize, 1398_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize,  2457_i32),
        (949_usize,  2176_i32),
        (2968_usize, 1398_i32),
    ]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_insert_before1() {
    let mut map = TypeErasedIndexMap::new::<usize, i32>();

    assert_eq!(map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(0, 370_usize, 2339_i32),  (0, None));
    assert_eq!(map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(0, 1977_usize, 2387_i32), (0, None));
    assert_eq!(map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(0, 1244_usize, 2741_i32), (0, None));
    assert_eq!(map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(0, 1733_usize, 1838_i32), (0, None));
    assert_eq!(map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(0, 289_usize, 464_i32),   (0, None));
    assert_eq!(map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(0, 2712_usize, 509_i32),  (0, None));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_insert_before2() {
    let mut map = TypeErasedIndexMap::new::<usize, i32>();

    assert_eq!(map.len(), 0);

    let _ = map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(0, 370_usize, 2339_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (370_usize, 2339_i32),
    ]);

    let _ = map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(0, 1977_usize, 2387_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    let _ = map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(0, 1244_usize, 2741_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    let _ = map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(0, 1733_usize, 1838_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    let _ = map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(0, 289_usize, 464_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    let _ = map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(0, 2712_usize, 509_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (2712_usize, 509_i32),
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_insert_before3() {
    let mut map = TypeErasedIndexMap::from([
        (2712_usize, 509_i32),
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    assert_eq!(map.len(), 6);

    let result = map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(4, 289_usize, i32::MAX);
    assert_eq!(result, (3, Some(464_i32)));
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (2712_usize, 509_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (289_usize,  i32::MAX),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_insert_before4() {
    let mut map = TypeErasedIndexMap::from([
        (2712_usize, 509_i32),
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    assert_eq!(map.len(), 6);

    let result = map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(1, 370_usize, i32::MAX);
    assert_eq!(result, (1, Some(2339_i32)));
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (2712_usize, 509_i32),
        (370_usize,  i32::MAX),
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
    ]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_insert_before5() {
    let mut map = TypeErasedIndexMap::from([
        (2712_usize, 509_i32),
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    assert_eq!(map.len(), 6);

    let result = map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(3, 1244_usize, i32::MAX);
    assert_eq!(result, (3, Some(2741_i32)));
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (2712_usize, 509_i32),
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, i32::MAX),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_insert_before6() {
    let mut map = TypeErasedIndexMap::from([
        (2712_usize, 509_i32),
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    assert_eq!(map.len(), 6);

    let result = map.insert_before::<usize, i32, hash::RandomState, alloc::Global>(5, usize::MAX, i32::MAX);
    assert_eq!(result, (5, None));
    assert_eq!(map.len(), 7);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (2712_usize, 509_i32),
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (usize::MAX, i32::MAX),
        (370_usize,  2339_i32),
    ]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_insert1() {
    let mut map = TypeErasedIndexMap::new::<usize, i32>();

    assert_eq!(map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 1809_usize, 2381_i32), None);
    assert_eq!(map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 603_usize, 2834_i32),  None);
    assert_eq!(map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 2564_usize, 621_i32),  None);
    assert_eq!(map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 360_usize, 1352_i32),  None);
    assert_eq!(map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 57_usize, 2657_i32),   None);
    assert_eq!(map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 477_usize, 2084_i32),  None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_insert2() {
    let mut map = TypeErasedIndexMap::new::<usize, i32>();

    assert_eq!(map.len(), 0);
    assert!(map.is_empty());

    let _ = map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 1809_usize, 2381_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 603_usize, 2834_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 2564_usize, 621_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 360_usize, 1352_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 57_usize, 2657_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (57_usize,   2657_i32),
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 477_usize, 2084_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (477_usize,  2084_i32),
        (57_usize,   2657_i32),
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_insert3() {
    let mut map = TypeErasedIndexMap::new::<usize, i32>();

    assert_eq!(map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 477_usize, 2084_i32),  None);
    assert_eq!(map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(1, 57_usize, 2657_i32),   None);
    assert_eq!(map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(2, 360_usize, 1352_i32),  None);
    assert_eq!(map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(3, 2564_usize, 621_i32),  None);
    assert_eq!(map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(4, 603_usize, 2834_i32),  None);
    assert_eq!(map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(5, 1809_usize, 2381_i32), None);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shift_insert4() {
    let mut map = TypeErasedIndexMap::new::<usize, i32>();

    assert_eq!(map.len(), 0);
    assert!(map.is_empty());

    let _ = map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(0, 477_usize, 2084_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (477_usize, 2084_i32),
    ]);

    let _ = map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(1, 57_usize, 2657_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (477_usize, 2084_i32),
        (57_usize,  2657_i32),
    ]);

    let _ = map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(2, 360_usize, 1352_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (477_usize, 2084_i32),
        (57_usize,  2657_i32),
        (360_usize, 1352_i32),
    ]);

    let _ = map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(3, 2564_usize, 621_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (477_usize,  2084_i32),
        (57_usize,   2657_i32),
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
    ]);

    let _ = map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(4, 603_usize, 2834_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (477_usize,  2084_i32),
        (57_usize,   2657_i32),
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
    ]);

    let _ = map.shift_insert::<usize, i32, hash::RandomState, alloc::Global>(5, 1809_usize, 2381_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(), &[
        (477_usize,  2084_i32),
        (57_usize,   2657_i32),
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_append1() {
    let mut map1 = TypeErasedIndexMap::from([
        (605_usize,  2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    let mut map2 = TypeErasedIndexMap::from([
        (1062_usize, 1113_i32),
        (1875_usize, 800_i32),
        (1724_usize, 2910_i32),
    ]);
    let expected = TypeErasedIndexMap::from([
        (605_usize,  2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
        (1062_usize, 1113_i32),
        (1875_usize, 800_i32),
        (1724_usize, 2910_i32),
    ]);
    map1.append::<usize, i32, hash::RandomState, hash::RandomState, alloc::Global>(&mut map2);

    assert!(map2.is_empty());
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.len(), 7);
    assert_eq!(
        map1.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_append2() {
    let mut map1 = TypeErasedIndexMap::from([
        (605_usize,  2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    let mut map2 = TypeErasedIndexMap::from([
        (1804_usize, i32::MAX),
        (1875_usize, 800_i32),
        (1660_usize, i32::MAX),
    ]);
    let expected = TypeErasedIndexMap::from([
        (605_usize,  2879_i32),
        (1804_usize, i32::MAX),
        (1532_usize, 1980_i32),
        (1660_usize, i32::MAX),
        (1875_usize, 800_i32),
    ]);
    map1.append::<usize, i32, hash::RandomState, hash::RandomState, alloc::Global>(&mut map2);

    assert!(map2.is_empty());
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.len(), 5);
    assert_eq!(
        map1.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_append3() {
    let mut map1 = TypeErasedIndexMap::from([
        (605_usize,  2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    let mut map2 = TypeErasedIndexMap::new::<usize, i32>();
    let expected = TypeErasedIndexMap::from([
        (605_usize,  2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    map1.append::<usize, i32, hash::RandomState, hash::RandomState, alloc::Global>(&mut map2);

    assert!(map2.is_empty());
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.len(), 4);
    assert_eq!(
        map1.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_append4() {
    let mut map1 = TypeErasedIndexMap::new::<usize, i32>();
    let mut map2 = TypeErasedIndexMap::from([
        (605_usize,  2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    let expected = TypeErasedIndexMap::from([
        (605_usize,  2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    map1.append::<usize, i32, hash::RandomState, hash::RandomState, alloc::Global>(&mut map2);

    assert!(map2.is_empty());
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.len(), 4);
    assert_eq!(
        map1.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_append5() {
    let mut map1 = TypeErasedIndexMap::from([(usize::MAX, 1_i32)]);
    let mut map2 = TypeErasedIndexMap::from([(usize::MAX, i32::MAX)]);
    let expected = TypeErasedIndexMap::from([(usize::MAX, i32::MAX)]);
    map1.append::<usize, i32, hash::RandomState, hash::RandomState, alloc::Global>(&mut map2);

    assert!(map2.is_empty());
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.len(), 1);
    assert_eq!(
        map1.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_retain1() {
    let mut map = TypeErasedIndexMap::from([
        (344_usize,  ()),
        (1646_usize, ()),
        (2371_usize, ()),
        (52_usize,   ()),
        (789_usize,  ()),
        (1205_usize, ()),
        (28_usize,   ()),
        (136_usize,  ()),
    ]);
    let expected = map.clone::<usize, (), hash::RandomState, alloc::Global>();
    map.retain::<_, usize, (), hash::RandomState, alloc::Global>(|_k, _v| true);

    assert_eq!(map.len(), 8);
    assert_eq!(
        map.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_retain2() {
    let mut map = TypeErasedIndexMap::from([
        (344_usize,  ()),
        (1646_usize, ()),
        (2371_usize, ()),
        (52_usize,   ()),
        (789_usize,  ()),
        (1205_usize, ()),
        (28_usize,   ()),
        (136_usize,  ()),
    ]);
    let expected = TypeErasedIndexMap::new::<usize, ()>();
    map.retain::<_, usize, (), hash::RandomState, alloc::Global>(|_k, _v| false);

    assert_eq!(map.len(), 0);
    assert_eq!(
        map.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_retain3() {
    let mut map = TypeErasedIndexMap::from([
        (344_usize,  ()),
        (1646_usize, ()),
        (2371_usize, ()),
        (52_usize,   ()),
        (789_usize,  ()),
        (1205_usize, ()),
        (28_usize,   ()),
        (136_usize,  ()),
    ]);
    let expected = TypeErasedIndexMap::from([
        (344_usize,  ()),
        (1646_usize, ()),
        (52_usize,   ()),
        (28_usize,   ()),
        (136_usize,  ()),
    ]);
    map.retain::<_, usize, (), hash::RandomState, alloc::Global>(|k, _v| k % 2 == 0);

    assert_eq!(map.len(), 5);
    assert_eq!(
        map.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_retain4() {
    let mut map = TypeErasedIndexMap::from([
        (344_usize,  ()),
        (1646_usize, ()),
        (2371_usize, ()),
        (52_usize,   ()),
        (789_usize,  ()),
        (1205_usize, ()),
        (28_usize,   ()),
        (136_usize,  ()),
    ]);
    let expected = TypeErasedIndexMap::from([
        (2371_usize, ()),
        (789_usize,  ()),
        (1205_usize, ()),
    ]);
    map.retain::<_, usize, (), hash::RandomState, alloc::Global>(|k, _v| k % 2 != 0);

    assert_eq!(map.len(), 3);
    assert_eq!(
        map.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_sort_keys1() {
    let mut map = TypeErasedIndexMap::from([
        (6_usize,   ()),
        (7_usize,   ()),
        (10_usize,  ()),
        (17_usize,  ()),
        (22_usize,  ()),
        (23_usize,  ()),
        (47_usize,  ()),
        (79_usize,  ()),
        (141_usize, ()),
        (176_usize, ()),
        (200_usize, ()),
    ]);
    let expected = TypeErasedIndexMap::from([
        (6_usize,   ()),
        (7_usize,   ()),
        (10_usize,  ()),
        (17_usize,  ()),
        (22_usize,  ()),
        (23_usize,  ()),
        (47_usize,  ()),
        (79_usize,  ()),
        (141_usize, ()),
        (176_usize, ()),
        (200_usize, ()),
    ]);
    map.sort_keys::<usize, (), hash::RandomState, alloc::Global>();

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_sort_keys2() {
    let mut map = TypeErasedIndexMap::from([
        (10_usize,  ()),
        (47_usize,  ()),
        (22_usize,  ()),
        (17_usize,  ()),
        (141_usize, ()),
        (6_usize,   ()),
        (176_usize, ()),
        (23_usize,  ()),
        (79_usize,  ()),
        (200_usize, ()),
        (7_usize,   ()),
    ]);
    let expected = TypeErasedIndexMap::from([
        (6_usize,   ()),
        (7_usize,   ()),
        (10_usize,  ()),
        (17_usize,  ()),
        (22_usize,  ()),
        (23_usize,  ()),
        (47_usize,  ()),
        (79_usize,  ()),
        (141_usize, ()),
        (176_usize, ()),
        (200_usize, ()),
    ]);
    map.sort_keys::<usize, (), hash::RandomState, alloc::Global>();

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_sort_keys3() {
    let mut map = TypeErasedIndexMap::from([
        (200_usize, ()),
        (176_usize, ()),
        (141_usize, ()),
        (79_usize,  ()),
        (47_usize,  ()),
        (23_usize,  ()),
        (22_usize,  ()),
        (17_usize,  ()),
        (10_usize,  ()),
        (7_usize,   ()),
        (6_usize,   ()),
    ]);
    let expected = TypeErasedIndexMap::from([
        (6_usize,   ()),
        (7_usize,   ()),
        (10_usize,  ()),
        (17_usize,  ()),
        (22_usize,  ()),
        (23_usize,  ()),
        (47_usize,  ()),
        (79_usize,  ()),
        (141_usize, ()),
        (176_usize, ()),
        (200_usize, ()),
    ]);
    map.sort_keys::<usize, (), hash::RandomState, alloc::Global>();

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_sort_by1() {
    let mut map = TypeErasedIndexMap::from([
        (1952_usize, 1390_i32),
        (2900_usize, 2846_i32),
        (2999_usize, 760_i32),
        (828_usize,  491_i32),
        (1738_usize, 1984_i32),
        (339_usize,  1996_i32),
    ]);
    let expected = TypeErasedIndexMap::from([
        (828_usize,  491_i32),
        (2999_usize, 760_i32),
        (1952_usize, 1390_i32),
        (1738_usize, 1984_i32),
        (339_usize,  1996_i32),
        (2900_usize, 2846_i32),
    ]);
    map.sort_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k1, v1, _k2, v2| v1.cmp(v2));

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_sort_by2() {
    let mut map = TypeErasedIndexMap::from([
        (String::from("4"),   ()),
        (String::from("101"), ()),
        (String::from("1"),   ()),
        (String::from("2"),   ()),
        (String::from("10"),  ()),
        (String::from("3"),   ()),
    ]);
    let expected = TypeErasedIndexMap::from([
        (String::from("1"),   ()),
        (String::from("10"),  ()),
        (String::from("101"), ()),
        (String::from("2"),   ()),
        (String::from("3"),   ()),
        (String::from("4"),   ()),
    ]);
    map.sort_by::<_, String, (), hash::RandomState, alloc::Global>(|k1, _v1, k2, _v2| k1.cmp(k2));

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<String, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<String, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_sort_by3() {
    let mut map = TypeErasedIndexMap::from([
        (String::from("400"), ()),
        (String::from("101"), ()),
        (String::from("1"),   ()),
        (String::from("2"),   ()),
        (String::from("10"),  ()),
        (String::from("3"),   ()),
    ]);
    let expected = TypeErasedIndexMap::from([
        (String::from("1"),   ()),
        (String::from("2"),   ()),
        (String::from("3"),   ()),
        (String::from("10"),  ()),
        (String::from("400"), ()),
        (String::from("101"), ()),
    ]);
    map.sort_by::<_, String, (), hash::RandomState, alloc::Global>(|k1, _v1, k2, _v2| k1.len().cmp(&k2.len()));

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<String, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<String, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_sort_unstable_keys1() {
    let mut map = TypeErasedIndexMap::from([
        (6_usize,   ()),
        (7_usize,   ()),
        (10_usize,  ()),
        (17_usize,  ()),
        (22_usize,  ()),
        (23_usize,  ()),
        (47_usize,  ()),
        (79_usize,  ()),
        (141_usize, ()),
        (176_usize, ()),
        (200_usize, ()),
    ]);
    let expected = TypeErasedIndexMap::from([
        (6_usize,   ()),
        (7_usize,   ()),
        (10_usize,  ()),
        (17_usize,  ()),
        (22_usize,  ()),
        (23_usize,  ()),
        (47_usize,  ()),
        (79_usize,  ()),
        (141_usize, ()),
        (176_usize, ()),
        (200_usize, ()),
    ]);
    map.sort_unstable_keys::<usize, (), hash::RandomState, alloc::Global>();

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_sort_unstable_keys2() {
    let mut map = TypeErasedIndexMap::from([
        (10_usize,  ()),
        (47_usize,  ()),
        (22_usize,  ()),
        (17_usize,  ()),
        (141_usize, ()),
        (6_usize,   ()),
        (176_usize, ()),
        (23_usize,  ()),
        (79_usize,  ()),
        (200_usize, ()),
        (7_usize,   ()),
    ]);
    let expected = TypeErasedIndexMap::from([
        (6_usize,   ()),
        (7_usize,   ()),
        (10_usize,  ()),
        (17_usize,  ()),
        (22_usize,  ()),
        (23_usize,  ()),
        (47_usize,  ()),
        (79_usize,  ()),
        (141_usize, ()),
        (176_usize, ()),
        (200_usize, ()),
    ]);
    map.sort_unstable_keys::<usize, (), hash::RandomState, alloc::Global>();

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_sort_unstable_keys3() {
    let mut map = TypeErasedIndexMap::from([
        (200_usize, ()),
        (176_usize, ()),
        (141_usize, ()),
        (79_usize,  ()),
        (47_usize,  ()),
        (23_usize,  ()),
        (22_usize,  ()),
        (17_usize,  ()),
        (10_usize,  ()),
        (7_usize,   ()),
        (6_usize,   ()),
    ]);
    let expected = TypeErasedIndexMap::from([
        (6_usize,   ()),
        (7_usize,   ()),
        (10_usize,  ()),
        (17_usize,  ()),
        (22_usize,  ()),
        (23_usize,  ()),
        (47_usize,  ()),
        (79_usize,  ()),
        (141_usize, ()),
        (176_usize, ()),
        (200_usize, ()),
    ]);
    map.sort_unstable_keys::<usize, (), hash::RandomState, alloc::Global>();

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_sort_unstable_by1() {
    let mut map = TypeErasedIndexMap::from([
        (1952_usize, 1390_i32),
        (2900_usize, 2846_i32),
        (2999_usize, 760_i32),
        (828_usize,  491_i32),
        (1738_usize, 1984_i32),
        (339_usize,  1996_i32),
    ]);
    let expected = TypeErasedIndexMap::from([
        (828_usize,  491_i32),
        (2999_usize, 760_i32),
        (1952_usize, 1390_i32),
        (1738_usize, 1984_i32),
        (339_usize,  1996_i32),
        (2900_usize, 2846_i32),
    ]);
    map.sort_unstable_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k1, v1, _k2, v2| v1.cmp(v2));

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_sort_unstable_by2() {
    let mut map = TypeErasedIndexMap::from([
        (String::from("4"),   ()),
        (String::from("101"), ()),
        (String::from("1"),   ()),
        (String::from("2"),   ()),
        (String::from("10"),  ()),
        (String::from("3"),   ()),
    ]);
    let expected = TypeErasedIndexMap::from([
        (String::from("1"),   ()),
        (String::from("10"),  ()),
        (String::from("101"), ()),
        (String::from("2"),   ()),
        (String::from("3"),   ()),
        (String::from("4"),   ()),
    ]);
    map.sort_unstable_by::<_, String, (), hash::RandomState, alloc::Global>(|k1, _v1, k2, _v2| k1.cmp(k2));

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<String, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<String, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_sort_unstable_by3() {
    let mut map = TypeErasedIndexMap::from([
        (String::from("400"), ()),
        (String::from("101"), ()),
        (String::from("1"),   ()),
        (String::from("2"),   ()),
        (String::from("10"),  ()),
        (String::from("3"),   ()),
    ]);
    let expected = TypeErasedIndexMap::from([
        (String::from("1"),   ()),
        (String::from("2"),   ()),
        (String::from("3"),   ()),
        (String::from("10"),  ()),
        (String::from("400"), ()),
        (String::from("101"), ()),
    ]);
    map.sort_unstable_by::<_, String, (), hash::RandomState, alloc::Global>(|k1, _v1, k2, _v2| k1.len().cmp(&k2.len()));

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<String, (), hash::RandomState, alloc::Global>(),
        expected.as_slice::<String, (), hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_reverse() {
    let mut map = TypeErasedIndexMap::from([
        (39_usize,   2757_i32),
        (144_usize,  1357_i32),
        (1846_usize, 1138_i32),
        (698_usize,  473_i32),
        (642_usize,  2172_i32),
        (2101_usize, 1894_i32),
    ]);
    let expected = TypeErasedIndexMap::from([
        (2101_usize, 1894_i32),
        (642_usize,  2172_i32),
        (698_usize,  473_i32),
        (1846_usize, 1138_i32),
        (144_usize,  1357_i32),
        (39_usize,   2757_i32),
    ]);
    map.reverse::<usize, i32, hash::RandomState, alloc::Global>();

    assert_eq!(map.len(), expected.len());
    assert_eq!(
        map.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
        expected.as_slice::<usize, i32, hash::RandomState, alloc::Global>(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_binary_search_by1() {
    let map = TypeErasedIndexMap::new::<usize, i32>();

    for i in -128..128 {
        assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&i)), Err(0));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_binary_search_by2() {
    let map = TypeErasedIndexMap::from([(92_usize, 4_i32)]);

    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&0)), Err(0));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&1)), Err(0));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&2)), Err(0));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&3)), Err(0));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&4)), Ok(0));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&5)), Err(1));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&6)), Err(1));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&7)), Err(1));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&8)), Err(1));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_binary_search_by3() {
    let map = TypeErasedIndexMap::from([
        (130_usize, 1_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
    ]);

    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&0)), Err(0));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&1)), Ok(0));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&2)), Err(1));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&3)), Err(1));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&4)), Ok(1));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&5)), Err(2));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&6)), Err(2));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&7)), Ok(2));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&8)), Err(3));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&9)), Err(3));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_binary_search_by4() {
    let map = TypeErasedIndexMap::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
        (9_usize,   8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&0)),  Err(0));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&1)),  Ok(0));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&2)),  Err(1));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&3)),  Ok(1));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&4)),  Ok(2));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&5)),  Err(3));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&6)),  Err(3));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&7)),  Ok(3));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&8)),  Ok(4));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&9)),  Ok(5));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&10)), Err(6));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&11)), Err(6));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_binary_search_by5() {
    let map = TypeErasedIndexMap::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (60_usize,  4_i32),
        (9_usize,   4_i32),
        (16_usize,  7_i32),
        (19_usize,  8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&0)),  Err(0));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&1)),  Ok(0));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&2)),  Err(1));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&3)),  Ok(1));

    assert!(match map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&4)) {
        Ok(2..=4) => true,
        _ => false,
    });

    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&5)),  Err(5));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&6)),  Err(5));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&7)),  Ok(5));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&8)),  Ok(6));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&9)),  Ok(7));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&10)), Err(8));
    assert_eq!(map.binary_search_by::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| v.cmp(&11)), Err(8));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_binary_search_by_key1() {
    let map = TypeErasedIndexMap::new::<usize, i32>();

    for i in -128..128 {
        assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&i, |_k, v| *v), Err(0));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_binary_search_by_key2() {
    let map = TypeErasedIndexMap::from([(92_usize, 4_i32)]);

    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&0, |_k, v| *v), Err(0));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&1, |_k, v| *v), Err(0));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&2, |_k, v| *v), Err(0));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&3, |_k, v| *v), Err(0));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&4, |_k, v| *v), Ok(0));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&5, |_k, v| *v), Err(1));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&6, |_k, v| *v), Err(1));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&7, |_k, v| *v), Err(1));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&8, |_k, v| *v), Err(1));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_binary_search_by_key3() {
    let map = TypeErasedIndexMap::from([
        (130_usize, 1_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
    ]);

    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&0, |_k, v| *v), Err(0));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&1, |_k, v| *v), Ok(0));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&2, |_k, v| *v), Err(1));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&3, |_k, v| *v), Err(1));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&4, |_k, v| *v), Ok(1));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&5, |_k, v| *v), Err(2));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&6, |_k, v| *v), Err(2));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&7, |_k, v| *v), Ok(2));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&8, |_k, v| *v), Err(3));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&9, |_k, v| *v), Err(3));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_binary_search_by_key4() {
    let map = TypeErasedIndexMap::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
        (9_usize,   8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&0,  |_k, v| *v),  Err(0));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&1,  |_k, v| *v),  Ok(0));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&2,  |_k, v| *v),  Err(1));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&3,  |_k, v| *v),  Ok(1));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&4,  |_k, v| *v),  Ok(2));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&5,  |_k, v| *v),  Err(3));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&6,  |_k, v| *v),  Err(3));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&7,  |_k, v| *v),  Ok(3));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&8,  |_k, v| *v),  Ok(4));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&9,  |_k, v| *v),  Ok(5));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&10, |_k, v| *v), Err(6));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&11, |_k, v| *v), Err(6));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_binary_search_by_key5() {
    let map = TypeErasedIndexMap::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (60_usize,  4_i32),
        (9_usize,   4_i32),
        (16_usize,  7_i32),
        (19_usize,  8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&0, |_k, v| *v),  Err(0));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&1, |_k, v| *v),  Ok(0));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&2, |_k, v| *v),  Err(1));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&3, |_k, v| *v),  Ok(1));

    assert!(match map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&4, |_k, v| *v) {
        Ok(2..=4) => true,
        _ => false,
    });

    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&5,  |_k, v| *v), Err(5));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&6,  |_k, v| *v), Err(5));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&7,  |_k, v| *v), Ok(5));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&8,  |_k, v| *v), Ok(6));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&9,  |_k, v| *v), Ok(7));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&10, |_k, v| *v), Err(8));
    assert_eq!(map.binary_search_by_key::<_, _, usize, i32, hash::RandomState, alloc::Global>(&11, |_k, v| *v), Err(8));
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_partition_point1() {
    let map = TypeErasedIndexMap::new::<usize, i32>();

    for i in -128..128 {
        assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < i), 0);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_partition_point2() {
    let map = TypeErasedIndexMap::from([(92_usize, 4_i32)]);

    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 0), 0);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 1), 0);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 2), 0);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 3), 0);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 4), 0);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 5), 1);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 6), 1);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 7), 1);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 8), 1);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_partition_point3() {
    let map = TypeErasedIndexMap::from([
        (130_usize, 1_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
    ]);

    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 0), 0);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 1), 0);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 2), 1);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 3), 1);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 4), 1);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 5), 2);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 6), 2);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 7), 2);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 8), 3);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 9), 3);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_partition_point4() {
    let map = TypeErasedIndexMap::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
        (9_usize,   8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 0),  0);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 1),  0);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 2),  1);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 3),  1);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 4),  2);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 5),  3);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 6),  3);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 7),  3);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 8),  4);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 9),  5);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 10), 6);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 11), 6);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_partition_point5() {
    let map = TypeErasedIndexMap::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (60_usize,  4_i32),
        (9_usize,   4_i32),
        (16_usize,  7_i32),
        (19_usize,  8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 0), 0);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 1), 0);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 2), 1);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 3), 1);

    assert!(match map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 4) {
        2..=4 => true,
        _ => false,
    });

    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 5),  5);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 6),  5);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 7),  5);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 8),  6);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 9),  7);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 10), 8);
    assert_eq!(map.partition_point::<_, usize, i32, hash::RandomState, alloc::Global>(|_k, v| *v < 11), 8);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_reserve1() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    let additional = 100;

    assert_eq!(map.capacity(), 0);

    map.reserve::<usize, usize, hash::RandomState, alloc::Global>(additional);

    assert!(map.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_reserve2() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    let additional = 100;

    assert_eq!(map.capacity(), 0);

    map.reserve::<usize, usize, hash::RandomState, alloc::Global>(additional);

    assert!(map.capacity() >= additional);

    let old_capacity = map.capacity();
    map.insert::<usize, usize, hash::RandomState, alloc::Global>(0, usize::MAX);
    for i in 1..(map.capacity() - 1) {
        map.insert::<usize, usize, hash::RandomState, alloc::Global>(i, 0);
    }

    map.insert::<usize, usize, hash::RandomState, alloc::Global>(map.capacity() - 1, usize::MAX);

    assert_eq!(map.len(), map.capacity());
    assert_eq!(map.capacity(), old_capacity);

    assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[0], usize::MAX);
    for i in 1..(map.len() - 1) {
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[i], 0);
    }
    assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[map.len() - 1], usize::MAX);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_reserve3() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.len(), 0);

    for i in 0..4 {
        let old_capacity = map.capacity();
        map.reserve::<usize, usize, hash::RandomState, alloc::Global>(additional);

        assert!(map.capacity() >= old_capacity + additional);
        assert!(map.len() <= map.capacity());

        let length = map.len();
        map.insert::<usize, usize, hash::RandomState, alloc::Global>(length, usize::MAX);
        for j in (length + 1)..(map.capacity() - 1) {
            map.insert::<usize, usize, hash::RandomState, alloc::Global>(j, i);
        }
        map.insert::<usize, usize, hash::RandomState, alloc::Global>(map.capacity() - 1, usize::MAX);

        assert_eq!(map.len(), map.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..4 {
        for j in (current_start + 1)..map.len() {
            if map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[current_start], usize::MAX);
        for value in map
            .as_slice::<usize, usize, hash::RandomState, alloc::Global>()[(current_start + 1)..current_end]
            .values()
            .copied()
        {
            assert_eq!(value, i);
        }
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_reserve_exact1() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    let additional = 100;

    assert_eq!(map.capacity(), 0);

    map.reserve_exact::<usize, usize, hash::RandomState, alloc::Global>(additional);

    assert!(map.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_reserve_exact2() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    let additional = 100;

    assert_eq!(map.capacity(), 0);

    map.reserve_exact::<usize, usize, hash::RandomState, alloc::Global>(additional);

    assert!(map.capacity() >= additional);

    let old_capacity = map.capacity();
    map.insert::<usize, usize, hash::RandomState, alloc::Global>(0, usize::MAX);
    for i in 1..(map.capacity() - 1) {
        map.insert::<usize, usize, hash::RandomState, alloc::Global>(i, 0);
    }

    map.insert::<usize, usize, hash::RandomState, alloc::Global>(map.capacity() - 1, usize::MAX);

    assert_eq!(map.len(), map.capacity());
    assert_eq!(map.capacity(), old_capacity);

    assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[0], usize::MAX);
    for i in 1..(map.len() - 1) {
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[i], 0);
    }
    assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[map.len() - 1], usize::MAX);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_reserve_exact3() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.len(), 0);

    for i in 0..32 {
        let old_capacity = map.capacity();
        map.reserve_exact::<usize, usize, hash::RandomState, alloc::Global>(additional);

        assert!(map.capacity() >= old_capacity + additional);
        assert!(map.len() <= map.capacity());

        let length = map.len();
        map.insert::<usize, usize, hash::RandomState, alloc::Global>(length, usize::MAX);
        for j in (length + 1)..(map.capacity() - 1) {
            map.insert::<usize, usize, hash::RandomState, alloc::Global>(j, i);
        }
        map.insert::<usize, usize, hash::RandomState, alloc::Global>(map.capacity() - 1, usize::MAX);

        assert_eq!(map.len(), map.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..32 {
        for j in (current_start + 1)..map.len() {
            if map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[current_start], usize::MAX);
        for value in map
            .as_slice::<usize, usize, hash::RandomState, alloc::Global>()[(current_start + 1)..current_end]
            .values()
            .copied()
        {
            assert_eq!(value, i);
        }
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_try_reserve1() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.try_reserve::<usize, usize, hash::RandomState, alloc::Global>(additional), Ok(()));
    assert!(map.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_try_reserve2() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.try_reserve::<usize, usize, hash::RandomState, alloc::Global>(additional), Ok(()));
    assert!(map.capacity() >= additional);

    let old_capacity = map.capacity();
    map.insert::<usize, usize, hash::RandomState, alloc::Global>(0, usize::MAX);
    for i in 1..(map.capacity() - 1) {
        map.insert::<usize, usize, hash::RandomState, alloc::Global>(i, 0);
    }

    map.insert::<usize, usize, hash::RandomState, alloc::Global>(map.capacity() - 1, usize::MAX);

    assert_eq!(map.len(), map.capacity());
    assert_eq!(map.capacity(), old_capacity);

    assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[0], usize::MAX);
    for i in 1..(map.len() - 1) {
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[i], 0);
    }
    assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[map.len() - 1], usize::MAX);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_try_reserve3() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.len(), 0);

    for i in 0..4 {
        let old_capacity = map.capacity();
        assert_eq!(map.try_reserve::<usize, usize, hash::RandomState, alloc::Global>(additional), Ok(()));

        assert!(map.capacity() >= old_capacity + additional);
        assert!(map.len() <= map.capacity());

        let length = map.len();
        map.insert::<usize, usize, hash::RandomState, alloc::Global>(length, usize::MAX);
        for j in (length + 1)..(map.capacity() - 1) {
            map.insert::<usize, usize, hash::RandomState, alloc::Global>(j, i);
        }
        map.insert::<usize, usize, hash::RandomState, alloc::Global>(map.capacity() - 1, usize::MAX);

        assert_eq!(map.len(), map.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..4 {
        for j in (current_start + 1)..map.len() {
            if map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[current_start], usize::MAX);
        for value in map
            .as_slice::<usize, usize, hash::RandomState, alloc::Global>()[(current_start + 1)..current_end]
            .values()
            .copied()
        {
            assert_eq!(value, i);
        }
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_try_reserve_exact1() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.try_reserve_exact::<usize, usize, hash::RandomState, alloc::Global>(additional), Ok(()));
    assert!(map.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_try_reserve_exact2() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.try_reserve_exact::<usize, usize, hash::RandomState, alloc::Global>(additional), Ok(()));
    assert!(map.capacity() >= additional);

    let old_capacity = map.capacity();
    map.insert::<usize, usize, hash::RandomState, alloc::Global>(0, usize::MAX);
    for i in 1..(map.capacity() - 1) {
        map.insert::<usize, usize, hash::RandomState, alloc::Global>(i, 0);
    }

    map.insert::<usize, usize, hash::RandomState, alloc::Global>(map.capacity() - 1, usize::MAX);

    assert_eq!(map.len(), map.capacity());
    assert_eq!(map.capacity(), old_capacity);

    assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[0], usize::MAX);
    for i in 1..(map.len() - 1) {
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[i], 0);
    }
    assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[map.len() - 1], usize::MAX);
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_try_reserve_exact3() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.len(), 0);

    for i in 0..32 {
        let old_capacity = map.capacity();
        assert_eq!(map.try_reserve_exact::<usize, usize, hash::RandomState, alloc::Global>(additional), Ok(()));

        assert!(map.capacity() >= old_capacity + additional);
        assert!(map.len() <= map.capacity());

        let length = map.len();
        map.insert::<usize, usize, hash::RandomState, alloc::Global>(length, usize::MAX);
        for j in (length + 1)..(map.capacity() - 1) {
            map.insert::<usize, usize, hash::RandomState, alloc::Global>(j, i);
        }
        map.insert::<usize, usize, hash::RandomState, alloc::Global>(map.capacity() - 1, usize::MAX);

        assert_eq!(map.len(), map.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..32 {
        for j in (current_start + 1)..map.len() {
            if map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[current_start], usize::MAX);
        for value in map
            .as_slice::<usize, usize, hash::RandomState, alloc::Global>()[(current_start + 1)..current_end]
            .values()
            .copied()
        {
            assert_eq!(value, i);
        }
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shrink_to_fit1() {
    let mut map = TypeErasedIndexMap::with_capacity::<usize, usize>(10);
    assert_eq!(map.capacity(), 10);

    map.extend::<_, usize, usize, hash::RandomState, alloc::Global>([
        (1_usize, usize::MAX),
        (2_usize, usize::MAX),
        (3_usize, usize::MAX),
    ]);
    assert!(map.len() <= map.capacity());
    map.shrink_to_fit::<usize, usize, hash::RandomState, alloc::Global>();
    assert_eq!(map.len(), map.capacity());
}

#[rustfmt::skip]
#[test]
fn test_type_erased_index_map_shrink_to_fit2() {
    let mut map = TypeErasedIndexMap::new::<usize, usize>();
    for i in 0..128 {
        assert_eq!(map.len(), i);

        map.insert::<usize, usize, hash::RandomState, alloc::Global>(i, i * i);

        assert_eq!(map.len(), i + 1);
        assert!(map.capacity() >= i + 1);
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[i], i * i);
        assert_eq!(map.get::<_, usize, usize, hash::RandomState, alloc::Global>(&i), Some(&(i * i)));

        map.shrink_to_fit::<usize, usize, hash::RandomState, alloc::Global>();

        assert_eq!(map.len(), i + 1);
        assert_eq!(map.capacity(), i + 1);
        assert_eq!(map.as_slice::<usize, usize, hash::RandomState, alloc::Global>()[i], i * i);
        assert_eq!(map.get::<_, usize, usize, hash::RandomState, alloc::Global>(&i), Some(&(i * i)));
    }
}
