use opaque_index_map::OpaqueIndexMap;
use opaque_vec::OpaqueVec;

use core::hash;

fn run_test_opaque_index_map_insert_contains_key<K, V>(entries: &[(K, V)])
where
    K: Clone + Eq + hash::Hash + 'static,
    V: Clone + Eq + 'static,
{
    let mut map = OpaqueIndexMap::new::<K, V>();

    for key in entries.iter().map(|tuple| &tuple.0) {
        assert!(!map.contains_key::<K, K, V>(key));
    }

    for (key, value) in entries.iter().cloned() {
        map.insert::<K, V>(key, value);
    }

    for key in entries.iter().map(|tuple| &tuple.0) {
        assert!(map.contains_key::<K, K, V>(key));
    }
}

#[test]
fn test_opaque_map_insert_contains_key_empty() {
    let map = OpaqueIndexMap::new::<u32, i32>();
    for key in 0..65536 {
        assert!(!map.contains_key::<u32, u32, i32>(&key));
    }
}

#[test]
fn test_opaque_map_insert_get1() {
    let entries = {
        let entries: [(u32, i32); 1] = [(0, 1)];
        OpaqueVec::from(&entries)
    };

    run_test_opaque_index_map_insert_contains_key(entries.as_slice::<(u32, i32)>());
}

#[test]
fn test_opaque_map_insert_get2() {
    let entries = {
        let entries: [(u32, i32); 2] = [(0, 1), (1, 2)];
        OpaqueVec::from(&entries)
    };

    run_test_opaque_index_map_insert_contains_key(entries.as_slice::<(u32, i32)>());
}

#[test]
fn test_opaque_map_insert_get3() {
    let entries = {
        let entries: [(u32, i32); 3] = [(0, 1), (1, 2), (2, 3)];
        OpaqueVec::from(&entries)
    };

    run_test_opaque_index_map_insert_contains_key(entries.as_slice::<(u32, i32)>());
}

#[test]
fn test_opaque_map_insert_get4() {
    let entries = {
        let entries: [(u32, i32); 4] = [(0, 1), (1, 2), (2, 3), (3, 4)];
        OpaqueVec::from(&entries)
    };

    run_test_opaque_index_map_insert_contains_key(entries.as_slice::<(u32, i32)>());
}
