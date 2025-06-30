use opaque_index_map::set::TypedProjIndexSet;

use core::any;
use core::fmt;
use std::alloc;
use std::iter;
use std::hash;

#[test]
fn test_empty_len1() {
    let proj_set: TypedProjIndexSet<u64> = TypedProjIndexSet::new();
    let expected = 0;
    let result = proj_set.len();

    assert_eq!(result, expected);
}

#[test]
fn test_empty_is_empty1() {
    let proj_set: TypedProjIndexSet<u64> = TypedProjIndexSet::new();

    assert!(proj_set.is_empty());
}

#[test]
fn test_empty_contains_no_values1() {
    let proj_set: TypedProjIndexSet<u64> = TypedProjIndexSet::new();
    for value in 0..65536 {
        assert!(!proj_set.contains(&value));
    }
}

#[test]
fn test_empty_get1() {
    let proj_set: TypedProjIndexSet<u64> = TypedProjIndexSet::new();
    for value in 0..65536 {
        let result = proj_set.get(&value);

        assert!(result.is_none());
    }
}

#[test]
fn test_empty_len2() {
    let proj_set: TypedProjIndexSet<u64> = TypedProjIndexSet::new();
    let expected = 0;
    let result = proj_set.len();

    assert_eq!(result, expected);
}

#[test]
fn test_empty_is_empty2() {
    let proj_set: TypedProjIndexSet<u64> = TypedProjIndexSet::new();

    assert!(proj_set.is_empty());
}

#[test]
fn test_empty_contains_no_values2() {
    let proj_set: TypedProjIndexSet<u64> = TypedProjIndexSet::new();
    for value in 0..65536 {
        assert!(!proj_set.contains(&value));
    }
}

#[test]
fn test_empty_get2() {
    let proj_set: TypedProjIndexSet<u64> = TypedProjIndexSet::new();
    for value in 0..65536 {
        let result = proj_set.get(&value);

        assert!(result.is_none());
    }
}
