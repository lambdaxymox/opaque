use opaque_index_map::{GetDisjointMutError, TypeProjectedIndexMap};

use core::any;
use core::fmt;
use std::iter;
use std::hash;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[test]
fn test_type_projected_index_map_empty_len1() {
    let proj_map: TypeProjectedIndexMap<u64, i64> = TypeProjectedIndexMap::new();

    assert_eq!(proj_map.len(), 0);
}

#[test]
fn test_type_projected_index_map_empty_is_empty1() {
    let proj_map: TypeProjectedIndexMap<u64, i64> = TypeProjectedIndexMap::new();

    assert!(proj_map.is_empty());
}

#[test]
fn test_type_projected_index_map_empty_contains_no_values1() {
    let proj_map: TypeProjectedIndexMap<u64, i64> = TypeProjectedIndexMap::new();
    for key in 0..65536 {
        assert!(!proj_map.contains_key(&key));
    }
}

#[test]
fn test_type_projected_index_map_empty_get1() {
    let proj_map: TypeProjectedIndexMap<u64, i64> = TypeProjectedIndexMap::new();
    for key in 0..65536 {
        let result = proj_map.get(&key);

        assert!(result.is_none());
    }
}

#[test]
fn test_type_projected_index_map_empty_len2() {
    let proj_map: TypeProjectedIndexMap<usize, i64> = TypeProjectedIndexMap::new();

    assert_eq!(proj_map.len(), 0);
}

#[test]
fn test_type_projected_index_map_empty_is_empty2() {
    let proj_map: TypeProjectedIndexMap<usize, i64> = TypeProjectedIndexMap::new();

    assert!(proj_map.is_empty());
}

#[test]
fn test_type_projected_index_map_empty_contains_no_values2() {
    let proj_map: TypeProjectedIndexMap<usize, i64> = TypeProjectedIndexMap::new();
    for key in 0..65536 {
        assert!(!proj_map.contains_key(&key));
    }
}

#[test]
fn test_type_projected_index_map_empty_get2() {
    let proj_map: TypeProjectedIndexMap<usize, i64> = TypeProjectedIndexMap::new();
    for key in 0..65536 {
        let result = proj_map.get(&key);

        assert!(result.is_none());
    }
}

#[test]
fn test_type_projected_index_map_get_index_of1() {
    let mut map = TypeProjectedIndexMap::new();
    assert_eq!(map.get_index_of(&"a"), None);

    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    assert_eq!(map.get_index_of(&"a"), Some(0));
    assert_eq!(map.get_index_of(&"b"), Some(1));
    assert_eq!(map.get_index_of(&"c"), Some(2));
    assert_eq!(map.get_index_of(&"d"), None);
}

#[test]
fn test_type_projected_index_map_get_index_of2() {
    let mut map = TypeProjectedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get_index_of(&0_usize), Some(0));
    assert_eq!(map.get_index_of(&1_usize), Some(1));
    assert_eq!(map.get_index_of(&2_usize), Some(2));
    assert_eq!(map.get_index_of(&3_usize), Some(3));
    assert_eq!(map.get_index_of(&4_usize), Some(4));
    assert_eq!(map.get_index_of(&5_usize), Some(5));
    assert_eq!(map.get_index_of(&6_usize), None);
}

#[test]
fn test_type_projected_index_map_get_index_of3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1), ("b", 2), ("c", 3)]);

    assert_eq!(map.get_index_of(&"a"), Some(0));
    assert_eq!(map.get_index_of(&"c"), Some(2));
    assert_eq!(map.get_index_of(&"b"), Some(1));

    map.swap_remove("b");

    assert_eq!(map.get_index_of(&"a"), Some(0));
    assert_eq!(map.get_index_of(&"c"), Some(1));
    assert_eq!(map.get_index_of(&"b"), None);
}

#[test]
fn test_type_projected_index_map_get_index_of4() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_index_of(&'*'), None);

    map.insert_before(10, '*', ());
    assert_eq!(map.get_index_of(&'*'), Some(10));

    map.insert_before(10, 'a', ());
    assert_eq!(map.get_index_of(&'a'), Some(9));
    assert_eq!(map.get_index_of(&'*'), Some(10));
}

#[test]
fn test_type_projected_index_map_get_index_of5() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_index_of(&'*'), None);

    map.shift_insert(10, '*', ());
    assert_eq!(map.get_index_of(&'*'), Some(10));

    map.shift_insert(10, 'a', ());
    assert_eq!(map.get_index_of(&'a'), Some(10));
    assert_eq!(map.get_index_of(&'*'), Some(9));
}

