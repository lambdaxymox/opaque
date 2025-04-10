extern crate core;

use core::hash;
use opaque_vec::OpaqueVec;
use opaque_index_map::OpaqueIndexMap;

fn run_test_opaque_index_map_insert_as_slice<K, V>(entries: &[(K, V)])
where
    K: Clone + Eq + hash::Hash + 'static,
    V: Clone + Eq + 'static,
{
    let mut map = OpaqueIndexMap::new::<u32, i32>();
    for (key, value) in entries.iter().cloned() {
        map.insert(key, value);
    }

    let expected_values = entries.iter().map(|tuple| tuple.1.clone()).collect::<OpaqueVec>();
    let expected = expected_values.as_slice::<i32>();
    let result_values: OpaqueVec = map
        .as_slice::<u32, i32>()
        .values()
        .cloned()
        .collect();
    let result = result_values.as_slice::<i32>();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_index_map_insert_as_slice_empty() {
    let entries: [(u32, i32); 0] = [];

    run_test_opaque_index_map_insert_as_slice(&entries);
}

#[test]
fn test_opaque_index_map_insert_as_slice1() {
    let entries: [(u32, i32); 1] = [(0, 1)];

    run_test_opaque_index_map_insert_as_slice(&entries);
}

#[test]
fn test_opaque_index_map_insert_as_slice2() {
    let entries: [(u32, i32); 2] = [(0, 1), (1, 2)];

    run_test_opaque_index_map_insert_as_slice(&entries);
}

#[test]
fn test_opaque_index_map_insert_as_slice3() {
    let entries: [(u32, i32); 3] = [(0, 1), (1, 2), (2, 3)];

    run_test_opaque_index_map_insert_as_slice(&entries);
}

#[test]
fn test_opaque_index_map_insert_as_slice4() {
    let entries: [(u32, i32); 4] = [(0, 1), (1, 2), (2, 3), (3, 4)];

    run_test_opaque_index_map_insert_as_slice(&entries);
}
