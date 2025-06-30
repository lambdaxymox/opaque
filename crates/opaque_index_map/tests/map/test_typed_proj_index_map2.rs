use opaque_index_map::map::TypedProjIndexMap;

use core::any;
use core::fmt;
use std::alloc;
use std::iter;
use std::hash;

#[test]
fn run_test_typed_proj_index_map_empty_len1() {
    let proj_map: TypedProjIndexMap<u64, i64> = TypedProjIndexMap::new();

    assert_eq!(proj_map.len(), 0);
}

#[test]
fn run_test_typed_proj_index_map_empty_is_empty1() {
    let proj_map: TypedProjIndexMap<u64, i64> = TypedProjIndexMap::new();

    assert!(proj_map.is_empty());
}

#[test]
fn run_test_typed_proj_index_map_empty_contains_no_values1() {
    let proj_map: TypedProjIndexMap<u64, i64> = TypedProjIndexMap::new();
    for key in 0..65536 {
        assert!(!proj_map.contains_key(&key));
    }
}

#[test]
fn run_test_typed_proj_index_map_empty_get1() {
    let proj_map: TypedProjIndexMap<u64, i64> = TypedProjIndexMap::new();
    for key in 0..65536 {
        let result = proj_map.get(&key);

        assert!(result.is_none());
    }
}

#[test]
fn run_test_typed_proj_index_map_empty_len2() {
    let proj_map: TypedProjIndexMap<usize, i64> = TypedProjIndexMap::new();

    assert_eq!(proj_map.len(), 0);
}

#[test]
fn run_test_typed_proj_index_map_empty_is_empty2() {
    let proj_map: TypedProjIndexMap<usize, i64> = TypedProjIndexMap::new();

    assert!(proj_map.is_empty());
}

#[test]
fn run_test_typed_proj_index_map_empty_contains_no_values2() {
    let proj_map: TypedProjIndexMap<usize, i64> = TypedProjIndexMap::new();
    for key in 0..65536 {
        assert!(!proj_map.contains_key(&key));
    }
}

#[test]
fn run_test_typed_proj_index_map_empty_get2() {
    let proj_map: TypedProjIndexMap<usize, i64> = TypedProjIndexMap::new();
    for key in 0..65536 {
        let result = proj_map.get(&key);

        assert!(result.is_none());
    }
}
