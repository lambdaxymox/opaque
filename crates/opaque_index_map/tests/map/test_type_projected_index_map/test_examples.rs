use opaque_index_map::{
    GetDisjointMutError,
    TypeProjectedIndexMap,
};
use opaque_vec::TypeProjectedVec;

use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_empty_len1() {
    let proj_map: TypeProjectedIndexMap<u64, i64> = TypeProjectedIndexMap::new();

    assert_eq!(proj_map.len(), 0);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_empty_is_empty1() {
    let proj_map: TypeProjectedIndexMap<u64, i64> = TypeProjectedIndexMap::new();

    assert!(proj_map.is_empty());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_empty_contains_no_values1() {
    let proj_map: TypeProjectedIndexMap<u64, i64> = TypeProjectedIndexMap::new();
    for key in 0..65536 {
        assert!(!proj_map.contains_key(&key));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_empty_get1() {
    let proj_map: TypeProjectedIndexMap<u64, i64> = TypeProjectedIndexMap::new();
    for key in 0..65536 {
        let result = proj_map.get(&key);

        assert!(result.is_none());
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_empty_len2() {
    let proj_map: TypeProjectedIndexMap<usize, i64> = TypeProjectedIndexMap::new();

    assert_eq!(proj_map.len(), 0);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_empty_is_empty2() {
    let proj_map: TypeProjectedIndexMap<usize, i64> = TypeProjectedIndexMap::new();

    assert!(proj_map.is_empty());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_empty_contains_no_values2() {
    let proj_map: TypeProjectedIndexMap<usize, i64> = TypeProjectedIndexMap::new();
    for key in 0..65536 {
        assert!(!proj_map.contains_key(&key));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_empty_get2() {
    let proj_map: TypeProjectedIndexMap<usize, i64> = TypeProjectedIndexMap::new();
    for key in 0..65536 {
        let result = proj_map.get(&key);

        assert!(result.is_none());
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index_of1() {
    let mut map = TypeProjectedIndexMap::new();
    assert_eq!(map.get_index_of(&"a"), None);

    map.insert("a", 1_i32);
    map.insert("b", 2_i32);
    map.insert("c", 3_i32);

    assert_eq!(map.get_index_of(&"a"), Some(0));
    assert_eq!(map.get_index_of(&"b"), Some(1));
    assert_eq!(map.get_index_of(&"c"), Some(2));
    assert_eq!(map.get_index_of(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index_of2() {
    let map = TypeProjectedIndexMap::from([
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index_of3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32), ("c", 3_i32)]);

    assert_eq!(map.get_index_of(&"a"), Some(0));
    assert_eq!(map.get_index_of(&"c"), Some(2));
    assert_eq!(map.get_index_of(&"b"), Some(1));

    map.swap_remove("b");

    assert_eq!(map.get_index_of(&"a"), Some(0));
    assert_eq!(map.get_index_of(&"c"), Some(1));
    assert_eq!(map.get_index_of(&"b"), None);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index_of6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32)]);

    assert_eq!(map.get_index_of(&"a"), Some(0));
    assert_eq!(map.get_index_of(&"b"), Some(1));
    assert_eq!(map.get_index_of(&"c"), None);

    map.insert("c", 3_i32);

    assert_eq!(map.get_index_of(&"a"), Some(0));
    assert_eq!(map.get_index_of(&"b"), Some(1));
    assert_eq!(map.get_index_of(&"c"), Some(2));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.get(&"a"), None);
    assert_eq!(map.get(&"b"), None);
    assert_eq!(map.get(&"c"), None);
    assert_eq!(map.get(&"d"), None);

    map.insert("a", 1_i32);
    map.insert("b", 2_i32);
    map.insert("c", 3_i32);

    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"b"), Some(&2));
    assert_eq!(map.get(&"c"), Some(&3));
    assert_eq!(map.get(&"d"), None);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32), ("c", 3_i32)]);

    assert_eq!(map.get(&"a"), Some(&1_i32));
    assert_eq!(map.get(&"c"), Some(&3_i32));
    assert_eq!(map.get(&"b"), Some(&2_i32));

    map.swap_remove("b");

    assert_eq!(map.get(&"a"), Some(&1_i32));
    assert_eq!(map.get(&"c"), Some(&3_i32));
    assert_eq!(map.get(&"b"), None);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32)]);

    assert_eq!(map.get(&"a"), Some(&1_i32));
    assert_eq!(map.get(&"b"), Some(&2_i32));
    assert_eq!(map.get(&"c"), None);

    map.insert("c", 3_i32);

    assert_eq!(map.get(&"a"), Some(&1_i32));
    assert_eq!(map.get(&"b"), Some(&2_i32));
    assert_eq!(map.get(&"c"), Some(&3_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_key_value1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.get_key_value(&"a"), None);
    assert_eq!(map.get_key_value(&"b"), None);
    assert_eq!(map.get_key_value(&"c"), None);
    assert_eq!(map.get_key_value(&"d"), None);

    map.insert("a", 1_i32);
    map.insert("b", 2_i32);
    map.insert("c", 3_i32);

    assert_eq!(map.get_key_value(&"a"), Some((&"a", &1_i32)));
    assert_eq!(map.get_key_value(&"b"), Some((&"b", &2_i32)));
    assert_eq!(map.get_key_value(&"c"), Some((&"c", &3_i32)));
    assert_eq!(map.get_key_value(&"d"), None);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_key_value3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32), ("c", 3_i32)]);

    assert_eq!(map.get_key_value(&"a"), Some((&"a", &1_i32)));
    assert_eq!(map.get_key_value(&"c"), Some((&"c", &3_i32)));
    assert_eq!(map.get_key_value(&"b"), Some((&"b", &2_i32)));

    map.swap_remove("b");

    assert_eq!(map.get_key_value(&"a"), Some((&"a", &1_i32)));
    assert_eq!(map.get_key_value(&"c"), Some((&"c", &3_i32)));
    assert_eq!(map.get_key_value(&"b"), None);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_key_value6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32)]);

    assert_eq!(map.get_key_value(&"a"), Some((&"a", &1_i32)));
    assert_eq!(map.get_key_value(&"b"), Some((&"b", &2_i32)));
    assert_eq!(map.get_key_value(&"c"), None);

    map.insert("c", 3_i32);

    assert_eq!(map.get_key_value(&"a"), Some((&"a", &1_i32)));
    assert_eq!(map.get_key_value(&"b"), Some((&"b", &2_i32)));
    assert_eq!(map.get_key_value(&"c"), Some((&"c", &3_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_full1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.get_full(&"a"), None);
    assert_eq!(map.get_full(&"b"), None);
    assert_eq!(map.get_full(&"c"), None);
    assert_eq!(map.get_full(&"d"), None);

    map.insert("a", 1_i32);
    map.insert("b", 2_i32);
    map.insert("c", 3_i32);

    assert_eq!(map.get_full(&"a"), Some((0, &"a", &1_i32)));
    assert_eq!(map.get_full(&"b"), Some((1, &"b", &2_i32)));
    assert_eq!(map.get_full(&"c"), Some((2, &"c", &3_i32)));
    assert_eq!(map.get_full(&"d"), None);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_full3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32), ("c", 3_i32)]);

    assert_eq!(map.get_full(&"a"), Some((0, &"a", &1_i32)));
    assert_eq!(map.get_full(&"c"), Some((2, &"c", &3_i32)));
    assert_eq!(map.get_full(&"b"), Some((1, &"b", &2_i32)));

    map.swap_remove("b");

    assert_eq!(map.get_full(&"a"), Some((0, &"a", &1_i32)));
    assert_eq!(map.get_full(&"c"), Some((1, &"c", &3_i32)));
    assert_eq!(map.get_full(&"b"), None);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_full6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32)]);

    assert_eq!(map.get_full(&"a"), Some((0, &"a", &1_i32)));
    assert_eq!(map.get_full(&"b"), Some((1, &"b", &2_i32)));
    assert_eq!(map.get_full(&"c"), None);

    map.insert("c", 3_i32);

    assert_eq!(map.get_full(&"a"), Some((0, &"a", &1_i32)));
    assert_eq!(map.get_full(&"b"), Some((1, &"b", &2_i32)));
    assert_eq!(map.get_full(&"c"), Some((2, &"c", &3_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.get_index(0), None);
    assert_eq!(map.get_index(1), None);
    assert_eq!(map.get_index(2), None);
    assert_eq!(map.get_index(3), None);

    map.insert("a", 1_i32);
    map.insert("b", 2_i32);
    map.insert("c", 3_i32);

    assert_eq!(map.get_index(0), Some((&"a", &1_i32)));
    assert_eq!(map.get_index(1), Some((&"b", &2_i32)));
    assert_eq!(map.get_index(2), Some((&"c", &3_i32)));
    assert_eq!(map.get_index(3), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index2() {
    let map = TypeProjectedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get_index(0), Some((&0_usize, &1_i32)));
    assert_eq!(map.get_index(1), Some((&1_usize, &2_i32)));
    assert_eq!(map.get_index(2), Some((&2_usize, &3_i32)));
    assert_eq!(map.get_index(3), Some((&3_usize, &4_i32)));
    assert_eq!(map.get_index(4), Some((&4_usize, &5_i32)));
    assert_eq!(map.get_index(5), Some((&5_usize, &6_i32)));
    assert_eq!(map.get_index(6), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32), ("c", 3_i32)]);

    assert_eq!(map.get_index(0), Some((&"a", &1_i32)));
    assert_eq!(map.get_index(2), Some((&"c", &3_i32)));
    assert_eq!(map.get_index(1), Some((&"b", &2_i32)));

    map.swap_remove("b");

    assert_eq!(map.get_index(0), Some((&"a", &1_i32)));
    assert_eq!(map.get_index(2), None);
    assert_eq!(map.get_index(1), Some((&"c", &3_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index4() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_index(10), Some((&'k', &())));

    map.insert_before(10, '*', ());
    assert_eq!(map.get_index(10), Some((&'*', &())));

    map.insert_before(10, 'a', ());
    assert_eq!(map.get_index(10), Some((&'*', &())));
    assert_eq!(map.get_index(9), Some((&'a', &())));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index5() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_index(10), Some((&'k', &())));

    map.shift_insert(10, '*', ());
    assert_eq!(map.get_index(10), Some((&'*', &())));

    map.shift_insert(10, 'a', ());
    assert_eq!(map.get_index(0),  Some((&'b', &())));
    assert_eq!(map.get_index(10), Some((&'a', &())));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32)]);

    assert_eq!(map.get_index(0), Some((&"a", &1_i32)));
    assert_eq!(map.get_index(1), Some((&"b", &2_i32)));
    assert_eq!(map.get_index(2), None);

    map.insert("c", 3_i32);

    assert_eq!(map.get_index(0), Some((&"a", &1_i32)));
    assert_eq!(map.get_index(1), Some((&"b", &2_i32)));
    assert_eq!(map.get_index(2), Some((&"c", &3_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_mut1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.get_mut(&"a"), None);
    assert_eq!(map.get_mut(&"b"), None);
    assert_eq!(map.get_mut(&"c"), None);
    assert_eq!(map.get_mut(&"d"), None);

    map.insert("a", 1_i32);
    map.insert("b", 2_i32);
    map.insert("c", 3_i32);

    assert_eq!(map.get_mut(&"a"), Some(&mut 1));
    assert_eq!(map.get_mut(&"b"), Some(&mut 2));
    assert_eq!(map.get_mut(&"c"), Some(&mut 3));
    assert_eq!(map.get_mut(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_mut2() {
    let mut map = TypeProjectedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get_mut(&0_usize), Some(&mut 1_i32));
    assert_eq!(map.get_mut(&1_usize), Some(&mut 2_i32));
    assert_eq!(map.get_mut(&2_usize), Some(&mut 3_i32));
    assert_eq!(map.get_mut(&3_usize), Some(&mut 4_i32));
    assert_eq!(map.get_mut(&4_usize), Some(&mut 5_i32));
    assert_eq!(map.get_mut(&5_usize), Some(&mut 6_i32));
    assert_eq!(map.get_mut(&6_usize), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_mut3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32), ("c", 3_i32)]);

    assert_eq!(map.get_mut(&"a"), Some(&mut 1_i32));
    assert_eq!(map.get_mut(&"c"), Some(&mut 3_i32));
    assert_eq!(map.get_mut(&"b"), Some(&mut 2_i32));

    map.swap_remove("b");

    assert_eq!(map.get_mut(&"a"), Some(&mut 1_i32));
    assert_eq!(map.get_mut(&"c"), Some(&mut 3_i32));
    assert_eq!(map.get_mut(&"b"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_mut4() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_mut(&'*'), None);

    map.insert_before(10, '*', ());
    assert_eq!(map.get_mut(&'*'), Some(&mut ()));

    map.insert_before(10, 'a', ());
    assert_eq!(map.get_mut(&'a'), Some(&mut ()));
    assert_eq!(map.get_mut(&'*'), Some(&mut ()));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_mut5() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_mut(&'*'), None);

    map.shift_insert(10, '*', ());
    assert_eq!(map.get_mut(&'*'), Some(&mut ()));

    map.shift_insert(10, 'a', ());
    assert_eq!(map.get_mut(&'a'), Some(&mut ()));
    assert_eq!(map.get_mut(&'*'), Some(&mut ()));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_mut6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32)]);

    assert_eq!(map.get_mut(&"a"), Some(&mut 1_i32));
    assert_eq!(map.get_mut(&"b"), Some(&mut 2_i32));
    assert_eq!(map.get_mut(&"c"), None);

    map.insert("c", 3_i32);

    assert_eq!(map.get_mut(&"a"), Some(&mut 1_i32));
    assert_eq!(map.get_mut(&"b"), Some(&mut 2_i32));
    assert_eq!(map.get_mut(&"c"), Some(&mut 3_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_key_value_mut1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.get_key_value_mut(&"a"), None);
    assert_eq!(map.get_key_value_mut(&"b"), None);
    assert_eq!(map.get_key_value_mut(&"c"), None);
    assert_eq!(map.get_key_value_mut(&"d"), None);

    map.insert("a", 1_i32);
    map.insert("b", 2_i32);
    map.insert("c", 3_i32);

    assert_eq!(map.get_key_value_mut(&"a"), Some((&"a", &mut 1_i32)));
    assert_eq!(map.get_key_value_mut(&"b"), Some((&"b", &mut 2_i32)));
    assert_eq!(map.get_key_value_mut(&"c"), Some((&"c", &mut 3_i32)));
    assert_eq!(map.get_key_value_mut(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_key_value_mut2() {
    let mut map = TypeProjectedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get_key_value_mut(&0_usize), Some((&0_usize, &mut 1_i32)));
    assert_eq!(map.get_key_value_mut(&1_usize), Some((&1_usize, &mut 2_i32)));
    assert_eq!(map.get_key_value_mut(&2_usize), Some((&2_usize, &mut 3_i32)));
    assert_eq!(map.get_key_value_mut(&3_usize), Some((&3_usize, &mut 4_i32)));
    assert_eq!(map.get_key_value_mut(&4_usize), Some((&4_usize, &mut 5_i32)));
    assert_eq!(map.get_key_value_mut(&5_usize), Some((&5_usize, &mut 6_i32)));
    assert_eq!(map.get_key_value_mut(&6_usize), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_key_value_mut3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32), ("c", 3_i32)]);

    assert_eq!(map.get_key_value_mut(&"a"), Some((&"a", &mut 1_i32)));
    assert_eq!(map.get_key_value_mut(&"c"), Some((&"c", &mut 3_i32)));
    assert_eq!(map.get_key_value_mut(&"b"), Some((&"b", &mut 2_i32)));

    map.swap_remove("b");

    assert_eq!(map.get_key_value_mut(&"a"), Some((&"a", &mut 1_i32)));
    assert_eq!(map.get_key_value_mut(&"c"), Some((&"c", &mut 3_i32)));
    assert_eq!(map.get_key_value_mut(&"b"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_key_value_mut4() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_key_value_mut(&'*'), None);

    map.insert_before(10, '*', ());
    assert_eq!(map.get_key_value_mut(&'*'), Some((&'*', &mut ())));

    map.insert_before(10, 'a', ());
    assert_eq!(map.get_key_value_mut(&'a'), Some((&'a', &mut ())));
    assert_eq!(map.get_key_value_mut(&'*'), Some((&'*', &mut ())));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_key_value_mut5() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_key_value_mut(&'*'), None);

    map.shift_insert(10, '*', ());
    assert_eq!(map.get_key_value_mut(&'*'), Some((&'*', &mut ())));

    map.shift_insert(10, 'a', ());
    assert_eq!(map.get_key_value_mut(&'a'), Some((&'a', &mut ())));
    assert_eq!(map.get_key_value_mut(&'*'), Some((&'*', &mut ())));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_key_value_mut6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32)]);

    assert_eq!(map.get_key_value_mut(&"a"), Some((&"a", &mut 1_i32)));
    assert_eq!(map.get_key_value_mut(&"b"), Some((&"b", &mut 2_i32)));
    assert_eq!(map.get_key_value_mut(&"c"), None);

    map.insert("c", 3_i32);

    assert_eq!(map.get_key_value_mut(&"a"), Some((&"a", &mut 1_i32)));
    assert_eq!(map.get_key_value_mut(&"b"), Some((&"b", &mut 2_i32)));
    assert_eq!(map.get_key_value_mut(&"c"), Some((&"c", &mut 3_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_full_mut1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.get_full_mut(&"a"), None);
    assert_eq!(map.get_full_mut(&"b"), None);
    assert_eq!(map.get_full_mut(&"c"), None);
    assert_eq!(map.get_full_mut(&"d"), None);

    map.insert("a", 1_i32);
    map.insert("b", 2_i32);
    map.insert("c", 3_i32);

    assert_eq!(map.get_full_mut(&"a"), Some((0, &"a", &mut 1_i32)));
    assert_eq!(map.get_full_mut(&"b"), Some((1, &"b", &mut 2_i32)));
    assert_eq!(map.get_full_mut(&"c"), Some((2, &"c", &mut 3_i32)));
    assert_eq!(map.get_full_mut(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_full_mut2() {
    let mut map = TypeProjectedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get_full_mut(&0_usize), Some((0, &0_usize, &mut 1_i32)));
    assert_eq!(map.get_full_mut(&1_usize), Some((1, &1_usize, &mut 2_i32)));
    assert_eq!(map.get_full_mut(&2_usize), Some((2, &2_usize, &mut 3_i32)));
    assert_eq!(map.get_full_mut(&3_usize), Some((3, &3_usize, &mut 4_i32)));
    assert_eq!(map.get_full_mut(&4_usize), Some((4, &4_usize, &mut 5_i32)));
    assert_eq!(map.get_full_mut(&5_usize), Some((5, &5_usize, &mut 6_i32)));
    assert_eq!(map.get_full_mut(&6_usize), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_full_mut3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32), ("c", 3_i32)]);

    assert_eq!(map.get_full_mut(&"a"), Some((0, &"a", &mut 1_i32)));
    assert_eq!(map.get_full_mut(&"c"), Some((2, &"c", &mut 3_i32)));
    assert_eq!(map.get_full_mut(&"b"), Some((1, &"b", &mut 2_i32)));

    map.swap_remove("b");

    assert_eq!(map.get_full_mut(&"a"), Some((0, &"a", &mut 1_i32)));
    assert_eq!(map.get_full_mut(&"c"), Some((1, &"c", &mut 3_i32)));
    assert_eq!(map.get_full_mut(&"b"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_full_mut4() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_full_mut(&'*'), None);

    map.insert_before(10, '*', ());
    assert_eq!(map.get_full_mut(&'*'), Some((10, &'*', &mut ())));

    map.insert_before(10, 'a', ());
    assert_eq!(map.get_full_mut(&'a'), Some((9, &'a', &mut ())));
    assert_eq!(map.get_full_mut(&'*'), Some((10, &'*', &mut ())));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_full_mut5() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_full_mut(&'*'), None);

    map.shift_insert(10, '*', ());
    assert_eq!(map.get_full_mut(&'*'), Some((10, &'*', &mut ())));

    map.shift_insert(10, 'a', ());
    assert_eq!(map.get_full_mut(&'a'), Some((10, &'a', &mut ())));
    assert_eq!(map.get_full_mut(&'*'), Some((9, &'*', &mut ())));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_full_mut6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32)]);

    assert_eq!(map.get_full_mut(&"a"), Some((0, &"a", &mut 1_i32)));
    assert_eq!(map.get_full_mut(&"b"), Some((1, &"b", &mut 2_i32)));
    assert_eq!(map.get_full_mut(&"c"), None);

    map.insert("c", 3_i32);

    assert_eq!(map.get_full_mut(&"a"), Some((0, &"a", &mut 1_i32)));
    assert_eq!(map.get_full_mut(&"b"), Some((1, &"b", &mut 2_i32)));
    assert_eq!(map.get_full_mut(&"c"), Some((2, &"c", &mut 3_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index_mut1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.get_index_mut(0), None);
    assert_eq!(map.get_index_mut(1), None);
    assert_eq!(map.get_index_mut(2), None);
    assert_eq!(map.get_index_mut(3), None);

    map.insert("a", 1_i32);
    map.insert("b", 2_i32);
    map.insert("c", 3_i32);

    assert_eq!(map.get_index_mut(0), Some((&"a", &mut 1_i32)));
    assert_eq!(map.get_index_mut(1), Some((&"b", &mut 2_i32)));
    assert_eq!(map.get_index_mut(2), Some((&"c", &mut 3_i32)));
    assert_eq!(map.get_index_mut(3), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index_mut_mut2() {
    let mut map = TypeProjectedIndexMap::from([
        (0_usize, 1_i32),
        (1_usize, 2_i32),
        (2_usize, 3_i32),
        (3_usize, 4_i32),
        (4_usize, 5_i32),
        (5_usize, 6_i32),
    ]);

    assert_eq!(map.get_index_mut(0), Some((&0_usize, &mut 1_i32)));
    assert_eq!(map.get_index_mut(1), Some((&1_usize, &mut 2_i32)));
    assert_eq!(map.get_index_mut(2), Some((&2_usize, &mut 3_i32)));
    assert_eq!(map.get_index_mut(3), Some((&3_usize, &mut 4_i32)));
    assert_eq!(map.get_index_mut(4), Some((&4_usize, &mut 5_i32)));
    assert_eq!(map.get_index_mut(5), Some((&5_usize, &mut 6_i32)));
    assert_eq!(map.get_index_mut(6), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index_mut3() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32), ("c", 3_i32)]);

    assert_eq!(map.get_index_mut(0), Some((&"a", &mut 1_i32)));
    assert_eq!(map.get_index_mut(2), Some((&"c", &mut 3_i32)));
    assert_eq!(map.get_index_mut(1), Some((&"b", &mut 2_i32)));

    map.swap_remove("b");

    assert_eq!(map.get_index_mut(0), Some((&"a", &mut 1_i32)));
    assert_eq!(map.get_index_mut(2), None);
    assert_eq!(map.get_index_mut(1), Some((&"c", &mut 3_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index_mut4() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_index_mut(10), Some((&'k', &mut ())));

    map.insert_before(10, '*', ());
    assert_eq!(map.get_index_mut(10), Some((&'*', &mut ())));

    map.insert_before(10, 'a', ());
    assert_eq!(map.get_index_mut(10), Some((&'*', &mut ())));
    assert_eq!(map.get_index_mut(9), Some((&'a', &mut ())));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index_mut5() {
    let mut map: TypeProjectedIndexMap<char, ()> = ('a'..='z').map(|c| (c, ())).collect();
    assert_eq!(map.get_index_mut(10), Some((&'k', &mut ())));

    map.shift_insert(10, '*', ());
    assert_eq!(map.get_index_mut(10), Some((&'*', &mut ())));

    map.shift_insert(10, 'a', ());
    assert_eq!(map.get_index_mut(0),  Some((&'b', &mut ())));
    assert_eq!(map.get_index_mut(10), Some((&'a', &mut ())));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_index_mut6() {
    let mut map = TypeProjectedIndexMap::from([("a", 1_i32), ("b", 2_i32)]);

    assert_eq!(map.get_index_mut(0), Some((&"a", &mut 1_i32)));
    assert_eq!(map.get_index_mut(1), Some((&"b", &mut 2_i32)));
    assert_eq!(map.get_index_mut(2), None);

    map.insert("c", 3_i32);

    assert_eq!(map.get_index_mut(0), Some((&"a", &mut 1_i32)));
    assert_eq!(map.get_index_mut(1), Some((&"b", &mut 2_i32)));
    assert_eq!(map.get_index_mut(2), Some((&"c", &mut 3_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_disjoint_mut1() {
    let mut map: TypeProjectedIndexMap<&str, i32> = TypeProjectedIndexMap::new();
    let expected = [None, None, None, None, None, None];
    let result = map.get_disjoint_mut([&"1", &"2", &"3", &"4", &"5", &"6"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_disjoint_mut2() {
    let mut map = TypeProjectedIndexMap::from([
        ("1", 10_i32),
        ("2", 20_i32),
        ("3", 30_i32),
        ("4", 40_i32),
        ("5", 50_i32),
        ("6", 60_i32),
    ]);
    let expected = [
        Some(&mut 10_i32),
        Some(&mut 20_i32),
        Some(&mut 30_i32),
        Some(&mut 40_i32),
        Some(&mut 50_i32),
        Some(&mut 60_i32),
    ];
    let result = map.get_disjoint_mut([&"1", &"2", &"3", &"4", &"5", &"6"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_disjoint_mut3() {
    let mut map = TypeProjectedIndexMap::from([
        ("1", 10_i32),
        ("2", 20_i32),
        ("3", 30_i32),
        ("4", 40_i32),
        ("5", 50_i32),
        ("6", 60_i32),
    ]);
    let expected = [
        Some(&mut 10_i32),
        Some(&mut 20_i32),
        Some(&mut 30_i32),
    ];
    let result = map.get_disjoint_mut([&"1", &"2", &"3"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_disjoint_mut4() {
    let mut map = TypeProjectedIndexMap::from([
        ("1", 10_i32),
        ("2", 20_i32),
        ("3", 30_i32),
        ("4", 40_i32),
        ("5", 50_i32),
        ("6", 60_i32),
    ]);
    let expected = [
        Some(&mut 40_i32),
        Some(&mut 50_i32),
        Some(&mut 60_i32),
    ];
    let result = map.get_disjoint_mut([&"4", &"5", &"6"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_disjoint_mut5() {
    let mut map = TypeProjectedIndexMap::from([
        ("1", 10_i32),
        ("2", 20_i32),
        ("3", 30_i32),
        ("4", 40_i32),
        ("5", 50_i32),
        ("6", 60_i32),
    ]);
    let expected = [
        Some(&mut 10_i32),
        Some(&mut 30_i32),
        Some(&mut 50_i32),
    ];
    let result = map.get_disjoint_mut([&"1", &"3", &"5"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_disjoint_mut6() {
    let mut map = TypeProjectedIndexMap::from([
        ("1", 10_i32),
        ("2", 20_i32),
        ("3", 30_i32),
        ("4", 40_i32),
        ("5", 50_i32),
        ("6", 60_i32),
    ]);
    let expected = [
        Some(&mut 20_i32),
        Some(&mut 40_i32),
        Some(&mut 60_i32),
    ];
    let result = map.get_disjoint_mut([&"2", &"4", &"6"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_disjoint_mut_partial_success1() {
    let mut map = TypeProjectedIndexMap::from([
        ("1", 10_i32),
        ("2", 20_i32),
        ("3", 30_i32),
        ("4", 40_i32),
        ("5", 50_i32),
        ("6", 60_i32),
    ]);
    let expected = [
        Some(&mut 10_i32),
        None,
        Some(&mut 30_i32),
        None,
        Some(&mut 50_i32),
        None,
    ];
    let result = map.get_disjoint_mut([&"1", &"20", &"3", &"40", &"5", &"60"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_get_disjoint_mut_partial_success2() {
    let mut map = TypeProjectedIndexMap::from([
        ("1", 10_i32),
        ("2", 20_i32),
        ("3", 30_i32),
        ("4", 40_i32),
        ("5", 50_i32),
        ("6", 60_i32),
    ]);
    let expected = [
        Some(&mut 10_i32),
        Some(&mut 20_i32),
        Some(&mut 30_i32),
        None,
        Some(&mut 40_i32),
        Some(&mut 50_i32),
        Some(&mut 60_i32),
        None,
    ];
    let result = map.get_disjoint_mut(["1", "2", "3", "200", "4", "5", "6", "100"]);

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
#[should_panic]
fn test_type_projected_index_map_get_disjoint_mut_repeat_indices1() {
    let mut map = TypeProjectedIndexMap::from([
        ("1", 10_i32),
        ("2", 20_i32),
        ("3", 30_i32),
        ("4", 40_i32),
        ("5", 50_i32),
        ("6", 60_i32),
    ]);
    let _ = map.get_disjoint_mut(["1", "2", "2", "4", "5", "6"]);

    assert!(true);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_values2() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();
    let expected = TypeProjectedVec::new();
    let result: TypeProjectedVec<i32> = map.values().cloned().collect();

    assert_eq!(result, expected);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_iter1() {
    let map = TypeProjectedIndexMap::from([
        (89_usize, 92_i32),
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);

    for (key, _value) in map.iter() {
        assert!(map.contains_key(key));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_iter2() {
    let map = TypeProjectedIndexMap::from([
        (89_usize, 92_i32),
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);

    for (key, value) in map.iter() {
        let expected = Some(value);
        let result = map.get(key);

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_iter3() {
    let map = TypeProjectedIndexMap::from([
        (89_usize, 92_i32),
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);
    let expected = TypeProjectedVec::from([
        (89_usize, 92_i32),
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);
    let result: TypeProjectedVec<(usize, i32)> = map
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_iter4() {
    let map = TypeProjectedIndexMap::from([
        (89_usize, 92_i32),
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);
    let mut iter = map.iter();

    assert_eq!(iter.next(), Some((&89_usize, &92_i32)));
    assert_eq!(iter.next(), Some((&40_usize, &59_i32)));
    assert_eq!(iter.next(), Some((&80_usize, &87_i32)));
    assert_eq!(iter.next(), Some((&39_usize, &5_i32)));
    assert_eq!(iter.next(), Some((&62_usize, &11_i32)));
    assert_eq!(iter.next(), Some((&81_usize, &36_i32)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_iter5() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();
    let mut iter = map.iter();

    for _ in 0..65536 {
        assert!(iter.next().is_none());
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_into_iter1() {
    let map = TypeProjectedIndexMap::from([
        (89_usize, 92_i32),
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);

    for (key, _value) in map.clone().into_iter() {
        assert!(map.contains_key(&key));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_into_iter2() {
    let map = TypeProjectedIndexMap::from([
        (89_usize, 92_i32),
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);

    for (key, value) in map.clone().into_iter() {
        let expected = Some(&value);
        let result = map.get(&key);

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_into_iter3() {
    let map = TypeProjectedIndexMap::from([
        (89_usize, 92_i32),
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);
    let expected = TypeProjectedVec::from([
        (89_usize, 92_i32),
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);
    let result: TypeProjectedVec<(usize, i32)> = map
        .into_iter()
        .collect();

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_into_iter4() {
    let map = TypeProjectedIndexMap::from([
        (89_usize, 92_i32),
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);
    let mut iter = map.into_iter();

    assert_eq!(iter.next(), Some((89_usize, 92_i32)));
    assert_eq!(iter.next(), Some((40_usize, 59_i32)));
    assert_eq!(iter.next(), Some((80_usize, 87_i32)));
    assert_eq!(iter.next(), Some((39_usize, 5_i32)));
    assert_eq!(iter.next(), Some((62_usize, 11_i32)));
    assert_eq!(iter.next(), Some((81_usize, 36_i32)));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_into_iter5() {
    let map = TypeProjectedIndexMap::from([
        (89_usize, 92_i32),
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);
    let mut iter = map.into_iter();

    assert_eq!(iter.len(), 6);
    assert_eq!(iter.as_slice(), &[
        (89_usize, 92_i32),
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);

    let _ = iter.next();
    assert_eq!(iter.len(), 5);
    assert_eq!(iter.as_slice(), &[
        (40_usize, 59_i32),
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);

    let _ = iter.next();
    assert_eq!(iter.len(), 4);
    assert_eq!(iter.as_slice(), &[
        (80_usize, 87_i32),
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);

    let _ = iter.next();
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.as_slice(), &[
        (39_usize, 5_i32),
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);

    let _ = iter.next();
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.as_slice(), &[
        (62_usize, 11_i32),
        (81_usize, 36_i32),
    ]);

    let _ = iter.next();
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.as_slice(), &[
        (81_usize, 36_i32),
    ]);

    let _ = iter.next();
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.as_slice(), &[]);

    let _ = iter.next();
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.as_slice(), &[]);

    let _ = iter.next();
    assert_eq!(iter.len(), 0);
    assert_eq!(iter.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_into_iter6() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();
    let mut iter = map.into_iter();

    for _ in 0..65536 {
        assert!(iter.next().is_none());
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_into_iter7() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();
    let mut iter = map.into_iter();

    for _ in 0..65536 {
        let _ = iter.next().is_none();
        assert_eq!(iter.len(), 0);
        assert!(iter.as_slice().is_empty());
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_clear1() {
    let mut map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    map.clear();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
#[test]
#[should_panic]
fn test_type_projected_index_map_split_off_out_of_bounds1() {
    let mut map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();
    let _ = map.split_off(map.len() + 1);

    assert!(true);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove(&5_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
    ]);

    let _ = map.swap_remove(&6_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
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
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
    ]);

    let _ = map.swap_remove(&2_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
    ]);

    let _ = map.swap_remove(&1_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove_entry(&5_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
    ]);

    let _ = map.swap_remove_entry(&6_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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

#[rustfmt::skip]
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
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove_full(&5_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
    ]);

    let _ = map.swap_remove_full(&6_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
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
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
    ]);

    let _ = map.swap_remove_full(&2_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
    ]);

    let _ = map.swap_remove_full(&1_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_swap_remove_index1() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove_index(0), Some((1_usize, 20_i32)));
    assert_eq!(map.swap_remove_index(1), Some((2_usize, 2043_i32)));
    assert_eq!(map.swap_remove_index(2), Some((3_usize, 4904_i32)));
    assert_eq!(map.swap_remove_index(2), Some((4_usize, 20994_i32)));
    assert_eq!(map.swap_remove_index(1), Some((5_usize, 302_i32)));
    assert_eq!(map.swap_remove_index(0), Some((6_usize, 5_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_swap_remove_index2() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.swap_remove_index(0);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove_index(1);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = map.swap_remove_index(2);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = map.swap_remove_index(2);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove_index(1);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (6_usize, 5_i32),
    ]);

    let _ = map.swap_remove_index(0);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_swap_remove_index3() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.swap_remove_index(5);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    let _ = map.swap_remove_index(4);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = map.swap_remove_index(3);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);

    let _ = map.swap_remove_index(2);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
    ]);

    let _ = map.swap_remove_index(1);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1_usize, 20_i32),
    ]);

    let _ = map.swap_remove_index(0);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_swap_remove_index4() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(map.swap_remove_index(5), Some((6_usize, 5_i32)));
    assert_eq!(map.swap_remove_index(4), Some((5_usize, 302_i32)));
    assert_eq!(map.swap_remove_index(3), Some((4_usize, 20994_i32)));
    assert_eq!(map.swap_remove_index(2), Some((3_usize, 4904_i32)));
    assert_eq!(map.swap_remove_index(1), Some((2_usize, 2043_i32)));
    assert_eq!(map.swap_remove_index(0), Some((1_usize, 20_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_swap_remove_index_out_of_bounds1() {
    let mut map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();

    for i in 0..65536 {
        assert_eq!(map.swap_remove_index(i), None);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_swap_remove_index_out_of_bounds2() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    for i in map.len()..65536 {
        assert_eq!(map.swap_remove_index(i), None);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove1() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove(&1655_usize), Some(2427_i32));
    assert_eq!(map.shift_remove(&1992_usize), Some(2910_i32));
    assert_eq!(map.shift_remove(&783_usize),  Some(603_i32));
    assert_eq!(map.shift_remove(&376_usize),  Some(834_i32));
    assert_eq!(map.shift_remove(&199_usize),  Some(1881_i32));
    assert_eq!(map.shift_remove(&1098_usize), Some(1466_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove2() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove(&1655_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove(&1992_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove(&783_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove(&376_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove(&199_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove(&1098_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove3() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove(&1098_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
    ]);

    let _ = map.shift_remove(&199_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
    ]);

    let _ = map.shift_remove(&376_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
    ]);

    let _ = map.shift_remove(&783_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
    ]);

    let _ = map.shift_remove(&1992_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
    ]);

    let _ = map.shift_remove(&1655_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove4() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove(&1098_usize), Some(1466_i32));
    assert_eq!(map.shift_remove(&199_usize),  Some(1881_i32));
    assert_eq!(map.shift_remove(&376_usize),  Some(834_i32));
    assert_eq!(map.shift_remove(&783_usize),  Some(603_i32));
    assert_eq!(map.shift_remove(&1992_usize), Some(2910_i32));
    assert_eq!(map.shift_remove(&1655_usize), Some(2427_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_entry1() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_entry(&1655_usize), Some((1655_usize, 2427_i32)));
    assert_eq!(map.shift_remove_entry(&1992_usize), Some((1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_entry(&783_usize),  Some((783_usize, 603_i32)));
    assert_eq!(map.shift_remove_entry(&376_usize),  Some((376_usize, 834_i32)));
    assert_eq!(map.shift_remove_entry(&199_usize),  Some((199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_entry(&1098_usize), Some((1098_usize, 1466_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_entry2() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove_entry(&1655_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_entry(&1992_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_entry(&783_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_entry(&376_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_entry(&199_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_entry(&1098_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_entry3() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove_entry(&1098_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
    ]);

    let _ = map.shift_remove_entry(&199_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
    ]);

    let _ = map.shift_remove_entry(&376_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
    ]);

    let _ = map.shift_remove_entry(&783_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
    ]);

    let _ = map.shift_remove_entry(&1992_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
    ]);

    let _ = map.shift_remove_entry(&1655_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_entry4() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_entry(&1098_usize), Some((1098_usize, 1466_i32)));
    assert_eq!(map.shift_remove_entry(&199_usize),  Some((199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_entry(&376_usize),  Some((376_usize, 834_i32)));
    assert_eq!(map.shift_remove_entry(&783_usize),  Some((783_usize, 603_i32)));
    assert_eq!(map.shift_remove_entry(&1992_usize), Some((1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_entry(&1655_usize), Some((1655_usize, 2427_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_full1() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_full(&1655_usize), Some((0, 1655_usize, 2427_i32)));
    assert_eq!(map.shift_remove_full(&1992_usize), Some((0, 1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_full(&783_usize),  Some((0, 783_usize, 603_i32)));
    assert_eq!(map.shift_remove_full(&376_usize),  Some((0, 376_usize, 834_i32)));
    assert_eq!(map.shift_remove_full(&199_usize),  Some((0, 199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_full(&1098_usize), Some((0, 1098_usize, 1466_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_full2() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove_full(&1655_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_full(&1992_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_full(&783_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_full(&376_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_full(&199_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_full(&1098_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_full3() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove_full(&1098_usize);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
    ]);

    let _ = map.shift_remove_full(&199_usize);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
    ]);

    let _ = map.shift_remove_full(&376_usize);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
    ]);

    let _ = map.shift_remove_full(&783_usize);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
    ]);

    let _ = map.shift_remove_full(&1992_usize);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
    ]);

    let _ = map.shift_remove_full(&1655_usize);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_full4() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_full(&1098_usize), Some((5, 1098_usize, 1466_i32)));
    assert_eq!(map.shift_remove_full(&199_usize),  Some((4, 199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_full(&376_usize),  Some((3, 376_usize, 834_i32)));
    assert_eq!(map.shift_remove_full(&783_usize),  Some((2, 783_usize, 603_i32)));
    assert_eq!(map.shift_remove_full(&1992_usize), Some((1, 1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_full(&1655_usize), Some((0, 1655_usize, 2427_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_index1() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_index(0), Some((1655_usize, 2427_i32)));
    assert_eq!(map.shift_remove_index(0), Some((1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_index(0), Some((783_usize, 603_i32)));
    assert_eq!(map.shift_remove_index(0), Some((376_usize, 834_i32)));
    assert_eq!(map.shift_remove_index(0), Some((199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_index(0), Some((1098_usize, 1466_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_index2() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove_index(0);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_index(0);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_index(0);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_index(0);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_index(0);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1098_usize, 1466_i32),
    ]);

    let _ = map.shift_remove_index(0);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_index3() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.len(), 6);

    let _ = map.shift_remove_index(5);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
    ]);

    let _ = map.shift_remove_index(4);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
    ]);

    let _ = map.shift_remove_index(3);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
    ]);

    let _ = map.shift_remove_index(2);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
    ]);

    let _ = map.shift_remove_index(1);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1655_usize, 2427_i32),
    ]);

    let _ = map.shift_remove_index(0);
    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_index4() {
    let mut map = TypeProjectedIndexMap::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(map.shift_remove_index(5), Some((1098_usize, 1466_i32)));
    assert_eq!(map.shift_remove_index(4), Some((199_usize, 1881_i32)));
    assert_eq!(map.shift_remove_index(3), Some((376_usize, 834_i32)));
    assert_eq!(map.shift_remove_index(2), Some((783_usize, 603_i32)));
    assert_eq!(map.shift_remove_index(1), Some((1992_usize, 2910_i32)));
    assert_eq!(map.shift_remove_index(0), Some((1655_usize, 2427_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_index_out_of_bounds1() {
    let mut map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();

    for i in 0..65536 {
        assert_eq!(map.shift_remove_index(i), None);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_remove_index_out_of_bounds2() {
    let mut map = TypeProjectedIndexMap::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    for i in map.len()..65536 {
        assert_eq!(map.shift_remove_index(i), None);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_insert1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.insert(1803_usize, 1778_i32), None);
    assert_eq!(map.insert(1057_usize, 2437_i32), None);
    assert_eq!(map.insert(1924_usize, 185_i32),  None);
    assert_eq!(map.insert(302_usize, 2457_i32),  None);
    assert_eq!(map.insert(949_usize, 2176_i32),  None);
    assert_eq!(map.insert(2968_usize, 1398_i32), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_insert2() {
    let mut map = TypeProjectedIndexMap::new();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    let _ = map.insert(1803_usize, 1778_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
    ]);

    let _ = map.insert(1057_usize, 2437_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
    ]);

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
        (302_usize,  2457_i32),
    ]);

    let _ = map.insert(949_usize, 2176_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize,  2457_i32),
        (949_usize,  2176_i32),
    ]);

    let _ = map.insert(2968_usize, 1398_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
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
fn test_type_projected_index_map_insert_full1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.insert_full(1803_usize, 1778_i32), (0, None));
    assert_eq!(map.insert_full(1057_usize, 2437_i32), (1, None));
    assert_eq!(map.insert_full(1924_usize, 185_i32),  (2, None));
    assert_eq!(map.insert_full(302_usize, 2457_i32),  (3, None));
    assert_eq!(map.insert_full(949_usize, 2176_i32),  (4, None));
    assert_eq!(map.insert_full(2968_usize, 1398_i32), (5, None));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_insert_full2() {
    let mut map = TypeProjectedIndexMap::new();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    let _ = map.insert_full(1803_usize, 1778_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
    ]);

    let _ = map.insert_full(1057_usize, 2437_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
    ]);

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
        (302_usize,  2457_i32),
    ]);

    let _ = map.insert_full(949_usize, 2176_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize,  2457_i32),
        (949_usize,  2176_i32),
    ]);

    let _ = map.insert_full(2968_usize, 1398_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
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
fn test_type_projected_index_map_insert_before1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.insert_before(0, 370_usize, 2339_i32),  (0, None));
    assert_eq!(map.insert_before(0, 1977_usize, 2387_i32), (0, None));
    assert_eq!(map.insert_before(0, 1244_usize, 2741_i32), (0, None));
    assert_eq!(map.insert_before(0, 1733_usize, 1838_i32), (0, None));
    assert_eq!(map.insert_before(0, 289_usize, 464_i32),   (0, None));
    assert_eq!(map.insert_before(0, 2712_usize, 509_i32),  (0, None));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_insert_before2() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.len(), 0);

    let _ = map.insert_before(0, 370_usize, 2339_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (370_usize, 2339_i32),
    ]);

    let _ = map.insert_before(0, 1977_usize, 2387_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    let _ = map.insert_before(0, 1244_usize, 2741_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    let _ = map.insert_before(0, 1733_usize, 1838_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    let _ = map.insert_before(0, 289_usize, 464_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    let _ = map.insert_before(0, 2712_usize, 509_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
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
fn test_type_projected_index_map_insert_before3() {
    let mut map = TypeProjectedIndexMap::from([
        (2712_usize, 509_i32),
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    assert_eq!(map.len(), 6);

    let result = map.insert_before(4, 289_usize, i32::MAX);
    assert_eq!(result, (3, Some(464_i32)));
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
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
fn test_type_projected_index_map_insert_before4() {
    let mut map = TypeProjectedIndexMap::from([
        (2712_usize, 509_i32),
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    assert_eq!(map.len(), 6);

    let result = map.insert_before(1, 370_usize, i32::MAX);
    assert_eq!(result, (1, Some(2339_i32)));
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
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
fn test_type_projected_index_map_insert_before5() {
    let mut map = TypeProjectedIndexMap::from([
        (2712_usize, 509_i32),
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    assert_eq!(map.len(), 6);

    let result = map.insert_before(3, 1244_usize, i32::MAX);
    assert_eq!(result, (3, Some(2741_i32)));
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
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
fn test_type_projected_index_map_insert_before6() {
    let mut map = TypeProjectedIndexMap::from([
        (2712_usize, 509_i32),
        (289_usize,  464_i32),
        (1733_usize, 1838_i32),
        (1244_usize, 2741_i32),
        (1977_usize, 2387_i32),
        (370_usize,  2339_i32),
    ]);

    assert_eq!(map.len(), 6);

    let result = map.insert_before(5, usize::MAX, i32::MAX);
    assert_eq!(result, (5, None));
    assert_eq!(map.len(), 7);
    assert_eq!(map.as_slice(), &[
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
fn test_type_projected_index_map_shift_insert1() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.shift_insert(0, 1809_usize, 2381_i32), None);
    assert_eq!(map.shift_insert(0, 603_usize, 2834_i32),  None);
    assert_eq!(map.shift_insert(0, 2564_usize, 621_i32),  None);
    assert_eq!(map.shift_insert(0, 360_usize, 1352_i32),  None);
    assert_eq!(map.shift_insert(0, 57_usize, 2657_i32),   None);
    assert_eq!(map.shift_insert(0, 477_usize, 2084_i32),  None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_insert2() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.len(), 0);
    assert!(map.is_empty());

    let _ = map.shift_insert(0, 1809_usize, 2381_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert(0, 603_usize, 2834_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert(0, 2564_usize, 621_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert(0, 360_usize, 1352_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert(0, 57_usize, 2657_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (57_usize,   2657_i32),
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);

    let _ = map.shift_insert(0, 477_usize, 2084_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
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
fn test_type_projected_index_map_shift_insert3() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.shift_insert(0, 477_usize, 2084_i32),  None);
    assert_eq!(map.shift_insert(1, 57_usize, 2657_i32),   None);
    assert_eq!(map.shift_insert(2, 360_usize, 1352_i32),  None);
    assert_eq!(map.shift_insert(3, 2564_usize, 621_i32),  None);
    assert_eq!(map.shift_insert(4, 603_usize, 2834_i32),  None);
    assert_eq!(map.shift_insert(5, 1809_usize, 2381_i32), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shift_insert4() {
    let mut map = TypeProjectedIndexMap::new();

    assert_eq!(map.len(), 0);
    assert!(map.is_empty());

    let _ = map.shift_insert(0, 477_usize, 2084_i32);
    assert_eq!(map.len(), 1);
    assert_eq!(map.as_slice(), &[
        (477_usize, 2084_i32),
    ]);

    let _ = map.shift_insert(1, 57_usize, 2657_i32);
    assert_eq!(map.len(), 2);
    assert_eq!(map.as_slice(), &[
        (477_usize, 2084_i32),
        (57_usize,  2657_i32),
    ]);

    let _ = map.shift_insert(2, 360_usize, 1352_i32);
    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), &[
        (477_usize, 2084_i32),
        (57_usize,  2657_i32),
        (360_usize, 1352_i32),
    ]);

    let _ = map.shift_insert(3, 2564_usize, 621_i32);
    assert_eq!(map.len(), 4);
    assert_eq!(map.as_slice(), &[
        (477_usize,  2084_i32),
        (57_usize,   2657_i32),
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
    ]);

    let _ = map.shift_insert(4, 603_usize, 2834_i32);
    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), &[
        (477_usize,  2084_i32),
        (57_usize,   2657_i32),
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
    ]);

    let _ = map.shift_insert(5, 1809_usize, 2381_i32);
    assert_eq!(map.len(), 6);
    assert_eq!(map.as_slice(), &[
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
fn test_type_projected_index_map_append1() {
    let mut map1 = TypeProjectedIndexMap::from([
        (605_usize,  2879_i32),
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
        (605_usize,  2879_i32),
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_append2() {
    let mut map1 = TypeProjectedIndexMap::from([
        (605_usize,  2879_i32),
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
        (605_usize,  2879_i32),
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_append3() {
    let mut map1 = TypeProjectedIndexMap::from([
        (605_usize,  2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    let mut map2 = TypeProjectedIndexMap::new();
    let expected = TypeProjectedIndexMap::from([
        (605_usize,  2879_i32),
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_append4() {
    let mut map1 = TypeProjectedIndexMap::new();
    let mut map2 = TypeProjectedIndexMap::from([
        (605_usize,  2879_i32),
        (1804_usize, 1728_i32),
        (1532_usize, 1980_i32),
        (1660_usize, 1711_i32),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (605_usize,  2879_i32),
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

#[rustfmt::skip]
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

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_retain1() {
    let mut map = TypeProjectedIndexMap::from([
        (344_usize,  ()),
        (1646_usize, ()),
        (2371_usize, ()),
        (52_usize,   ()),
        (789_usize,  ()),
        (1205_usize, ()),
        (28_usize,   ()),
        (136_usize,  ()),
    ]);
    let expected = map.clone();
    map.retain(|_k, _v| true);

    assert_eq!(map.len(), 8);
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_retain2() {
    let mut map = TypeProjectedIndexMap::from([
        (344_usize,  ()),
        (1646_usize, ()),
        (2371_usize, ()),
        (52_usize,   ()),
        (789_usize,  ()),
        (1205_usize, ()),
        (28_usize,   ()),
        (136_usize,  ()),
    ]);
    let expected = TypeProjectedIndexMap::new();
    map.retain(|_k, _v| false);

    assert_eq!(map.len(), 0);
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_retain3() {
    let mut map = TypeProjectedIndexMap::from([
        (344_usize,  ()),
        (1646_usize, ()),
        (2371_usize, ()),
        (52_usize,   ()),
        (789_usize,  ()),
        (1205_usize, ()),
        (28_usize,   ()),
        (136_usize,  ()),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (344_usize,  ()),
        (1646_usize, ()),
        (52_usize,   ()),
        (28_usize,   ()),
        (136_usize,  ()),
    ]);
    map.retain(|k, _v| k % 2 == 0);

    assert_eq!(map.len(), 5);
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_retain4() {
    let mut map = TypeProjectedIndexMap::from([
        (344_usize,  ()),
        (1646_usize, ()),
        (2371_usize, ()),
        (52_usize,   ()),
        (789_usize,  ()),
        (1205_usize, ()),
        (28_usize,   ()),
        (136_usize,  ()),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (2371_usize, ()),
        (789_usize,  ()),
        (1205_usize, ()),
    ]);
    map.retain(|k, _v| k % 2 != 0);

    assert_eq!(map.len(), 3);
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_sort_keys1() {
    let mut map = TypeProjectedIndexMap::from([
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
    let expected = TypeProjectedIndexMap::from([
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
    map.sort_keys();

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_sort_keys2() {
    let mut map = TypeProjectedIndexMap::from([
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
    let expected = TypeProjectedIndexMap::from([
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
    map.sort_keys();

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_sort_keys3() {
    let mut map = TypeProjectedIndexMap::from([
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
    let expected = TypeProjectedIndexMap::from([
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
    map.sort_keys();

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_sort_by1() {
    let mut map = TypeProjectedIndexMap::from([
        (1952_usize, 1390_i32),
        (2900_usize, 2846_i32),
        (2999_usize, 760_i32),
        (828_usize,  491_i32),
        (1738_usize, 1984_i32),
        (339_usize,  1996_i32),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (828_usize,  491_i32),
        (2999_usize, 760_i32),
        (1952_usize, 1390_i32),
        (1738_usize, 1984_i32),
        (339_usize,  1996_i32),
        (2900_usize, 2846_i32),
    ]);
    map.sort_by(|_k1, v1, _k2, v2| v1.cmp(v2));

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_sort_by2() {
    let mut map = TypeProjectedIndexMap::from([
        (String::from("4"),   ()),
        (String::from("101"), ()),
        (String::from("1"),   ()),
        (String::from("2"),   ()),
        (String::from("10"),  ()),
        (String::from("3"),   ()),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (String::from("1"),   ()),
        (String::from("10"),  ()),
        (String::from("101"), ()),
        (String::from("2"),   ()),
        (String::from("3"),   ()),
        (String::from("4"),   ()),
    ]);
    map.sort_by(|k1, _v1, k2, _v2| k1.cmp(k2));

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_sort_by3() {
    let mut map = TypeProjectedIndexMap::from([
        (String::from("400"), ()),
        (String::from("101"), ()),
        (String::from("1"),   ()),
        (String::from("2"),   ()),
        (String::from("10"),  ()),
        (String::from("3"),   ()),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (String::from("1"),   ()),
        (String::from("2"),   ()),
        (String::from("3"),   ()),
        (String::from("10"),  ()),
        (String::from("400"), ()),
        (String::from("101"), ()),
    ]);
    map.sort_by(|k1, _v1, k2, _v2| k1.len().cmp(&k2.len()));

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_sort_unstable_keys1() {
    let mut map = TypeProjectedIndexMap::from([
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
    let expected = TypeProjectedIndexMap::from([
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
    map.sort_unstable_keys();

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_sort_unstable_keys2() {
    let mut map = TypeProjectedIndexMap::from([
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
    let expected = TypeProjectedIndexMap::from([
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
    map.sort_unstable_keys();

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_sort_unstable_keys3() {
    let mut map = TypeProjectedIndexMap::from([
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
    let expected = TypeProjectedIndexMap::from([
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
    map.sort_unstable_keys();

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_sort_unstable_by1() {
    let mut map = TypeProjectedIndexMap::from([
        (1952_usize, 1390_i32),
        (2900_usize, 2846_i32),
        (2999_usize, 760_i32),
        (828_usize,  491_i32),
        (1738_usize, 1984_i32),
        (339_usize,  1996_i32),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (828_usize,  491_i32),
        (2999_usize, 760_i32),
        (1952_usize, 1390_i32),
        (1738_usize, 1984_i32),
        (339_usize,  1996_i32),
        (2900_usize, 2846_i32),
    ]);
    map.sort_unstable_by(|_k1, v1, _k2, v2| v1.cmp(v2));

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_sort_unstable_by2() {
    let mut map = TypeProjectedIndexMap::from([
        (String::from("4"),   ()),
        (String::from("101"), ()),
        (String::from("1"),   ()),
        (String::from("2"),   ()),
        (String::from("10"),  ()),
        (String::from("3"),   ()),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (String::from("1"),   ()),
        (String::from("10"),  ()),
        (String::from("101"), ()),
        (String::from("2"),   ()),
        (String::from("3"),   ()),
        (String::from("4"),   ()),
    ]);
    map.sort_unstable_by(|k1, _v1, k2, _v2| k1.cmp(k2));

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_sort_unstable_by3() {
    let mut map = TypeProjectedIndexMap::from([
        (String::from("400"), ()),
        (String::from("101"), ()),
        (String::from("1"),   ()),
        (String::from("2"),   ()),
        (String::from("10"),  ()),
        (String::from("3"),   ()),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (String::from("1"),   ()),
        (String::from("2"),   ()),
        (String::from("3"),   ()),
        (String::from("10"),  ()),
        (String::from("400"), ()),
        (String::from("101"), ()),
    ]);
    map.sort_unstable_by(|k1, _v1, k2, _v2| k1.len().cmp(&k2.len()));

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_reverse() {
    let mut map = TypeProjectedIndexMap::from([
        (39_usize,   2757_i32),
        (144_usize,  1357_i32),
        (1846_usize, 1138_i32),
        (698_usize,  473_i32),
        (642_usize,  2172_i32),
        (2101_usize, 1894_i32),
    ]);
    let expected = TypeProjectedIndexMap::from([
        (2101_usize, 1894_i32),
        (642_usize,  2172_i32),
        (698_usize,  473_i32),
        (1846_usize, 1138_i32),
        (144_usize,  1357_i32),
        (39_usize,   2757_i32),
    ]);
    map.reverse();

    assert_eq!(map.len(), expected.len());
    assert_eq!(map.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_binary_search_by1() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();

    for i in -128..128 {
        assert_eq!(map.binary_search_by(|_k, v| v.cmp(&i)), Err(0));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_binary_search_by2() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([(92_usize, 4_i32)]);

    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&0_i32)), Err(0));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&1_i32)), Err(0));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&2_i32)), Err(0));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&3_i32)), Err(0));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&4_i32)), Ok(0));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&5_i32)), Err(1));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&6_i32)), Err(1));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&7_i32)), Err(1));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&8_i32)), Err(1));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_binary_search_by3() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([
        (130_usize, 1_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
    ]);

    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&0_i32)), Err(0));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&1_i32)), Ok(0));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&2_i32)), Err(1));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&3_i32)), Err(1));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&4_i32)), Ok(1));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&5_i32)), Err(2));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&6_i32)), Err(2));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&7_i32)), Ok(2));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&8_i32)), Err(3));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&9_i32)), Err(3));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_binary_search_by4() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
        (9_usize,   8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&0_i32)),  Err(0));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&1_i32)),  Ok(0));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&2_i32)),  Err(1));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&3_i32)),  Ok(1));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&4_i32)),  Ok(2));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&5_i32)),  Err(3));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&6_i32)),  Err(3));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&7_i32)),  Ok(3));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&8_i32)),  Ok(4));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&9_i32)),  Ok(5));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&10_i32)), Err(6));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&11_i32)), Err(6));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_binary_search_by5() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (60_usize,  4_i32),
        (9_usize,   4_i32),
        (16_usize,  7_i32),
        (19_usize,  8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&0_i32)),  Err(0));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&1_i32)),  Ok(0));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&2_i32)),  Err(1));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&3_i32)),  Ok(1));

    assert!(match map.binary_search_by(|_k, v| v.cmp(&4_i32)) {
        Ok(2..=4) => true,
        _ => false,
    });

    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&5_i32)),  Err(5));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&6_i32)),  Err(5));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&7_i32)),  Ok(5));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&8_i32)),  Ok(6));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&9_i32)),  Ok(7));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&10_i32)), Err(8));
    assert_eq!(map.binary_search_by(|_k, v| v.cmp(&11_i32)), Err(8));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_binary_search_by_key1() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();

    for i in -128..128 {
        assert_eq!(map.binary_search_by_key(&i, |_k, v| *v), Err(0));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_binary_search_by_key2() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([(92_usize, 4_i32)]);

    assert_eq!(map.binary_search_by_key(&0_i32, |_k, v| *v), Err(0));
    assert_eq!(map.binary_search_by_key(&1_i32, |_k, v| *v), Err(0));
    assert_eq!(map.binary_search_by_key(&2_i32, |_k, v| *v), Err(0));
    assert_eq!(map.binary_search_by_key(&3_i32, |_k, v| *v), Err(0));
    assert_eq!(map.binary_search_by_key(&4_i32, |_k, v| *v), Ok(0));
    assert_eq!(map.binary_search_by_key(&5_i32, |_k, v| *v), Err(1));
    assert_eq!(map.binary_search_by_key(&6_i32, |_k, v| *v), Err(1));
    assert_eq!(map.binary_search_by_key(&7_i32, |_k, v| *v), Err(1));
    assert_eq!(map.binary_search_by_key(&8_i32, |_k, v| *v), Err(1));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_binary_search_by_key3() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([
        (130_usize, 1_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
    ]);

    assert_eq!(map.binary_search_by_key(&0_i32, |_k, v| *v), Err(0));
    assert_eq!(map.binary_search_by_key(&1_i32, |_k, v| *v), Ok(0));
    assert_eq!(map.binary_search_by_key(&2_i32, |_k, v| *v), Err(1));
    assert_eq!(map.binary_search_by_key(&3_i32, |_k, v| *v), Err(1));
    assert_eq!(map.binary_search_by_key(&4_i32, |_k, v| *v), Ok(1));
    assert_eq!(map.binary_search_by_key(&5_i32, |_k, v| *v), Err(2));
    assert_eq!(map.binary_search_by_key(&6_i32, |_k, v| *v), Err(2));
    assert_eq!(map.binary_search_by_key(&7_i32, |_k, v| *v), Ok(2));
    assert_eq!(map.binary_search_by_key(&8_i32, |_k, v| *v), Err(3));
    assert_eq!(map.binary_search_by_key(&9_i32, |_k, v| *v), Err(3));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_binary_search_by_key4() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
        (9_usize,   8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(map.binary_search_by_key(&0_i32,  |_k, v| *v),  Err(0));
    assert_eq!(map.binary_search_by_key(&1_i32,  |_k, v| *v),  Ok(0));
    assert_eq!(map.binary_search_by_key(&2_i32,  |_k, v| *v),  Err(1));
    assert_eq!(map.binary_search_by_key(&3_i32,  |_k, v| *v),  Ok(1));
    assert_eq!(map.binary_search_by_key(&4_i32,  |_k, v| *v),  Ok(2));
    assert_eq!(map.binary_search_by_key(&5_i32,  |_k, v| *v),  Err(3));
    assert_eq!(map.binary_search_by_key(&6_i32,  |_k, v| *v),  Err(3));
    assert_eq!(map.binary_search_by_key(&7_i32,  |_k, v| *v),  Ok(3));
    assert_eq!(map.binary_search_by_key(&8_i32,  |_k, v| *v),  Ok(4));
    assert_eq!(map.binary_search_by_key(&9_i32,  |_k, v| *v),  Ok(5));
    assert_eq!(map.binary_search_by_key(&10_i32, |_k, v| *v), Err(6));
    assert_eq!(map.binary_search_by_key(&11_i32, |_k, v| *v), Err(6));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_binary_search_by_key5() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (60_usize,  4_i32),
        (9_usize,   4_i32),
        (16_usize,  7_i32),
        (19_usize,  8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(map.binary_search_by_key(&0_i32, |_k, v| *v),  Err(0));
    assert_eq!(map.binary_search_by_key(&1_i32, |_k, v| *v),  Ok(0));
    assert_eq!(map.binary_search_by_key(&2_i32, |_k, v| *v),  Err(1));
    assert_eq!(map.binary_search_by_key(&3_i32, |_k, v| *v),  Ok(1));

    assert!(match map.binary_search_by_key(&4_i32, |_k, v| *v) {
        Ok(2..=4) => true,
        _ => false,
    });

    assert_eq!(map.binary_search_by_key(&5_i32,  |_k, v| *v), Err(5));
    assert_eq!(map.binary_search_by_key(&6_i32,  |_k, v| *v), Err(5));
    assert_eq!(map.binary_search_by_key(&7_i32,  |_k, v| *v), Ok(5));
    assert_eq!(map.binary_search_by_key(&8_i32,  |_k, v| *v), Ok(6));
    assert_eq!(map.binary_search_by_key(&9_i32,  |_k, v| *v), Ok(7));
    assert_eq!(map.binary_search_by_key(&10_i32, |_k, v| *v), Err(8));
    assert_eq!(map.binary_search_by_key(&11_i32, |_k, v| *v), Err(8));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_partition_point1() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::new();

    for i in -128..128 {
        assert_eq!(map.partition_point(|_k, v| *v < i), 0);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_partition_point2() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([(92_usize, 4_i32)]);

    assert_eq!(map.partition_point(|_k, v| *v < 0_i32), 0);
    assert_eq!(map.partition_point(|_k, v| *v < 1_i32), 0);
    assert_eq!(map.partition_point(|_k, v| *v < 2_i32), 0);
    assert_eq!(map.partition_point(|_k, v| *v < 3_i32), 0);
    assert_eq!(map.partition_point(|_k, v| *v < 4_i32), 0);
    assert_eq!(map.partition_point(|_k, v| *v < 5_i32), 1);
    assert_eq!(map.partition_point(|_k, v| *v < 6_i32), 1);
    assert_eq!(map.partition_point(|_k, v| *v < 7_i32), 1);
    assert_eq!(map.partition_point(|_k, v| *v < 8_i32), 1);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_partition_point3() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([
        (130_usize, 1_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
    ]);

    assert_eq!(map.partition_point(|_k, v| *v < 0_i32), 0);
    assert_eq!(map.partition_point(|_k, v| *v < 1_i32), 0);
    assert_eq!(map.partition_point(|_k, v| *v < 2_i32), 1);
    assert_eq!(map.partition_point(|_k, v| *v < 3_i32), 1);
    assert_eq!(map.partition_point(|_k, v| *v < 4_i32), 1);
    assert_eq!(map.partition_point(|_k, v| *v < 5_i32), 2);
    assert_eq!(map.partition_point(|_k, v| *v < 6_i32), 2);
    assert_eq!(map.partition_point(|_k, v| *v < 7_i32), 2);
    assert_eq!(map.partition_point(|_k, v| *v < 8_i32), 3);
    assert_eq!(map.partition_point(|_k, v| *v < 9_i32), 3);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_partition_point4() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
        (9_usize,   8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(map.partition_point(|_k, v| *v < 0_i32),  0);
    assert_eq!(map.partition_point(|_k, v| *v < 1_i32),  0);
    assert_eq!(map.partition_point(|_k, v| *v < 2_i32),  1);
    assert_eq!(map.partition_point(|_k, v| *v < 3_i32),  1);
    assert_eq!(map.partition_point(|_k, v| *v < 4_i32),  2);
    assert_eq!(map.partition_point(|_k, v| *v < 5_i32),  3);
    assert_eq!(map.partition_point(|_k, v| *v < 6_i32),  3);
    assert_eq!(map.partition_point(|_k, v| *v < 7_i32),  3);
    assert_eq!(map.partition_point(|_k, v| *v < 8_i32),  4);
    assert_eq!(map.partition_point(|_k, v| *v < 9_i32),  5);
    assert_eq!(map.partition_point(|_k, v| *v < 10_i32), 6);
    assert_eq!(map.partition_point(|_k, v| *v < 11_i32), 6);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_partition_point5() {
    let map: TypeProjectedIndexMap<usize, i32> = TypeProjectedIndexMap::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (60_usize,  4_i32),
        (9_usize,   4_i32),
        (16_usize,  7_i32),
        (19_usize,  8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(map.partition_point(|_k, v| *v < 0_i32), 0);
    assert_eq!(map.partition_point(|_k, v| *v < 1_i32), 0);
    assert_eq!(map.partition_point(|_k, v| *v < 2_i32), 1);
    assert_eq!(map.partition_point(|_k, v| *v < 3_i32), 1);

    assert!(match map.partition_point(|_k, v| *v < 4_i32) {
        2..=4 => true,
        _ => false,
    });

    assert_eq!(map.partition_point(|_k, v| *v < 5_i32),  5);
    assert_eq!(map.partition_point(|_k, v| *v < 6_i32),  5);
    assert_eq!(map.partition_point(|_k, v| *v < 7_i32),  5);
    assert_eq!(map.partition_point(|_k, v| *v < 8_i32),  6);
    assert_eq!(map.partition_point(|_k, v| *v < 9_i32),  7);
    assert_eq!(map.partition_point(|_k, v| *v < 10_i32), 8);
    assert_eq!(map.partition_point(|_k, v| *v < 11_i32), 8);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_reserve1() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);

    map.reserve(additional);

    assert!(map.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_reserve2() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);

    map.reserve(additional);

    assert!(map.capacity() >= additional);

    let old_capacity = map.capacity();
    map.insert(0, usize::MAX);
    for i in 1..(map.capacity() - 1) {
        map.insert(i, 0_usize);
    }

    map.insert(map.capacity() - 1, usize::MAX);

    assert_eq!(map.len(), map.capacity());
    assert_eq!(map.capacity(), old_capacity);

    assert_eq!(map[0], usize::MAX);
    for i in 1..(map.len() - 1) {
        assert_eq!(map[i], 0_usize);
    }
    assert_eq!(map[map.len() - 1], usize::MAX);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_reserve3() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.len(), 0);

    for i in 0..4 {
        let old_capacity = map.capacity();
        map.reserve(additional);

        assert!(map.capacity() >= old_capacity + additional);
        assert!(map.len() <= map.capacity());

        let length = map.len();
        map.insert(length, usize::MAX);
        for j in (length + 1)..(map.capacity() - 1) {
            map.insert(j, i);
        }
        map.insert(map.capacity() - 1, usize::MAX);

        assert_eq!(map.len(), map.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..4 {
        for j in (current_start + 1)..map.len() {
            if map[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(map[current_start], usize::MAX);
        for value in map[(current_start + 1)..current_end].values().copied() {
            assert_eq!(value, i);
        }
        assert_eq!(map[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_reserve_exact1() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);

    map.reserve_exact(additional);

    assert!(map.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_reserve_exact2() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);

    map.reserve_exact(additional);

    assert!(map.capacity() >= additional);

    let old_capacity = map.capacity();
    map.insert(0, usize::MAX);
    for i in 1..(map.capacity() - 1) {
        map.insert(i, 0_usize);
    }

    map.insert(map.capacity() - 1, usize::MAX);

    assert_eq!(map.len(), map.capacity());
    assert_eq!(map.capacity(), old_capacity);

    assert_eq!(map[0], usize::MAX);
    for i in 1..(map.len() - 1) {
        assert_eq!(map[i], 0_usize);
    }
    assert_eq!(map[map.len() - 1], usize::MAX);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_reserve_exact3() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.len(), 0);

    for i in 0..32 {
        let old_capacity = map.capacity();
        map.reserve_exact(additional);

        assert!(map.capacity() >= old_capacity + additional);
        assert!(map.len() <= map.capacity());

        let length = map.len();
        map.insert(length, usize::MAX);
        for j in (length + 1)..(map.capacity() - 1) {
            map.insert(j, i);
        }
        map.insert(map.capacity() - 1, usize::MAX);

        assert_eq!(map.len(), map.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..32 {
        for j in (current_start + 1)..map.len() {
            if map[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(map[current_start], usize::MAX);
        for value in map[(current_start + 1)..current_end].values().copied() {
            assert_eq!(value, i);
        }
        assert_eq!(map[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_try_reserve1() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.try_reserve(additional), Ok(()));
    assert!(map.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_try_reserve2() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.try_reserve(additional), Ok(()));
    assert!(map.capacity() >= additional);

    let old_capacity = map.capacity();
    map.insert(0, usize::MAX);
    for i in 1..(map.capacity() - 1) {
        map.insert(i, 0_usize);
    }

    map.insert(map.capacity() - 1, usize::MAX);

    assert_eq!(map.len(), map.capacity());
    assert_eq!(map.capacity(), old_capacity);

    assert_eq!(map[0], usize::MAX);
    for i in 1..(map.len() - 1) {
        assert_eq!(map[i], 0_usize);
    }
    assert_eq!(map[map.len() - 1], usize::MAX);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_try_reserve3() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.len(), 0);

    for i in 0..4 {
        let old_capacity = map.capacity();
        assert_eq!(map.try_reserve(additional), Ok(()));

        assert!(map.capacity() >= old_capacity + additional);
        assert!(map.len() <= map.capacity());

        let length = map.len();
        map.insert(length, usize::MAX);
        for j in (length + 1)..(map.capacity() - 1) {
            map.insert(j, i);
        }
        map.insert(map.capacity() - 1, usize::MAX);

        assert_eq!(map.len(), map.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..4 {
        for j in (current_start + 1)..map.len() {
            if map[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(map[current_start], usize::MAX);
        for value in map[(current_start + 1)..current_end].values().copied() {
            assert_eq!(value, i);
        }
        assert_eq!(map[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_try_reserve_exact1() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.try_reserve_exact(additional), Ok(()));
    assert!(map.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_try_reserve_exact2() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.try_reserve_exact(additional), Ok(()));
    assert!(map.capacity() >= additional);

    let old_capacity = map.capacity();
    map.insert(0, usize::MAX);
    for i in 1..(map.capacity() - 1) {
        map.insert(i, 0_usize);
    }

    map.insert(map.capacity() - 1, usize::MAX);

    assert_eq!(map.len(), map.capacity());
    assert_eq!(map.capacity(), old_capacity);

    assert_eq!(map[0], usize::MAX);
    for i in 1..(map.len() - 1) {
        assert_eq!(map[i], 0_usize);
    }
    assert_eq!(map[map.len() - 1], usize::MAX);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_try_reserve_exact3() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    let additional = 100;

    assert_eq!(map.capacity(), 0);
    assert_eq!(map.len(), 0);

    for i in 0..32 {
        let old_capacity = map.capacity();
        assert_eq!(map.try_reserve_exact(additional), Ok(()));

        assert!(map.capacity() >= old_capacity + additional);
        assert!(map.len() <= map.capacity());

        let length = map.len();
        map.insert(length, usize::MAX);
        for j in (length + 1)..(map.capacity() - 1) {
            map.insert(j, i);
        }
        map.insert(map.capacity() - 1, usize::MAX);

        assert_eq!(map.len(), map.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..32 {
        for j in (current_start + 1)..map.len() {
            if map[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(map[current_start], usize::MAX);
        for value in map[(current_start + 1)..current_end].values().copied() {
            assert_eq!(value, i);
        }
        assert_eq!(map[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shrink_to_fit1() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::with_capacity(10);
    assert_eq!(map.capacity(), 10);

    map.extend([(1_usize, usize::MAX), (2_usize, usize::MAX), (3_usize, usize::MAX)]);
    assert!(map.len() <= map.capacity());
    map.shrink_to_fit();
    assert_eq!(map.len(), map.capacity());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_map_shrink_to_fit2() {
    let mut map: TypeProjectedIndexMap<usize, usize> = TypeProjectedIndexMap::new();
    for i in 0..128 {
        assert_eq!(map.len(), i);

        map.insert(i, i * i);

        assert_eq!(map.len(), i + 1);
        assert!(map.capacity() >= i + 1);
        assert_eq!(map[i], i * i);
        assert_eq!(map.get(&i), Some(&(i * i)));

        map.shrink_to_fit();

        assert_eq!(map.len(), i + 1);
        assert_eq!(map.capacity(), i + 1);
        assert_eq!(map[i], i * i);
        assert_eq!(map.get(&i), Some(&(i * i)));
    }
}
