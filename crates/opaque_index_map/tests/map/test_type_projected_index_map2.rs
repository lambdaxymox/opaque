use opaque_index_map::{GetDisjointMutError, TypeProjectedIndexMap};
use opaque_vec::TypeProjectedVec;

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
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 20_i32),
        (3_usize, 30_i32),
        (4_usize, 40_i32),
        (5_usize, 50_32),
        (6_usize, 60_i32),
    ]);
    let _ = map.get_disjoint_mut([&1, &1, &1, &2, &2, &3]);

    assert!(true);
}

#[test]
fn test_type_projected_index_map_get_disjoint_indices_mut1() {
    let mut map = TypeProjectedIndexMap::from([
        (1_u32, 10_i32),
        (2_u32, 20_i32),
        (3_u32, 30_i32),
    ]);
    let expected = Ok([]);
    let result = map.get_disjoint_indices_mut([]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_indices_mut2() {
    let mut map = TypeProjectedIndexMap::from([
        (1_u32, 10_i32),
        (2_u32, 20_i32),
        (3_u32, 30_i32),
    ]);
    let expected = Ok([(&1_u32, &mut 10_i32)]);
    let result = map.get_disjoint_indices_mut([0]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_indices_mut3() {
    let mut map = TypeProjectedIndexMap::from([
        (1_u32, 10_i32),
        (2_u32, 20_i32),
        (3_u32, 30_i32),
    ]);
    let expected = Ok([(&2_u32, &mut 20_i32)]);
    let result = map.get_disjoint_indices_mut([1]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_indices_mut4() {
    let mut map = TypeProjectedIndexMap::from([
        (1_u32, 10_i32),
        (2_u32, 20_i32),
        (3_u32, 30_i32),
    ]);
    let expected = Ok([(&1_u32, &mut 10_i32), (&2_u32, &mut 20_i32)]);
    let result = map.get_disjoint_indices_mut([0, 1]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_indices_mut_out_of_bounds() {
    let mut map = TypeProjectedIndexMap::from([
        (1_u32, 10_i32),
        (2_u32, 20_i32),
        (3_u32, 30_i32),
    ]);
    let expected =  Err(GetDisjointMutError::IndexOutOfBounds);
    let result = map.get_disjoint_indices_mut([1, 3]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_get_disjoint_indices_mut_fail_duplicate() {
    let mut map = TypeProjectedIndexMap::from([
        (1_u32, 10_i32),
        (2_u32, 20_i32),
        (3_u32, 30_i32),
    ]);
    let expected = Err(GetDisjointMutError::OverlappingIndices);
    let result = map.get_disjoint_indices_mut([1, 0, 1]);

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_keys1() {
    let map = TypeProjectedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    for key in map.keys() {
        assert!(map.contains_key(key));
    }
}

#[test]
fn test_type_projected_index_map_keys2() {
    let map = TypeProjectedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    let expected = TypeProjectedVec::from([1_usize, 2_usize, 3_usize]);
    let result: TypeProjectedVec<usize> = map.keys().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_keys3() {
    let map = TypeProjectedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    let mut iter = map.keys();

    assert_eq!(iter.next(), Some(&1_usize));
    assert_eq!(iter.next(), Some(&2_usize));
    assert_eq!(iter.next(), Some(&3_usize));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_map_keys4() {
    let map = TypeProjectedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    let mut iter = map.keys();

    assert_eq!(map.get(iter.next().unwrap()), Some(&10_i32));
    assert_eq!(map.get(iter.next().unwrap()), Some(&24_i32));
    assert_eq!(map.get(iter.next().unwrap()), Some(&58_i32));
}

#[test]
fn test_type_projected_index_map_values1() {
    let map = TypeProjectedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    let expected = TypeProjectedVec::from([10_i32, 24_i32, 58_i32]);
    let result: TypeProjectedVec<i32> = map.values().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_values2() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();
    let expected = TypeProjectedVec::new();
    let result: TypeProjectedVec<i32> = map.values().cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_map_values3() {
    let map = TypeProjectedIndexMap::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    let mut iter = map.values();

    assert_eq!(iter.next(), Some(&10_i32));
    assert_eq!(iter.next(), Some(&24_i32));
    assert_eq!(iter.next(), Some(&58_i32));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_map_clear1() {
    let mut map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[test]
fn test_type_projected_index_map_clear2() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert!(!map.is_empty());
    assert_eq!(map.len(), 6);

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[test]
fn test_type_projected_index_map_clear3() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert!(map.contains_key(&1_usize));
    assert!(map.contains_key(&2_usize));
    assert!(map.contains_key(&3_usize));
    assert!(map.contains_key(&4_usize));
    assert!(map.contains_key(&5_usize));
    assert!(map.contains_key(&6_usize));

    map.clear();

    assert!(!map.contains_key(&1_usize));
    assert!(!map.contains_key(&2_usize));
    assert!(!map.contains_key(&3_usize));
    assert!(!map.contains_key(&4_usize));
    assert!(!map.contains_key(&5_usize));
    assert!(!map.contains_key(&6_usize));
}

#[test]
fn test_type_projected_index_map_split_off1() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let expected1 = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);
    let expected2 = TypeProjectedIndexMap::from([
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let result2 = map.split_off(3);
    let result1 = map.clone();

    assert_eq!(result1.len(), expected1.len());
    assert_eq!(result2.len(), expected2.len());
    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);

}

#[test]
fn test_type_projected_index_map_split_off2() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let expected1 = map.clone();
    let expected2 = TypeProjectedIndexMap::new();
    let result2 = map.split_off(map.len());
    let result1 = map.clone();

    assert_eq!(result1.len(), expected1.len());
    assert_eq!(result2.len(), expected2.len());
    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);
}

#[test]
fn test_type_projected_index_map_split_off3() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let expected1 = TypeProjectedIndexMap::new();
    let expected2 = map.clone();
    let result2 = map.split_off(0);
    let result1 = map.clone();

    assert_eq!(result1.len(), expected1.len());
    assert_eq!(result2.len(), expected2.len());
    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);
}

#[test]
#[should_panic]
fn test_type_projected_index_map_split_off_out_of_bounds1() {
    let mut map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();
    let _ = map.split_off(map.len() + 1);

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_projected_index_map_split_off_out_of_bounds2() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let _ = map.split_off(map.len() + 1);

    assert!(true);
}

#[test]
fn test_type_projected_index_map_swap_remove1() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove(&1_usize), Some(20_i32));
    assert_eq!(map.swap_remove(&2_usize), Some(2043_i32));
    assert_eq!(map.swap_remove(&3_usize), Some(4904_i32));
    assert_eq!(map.swap_remove(&4_usize), Some(20994_i32));
    assert_eq!(map.swap_remove(&5_usize), Some(302_i32));
    assert_eq!(map.swap_remove(&6_usize), Some(5_i32));
}

#[test]
fn test_type_projected_index_map_swap_remove2() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);
    let _ = map.swap_remove(&1_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);
    let _ = map.swap_remove(&2_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);
    let _ = map.swap_remove(&3_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (4_usize, 20994_i32),
    ]);
    let _ = map.swap_remove(&4_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(6_usize, 5_i32), (5_usize, 302_i32)]);
    let _ = map.swap_remove(&5_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(6_usize, 5_i32)]);
    let _ = map.swap_remove(&6_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_type_projected_index_map_swap_remove3() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);
    let _ = map.swap_remove(&6_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);
    let _ = map.swap_remove(&5_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);
    let _ = map.swap_remove(&4_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);
    let _ = map.swap_remove(&3_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(1_usize, 20_i32), (2_usize, 2043_i32)]);
    let _ = map.swap_remove(&2_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(1_usize, 20_i32)]);
    let _ = map.swap_remove(&1_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_type_projected_index_map_swap_remove4() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove(&6_usize), Some(5_i32));
    assert_eq!(map.swap_remove(&5_usize), Some(302_i32));
    assert_eq!(map.swap_remove(&4_usize), Some(20994_i32));
    assert_eq!(map.swap_remove(&3_usize), Some(4904_i32));
    assert_eq!(map.swap_remove(&2_usize), Some(2043_i32));
    assert_eq!(map.swap_remove(&1_usize), Some(20_i32));
}

#[test]
fn test_type_projected_index_map_swap_remove_entry1() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove_entry(&1_usize), Some((1_usize, 20_i32)));
    assert_eq!(map.swap_remove_entry(&2_usize), Some((2_usize, 2043_i32)));
    assert_eq!(map.swap_remove_entry(&3_usize), Some((3_usize, 4904_i32)));
    assert_eq!(map.swap_remove_entry(&4_usize), Some((4_usize, 20994_i32)));
    assert_eq!(map.swap_remove_entry(&5_usize), Some((5_usize, 302_i32)));
    assert_eq!(map.swap_remove_entry(&6_usize), Some((6_usize, 5_i32)));
}

