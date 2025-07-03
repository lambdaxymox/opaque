use opaque_index_map::map::OpaqueIndexMap;

use core::any;
use core::fmt;
use std::iter;
use std::hash;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[test]
fn run_test_opaque_index_map_empty_len1() {
    let opaque_map = OpaqueIndexMap::new::<u64, i64>();

    assert_eq!(opaque_map.len(), 0);
}

#[test]
fn run_test_opaque_index_map_empty_is_empty1() {
    let opaque_map = OpaqueIndexMap::new::<u64, i64>();

    assert!(opaque_map.is_empty());
}

#[test]
fn run_test_opaque_index_map_empty_contains_no_values1() {
    let opaque_map = OpaqueIndexMap::new::<u64, i64>();
    for key in 0..65536 {
        assert!(!opaque_map.contains_key::<_, u64, i64, hash::RandomState, alloc::Global>(&key));
    }
}

#[test]
fn run_test_opaque_index_map_empty_get1() {
    let opaque_map = OpaqueIndexMap::new::<u64, i64>();
    for key in 0..65536 {
        let result = opaque_map.get::<_, u64, i64, hash::RandomState, alloc::Global>(&key);

        assert!(result.is_none());
    }
}

#[test]
fn run_test_opaque_index_map_empty_len2() {
   let opaque_map = OpaqueIndexMap::new::<usize, i64>();

    assert_eq!(opaque_map.len(), 0);
}

#[test]
fn run_test_opaque_index_map_empty_is_empty2() {
   let opaque_map = OpaqueIndexMap::new::<usize, i64>();

    assert!(opaque_map.is_empty());
}

#[test]
fn run_test_opaque_index_map_empty_contains_no_values2() {
   let opaque_map = OpaqueIndexMap::new::<usize, i64>();
    for key in 0..65536 {
        assert!(!opaque_map.contains_key::<_, usize, i64, hash::RandomState, alloc::Global>(&key));
    }
}

#[test]
fn run_test_opaque_index_map_empty_get2() {
   let opaque_map = OpaqueIndexMap::new::<usize, i64>();
    for key in 0..65536 {
        let result = opaque_map.get::<_, usize, i64, hash::RandomState, alloc::Global>(&key);

        assert!(result.is_none());
    }
}