#[test]
fn test_type_projected_index_map_get_index_of6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1), ("b", 2)]);

    assert_eq!(map.get_index_of(&"a"), Some(0));
    assert_eq!(map.get_index_of(&"b"), Some(1));
    assert_eq!(map.get_index_of(&"c"), None);

    map.insert("c", 3);

    assert_eq!(map.get_index_of(&"a"), Some(0));
    assert_eq!(map.get_index_of(&"b"), Some(1));
    assert_eq!(map.get_index_of(&"c"), Some(2));
}

#[test]
fn test_type_projected_index_map_get1() {
    let mut map = TypeProjectedIndexMap::new();
    assert_eq!(map.get_index_of(&"a"), None);

    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"b"), Some(&2));
    assert_eq!(map.get(&"c"), Some(&3));
    assert_eq!(map.get(&"d"), None);
}

#[test]
fn test_type_projected_index_map_get2() {
    let map = TypeProjectedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get(&0_usize), Some(&1_i32));
    assert_eq!(map.get(&1_usize), Some(&2_i32));
    assert_eq!(map.get(&2_usize), Some(&3_i32));
    assert_eq!(map.get(&3_usize), Some(&4_i32));
    assert_eq!(map.get(&4_usize), Some(&5_i32));
    assert_eq!(map.get(&5_usize), Some(&6_i32));
    assert_eq!(map.get(&6_usize), None);
}

#[test]
fn test_type_projected_index_map_get3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1), ("b", 2), ("c", 3)]);

    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"c"), Some(&3));
    assert_eq!(map.get(&"b"), Some(&2));

    map.swap_remove("b");

    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"c"), Some(&3));
    assert_eq!(map.get(&"b"), None);
}

#[test]
fn test_type_projected_index_map_get4() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get(&'*'), None);

    map.insert_before(10, '*', ());
    assert_eq!(map.get(&'*'), Some(&()));

    map.insert_before(10, 'a', ());
    assert_eq!(map.get(&'a'), Some(&()));
    assert_eq!(map.get(&'*'), Some(&()));
}

#[test]
fn test_type_projected_index_map_get5() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get(&'*'), None);

    map.shift_insert(10, '*', ());
    assert_eq!(map.get(&'*'), Some(&()));

    map.shift_insert(10, 'a', ());
    assert_eq!(map.get(&'a'), Some(&()));
    assert_eq!(map.get(&'*'), Some(&()));
}

#[test]
fn test_type_projected_index_map_get6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1), ("b", 2)]);

    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"b"), Some(&2));
    assert_eq!(map.get(&"c"), None);

    map.insert("c", 3);

    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"b"), Some(&2));
    assert_eq!(map.get(&"c"), Some(&3));
}

#[test]
fn test_type_projected_index_map_get_key_value1() {
    let mut map = TypeProjectedIndexMap::new();
    assert_eq!(map.get_index_of(&"a"), None);

    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    assert_eq!(map.get_key_value(&"a"), Some((&"a", &1)));
    assert_eq!(map.get_key_value(&"b"), Some((&"b", &2)));
    assert_eq!(map.get_key_value(&"c"), Some((&"c", &3)));
    assert_eq!(map.get_key_value(&"d"), None);
}

#[test]
fn test_type_projected_index_map_get_key_value2() {
    let map = TypeProjectedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get_key_value(&0_usize), Some((&0_usize, &1_i32)));
    assert_eq!(map.get_key_value(&1_usize), Some((&1_usize, &2_i32)));
    assert_eq!(map.get_key_value(&2_usize), Some((&2_usize, &3_i32)));
    assert_eq!(map.get_key_value(&3_usize), Some((&3_usize, &4_i32)));
    assert_eq!(map.get_key_value(&4_usize), Some((&4_usize, &5_i32)));
    assert_eq!(map.get_key_value(&5_usize), Some((&5_usize, &6_i32)));
    assert_eq!(map.get_key_value(&6_usize), None);
}

#[test]
fn test_type_projected_index_map_get_key_value3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1), ("b", 2), ("c", 3)]);

    assert_eq!(map.get_key_value(&"a"), Some((&"a", &1)));
    assert_eq!(map.get_key_value(&"c"), Some((&"c", &3)));
    assert_eq!(map.get_key_value(&"b"), Some((&"b", &2)));

    map.swap_remove("b");

    assert_eq!(map.get_key_value(&"a"), Some((&"a", &1)));
    assert_eq!(map.get_key_value(&"c"), Some((&"c", &3)));
    assert_eq!(map.get_key_value(&"b"), None);
}