#[test]
fn test_type_projected_index_map_swap_remove_entry2() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);
    let _ = map.swap_remove_entry(&1_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);
    let _ = map.swap_remove_entry(&2_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);
    let _ = map.swap_remove_entry(&3_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (4_usize, 20994_i32),
    ]);
    let _ = map.swap_remove_entry(&4_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(6_usize, 5_i32), (5_usize, 302_i32)]);
    let _ = map.swap_remove_entry(&5_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(6_usize, 5_i32)]);
    let _ = map.swap_remove_entry(&6_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_type_projected_index_map_swap_remove_entry3() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);
    let _ = map.swap_remove_entry(&6_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);
    let _ = map.swap_remove_entry(&5_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);
    let _ = map.swap_remove_entry(&4_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);
    let _ = map.swap_remove_entry(&3_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(1_usize, 20_i32), (2_usize, 2043_i32)]);
    let _ = map.swap_remove_entry(&2_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(1_usize, 20_i32)]);
    let _ = map.swap_remove_entry(&1_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_type_projected_index_map_swap_remove_entry4() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove_entry(&6_usize), Some((6_usize, 5_i32)));
    assert_eq!(map.swap_remove_entry(&5_usize), Some((5_usize, 302_i32)));
    assert_eq!(map.swap_remove_entry(&4_usize), Some((4_usize, 20994_i32)));
    assert_eq!(map.swap_remove_entry(&3_usize), Some((3_usize, 4904_i32)));
    assert_eq!(map.swap_remove_entry(&2_usize), Some((2_usize, 2043_i32)));
    assert_eq!(map.swap_remove_entry(&1_usize), Some((1_usize, 20_i32)));
}

