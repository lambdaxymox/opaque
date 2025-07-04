use opaque_index_map::set::TypeErasedIndexSet;

use core::any;
use core::fmt;
use std::iter;
use std::hash;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[test]
fn test_empty_len1() {
    let opaque_set = TypeErasedIndexSet::new::<u64>();
    let expected = 0;
    let result = opaque_set.len();

    assert_eq!(result, expected);
}

#[test]
fn test_empty_is_empty1() {
    let opaque_set = TypeErasedIndexSet::new::<u64>();

    assert!(opaque_set.is_empty());
}

#[test]
fn test_empty_contains_no_values1() {
    let opaque_set = TypeErasedIndexSet::new::<u64>();
    for value in 0..65536 {
        assert!(!opaque_set.contains::<_, u64, hash::RandomState, alloc::Global>(&value));
    }
}

#[test]
fn test_empty_get1() {
    let opaque_set = TypeErasedIndexSet::new::<u64>();
    for value in 0..65536 {
        let result = opaque_set.get::<_, u64, hash::RandomState, alloc::Global>(&value);

        assert!(result.is_none());
    }
}

#[test]
fn test_empty_len2() {
    let opaque_set = TypeErasedIndexSet::new::<usize>();
    let expected = 0;
    let result = opaque_set.len();

    assert_eq!(result, expected);
}

#[test]
fn test_empty_is_empty2() {
    let opaque_set = TypeErasedIndexSet::new::<usize>();

    assert!(opaque_set.is_empty());
}

#[test]
fn test_empty_contains_no_values2() {
    let opaque_set = TypeErasedIndexSet::new::<usize>();
    for value in 0..65536 {
        assert!(!opaque_set.contains::<_, usize, hash::RandomState, alloc::Global>(&value));
    }
}

#[test]
fn test_empty_get2() {
    let opaque_set = TypeErasedIndexSet::new::<usize>();
    for value in 0..65536 {
        let result = opaque_set.get::<_, usize, hash::RandomState, alloc::Global>(&value);

        assert!(result.is_none());
    }
}