#[test]
fn test_type_projected_index_map_get_key_value4() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_key_value(&'*'), None);

    map.insert_before(10, '*', ());
    assert_eq!(map.get_key_value(&'*'), Some((&'*', &())));

    map.insert_before(10, 'a', ());
    assert_eq!(map.get_key_value(&'a'), Some((&'a', &())));
    assert_eq!(map.get_key_value(&'*'), Some((&'*', &())));
}

#[test]
fn test_type_projected_index_map_get_key_value5() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_key_value(&'*'), None);

    map.shift_insert(10, '*', ());
    assert_eq!(map.get_key_value(&'*'), Some((&'*', &())));

    map.shift_insert(10, 'a', ());
    assert_eq!(map.get_key_value(&'a'), Some((&'a', &())));
    assert_eq!(map.get_key_value(&'*'), Some((&'*', &())));
}

#[test]
fn test_type_projected_index_map_get_key_value6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1), ("b", 2)]);

    assert_eq!(map.get_key_value(&"a"), Some((&"a", &1)));
    assert_eq!(map.get_key_value(&"b"), Some((&"b", &2)));
    assert_eq!(map.get_key_value(&"c"), None);

    map.insert("c", 3);

    assert_eq!(map.get_key_value(&"a"), Some((&"a", &1)));
    assert_eq!(map.get_key_value(&"b"), Some((&"b", &2)));
    assert_eq!(map.get_key_value(&"c"), Some((&"c", &3)));
}

#[test]
fn test_type_projected_index_map_get_full1() {
    let mut map = TypeProjectedIndexMap::new();
    assert_eq!(map.get_index_of(&"a"), None);

    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);

    assert_eq!(map.get_full(&"a"), Some((0, &"a", &1)));
    assert_eq!(map.get_full(&"b"), Some((1, &"b", &2)));
    assert_eq!(map.get_full(&"c"), Some((2, &"c", &3)));
    assert_eq!(map.get_full(&"d"), None);
}

#[test]
fn test_type_projected_index_map_get_full2() {
    let map = TypeProjectedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get_full(&0_usize), Some((0, &0_usize, &1_i32)));
    assert_eq!(map.get_full(&1_usize), Some((1, &1_usize, &2_i32)));
    assert_eq!(map.get_full(&2_usize), Some((2, &2_usize, &3_i32)));
    assert_eq!(map.get_full(&3_usize), Some((3, &3_usize, &4_i32)));
    assert_eq!(map.get_full(&4_usize), Some((4, &4_usize, &5_i32)));
    assert_eq!(map.get_full(&5_usize), Some((5, &5_usize, &6_i32)));
    assert_eq!(map.get_full(&6_usize), None);
}

#[test]
fn test_type_projected_index_map_get_full3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1), ("b", 2), ("c", 3)]);

    assert_eq!(map.get_full(&"a"), Some((0, &"a", &1)));
    assert_eq!(map.get_full(&"c"), Some((2, &"c", &3)));
    assert_eq!(map.get_full(&"b"), Some((1, &"b", &2)));

    map.swap_remove("b");

    assert_eq!(map.get_full(&"a"), Some((0, &"a", &1)));
    assert_eq!(map.get_full(&"c"), Some((1, &"c", &3)));
    assert_eq!(map.get_full(&"b"), None);
}

#[test]
fn test_type_projected_index_map_get_full4() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_full(&'*'), None);

    map.insert_before(10, '*', ());
    assert_eq!(map.get_full(&'*'), Some((10, &'*', &())));

    map.insert_before(10, 'a', ());
    assert_eq!(map.get_full(&'a'), Some((9, &'a', &())));
    assert_eq!(map.get_full(&'*'), Some((10, &'*', &())));
}

#[test]
fn test_type_projected_index_map_get_full5() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_full(&'*'), None);

    map.shift_insert(10, '*', ());
    assert_eq!(map.get_full(&'*'), Some((10, &'*', &())));

    map.shift_insert(10, 'a', ());
    assert_eq!(map.get_full(&'a'), Some((10, &'a', &())));
    assert_eq!(map.get_full(&'*'), Some((9, &'*', &())));
}