#[test]
fn test_type_projected_index_map_swap_remove_full1() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove_full(&1_usize), Some((0, 1_usize, 20_i32)));
    assert_eq!(map.swap_remove_full(&2_usize), Some((1, 2_usize, 2043_i32)));
    assert_eq!(map.swap_remove_full(&3_usize), Some((2, 3_usize, 4904_i32)));
    assert_eq!(map.swap_remove_full(&4_usize), Some((2, 4_usize, 20994_i32)));
    assert_eq!(map.swap_remove_full(&5_usize), Some((1, 5_usize, 302_i32)));
    assert_eq!(map.swap_remove_full(&6_usize), Some((0, 6_usize, 5_i32)));
}

#[test]
fn test_type_projected_index_map_swap_remove_full2() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);
    let _ = map.swap_remove_full(&1_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);
    let _ = map.swap_remove_full(&2_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);
    let _ = map.swap_remove_full(&3_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (4_usize, 20994_i32),
    ]);
    let _ = map.swap_remove_full(&4_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(6_usize, 5_i32), (5_usize, 302_i32)]);
    let _ = map.swap_remove_full(&5_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(6_usize, 5_i32)]);
    let _ = map.swap_remove_full(&6_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_type_projected_index_map_swap_remove_full3() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);
    let _ = map.swap_remove_full(&6_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);
    let _ = map.swap_remove_full(&5_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);
    let _ = map.swap_remove_full(&4_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);
    let _ = map.swap_remove_full(&3_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(1_usize, 20_i32), (2_usize, 2043_i32)]);
    let _ = map.swap_remove_full(&2_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(1_usize, 20_i32)]);
    let _ = map.swap_remove_full(&1_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_type_projected_index_map_swap_remove_full4() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove_full(&6_usize), Some((5, 6_usize, 5_i32)));
    assert_eq!(map.swap_remove_full(&5_usize), Some((4, 5_usize, 302_i32)));
    assert_eq!(map.swap_remove_full(&4_usize), Some((3, 4_usize, 20994_i32)));
    assert_eq!(map.swap_remove_full(&3_usize), Some((2, 3_usize, 4904_i32)));
    assert_eq!(map.swap_remove_full(&2_usize), Some((1, 2_usize, 2043_i32)));
    assert_eq!(map.swap_remove_full(&1_usize), Some((0, 1_usize, 20_i32)));
}

#[test]
fn test_type_projected_index_map_shift_remove1() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove(&1655_usize), Some(2427_i32));
    assert_eq!(map.shift_remove(&1992_usize), Some(2910_i32));
    assert_eq!(map.shift_remove(&783_usize), Some(603_i32));
    assert_eq!(map.shift_remove(&376_usize), Some(834_i32));
    assert_eq!(map.shift_remove(&199_usize), Some(1881_i32));
    assert_eq!(map.shift_remove(&1098_usize), Some(1466_i32));
}

#[test]
fn test_type_projected_index_map_shift_remove2() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);
    let _ = map.shift_remove(&1655_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);
    let _ = map.shift_remove(&1992_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);
    let _ = map.shift_remove(&783_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);
    let _ = map.shift_remove(&376_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(199_usize, 1881_i32), (1098_usize, 1466_i32)]);
    let _ = map.shift_remove(&199_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(1098_usize, 1466_i32)]);
    let _ = map.shift_remove(&1098_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_type_projected_index_map_shift_remove3() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);
    let _ = map.shift_remove(&1098_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
    ]);
    let _ = map.shift_remove(&199_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
    ]);
    let _ = map.shift_remove(&376_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
    ]);
    let _ = map.shift_remove(&783_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(1655_usize, 2427_i32), (1992_usize, 2910_i32)]);
    let _ = map.shift_remove(&1992_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(1655_usize, 2427_i32)]);
    let _ = map.shift_remove(&1655_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_type_projected_index_map_shift_remove4() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove(&1098_usize), Some(1466_i32));
    assert_eq!(map.shift_remove(&199_usize), Some(1881_i32));
    assert_eq!(map.shift_remove(&376_usize), Some(834_i32));
    assert_eq!(map.shift_remove(&783_usize), Some(603_i32));
    assert_eq!(map.shift_remove(&1992_usize), Some(2910_i32));
    assert_eq!(map.shift_remove(&1655_usize), Some(2427_i32));
}