#[test]
fn test_type_projected_index_map_get_full6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1), ("b", 2)]);

    assert_eq!(map.get_full(&"a"), Some((0, &"a", &1)));
    assert_eq!(map.get_full(&"b"), Some((1, &"b", &2)));
    assert_eq!(map.get_full(&"c"), None);

    map.insert("c", 3);

    assert_eq!(map.get_full(&"a"), Some((0, &"a", &1)));
    assert_eq!(map.get_full(&"b"), Some((1, &"b", &2)));
    assert_eq!(map.get_full(&"c"), Some((2, &"c", &3)));
}

#[test]
fn test_type_projected_index_map_get_disjoint_mut2() {
    let mut map = TypeProjectedIndexMap::from([
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
    let result = map.get_disjoint_mut([&"1", &"2", &"3", &"4", &"5", &"6"]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_mut3() {
    let mut map = TypeProjectedIndexMap::from([
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
    let result = map.get_disjoint_mut([&"1", &"2", &"3"]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_mut4() {
    let mut map = TypeProjectedIndexMap::from([
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
    let result = map.get_disjoint_mut([&"4", &"5", &"6"]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_mut5() {
    let mut map = TypeProjectedIndexMap::from([
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
    let result = map.get_disjoint_mut([&"1", &"3", &"5"]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_mut6() {
    let mut map = TypeProjectedIndexMap::from([
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
    let result = map.get_disjoint_mut([&"2", &"4", &"6"]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_mut_partial_success1() {
    let mut map = TypeProjectedIndexMap::from([
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
    let result = map.get_disjoint_mut([&"1", &"20", &"3", &"40", &"5", &"60"]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_mut_partial_success2() {
    let mut map = TypeProjectedIndexMap::from([
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
    let result = map.get_disjoint_mut(["1", "2", "3", "200", "4", "5", "6", "100"]);

    assert_eq!(result, expected);
}

#[test]
#[should_panic]
fn test_type_projected_index_map_get_disjoint_mut_repeat_indices1() {
    let mut map = TypeProjectedIndexMap::from([
        ("1", 10),
        ("2", 20),
        ("3", 30),
        ("4", 40),
        ("5", 50),
        ("6", 60),
    ]);
    let _ = map.get_disjoint_mut(["1", "2", "2", "4", "5", "6"]);

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_projected_index_map_get_disjoint_mut_repeat_indices2() {
    let mut map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([
        (1, 10),
        (2, 20),
        (3, 30),
        (4, 40),
        (5, 50),
        (6, 60),
    ]);
    let _ = map.get_disjoint_mut([&1, &1, &1, &2, &2, &3]);

    assert!(true);
}

#[test]
fn test_type_projected_index_map_get_disjoint_indices_mut1() {
    let mut map: TypeProjectedIndexMap<u32, u32> = TypeProjectedIndexMap::from([
        (1, 10),
        (2, 20),
        (3, 30),
    ]);
    let expected = Ok([]);
    let result = map.get_disjoint_indices_mut([]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_indices_mut2() {
    let mut map: TypeProjectedIndexMap<u32, u32> = TypeProjectedIndexMap::from([
        (1, 10),
        (2, 20),
        (3, 30),
    ]);
    let expected = Ok([(&1, &mut 10)]);
    let result = map.get_disjoint_indices_mut([0]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_indices_mut3() {
    let mut map: TypeProjectedIndexMap<u32, u32> = TypeProjectedIndexMap::from([
        (1, 10),
        (2, 20),
        (3, 30),
    ]);
    let expected = Ok([(&2, &mut 20)]);
    let result = map.get_disjoint_indices_mut([1]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_indices_mut4() {
    let mut map: TypeProjectedIndexMap<u32, u32> = TypeProjectedIndexMap::from([
        (1, 10),
        (2, 20),
        (3, 30),
    ]);
    let expected = Ok([(&1, &mut 10), (&2, &mut 20)]);
    let result = map.get_disjoint_indices_mut([0, 1]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_indices_mut_out_of_bounds() {
    let mut map: TypeProjectedIndexMap<u32, u32> = TypeProjectedIndexMap::from([
        (1, 10),
        (2, 20),
        (3, 30),
    ]);
    let expected =  Err(GetDisjointMutError::IndexOutOfBounds);
    let result = map.get_disjoint_indices_mut([1, 3]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_indices_mut_fail_duplicate() {
    let mut map: TypeProjectedIndexMap<u32, u32> = TypeProjectedIndexMap::from([
        (1, 10),
        (2, 20),
        (3, 30),
    ]);
    let expected = Err(GetDisjointMutError::OverlappingIndices);
    let result = map.get_disjoint_indices_mut([1, 0, 1]);

    assert_eq!(result, expected);
}