#[test]
fn test_type_projected_index_map_shift_remove_entry1() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_entry(&1655_usize), Some((1655_usize, 2427_i32)));
    assert_eq!(map.shift_remove_entry(&1992_usize), Some((1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_entry(&783_usize), Some((783_usize, 603_i32)));
    assert_eq!(map.shift_remove_entry(&376_usize), Some((376_usize, 834_i32)));
    assert_eq!(map.shift_remove_entry(&199_usize), Some((199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_entry(&1098_usize), Some((1098_usize, 1466_i32)));
}

#[test]
fn test_type_projected_index_map_shift_remove_entry2() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);
    let _ = map.shift_remove_entry(&1655_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);
    let _ = map.shift_remove_entry(&1992_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);
    let _ = map.shift_remove_entry(&783_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);
    let _ = map.shift_remove_entry(&376_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(199_usize, 1881_i32), (1098_usize, 1466_i32)]);
    let _ = map.shift_remove_entry(&199_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(1098_usize, 1466_i32)]);
    let _ = map.shift_remove_entry(&1098_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_type_projected_index_map_shift_remove_entry3() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);
    let _ = map.shift_remove_entry(&1098_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
    ]);
    let _ = map.shift_remove_entry(&199_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
    ]);
    let _ = map.shift_remove_entry(&376_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
    ]);
    let _ = map.shift_remove_entry(&783_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(1655_usize, 2427_i32), (1992_usize, 2910_i32)]);
    let _ = map.shift_remove_entry(&1992_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(1655_usize, 2427_i32)]);
    let _ = map.shift_remove_entry(&1655_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_type_projected_index_map_shift_remove_entry4() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_entry(&1098_usize), Some((1098_usize, 1466_i32)));
    assert_eq!(map.shift_remove_entry(&199_usize), Some((199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_entry(&376_usize), Some((376_usize, 834_i32)));
    assert_eq!(map.shift_remove_entry(&783_usize), Some((783_usize, 603_i32)));
    assert_eq!(map.shift_remove_entry(&1992_usize), Some((1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_entry(&1655_usize), Some((1655_usize, 2427_i32)));
}

#[test]
fn test_type_projected_index_map_shift_remove_full1() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_full(&1655_usize), Some((0, 1655_usize, 2427_i32)));
    assert_eq!(map.shift_remove_full(&1992_usize), Some((0, 1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_full(&783_usize), Some((0, 783_usize, 603_i32)));
    assert_eq!(map.shift_remove_full(&376_usize), Some((0, 376_usize, 834_i32)));
    assert_eq!(map.shift_remove_full(&199_usize), Some((0, 199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_full(&1098_usize), Some((0, 1098_usize, 1466_i32)));
}

#[test]
fn test_type_projected_index_map_shift_remove_full2() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);
    let _ = map.shift_remove_full(&1655_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);
    let _ = map.shift_remove_full(&1992_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);
    let _ = map.shift_remove_full(&783_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);
    let _ = map.shift_remove_full(&376_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(199_usize, 1881_i32), (1098_usize, 1466_i32)]);
    let _ = map.shift_remove_full(&199_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(1098_usize, 1466_i32)]);
    let _ = map.shift_remove_full(&1098_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_type_projected_index_map_shift_remove_full3() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);
    let _ = map.shift_remove_full(&1098_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
    ]);
    let _ = map.shift_remove_full(&199_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
    ]);
    let _ = map.shift_remove_full(&376_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
    ]);
    let _ = map.shift_remove_full(&783_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(1655_usize, 2427_i32), (1992_usize, 2910_i32)]);
    let _ = map.shift_remove_full(&1992_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(1655_usize, 2427_i32)]);
    let _ = map.shift_remove_full(&1655_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[test]
fn test_type_projected_index_map_shift_remove_full4() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize, 603_i32),
        (376_usize, 834_i32),
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_full(&1098_usize), Some((5, 1098_usize, 1466_i32)));
    assert_eq!(map.shift_remove_full(&199_usize), Some((4, 199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_full(&376_usize), Some((3, 376_usize, 834_i32)));
    assert_eq!(map.shift_remove_full(&783_usize), Some((2, 783_usize, 603_i32)));
    assert_eq!(map.shift_remove_full(&1992_usize), Some((1, 1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_full(&1655_usize), Some((0, 1655_usize, 2427_i32)));
}

#[test]
fn test_type_projected_index_map_insert1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.insert(1803_usize, 1778_i32), None);
    assert_eq!(map.insert(1057_usize, 2437_i32), None);
    assert_eq!(map.insert(1924_usize, 185_i32), None);
    assert_eq!(map.insert(302_usize, 2457_i32), None);
    assert_eq!(map.insert(949_usize, 2176_i32), None);
    assert_eq!(map.insert(2968_usize, 1398_i32), None);
}

#[test]
fn test_type_projected_index_map_insert2() {
    let mut map = TypeProjectedIndexMap::new();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    let _ = map.insert(1803_usize, 1778_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(1803_usize, 1778_i32)]);

    let _ = map.insert(1057_usize, 2437_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(1803_usize, 1778_i32), (1057_usize, 2437_i32)]);

    let _ = map.insert(1924_usize, 185_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
    ]);

    let _ = map.insert(302_usize, 2457_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize, 2457_i32),
    ]);

    let _ = map.insert(949_usize, 2176_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize, 2457_i32),
        (949_usize, 2176_i32),
    ]);

    let _ = map.insert(2968_usize, 1398_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize, 2457_i32),
        (949_usize, 2176_i32),
        (2968_usize, 1398_i32),
    ]);
}

#[test]
fn test_type_projected_index_map_insert_full1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.insert_full(1803_usize, 1778_i32), (0, None));
    assert_eq!(map.insert_full(1057_usize, 2437_i32), (1, None));
    assert_eq!(map.insert_full(1924_usize, 185_i32), (2, None));
    assert_eq!(map.insert_full(302_usize, 2457_i32), (3, None));
    assert_eq!(map.insert_full(949_usize, 2176_i32), (4, None));
    assert_eq!(map.insert_full(2968_usize, 1398_i32), (5, None));
}

#[test]
fn test_type_projected_index_map_insert_full2() {
    let mut map = TypeProjectedIndexMap::new();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    let _ = map.insert_full(1803_usize, 1778_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(1803_usize, 1778_i32)]);

    let _ = map.insert_full(1057_usize, 2437_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(1803_usize, 1778_i32), (1057_usize, 2437_i32)]);

    let _ = map.insert_full(1924_usize, 185_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
    ]);

    let _ = map.insert_full(302_usize, 2457_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize, 2457_i32),
    ]);

    let _ = map.insert_full(949_usize, 2176_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize, 2457_i32),
        (949_usize, 2176_i32),
    ]);

    let _ = map.insert_full(2968_usize, 1398_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize, 2457_i32),
        (949_usize, 2176_i32),
        (2968_usize, 1398_i32),
    ]);
}

#[test]
fn test_type_projected_index_map_insert_before1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.insert_before(0, 370_usize, 2339_i32), (0, None));
    assert_eq!(map.insert_before(0, 1977_usize, 2387_i32), (0, None));
    assert_eq!(map.insert_before(0, 1244_usize, 2741_i32), (0, None));
    assert_eq!(map.insert_before(0, 1733_usize, 1838_i32), (0, None));
    assert_eq!(map.insert_before(0, 289_usize, 464_i32), (0, None));
    assert_eq!(map.insert_before(0, 2712_usize, 509_i32), (0, None));
}

#[test]
fn test_type_projected_index_map_insert_before2() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.len(), 0);

    let _ = map.insert_before(0, 370_usize, 2339_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(370_usize, 2339_i32)]);

    let _ = map.insert_before(0, 1977_usize, 2387_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(1977_usize, 2387_i32), (370_usize, 2339_i32)]);

    let _ = map.insert_before(0, 1244_usize, 2741_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize, 2339_i32),
    ]);

    let _ = map.insert_before(0, 1733_usize, 1838_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize, 2339_i32),
    ]);

    let _ = map.insert_before(0, 289_usize, 464_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (289_usize, 464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize, 2339_i32),
    ]);

    let _ = map.insert_before(0, 2712_usize, 509_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
        (2712_usize, 509_i32),
        (289_usize, 464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize, 2339_i32),
    ]);
}

#[test]
fn test_type_projected_index_map_insert_before3() {
    let mut map = TypeProjectedIndexMap::from([
        (2712_usize, 509_i32),
        (289_usize, 464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize, 2339_i32),
    ]);

    assert_eq!(map.len(), 6);

    let result = map.insert_before(4, 289_usize, i32::MAX);
    assert_eq!(result, (3, Some(464_i32)));
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
        (2712_usize, 509_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (289_usize, i32::MAX),
        (1977_usize, 2387_i32),
        (370_usize, 2339_i32),
    ]);
}

#[test]
fn test_type_projected_index_map_insert_before4() {
    let mut map = TypeProjectedIndexMap::from([
        (2712_usize, 509_i32),
        (289_usize, 464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize, 2339_i32),
    ]);

    assert_eq!(map.len(), 6);

    let result = map.insert_before(1, 370_usize, i32::MAX);
    assert_eq!(result, (1, Some(2339_i32)));
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
        (2712_usize, 509_i32),
        (370_usize, i32::MAX),
        (289_usize, 464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
    ]);
}

#[test]
fn test_type_projected_index_map_insert_before5() {
    let mut map = TypeProjectedIndexMap::from([
        (2712_usize, 509_i32),
        (289_usize, 464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize, 2339_i32),
    ]);

    assert_eq!(map.len(), 6);

    let result = map.insert_before(3, 1244_usize, i32::MAX);
    assert_eq!(result, (3, Some(2741_i32)));
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
        (2712_usize, 509_i32),
        (289_usize, 464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, i32::MAX),
        (1977_usize, 2387_i32),
        (370_usize, 2339_i32),
    ]);
}

#[test]
fn test_type_projected_index_map_insert_before6() {
    let mut map = TypeProjectedIndexMap::from([
        (2712_usize, 509_i32),
        (289_usize, 464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize, 2339_i32),
    ]);

    assert_eq!(map.len(), 6);

    let result = map.insert_before(5, usize::MAX, i32::MAX);
    assert_eq!(result, (5, None));
    assert_eq!(map.len(), 7);
    assert_eq!(map.as_slice(), &[
        (2712_usize, 509_i32),
        (289_usize, 464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (usize::MAX, i32::MAX),
        (370_usize, 2339_i32),
    ]);
}

#[test]
fn test_type_projected_index_map_shift_insert1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.shift_insert(0, 1809_usize, 2381_i32), None);
    assert_eq!(map.shift_insert(0, 603_usize, 2834_i32), None);
    assert_eq!(map.shift_insert(0, 2564_usize, 621_i32), None);
    assert_eq!(map.shift_insert(0, 360_usize, 1352_i32), None);
    assert_eq!(map.shift_insert(0, 57_usize, 2657_i32), None);
    assert_eq!(map.shift_insert(0, 477_usize, 2084_i32), None);
}

#[test]
fn test_type_projected_index_map_shift_insert2() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.len(), 0);
    assert!(map.is_empty());

    let _ = map.shift_insert(0, 1809_usize, 2381_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(1809_usize, 2381_i32)]);

    let _ = map.shift_insert(0, 603_usize, 2834_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(603_usize, 2834_i32), (1809_usize, 2381_i32)]);

    let _ = map.shift_insert(0, 2564_usize, 621_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (2564_usize, 621_i32),
        (603_usize, 2834_i32),
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert(0, 360_usize, 1352_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (360_usize, 1352_i32),
        (2564_usize, 621_i32),
        (603_usize, 2834_i32),
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert(0, 57_usize, 2657_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (57_usize, 2657_i32),
        (360_usize, 1352_i32),
        (2564_usize, 621_i32),
        (603_usize, 2834_i32),
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert(0, 477_usize, 2084_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
        (477_usize, 2084_i32),
        (57_usize, 2657_i32),
        (360_usize, 1352_i32),
        (2564_usize, 621_i32),
        (603_usize, 2834_i32),
        (1809_usize, 2381_i32),
    ]);
}

#[test]
fn test_type_projected_index_map_shift_insert3() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.shift_insert(0, 477_usize, 2084_i32), None);
    assert_eq!(map.shift_insert(1, 57_usize, 2657_i32), None);
    assert_eq!(map.shift_insert(2, 360_usize, 1352_i32), None);
    assert_eq!(map.shift_insert(3, 2564_usize, 621_i32), None);
    assert_eq!(map.shift_insert(4, 603_usize, 2834_i32), None);
    assert_eq!(map.shift_insert(5, 1809_usize, 2381_i32), None);
}

#[test]
fn test_type_projected_index_map_shift_insert4() {
    let mut map = TypeProjectedIndexMap::from([
        (477_usize, 2084_i32),
        (57_usize, 2657_i32),
        (360_usize, 1352_i32),
        (2564_usize, 621_i32),
        (603_usize, 2834_i32),
        (1809_usize, 2381_i32),
    ]);
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.len(), 0);
    assert!(map.is_empty());

    let _ = map.shift_insert(0, 477_usize, 2084_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[(477_usize, 2084_i32)]);

    let _ = map.shift_insert(1, 57_usize, 2657_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[(477_usize, 2084_i32), (57_usize, 2657_i32)]);

    let _ = map.shift_insert(2, 360_usize, 1352_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (477_usize, 2084_i32),
        (57_usize, 2657_i32),
        (360_usize, 1352_i32),
    ]);

    let _ = map.shift_insert(3, 2564_usize, 621_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (477_usize, 2084_i32),
        (57_usize, 2657_i32),
        (360_usize, 1352_i32),
        (2564_usize, 621_i32),
    ]);

    let _ = map.shift_insert(4, 603_usize, 2834_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (477_usize, 2084_i32),
        (57_usize, 2657_i32),
        (360_usize, 1352_i32),
        (2564_usize, 621_i32),
        (603_usize, 2834_i32),
    ]);

    let _ = map.shift_insert(5, 1809_usize, 2381_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
        (477_usize, 2084_i32),
        (57_usize, 2657_i32),
        (360_usize, 1352_i32),
        (2564_usize, 621_i32),
        (603_usize, 2834_i32),
        (1809_usize, 2381_i32),
    ]);
}

#[test]
fn test_type_projected_index_map_append1() {
    let mut map1 = TypeProjectedIndexMap::from([
        (605_usize, 2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    let mut map2 = TypeProjectedIndexMap::from([
        (1062_usize, 1113_i32),
        (1875_usize, 800_i32),
        (1724_usize, 2910_i32),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (605_usize, 2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
        (1062_usize, 1113_i32),
        (1875_usize, 800_i32),
        (1724_usize, 2910_i32),
    ]);
    map1.append(&mut map2);

    assert!(map2.is_empty());
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.len(), 7);
    assert_eq!(map1.as_slice(), expected.as_slice());
}

#[test]
fn test_type_projected_index_map_append2() {
    let mut map1 = TypeProjectedIndexMap::from([
        (605_usize, 2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    let mut map2 = TypeProjectedIndexMap::from([
        (1804_usize, i32::MAX),
        (1875_usize, 800_i32),
        (1660_usize, i32::MAX),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (605_usize, 2879_i32),
        (1804_usize, i32::MAX),
        (1532_usize, 1980_i32),
        (1660_usize, i32::MAX),
        (1875_usize, 800_i32),
    ]);
    map1.append(&mut map2);

    assert!(map2.is_empty());
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.len(), 5);
    assert_eq!(map1.as_slice(), expected.as_slice());
}

#[test]
fn test_type_projected_index_map_append3() {
    let mut map1 = TypeProjectedIndexMap::from([
        (605_usize, 2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    let mut map2 = TypeProjectedIndexMap::new();
    let expected = TypeProjectedIndexMap::from([
        (605_usize, 2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    map1.append(&mut map2);

    assert!(map2.is_empty());
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.len(), 4);
    assert_eq!(map1.as_slice(), expected.as_slice());
}

#[test]
fn test_type_projected_index_map_append4() {
    let mut map1 = TypeProjectedIndexMap::new();
    let mut map2 = TypeProjectedIndexMap::from([
        (605_usize, 2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (605_usize, 2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    map1.append(&mut map2);

    assert!(map2.is_empty());
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.len(), 4);
    assert_eq!(map1.as_slice(), expected.as_slice());
}

#[test]
fn test_type_projected_index_map_append5() {
    let mut map1 = TypeProjectedIndexMap::from([(usize::MAX, 1_i32)]);
    let mut map2 = TypeProjectedIndexMap::from([(usize::MAX, i32::MAX)]);
    let expected = TypeProjectedIndexMap::from([(usize::MAX, i32::MAX)]);
    map1.append(&mut map2);

    assert!(map2.is_empty());
    assert_eq!(map2.len(), 0);
    assert_eq!(map1.len(), 1);
    assert_eq!(map1.as_slice(), expected.as_slice());
}
