use opaque_index_map::set::TypeProjectedIndexSet;
use opaque_vec::TypeProjectedVec;

use core::any;
use core::fmt;
use std::iter;
use std::hash;
use std::string::String;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[test]
fn test_type_projected_empty_len1() {
    let proj_set: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let expected = 0;
    let result = proj_set.len();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_empty_is_empty1() {
    let proj_set: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();

    assert!(proj_set.is_empty());
}

#[test]
fn test_type_projected_empty_contains_no_values1() {
    let proj_set: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    for value in 0..65536 {
        assert!(!proj_set.contains(&value));
    }
}

#[test]
fn test_type_projected_empty_get1() {
    let proj_set: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    for value in 0..65536 {
        let result = proj_set.get(&value);

        assert!(result.is_none());
    }
}

#[test]
fn test_type_projected_empty_len2() {
    let proj_set: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let expected = 0;
    let result = proj_set.len();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_empty_is_empty2() {
    let proj_set: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();

    assert!(proj_set.is_empty());
}

#[test]
fn test_type_projected_empty_contains_no_values2() {
    let proj_set: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    for value in 0..65536 {
        assert!(!proj_set.contains(&value));
    }
}

#[test]
fn test_type_projected_empty_get2() {
    let proj_set: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    for value in 0..65536 {
        let result = proj_set.get(&value);

        assert!(result.is_none());
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_eq1() {
    assert_eq!(
        TypeProjectedIndexSet::<usize>::new(),
        TypeProjectedIndexSet::<usize>::new(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_eq2() {
    assert_eq!(
        TypeProjectedIndexSet::from([0_usize]),
        TypeProjectedIndexSet::from([0_usize]),
    );
    assert_eq!(
        TypeProjectedIndexSet::from([1_usize]),
        TypeProjectedIndexSet::from([1_usize]),
    );
    assert_eq!(
        TypeProjectedIndexSet::from([2_usize]),
        TypeProjectedIndexSet::from([2_usize]),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_eq3() {
    assert_eq!(
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
    );
    assert_eq!(
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
        TypeProjectedIndexSet::from([1_usize, 0_usize]),
    );
    assert_eq!(
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
    );
    assert_eq!(
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
        TypeProjectedIndexSet::from([2_usize, 0_usize]),
    );
    assert_eq!(
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
    );
    assert_eq!(
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
        TypeProjectedIndexSet::from([2_usize, 1_usize]),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_eq4() {
    assert_eq!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    );
    assert_eq!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::from([0_usize, 2_usize, 1_usize]),
    );
    assert_eq!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::from([2_usize, 0_usize, 1_usize]),
    );
    assert_eq!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::from([2_usize, 1_usize, 0_usize]),
    );
    assert_eq!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::from([1_usize, 2_usize, 0_usize]),
    );
    assert_eq!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::from([1_usize, 0_usize, 2_usize]),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_eq1() {
    assert_ne!(
        TypeProjectedIndexSet::new(),
        TypeProjectedIndexSet::from([0_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize]),
        TypeProjectedIndexSet::new(),
    );
    assert_ne!(
        TypeProjectedIndexSet::new(),
        TypeProjectedIndexSet::from([1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize]),
        TypeProjectedIndexSet::new(),
    );
    assert_ne!(
        TypeProjectedIndexSet::new(),
        TypeProjectedIndexSet::from([2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([2_usize]),
        TypeProjectedIndexSet::new(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_eq2() {
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize]),
        TypeProjectedIndexSet::from([1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize]),
        TypeProjectedIndexSet::from([0_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize]),
        TypeProjectedIndexSet::from([2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([2_usize]),
        TypeProjectedIndexSet::from([0_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize]),
        TypeProjectedIndexSet::from([2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([2_usize]),
        TypeProjectedIndexSet::from([1_usize]),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_eq3() {
    assert_ne!(
        TypeProjectedIndexSet::new(),
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
        TypeProjectedIndexSet::new(),
    );
    assert_ne!(
        TypeProjectedIndexSet::new(),
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
        TypeProjectedIndexSet::new(),
    );
    assert_ne!(
        TypeProjectedIndexSet::new(),
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
        TypeProjectedIndexSet::new(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_eq4() {
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize]),
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
        TypeProjectedIndexSet::from([0_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize]),
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
        TypeProjectedIndexSet::from([1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([2_usize]),
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
        TypeProjectedIndexSet::from([2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize]),
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
        TypeProjectedIndexSet::from([0_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([2_usize]),
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
        TypeProjectedIndexSet::from([2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize]),
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
        TypeProjectedIndexSet::from([1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize]),
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
        TypeProjectedIndexSet::from([1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([2_usize]),
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
        TypeProjectedIndexSet::from([2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize]),
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
        TypeProjectedIndexSet::from([0_usize]),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_eq5() {
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_eq6() {
    assert_ne!(
        TypeProjectedIndexSet::new(),
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::new(),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_eq7() {
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize]),
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::from([0_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize]),
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::from([1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([2_usize]),
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::from([2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
        TypeProjectedIndexSet::from([2_usize, 0_usize, 1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::from([0_usize, 1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
        TypeProjectedIndexSet::from([2_usize, 0_usize, 1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::from([0_usize, 2_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
        TypeProjectedIndexSet::from([2_usize, 0_usize, 1_usize]),
    );
    assert_ne!(
        TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        TypeProjectedIndexSet::from([1_usize, 2_usize]),
    );
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_is_subset1() {
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::<usize>::new(),
        &TypeProjectedIndexSet::<usize>::new(),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_is_subset2() {
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_is_subset3() {
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_is_subset4() {
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_is_subset1() {
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_is_subset2() {
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_is_subset3() {
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_subset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_is_superset1() {
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::<usize>::new(),
        &TypeProjectedIndexSet::<usize>::new(),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_is_superset2() {
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_is_superset3() {
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_is_superset4() {
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_is_superset1() {
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_is_superset2() {
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_is_superset3() {
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_superset(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_is_disjoint1() {
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::<usize>::new(),
        &TypeProjectedIndexSet::<usize>::new(),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_is_disjoint2() {
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_is_disjoint3() {
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
     assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
     assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_is_disjoint4() {
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::new(),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::new(),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_is_disjoint1() {
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_is_disjoint2() {
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_not_is_disjoint3() {
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
    assert!(!TypeProjectedIndexSet::is_disjoint(
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
        &TypeProjectedIndexSet::from([0_usize, 1_usize, 2_usize]),
    ));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_index_of1() {
    let mut set = TypeProjectedIndexSet::new();
    assert_eq!(set.get_index_of(&"a"), None);

    set.insert("a");
    set.insert("b");
    set.insert("c");

    assert_eq!(set.get_index_of(&"a"), Some(0));
    assert_eq!(set.get_index_of(&"b"), Some(1));
    assert_eq!(set.get_index_of(&"c"), Some(2));
    assert_eq!(set.get_index_of(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_index_of2() {
    let mut set = TypeProjectedIndexSet::from([
        0_usize,
        1_usize,
        2_usize,
        3_usize,
        4_usize,
        5_usize,
    ]);

    assert_eq!(set.get_index_of(&0_usize), Some(0));
    assert_eq!(set.get_index_of(&1_usize), Some(1));
    assert_eq!(set.get_index_of(&2_usize), Some(2));
    assert_eq!(set.get_index_of(&3_usize), Some(3));
    assert_eq!(set.get_index_of(&4_usize), Some(4));
    assert_eq!(set.get_index_of(&5_usize), Some(5));
    assert_eq!(set.get_index_of(&6_usize), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_index_of3() {
    let mut set = TypeProjectedIndexSet::from(["a", "b", "c"]);

    assert_eq!(set.get_index_of(&"a"), Some(0));
    assert_eq!(set.get_index_of(&"c"), Some(2));
    assert_eq!(set.get_index_of(&"b"), Some(1));

    set.swap_remove("b");

    assert_eq!(set.get_index_of(&"a"), Some(0));
    assert_eq!(set.get_index_of(&"c"), Some(1));
    assert_eq!(set.get_index_of(&"b"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_index_of4() {
    let mut set: TypeProjectedIndexSet<char> = ('a'..='z').collect();
    assert_eq!(set.get_index_of(&'*'), None);

    set.insert_before(10, '*');
    assert_eq!(set.get_index_of(&'*'), Some(10));

    set.insert_before(10, 'a');
    assert_eq!(set.get_index_of(&'a'), Some(9));
    assert_eq!(set.get_index_of(&'*'), Some(10));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_index_of5() {
    let mut set: TypeProjectedIndexSet<char> = ('a'..='z').collect();
    assert_eq!(set.get_index_of(&'*'), None);

    set.shift_insert(10, '*');
    assert_eq!(set.get_index_of(&'*'), Some(10));

    set.shift_insert(10, 'a');
    assert_eq!(set.get_index_of(&'a'), Some(10));
    assert_eq!(set.get_index_of(&'*'), Some(9));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_index_of6() {
    let mut set = TypeProjectedIndexSet::from(["a", "b"]);

    assert_eq!(set.get_index_of(&"a"), Some(0));
    assert_eq!(set.get_index_of(&"b"), Some(1));
    assert_eq!(set.get_index_of(&"c"), None);

    set.insert("c");

    assert_eq!(set.get_index_of(&"a"), Some(0));
    assert_eq!(set.get_index_of(&"b"), Some(1));
    assert_eq!(set.get_index_of(&"c"), Some(2));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get1() {
    let mut set = TypeProjectedIndexSet::new();

    assert_eq!(set.get(&"a"), None);
    assert_eq!(set.get(&"b"), None);
    assert_eq!(set.get(&"c"), None);
    assert_eq!(set.get(&"d"), None);

    set.insert("a");
    set.insert("b");
    set.insert("c");

    assert_eq!(set.get(&"a"), Some(&"a"));
    assert_eq!(set.get(&"b"), Some(&"b"));
    assert_eq!(set.get(&"c"), Some(&"c"));
    assert_eq!(set.get(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get2() {
    let set = TypeProjectedIndexSet::from([
        0_usize,
        1_usize,
        2_usize,
        3_usize,
        4_usize,
        5_usize,
    ]);

    assert_eq!(set.get(&0_usize), Some(&0_usize));
    assert_eq!(set.get(&1_usize), Some(&1_usize));
    assert_eq!(set.get(&2_usize), Some(&2_usize));
    assert_eq!(set.get(&3_usize), Some(&3_usize));
    assert_eq!(set.get(&4_usize), Some(&4_usize));
    assert_eq!(set.get(&5_usize), Some(&5_usize));
    assert_eq!(set.get(&6_usize), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get3() {
    let mut set = TypeProjectedIndexSet::from(["a", "b", "c"]);

    assert_eq!(set.get(&"a"), Some(&"a"));
    assert_eq!(set.get(&"c"), Some(&"c"));
    assert_eq!(set.get(&"b"), Some(&"b"));

    set.swap_remove("b");

    assert_eq!(set.get(&"a"), Some(&"a"));
    assert_eq!(set.get(&"c"), Some(&"c"));
    assert_eq!(set.get(&"b"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get4() {
    let mut set: TypeProjectedIndexSet<char> = ('a'..='z').collect();
    assert_eq!(set.get(&'*'), None);

    set.insert_before(10, '*');
    assert_eq!(set.get(&'*'), Some(&'*'));

    set.insert_before(10, 'a');
    assert_eq!(set.get(&'a'), Some(&'a'));
    assert_eq!(set.get(&'*'), Some(&'*'));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get5() {
    let mut set: TypeProjectedIndexSet<char> = ('a'..='z').collect();
    assert_eq!(set.get(&'*'), None);

    set.shift_insert(10, '*');
    assert_eq!(set.get(&'*'), Some(&'*'));

    set.shift_insert(10, 'a');
    assert_eq!(set.get(&'a'), Some(&'a'));
    assert_eq!(set.get(&'*'), Some(&'*'));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get6() {
    let mut set = TypeProjectedIndexSet::from(["a", "b"]);

    assert_eq!(set.get(&"a"), Some(&"a"));
    assert_eq!(set.get(&"b"), Some(&"b"));
    assert_eq!(set.get(&"c"), None);

    set.insert("c");

    assert_eq!(set.get(&"a"), Some(&"a"));
    assert_eq!(set.get(&"b"), Some(&"b"));
    assert_eq!(set.get(&"c"), Some(&"c"));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_full1() {
    let mut set = TypeProjectedIndexSet::new();

    assert_eq!(set.get_full(&"a"), None);
    assert_eq!(set.get_full(&"b"), None);
    assert_eq!(set.get_full(&"c"), None);
    assert_eq!(set.get_full(&"d"), None);

    set.insert("a");
    set.insert("b");
    set.insert("c");

    assert_eq!(set.get_full(&"a"), Some((0, &"a")));
    assert_eq!(set.get_full(&"b"), Some((1, &"b")));
    assert_eq!(set.get_full(&"c"), Some((2, &"c")));
    assert_eq!(set.get_full(&"d"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_full2() {
    let set = TypeProjectedIndexSet::from([
        0_usize,
        1_usize,
        2_usize,
        3_usize,
        4_usize,
        5_usize,
    ]);

    assert_eq!(set.get_full(&0_usize), Some((0, &0_usize)));
    assert_eq!(set.get_full(&1_usize), Some((1, &1_usize)));
    assert_eq!(set.get_full(&2_usize), Some((2, &2_usize)));
    assert_eq!(set.get_full(&3_usize), Some((3, &3_usize)));
    assert_eq!(set.get_full(&4_usize), Some((4, &4_usize)));
    assert_eq!(set.get_full(&5_usize), Some((5, &5_usize)));
    assert_eq!(set.get_full(&6_usize), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_full3() {
    let mut set = TypeProjectedIndexSet::from(["a", "b", "c"]);

    assert_eq!(set.get_full(&"a"), Some((0, &"a")));
    assert_eq!(set.get_full(&"c"), Some((2, &"c")));
    assert_eq!(set.get_full(&"b"), Some((1, &"b")));

    set.swap_remove("b");

    assert_eq!(set.get_full(&"a"), Some((0, &"a")));
    assert_eq!(set.get_full(&"c"), Some((1, &"c")));
    assert_eq!(set.get_full(&"b"), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_full4() {
    let mut set: TypeProjectedIndexSet<char> = ('a'..='z').collect();
    assert_eq!(set.get_full(&'*'), None);

    set.insert_before(10, '*');
    assert_eq!(set.get_full(&'*'), Some((10, &'*')));

    set.insert_before(10, 'a');
    assert_eq!(set.get_full(&'a'), Some((9, &'a')));
    assert_eq!(set.get_full(&'*'), Some((10, &'*')));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_full5() {
    let mut set: TypeProjectedIndexSet<char> = ('a'..='z').collect();
    assert_eq!(set.get_full(&'*'), None);

    set.shift_insert(10, '*');
    assert_eq!(set.get_full(&'*'), Some((10, &'*', )));

    set.shift_insert(10, 'a');
    assert_eq!(set.get_full(&'a'), Some((10, &'a')));
    assert_eq!(set.get_full(&'*'), Some((9, &'*')));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_full6() {
    let mut set = TypeProjectedIndexSet::from(["a", "b"]);

    assert_eq!(set.get_full(&"a"), Some((0, &"a")));
    assert_eq!(set.get_full(&"b"), Some((1, &"b")));
    assert_eq!(set.get_full(&"c"), None);

    set.insert("c");

    assert_eq!(set.get_full(&"a"), Some((0, &"a")));
    assert_eq!(set.get_full(&"b"), Some((1, &"b")));
    assert_eq!(set.get_full(&"c"), Some((2, &"c")));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_index1() {
    let mut set = TypeProjectedIndexSet::new();

    assert_eq!(set.get_index(0), None);
    assert_eq!(set.get_index(1), None);
    assert_eq!(set.get_index(2), None);
    assert_eq!(set.get_index(3), None);

    set.insert("a");
    set.insert("b");
    set.insert("c");

    assert_eq!(set.get_index(0), Some(&"a"));
    assert_eq!(set.get_index(1), Some(&"b"));
    assert_eq!(set.get_index(2), Some(&"c"));
    assert_eq!(set.get_index(3), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_index2() {
    let set = TypeProjectedIndexSet::from([
        0_usize,
        1_usize,
        2_usize,
        3_usize,
        4_usize,
        5_usize,
    ]);

    assert_eq!(set.get_index(0), Some(&0_usize));
    assert_eq!(set.get_index(1), Some(&1_usize));
    assert_eq!(set.get_index(2), Some(&2_usize));
    assert_eq!(set.get_index(3), Some(&3_usize));
    assert_eq!(set.get_index(4), Some(&4_usize));
    assert_eq!(set.get_index(5), Some(&5_usize));
    assert_eq!(set.get_index(6), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_index3() {
    let mut set = TypeProjectedIndexSet::from(["a", "b", "c"]);

    assert_eq!(set.get_index(0), Some(&"a"));
    assert_eq!(set.get_index(2), Some(&"c"));
    assert_eq!(set.get_index(1), Some(&"b"));

    set.swap_remove("b");

    assert_eq!(set.get_index(0), Some(&"a"));
    assert_eq!(set.get_index(2), None);
    assert_eq!(set.get_index(1), Some(&"c"));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_index4() {
    let mut set: TypeProjectedIndexSet<char> = ('a'..='z').collect();
    assert_eq!(set.get_index(10), Some(&'k'));

    set.insert_before(10, '*');
    assert_eq!(set.get_index(10), Some(&'*'));

    set.insert_before(10, 'a');
    assert_eq!(set.get_index(10), Some(&'*'));
    assert_eq!(set.get_index(9), Some(&'a'));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_index5() {
    let mut set: TypeProjectedIndexSet<char> = ('a'..='z').collect();
    assert_eq!(set.get_index(10), Some(&'k'));

    set.shift_insert(10, '*');
    assert_eq!(set.get_index(10), Some(&'*'));

    set.shift_insert(10, 'a');
    assert_eq!(set.get_index(0),  Some(&'b'));
    assert_eq!(set.get_index(10), Some(&'a'));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_get_index6() {
    let mut set = TypeProjectedIndexSet::from(["a", "b"]);

    assert_eq!(set.get_index(0), Some(&"a"));
    assert_eq!(set.get_index(1), Some(&"b"));
    assert_eq!(set.get_index(2), None);

    set.insert("c");

    assert_eq!(set.get_index(0), Some(&"a"));
    assert_eq!(set.get_index(1), Some(&"b"));
    assert_eq!(set.get_index(2), Some(&"c"));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_iter1() {
    let set = TypeProjectedIndexSet::from([10_i32, 24_i32, 58_i32]);
    let expected = TypeProjectedVec::from([10_i32, 24_i32, 58_i32]);
    let result: TypeProjectedVec<i32> = set.iter().cloned().collect();

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_iter2() {
    let set: TypeProjectedIndexSet<i32> = TypeProjectedIndexSet::new();
    let expected = TypeProjectedVec::new();
    let result: TypeProjectedVec<i32> = set.iter().cloned().collect();

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_iter3() {
    let set = TypeProjectedIndexSet::from([
        (1_usize, 10_i32),
        (2_usize, 24_i32),
        (3_usize, 58_i32),
    ]);
    let mut iter = set.iter();

    assert_eq!(iter.next(), Some(&(1_usize, 10_i32)));
    assert_eq!(iter.next(), Some(&(2_usize, 24_i32)));
    assert_eq!(iter.next(), Some(&(3_usize, 58_i32)));
    assert_eq!(iter.next(), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_into_iter1() {
    let set = TypeProjectedIndexSet::from([
        89_usize,
        40_usize,
        80_usize,
        39_usize,
        62_usize,
        81_usize,
    ]);

    for value in set.clone().into_iter() {
        assert!(set.contains(&value));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_into_iter2() {
    let set = TypeProjectedIndexSet::from([
        89_usize,
        40_usize,
        80_usize,
        39_usize,
        62_usize,
        81_usize,
    ]);

    for value in set.clone().into_iter() {
        let expected = Some(&value);
        let result = set.get(&value);

        assert_eq!(result, expected);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_into_iter3() {
    let set = TypeProjectedIndexSet::from([
        89_usize,
        40_usize,
        80_usize,
        39_usize,
        62_usize,
        81_usize,
    ]);
    let expected = TypeProjectedVec::from([
        89_usize,
        40_usize,
        80_usize,
        39_usize,
        62_usize,
        81_usize,
    ]);
    let result: TypeProjectedVec<usize> = set
        .iter()
        .cloned()
        .collect();

    assert_eq!(result, expected);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_into_iter4() {
    let set = TypeProjectedIndexSet::from([
        89_usize,
        40_usize,
        80_usize,
        39_usize,
        62_usize,
        81_usize,
    ]);
    let mut iter = set.into_iter();

    assert_eq!(iter.next(), Some(89_usize));
    assert_eq!(iter.next(), Some(40_usize));
    assert_eq!(iter.next(), Some(80_usize));
    assert_eq!(iter.next(), Some(39_usize));
    assert_eq!(iter.next(), Some(62_usize));
    assert_eq!(iter.next(), Some(81_usize));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_into_iter5() {
    let set = TypeProjectedIndexSet::from([
        89_usize,
        40_usize,
        80_usize,
        39_usize,
        62_usize,
        81_usize,
    ]);
    let mut iter = set.into_iter();

    assert_eq!(iter.len(), 6);
    assert_eq!(iter.as_slice(), &[89_usize, 40_usize, 80_usize, 39_usize, 62_usize, 81_usize]);

    let _ = iter.next();
    assert_eq!(iter.len(), 5);
    assert_eq!(iter.as_slice(), &[40_usize, 80_usize, 39_usize, 62_usize, 81_usize]);

    let _ = iter.next();
    assert_eq!(iter.len(), 4);
    assert_eq!(iter.as_slice(), &[80_usize, 39_usize, 62_usize, 81_usize]);

    let _ = iter.next();
    assert_eq!(iter.len(), 3);
    assert_eq!(iter.as_slice(), &[39_usize, 62_usize, 81_usize]);

    let _ = iter.next();
    assert_eq!(iter.len(), 2);
    assert_eq!(iter.as_slice(), &[62_usize, 81_usize]);

    let _ = iter.next();
    assert_eq!(iter.len(), 1);
    assert_eq!(iter.as_slice(), &[81_usize]);

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
fn test_type_projected_index_set_into_iter6() {
    let set: TypeProjectedIndexSet<usize> = TypeProjectedIndexSet::new();
    let mut iter = set.into_iter();

    for _ in 0..65536 {
        assert!(iter.next().is_none());
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_into_iter7() {
    let set: TypeProjectedIndexSet<usize> = TypeProjectedIndexSet::new();
    let mut iter = set.into_iter();

    for _ in 0..65536 {
        let _ = iter.next().is_none();
        assert_eq!(iter.len(), 0);
        assert!(iter.as_slice().is_empty());
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_clear1() {
    let mut set: TypeProjectedIndexSet<i32> = TypeProjectedIndexSet::new();

    assert!(set.is_empty());
    assert_eq!(set.len(), 0);

    set.clear();

    assert!(set.is_empty());
    assert_eq!(set.len(), 0);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_clear2() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert!(!set.is_empty());
    assert_eq!(set.len(), 6);

    set.clear();

    assert!(set.is_empty());
    assert_eq!(set.len(), 0);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_clear3() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert!(set.contains(&(1_usize, 20_i32)));
    assert!(set.contains(&(2_usize, 2043_i32)));
    assert!(set.contains(&(3_usize, 4904_i32)));
    assert!(set.contains(&(4_usize, 20994_i32)));
    assert!(set.contains(&(5_usize, 302_i32)));
    assert!(set.contains(&(6_usize, 5_i32)));

    set.clear();

    assert!(!set.contains(&(1_usize, 20_i32)));
    assert!(!set.contains(&(2_usize, 2043_i32)));
    assert!(!set.contains(&(3_usize, 4904_i32)));
    assert!(!set.contains(&(4_usize, 20994_i32)));
    assert!(!set.contains(&(5_usize, 302_i32)));
    assert!(!set.contains(&(6_usize, 5_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_split_off1() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let expected1 = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);
    let expected2 = TypeProjectedIndexSet::from([
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let result2 = set.split_off(3);
    let result1 = set.clone();

    assert_eq!(result1.len(), expected1.len());
    assert_eq!(result2.len(), expected2.len());
    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);

}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_split_off2() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let expected1 = set.clone();
    let expected2 = TypeProjectedIndexSet::new();
    let result2 = set.split_off(set.len());
    let result1 = set.clone();

    assert_eq!(result1.len(), expected1.len());
    assert_eq!(result2.len(), expected2.len());
    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_split_off3() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let expected1 = TypeProjectedIndexSet::new();
    let expected2 = set.clone();
    let result2 = set.split_off(0);
    let result1 = set.clone();

    assert_eq!(result1.len(), expected1.len());
    assert_eq!(result2.len(), expected2.len());
    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);
}

#[rustfmt::skip]
#[test]
#[should_panic]
fn test_type_projected_index_set_split_off_out_of_bounds1() {
    let mut set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::new();
    let _ = set.split_off(set.len() + 1);

    assert!(true);
}

#[rustfmt::skip]
#[test]
#[should_panic]
fn test_type_projected_index_set_split_off_out_of_bounds2() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);
    let _ = set.split_off(set.len() + 1);

    assert!(true);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_remove1() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert!(set.swap_remove(&(1_usize, 20_i32)));
    assert!(set.swap_remove(&(2_usize, 2043_i32)));
    assert!(set.swap_remove(&(3_usize, 4904_i32)));
    assert!(set.swap_remove(&(4_usize, 20994_i32)));
    assert!(set.swap_remove(&(5_usize, 302_i32)));
    assert!(set.swap_remove(&(6_usize, 5_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_remove2() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(set.len(), 6);

    set.swap_remove(&(1_usize, 20_i32));
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    set.swap_remove(&(2_usize, 2043_i32));
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    set.swap_remove(&(3_usize, 4904_i32));
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (4_usize, 20994_i32),
    ]);

    set.swap_remove(&(4_usize, 20994_i32));
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
    ]);

    set.swap_remove(&(5_usize, 302_i32));
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
    ]);

    set.swap_remove(&(6_usize, 5_i32));
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_remove3() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(set.len(), 6);

    set.swap_remove(&(6_usize, 5_i32));
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    set.swap_remove(&(5_usize, 302_i32));
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    set.swap_remove(&(4_usize, 20994_i32));
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);

    set.swap_remove(&(3_usize, 4904_i32));
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
    ]);

    set.swap_remove(&(2_usize, 2043_i32));
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
    ]);

    set.swap_remove(&(1_usize, 20_i32));
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_remove4() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert!(set.swap_remove(&(6_usize, 5_i32)));
    assert!(set.swap_remove(&(5_usize, 302_i32)));
    assert!(set.swap_remove(&(4_usize, 20994_i32)));
    assert!(set.swap_remove(&(3_usize, 4904_i32)));
    assert!(set.swap_remove(&(2_usize, 2043_i32)));
    assert!(set.swap_remove(&(1_usize, 20_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_remove_full1() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(set.swap_remove_full(&(1_usize, 20_i32)),    Some((0, (1_usize, 20_i32))));
    assert_eq!(set.swap_remove_full(&(2_usize, 2043_i32)),  Some((1, (2_usize, 2043_i32))));
    assert_eq!(set.swap_remove_full(&(3_usize, 4904_i32)),  Some((2, (3_usize, 4904_i32))));
    assert_eq!(set.swap_remove_full(&(4_usize, 20994_i32)), Some((2, (4_usize, 20994_i32))));
    assert_eq!(set.swap_remove_full(&(5_usize, 302_i32)),   Some((1, (5_usize, 302_i32))));
    assert_eq!(set.swap_remove_full(&(6_usize, 5_i32)),     Some((0, (6_usize, 5_i32))));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_remove_full2() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(set.len(), 6);

    let _ = set.swap_remove_full(&(1_usize, 20_i32));
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    let _ = set.swap_remove_full(&(2_usize, 2043_i32));
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = set.swap_remove_full(&(3_usize, 4904_i32));
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = set.swap_remove_full(&(4_usize, 20994_i32));
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
    ]);

    let _ = set.swap_remove_full(&(5_usize, 302_i32));
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
    ]);

    let _ = set.swap_remove_full(&(6_usize, 5_i32));
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_remove_full3() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(set.len(), 6);

    let _ = set.swap_remove_full(&(6_usize, 5_i32));
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    let _ = set.swap_remove_full(&(5_usize, 302_i32));
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = set.swap_remove_full(&(4_usize, 20994_i32));
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);

    let _ = set.swap_remove_full(&(3_usize, 4904_i32));
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
    ]);

    let _ = set.swap_remove_full(&(2_usize, 2043_i32));
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
    ]);

    let _ = set.swap_remove_full(&(1_usize, 20_i32));
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_remove_full4() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(set.swap_remove_full(&(6_usize, 5_i32)),     Some((5, (6_usize, 5_i32))));
    assert_eq!(set.swap_remove_full(&(5_usize, 302_i32)),   Some((4, (5_usize, 302_i32))));
    assert_eq!(set.swap_remove_full(&(4_usize, 20994_i32)), Some((3, (4_usize, 20994_i32))));
    assert_eq!(set.swap_remove_full(&(3_usize, 4904_i32)),  Some((2, (3_usize, 4904_i32))));
    assert_eq!(set.swap_remove_full(&(2_usize, 2043_i32)),  Some((1, (2_usize, 2043_i32))));
    assert_eq!(set.swap_remove_full(&(1_usize, 20_i32)),    Some((0, (1_usize, 20_i32))));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_remove_index1() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(set.swap_remove_index(0), Some((1_usize, 20_i32)));
    assert_eq!(set.swap_remove_index(1), Some((2_usize, 2043_i32)));
    assert_eq!(set.swap_remove_index(2), Some((3_usize, 4904_i32)));
    assert_eq!(set.swap_remove_index(2), Some((4_usize, 20994_i32)));
    assert_eq!(set.swap_remove_index(1), Some((5_usize, 302_i32)));
    assert_eq!(set.swap_remove_index(0), Some((6_usize, 5_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_remove_index2() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(set.len(), 6);

    let _ = set.swap_remove_index(0);
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    let _ = set.swap_remove_index(1);
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = set.swap_remove_index(2);
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = set.swap_remove_index(2);
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
        (5_usize, 302_i32),
    ]);

    let _ = set.swap_remove_index(1);
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (6_usize, 5_i32),
    ]);

    let _ = set.swap_remove_index(0);
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_remove_index3() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(set.len(), 6);

    let _ = set.swap_remove_index(5);
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
    ]);

    let _ = set.swap_remove_index(4);
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
    ]);

    let _ = set.swap_remove_index(3);
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
    ]);

    let _ = set.swap_remove_index(2);
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
    ]);

    let _ = set.swap_remove_index(1);
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (1_usize, 20_i32),
    ]);

    let _ = set.swap_remove_index(0);
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_remove_index4() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    assert_eq!(set.swap_remove_index(5), Some((6_usize, 5_i32)));
    assert_eq!(set.swap_remove_index(4), Some((5_usize, 302_i32)));
    assert_eq!(set.swap_remove_index(3), Some((4_usize, 20994_i32)));
    assert_eq!(set.swap_remove_index(2), Some((3_usize, 4904_i32)));
    assert_eq!(set.swap_remove_index(1), Some((2_usize, 2043_i32)));
    assert_eq!(set.swap_remove_index(0), Some((1_usize, 20_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_insert_set_swap_remove_index_out_of_bounds1() {
    let mut set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::new();

    for i in 0..65536 {
        assert_eq!(set.swap_remove_index(i), None);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_insert_set_swap_remove_index_out_of_bounds2() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    for i in set.len()..65536 {
        assert_eq!(set.swap_remove_index(i), None);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_remove1() {
    let mut set = TypeProjectedIndexSet::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert!(set.shift_remove(&(1655_usize, 2427_i32)));
    assert!(set.shift_remove(&(1992_usize, 2910_i32)));
    assert!(set.shift_remove(&(783_usize,  603_i32)));
    assert!(set.shift_remove(&(376_usize,  834_i32)));
    assert!(set.shift_remove(&(199_usize,  1881_i32)));
    assert!(set.shift_remove(&(1098_usize, 1466_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_remove2() {
    let mut set = TypeProjectedIndexSet::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(set.len(), 6);

    set.shift_remove(&(1655_usize, 2427_i32));
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    set.shift_remove(&(1992_usize, 2910_i32));
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    set.shift_remove(&(783_usize,  603_i32));
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    set.shift_remove(&(376_usize,  834_i32));
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    set.shift_remove(&(199_usize,  1881_i32));
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (1098_usize, 1466_i32),
    ]);

    set.shift_remove(&(1098_usize, 1466_i32));
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_remove3() {
    let mut set = TypeProjectedIndexSet::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(set.len(), 6);

    set.shift_remove(&(1098_usize, 1466_i32));
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
    ]);

    set.shift_remove(&(199_usize,  1881_i32));
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
    ]);

    set.shift_remove(&(376_usize,  834_i32));
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
    ]);

    set.shift_remove(&(783_usize,  603_i32));
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
    ]);

    set.shift_remove(&(1992_usize, 2910_i32));
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
    ]);

    set.shift_remove(&(1655_usize, 2427_i32));
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_remove4() {
    let mut set = TypeProjectedIndexSet::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert!(set.shift_remove(&(1098_usize, 1466_i32)));
    assert!(set.shift_remove(&(199_usize,  1881_i32)));
    assert!(set.shift_remove(&(376_usize,  834_i32)));
    assert!(set.shift_remove(&(783_usize,  603_i32)));
    assert!(set.shift_remove(&(1992_usize, 2910_i32)));
    assert!(set.shift_remove(&(1655_usize, 2427_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_remove_full1() {
    let mut set = TypeProjectedIndexSet::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(set.shift_remove_full(&(1655_usize, 2427_i32)), Some((0, (1655_usize, 2427_i32))));
    assert_eq!(set.shift_remove_full(&(1992_usize, 2910_i32)), Some((0, (1992_usize, 2910_i32))));
    assert_eq!(set.shift_remove_full(&(783_usize,  603_i32)),  Some((0, (783_usize,  603_i32))));
    assert_eq!(set.shift_remove_full(&(376_usize,  834_i32)),  Some((0, (376_usize,  834_i32))));
    assert_eq!(set.shift_remove_full(&(199_usize,  1881_i32)), Some((0, (199_usize,  1881_i32))));
    assert_eq!(set.shift_remove_full(&(1098_usize, 1466_i32)), Some((0, (1098_usize, 1466_i32))));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_remove_full2() {
    let mut set = TypeProjectedIndexSet::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(set.len(), 6);

    let _ = set.shift_remove_full(&(1655_usize, 2427_i32));
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = set.shift_remove_full(&(1992_usize, 2910_i32));
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = set.shift_remove_full(&(783_usize,  603_i32));
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = set.shift_remove_full(&(376_usize,  834_i32));
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = set.shift_remove_full(&(199_usize,  1881_i32));
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (1098_usize, 1466_i32),
    ]);

    let _ = set.shift_remove_full(&(1098_usize, 1466_i32));
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_remove_full3() {
    let mut set = TypeProjectedIndexSet::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(set.len(), 6);

    let _ = set.shift_remove_full(&(1098_usize, 1466_i32));
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
    ]);

    let _ = set.shift_remove_full(&(199_usize,  1881_i32));
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
    ]);

    let _ = set.shift_remove_full(&(376_usize,  834_i32));
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
    ]);

    let _ = set.shift_remove_full(&(783_usize,  603_i32));
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
    ]);

    let _ = set.shift_remove_full(&(1992_usize, 2910_i32));
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
    ]);

    let _ = set.shift_remove_full(&(1655_usize, 2427_i32));
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_remove_full4() {
    let mut set = TypeProjectedIndexSet::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(set.shift_remove_full(&(1098_usize, 1466_i32)), Some((5, (1098_usize, 1466_i32))));
    assert_eq!(set.shift_remove_full(&(199_usize,  1881_i32)), Some((4, (199_usize, 1881_i32))));
    assert_eq!(set.shift_remove_full(&(376_usize,  834_i32)),  Some((3, (376_usize, 834_i32))));
    assert_eq!(set.shift_remove_full(&(783_usize,  603_i32)),  Some((2, (783_usize, 603_i32))));
    assert_eq!(set.shift_remove_full(&(1992_usize, 2910_i32)), Some((1, (1992_usize, 2910_i32))));
    assert_eq!(set.shift_remove_full(&(1655_usize, 2427_i32)), Some((0, (1655_usize, 2427_i32))));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_remove_index1() {
    let mut set = TypeProjectedIndexSet::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(set.shift_remove_index(0), Some((1655_usize, 2427_i32)));
    assert_eq!(set.shift_remove_index(0), Some((1992_usize, 2910_i32)));
    assert_eq!(set.shift_remove_index(0), Some((783_usize,  603_i32)));
    assert_eq!(set.shift_remove_index(0), Some((376_usize,  834_i32)));
    assert_eq!(set.shift_remove_index(0), Some((199_usize,  1881_i32)));
    assert_eq!(set.shift_remove_index(0), Some((1098_usize, 1466_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_remove_index2() {
    let mut set = TypeProjectedIndexSet::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(set.len(), 6);

    let _ = set.shift_remove_index(0);
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = set.shift_remove_index(0);
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = set.shift_remove_index(0);
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = set.shift_remove_index(0);
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (199_usize, 1881_i32),
        (1098_usize, 1466_i32),
    ]);

    let _ = set.shift_remove_index(0);
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (1098_usize, 1466_i32),
    ]);

    let _ = set.shift_remove_index(0);
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_remove_index3() {
    let mut set = TypeProjectedIndexSet::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(set.len(), 6);

    let _ = set.shift_remove_index(5);
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
    ]);

    let _ = set.shift_remove_index(4);
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
    ]);

    let _ = set.shift_remove_index(3);
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
    ]);

    let _ = set.shift_remove_index(2);
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
    ]);

    let _ = set.shift_remove_index(1);
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (1655_usize, 2427_i32),
    ]);

    let _ = set.shift_remove_index(0);
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_remove_index4() {
    let mut set = TypeProjectedIndexSet::from([
        (1655_usize, 2427_i32),
        (1992_usize, 2910_i32),
        (783_usize,  603_i32),
        (376_usize,  834_i32),
        (199_usize,  1881_i32),
        (1098_usize, 1466_i32),
    ]);

    assert_eq!(set.shift_remove_index(5), Some((1098_usize, 1466_i32)));
    assert_eq!(set.shift_remove_index(4), Some((199_usize, 1881_i32)));
    assert_eq!(set.shift_remove_index(3), Some((376_usize, 834_i32)));
    assert_eq!(set.shift_remove_index(2), Some((783_usize, 603_i32)));
    assert_eq!(set.shift_remove_index(1), Some((1992_usize, 2910_i32)));
    assert_eq!(set.shift_remove_index(0), Some((1655_usize, 2427_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_insert_set_shift_remove_index_out_of_bounds1() {
    let mut set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::new();

    for i in 0..65536 {
        assert_eq!(set.shift_remove_index(i), None);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_insert_set_shift_remove_index_out_of_bounds2() {
    let mut set = TypeProjectedIndexSet::from([
        (1_usize, 20_i32),
        (2_usize, 2043_i32),
        (3_usize, 4904_i32),
        (4_usize, 20994_i32),
        (5_usize, 302_i32),
        (6_usize, 5_i32),
    ]);

    for i in set.len()..65536 {
        assert_eq!(set.shift_remove_index(i), None);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_take1() {
    let mut set = TypeProjectedIndexSet::from([
        20_i32,
        21_i32,
        65_i32,
        6_i32,
        86_i32,
        54_i32,
        99_i32,
        17_i32,
    ]);

    assert_eq!(set.swap_take(&20_i32), Some(20_i32));
    assert_eq!(set.swap_take(&21_i32), Some(21_i32));
    assert_eq!(set.swap_take(&65_i32), Some(65_i32));
    assert_eq!(set.swap_take(&6_i32),  Some(6_i32));
    assert_eq!(set.swap_take(&86_i32), Some(86_i32));
    assert_eq!(set.swap_take(&54_i32), Some(54_i32));
    assert_eq!(set.swap_take(&99_i32), Some(99_i32));
    assert_eq!(set.swap_take(&17_i32), Some(17_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_take2() {
    let mut set = TypeProjectedIndexSet::from([
        20_i32,
        21_i32,
        65_i32,
        6_i32,
        86_i32,
        54_i32,
        99_i32,
        17_i32,
    ]);

    assert_eq!(set.len(), 8);

    let _ = set.swap_take(&20_i32);
    assert_eq!(set.len(), 7);
    assert_eq!(set.as_slice(), &[17_i32, 21_i32, 65_i32, 6_i32, 86_i32, 54_i32, 99_i32]);

    let _ = set.swap_take(&21_i32);
    assert_eq!(set.len(), 6);
    assert_eq!(set.as_slice(), &[17_i32, 99_i32, 65_i32, 6_i32, 86_i32, 54_i32]);

    let _ = set.swap_take(&65_i32);
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[17_i32, 99_i32, 54_i32, 6_i32, 86_i32]);

    let _ = set.swap_take(&6_i32);
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[17_i32, 99_i32, 54_i32, 86_i32]);

    let _ = set.swap_take(&86_i32);
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[17_i32, 99_i32, 54_i32]);

    let _ = set.swap_take(&54_i32);
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[17_i32, 99_i32]);

    let _ = set.swap_take(&99_i32);
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[17_i32]);

    let _ = set.swap_take(&17_i32);
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_take3() {
    let mut set = TypeProjectedIndexSet::from([
        20_i32,
        21_i32,
        65_i32,
        6_i32,
        86_i32,
        54_i32,
        99_i32,
        17_i32,
    ]);

    assert_eq!(set.swap_take(&17_i32), Some(17_i32));
    assert_eq!(set.swap_take(&99_i32), Some(99_i32));
    assert_eq!(set.swap_take(&54_i32), Some(54_i32));
    assert_eq!(set.swap_take(&86_i32), Some(86_i32));
    assert_eq!(set.swap_take(&6_i32),  Some(6_i32));
    assert_eq!(set.swap_take(&65_i32), Some(65_i32));
    assert_eq!(set.swap_take(&21_i32), Some(21_i32));
    assert_eq!(set.swap_take(&20_i32), Some(20_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_swap_take4() {
    let mut set = TypeProjectedIndexSet::from([
        20_i32,
        21_i32,
        65_i32,
        6_i32,
        86_i32,
        54_i32,
        99_i32,
        17_i32,
    ]);

    assert_eq!(set.len(), 8);

    let _ = set.swap_take(&17_i32);
    assert_eq!(set.len(), 7);
    assert_eq!(set.as_slice(), &[20_i32, 21_i32, 65_i32, 6_i32, 86_i32, 54_i32, 99_i32]);

    let _ = set.swap_take(&99_i32);
    assert_eq!(set.len(), 6);
    assert_eq!(set.as_slice(), &[20_i32, 21_i32, 65_i32, 6_i32, 86_i32, 54_i32]);

    let _ = set.swap_take(&54_i32);
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[20_i32, 21_i32, 65_i32, 6_i32, 86_i32]);

    let _ = set.swap_take(&86_i32);
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[20_i32, 21_i32, 65_i32, 6_i32]);

    let _ = set.swap_take(&6_i32);
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[20_i32, 21_i32, 65_i32]);

    let _ = set.swap_take(&65_i32);
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[20_i32, 21_i32]);

    let _ = set.swap_take(&21_i32);
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[20_i32]);

    let _ = set.swap_take(&20_i32);
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_take1() {
    let mut set = TypeProjectedIndexSet::from([
        20_i32,
        21_i32,
        65_i32,
        6_i32,
        86_i32,
        54_i32,
        99_i32,
        17_i32,
    ]);

    assert_eq!(set.shift_take(&20_i32), Some(20_i32));
    assert_eq!(set.shift_take(&21_i32), Some(21_i32));
    assert_eq!(set.shift_take(&65_i32), Some(65_i32));
    assert_eq!(set.shift_take(&6_i32),  Some(6_i32));
    assert_eq!(set.shift_take(&86_i32), Some(86_i32));
    assert_eq!(set.shift_take(&54_i32), Some(54_i32));
    assert_eq!(set.shift_take(&99_i32), Some(99_i32));
    assert_eq!(set.shift_take(&17_i32), Some(17_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_take2() {
    let mut set = TypeProjectedIndexSet::from([
        20_i32,
        21_i32,
        65_i32,
        6_i32,
        86_i32,
        54_i32,
        99_i32,
        17_i32,
    ]);

    assert_eq!(set.len(), 8);

    let _ = set.shift_take(&20_i32);
    assert_eq!(set.len(), 7);
    assert_eq!(set.as_slice(), &[21_i32, 65_i32, 6_i32, 86_i32, 54_i32, 99_i32, 17_i32]);

    let _ = set.shift_take(&21_i32);
    assert_eq!(set.len(), 6);
    assert_eq!(set.as_slice(), &[65_i32, 6_i32, 86_i32, 54_i32, 99_i32, 17_i32]);

    let _ = set.shift_take(&65_i32);
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[6_i32, 86_i32, 54_i32, 99_i32, 17_i32]);

    let _ = set.shift_take(&6_i32);
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[86_i32, 54_i32, 99_i32, 17_i32]);

    let _ = set.shift_take(&86_i32);
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[54_i32, 99_i32, 17_i32]);

    let _ = set.shift_take(&54_i32);
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[99_i32, 17_i32]);

    let _ = set.shift_take(&99_i32);
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[17_i32]);

    let _ = set.shift_take(&17_i32);
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_take3() {
    let mut set = TypeProjectedIndexSet::from([
        20_i32,
        21_i32,
        65_i32,
        6_i32,
        86_i32,
        54_i32,
        99_i32,
        17_i32,
    ]);

    assert_eq!(set.shift_take(&17_i32), Some(17_i32));
    assert_eq!(set.shift_take(&99_i32), Some(99_i32));
    assert_eq!(set.shift_take(&54_i32), Some(54_i32));
    assert_eq!(set.shift_take(&86_i32), Some(86_i32));
    assert_eq!(set.shift_take(&6_i32),  Some(6_i32));
    assert_eq!(set.shift_take(&65_i32), Some(65_i32));
    assert_eq!(set.shift_take(&21_i32), Some(21_i32));
    assert_eq!(set.shift_take(&20_i32), Some(20_i32));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_take4() {
    let mut set = TypeProjectedIndexSet::from([
        20_i32,
        21_i32,
        65_i32,
        6_i32,
        86_i32,
        54_i32,
        99_i32,
        17_i32,
    ]);

    assert_eq!(set.len(), 8);

    let _ = set.shift_take(&17_i32);
    assert_eq!(set.len(), 7);
    assert_eq!(set.as_slice(), &[20_i32, 21_i32, 65_i32, 6_i32, 86_i32, 54_i32, 99_i32]);

    let _ = set.shift_take(&99_i32);
    assert_eq!(set.len(), 6);
    assert_eq!(set.as_slice(), &[20_i32, 21_i32, 65_i32, 6_i32, 86_i32, 54_i32]);

    let _ = set.shift_take(&54_i32);
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[20_i32, 21_i32, 65_i32, 6_i32, 86_i32]);

    let _ = set.shift_take(&86_i32);
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[20_i32, 21_i32, 65_i32, 6_i32]);

    let _ = set.shift_take(&6_i32);
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[20_i32, 21_i32, 65_i32]);

    let _ = set.shift_take(&65_i32);
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[20_i32, 21_i32]);

    let _ = set.shift_take(&21_i32);
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[20_i32]);

    let _ = set.shift_take(&20_i32);
    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), &[]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_insert1() {
    let mut set = TypeProjectedIndexSet::new();

    assert!(set.insert((1803_usize, 1778_i32)));
    assert!(set.insert((1057_usize, 2437_i32)));
    assert!(set.insert((1924_usize, 185_i32)));
    assert!(set.insert((302_usize,  2457_i32)));
    assert!(set.insert((949_usize,  2176_i32)));
    assert!(set.insert((2968_usize, 1398_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_insert2() {
    let mut set = TypeProjectedIndexSet::new();

    assert!(set.is_empty());
    assert_eq!(set.len(), 0);

    set.insert((1803_usize, 1778_i32));
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (1803_usize, 1778_i32),
    ]);

    set.insert((1057_usize, 2437_i32));
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
    ]);

    set.insert((1924_usize, 185_i32));
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
    ]);

    set.insert((302_usize, 2457_i32));
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize,  2457_i32),
    ]);

    set.insert((949_usize, 2176_i32));
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize,  2457_i32),
        (949_usize,  2176_i32),
    ]);

    set.insert((2968_usize, 1398_i32));
    assert_eq!(set.len(), 6);
    assert_eq!(set.as_slice(), &[
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
fn test_type_projected_index_set_insert_full1() {
    let mut set = TypeProjectedIndexSet::new();

    assert_eq!(set.insert_full((1803_usize, 1778_i32)), (0, true));
    assert_eq!(set.insert_full((1057_usize, 2437_i32)), (1, true));
    assert_eq!(set.insert_full((1924_usize, 185_i32)),  (2, true));
    assert_eq!(set.insert_full((302_usize,  2457_i32)), (3, true));
    assert_eq!(set.insert_full((949_usize,  2176_i32)), (4, true));
    assert_eq!(set.insert_full((2968_usize, 1398_i32)), (5, true));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_insert_full2() {
    let mut set = TypeProjectedIndexSet::new();

    assert!(set.is_empty());
    assert_eq!(set.len(), 0);

    let _ = set.insert_full((1803_usize, 1778_i32));
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (1803_usize, 1778_i32),
    ]);

    let _ = set.insert_full((1057_usize, 2437_i32));
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
    ]);

    let _ = set.insert_full((1924_usize, 185_i32));
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
    ]);

    let _ = set.insert_full((302_usize, 2457_i32));
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize,  2457_i32),
    ]);

    let _ = set.insert_full((949_usize, 2176_i32));
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (1803_usize, 1778_i32),
        (1057_usize, 2437_i32),
        (1924_usize, 185_i32),
        (302_usize,  2457_i32),
        (949_usize,  2176_i32),
    ]);

    let _ = set.insert_full((2968_usize, 1398_i32));
    assert_eq!(set.len(), 6);
    assert_eq!(set.as_slice(), &[
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
fn test_type_projected_index_set_insert_before1() {
    let mut set = TypeProjectedIndexSet::new();

    assert_eq!(set.insert_before(0, 2339_i32), (0, true));
    assert_eq!(set.insert_before(0, 2387_i32), (0, true));
    assert_eq!(set.insert_before(0, 2741_i32), (0, true));
    assert_eq!(set.insert_before(0, 1838_i32), (0, true));
    assert_eq!(set.insert_before(0, 464_i32),  (0, true));
    assert_eq!(set.insert_before(0, 509_i32),  (0, true));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_insert_before2() {
    let mut set = TypeProjectedIndexSet::new();

    assert_eq!(set.len(), 0);

    let _ = set.insert_before(0, 2339_i32);
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[2339_i32]);

    let _ = set.insert_before(0, 2387_i32);
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[2387_i32, 2339_i32]);

    let _ = set.insert_before(0, 2741_i32);
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[2741_i32, 2387_i32, 2339_i32]);

    let _ = set.insert_before(0, 1838_i32);
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[1838_i32, 2741_i32, 2387_i32, 2339_i32]);

    let _ = set.insert_before(0, 464_i32);
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[464_i32, 1838_i32, 2741_i32, 2387_i32, 2339_i32]);

    let _ = set.insert_before(0, 509_i32);
    assert_eq!(set.len(), 6);
    assert_eq!(set.as_slice(), &[509_i32, 464_i32, 1838_i32, 2741_i32, 2387_i32, 2339_i32]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_insert_before3() {
    let mut set = TypeProjectedIndexSet::from([
        509_i32,
        464_i32,
        1838_i32,
        2741_i32,
        2387_i32,
        2339_i32,
    ]);

    assert_eq!(set.len(), 6);

    let result = set.insert_before(4, 509_i32);
    assert_eq!(result, (3, false));
    assert_eq!(set.len(), 6);
    assert_eq!(set.as_slice(), &[464_i32, 1838_i32, 2741_i32, 509_i32, 2387_i32, 2339_i32]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_insert_before4() {
    let mut set = TypeProjectedIndexSet::from([
        509_i32,
        464_i32,
        1838_i32,
        2741_i32,
        2387_i32,
        2339_i32,
    ]);

    assert_eq!(set.len(), 6);

    let result = set.insert_before(1, 2339_i32);
    assert_eq!(result, (1, false));
    assert_eq!(set.len(), 6);
    assert_eq!(set.as_slice(), &[509_i32, 2339_i32, 464_i32, 1838_i32, 2741_i32, 2387_i32]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_insert_before5() {
    let mut set = TypeProjectedIndexSet::from([
        509_i32,
        464_i32,
        1838_i32,
        2741_i32,
        2387_i32,
        2339_i32,
    ]);

    assert_eq!(set.len(), 6);

    let result = set.insert_before(3, 2741_i32);
    assert_eq!(result, (3, false));
    assert_eq!(set.len(), 6);
    assert_eq!(set.as_slice(), &[509_i32, 464_i32, 1838_i32, 2741_i32, 2387_i32, 2339_i32]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_insert_before6() {
    let mut set = TypeProjectedIndexSet::from([
        509_i32,
        464_i32,
        1838_i32,
        2741_i32,
        2387_i32,
        2339_i32,
    ]);

    assert_eq!(set.len(), 6);

    let result = set.insert_before(5, i32::MAX);
    assert_eq!(result, (5, true));
    assert_eq!(set.len(), 7);
    assert_eq!(set.as_slice(), &[509_i32, 464_i32, 1838_i32, 2741_i32, 2387_i32, i32::MAX, 2339_i32]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_insert1() {
    let mut set = TypeProjectedIndexSet::new();

    assert!(set.shift_insert(0, (1809_usize, 2381_i32)));
    assert!(set.shift_insert(0, (603_usize,  2834_i32)));
    assert!(set.shift_insert(0, (2564_usize, 621_i32)));
    assert!(set.shift_insert(0, (360_usize,  1352_i32)));
    assert!(set.shift_insert(0, (57_usize,   2657_i32)));
    assert!(set.shift_insert(0, (477_usize,  2084_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_insert2() {
    let mut set = TypeProjectedIndexSet::new();

    assert_eq!(set.len(), 0);
    assert!(set.is_empty());

    set.shift_insert(0, (1809_usize, 2381_i32));
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (1809_usize, 2381_i32),
    ]);

    set.shift_insert(0, (603_usize, 2834_i32));
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);

    set.shift_insert(0, (2564_usize, 621_i32));
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);

    set.shift_insert(0, (360_usize, 1352_i32));
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);

    set.shift_insert(0, (57_usize, 2657_i32));
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (57_usize,   2657_i32),
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
        (1809_usize, 2381_i32),
    ]);

    set.shift_insert(0, (477_usize, 2084_i32));
    assert_eq!(set.len(), 6);
    assert_eq!(set.as_slice(), &[
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
fn test_type_projected_index_set_shift_insert3() {
    let mut set = TypeProjectedIndexSet::new();

    assert!(set.shift_insert(0, (477_usize,  2084_i32)));
    assert!(set.shift_insert(1, (57_usize,   2657_i32)));
    assert!(set.shift_insert(2, (360_usize,  1352_i32)));
    assert!(set.shift_insert(3, (2564_usize, 621_i32)));
    assert!(set.shift_insert(4, (603_usize,  2834_i32)));
    assert!(set.shift_insert(5, (1809_usize, 2381_i32)));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shift_insert4() {
    let mut set = TypeProjectedIndexSet::new();

    assert_eq!(set.len(), 0);
    assert!(set.is_empty());

    set.shift_insert(0, (477_usize, 2084_i32));
    assert_eq!(set.len(), 1);
    assert_eq!(set.as_slice(), &[
        (477_usize, 2084_i32),
    ]);

    set.shift_insert(1, (57_usize, 2657_i32));
    assert_eq!(set.len(), 2);
    assert_eq!(set.as_slice(), &[
        (477_usize, 2084_i32),
        (57_usize,  2657_i32),
    ]);

    set.shift_insert(2, (360_usize, 1352_i32));
    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), &[
        (477_usize, 2084_i32),
        (57_usize,  2657_i32),
        (360_usize, 1352_i32),
    ]);

    set.shift_insert(3, (2564_usize, 621_i32));
    assert_eq!(set.len(), 4);
    assert_eq!(set.as_slice(), &[
        (477_usize,  2084_i32),
        (57_usize,   2657_i32),
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
    ]);

    set.shift_insert(4, (603_usize, 2834_i32));
    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), &[
        (477_usize,  2084_i32),
        (57_usize,   2657_i32),
        (360_usize,  1352_i32),
        (2564_usize, 621_i32),
        (603_usize,  2834_i32),
    ]);

    set.shift_insert(5, (1809_usize, 2381_i32));
    assert_eq!(set.len(), 6);
    assert_eq!(set.as_slice(), &[
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
fn test_type_projected_index_set_append1() {
    let mut set1 = TypeProjectedIndexSet::from([
        605_usize,
        1804_usize,
        1532_usize,
        1660_usize,
    ]);
    let mut set2 = TypeProjectedIndexSet::from([
        1062_usize,
        1875_usize,
        1724_usize,
    ]);
    let expected = TypeProjectedIndexSet::from([
        605_usize,
        1804_usize,
        1532_usize,
        1660_usize,
        1062_usize,
        1875_usize,
        1724_usize,
    ]);
    set1.append(&mut set2);

    assert!(set2.is_empty());
    assert_eq!(set2.len(), 0);
    assert_eq!(set1.len(), 7);
    assert_eq!(set1.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_append2() {
    let mut set1 = TypeProjectedIndexSet::from([
        605_usize,
        1804_usize,
        1532_usize,
        1660_usize,
    ]);
    let mut set2 = TypeProjectedIndexSet::from([
        1804_usize,
        1875_usize,
        1660_usize,
    ]);
    let expected = TypeProjectedIndexSet::from([
        605_usize,
        1804_usize,
        1532_usize,
        1660_usize,
        1875_usize,
    ]);
    set1.append(&mut set2);

    assert!(set2.is_empty());
    assert_eq!(set2.len(), 0);
    assert_eq!(set1.len(), 5);
    assert_eq!(set1.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_append3() {
    let mut set1 = TypeProjectedIndexSet::from([
        605_usize,
        1804_usize,
        1532_usize,
        1660_usize,
    ]);
    let mut set2 = TypeProjectedIndexSet::new();
    let expected = TypeProjectedIndexSet::from([
        605_usize,
        1804_usize,
        1532_usize,
        1660_usize,
    ]);
    set1.append(&mut set2);

    assert!(set2.is_empty());
    assert_eq!(set2.len(), 0);
    assert_eq!(set1.len(), 4);
    assert_eq!(set1.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_append4() {
    let mut set1 = TypeProjectedIndexSet::from([usize::MAX]);
    let mut set2 = TypeProjectedIndexSet::from([usize::MAX]);
    let expected = TypeProjectedIndexSet::from([usize::MAX]);
    set1.append(&mut set2);

    assert!(set2.is_empty());
    assert_eq!(set2.len(), 0);
    assert_eq!(set1.len(), 1);
    assert_eq!(set1.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_retain1() {
    let mut set = TypeProjectedIndexSet::from([
        344_usize,
        1646_usize,
        2371_usize,
        52_usize,
        789_usize,
        1205_usize,
        28_usize,
        136_usize,
    ]);
    let expected = set.clone();
    set.retain(|_v| true);

    assert_eq!(set.len(), 8);
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_retain2() {
    let mut set = TypeProjectedIndexSet::from([
        344_usize,
        1646_usize,
        2371_usize,
        52_usize,
        789_usize,
        1205_usize,
        28_usize,
        136_usize,
    ]);
    let expected = TypeProjectedIndexSet::new();
    set.retain(|_v| false);

    assert_eq!(set.len(), 0);
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_retain3() {
    let mut set = TypeProjectedIndexSet::from([
        344_usize,
        1646_usize,
        2371_usize,
        52_usize,
        789_usize,
        1205_usize,
        28_usize,
        136_usize,
    ]);
    let expected = TypeProjectedIndexSet::from([
        344_usize,
        1646_usize,
        52_usize,
        28_usize,
        136_usize,
    ]);
    set.retain(|v| v % 2 == 0);

    assert_eq!(set.len(), 5);
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_retain4() {
    let mut set = TypeProjectedIndexSet::from([
        344_usize,
        1646_usize,
        2371_usize,
        52_usize,
        789_usize,
        1205_usize,
        28_usize,
        136_usize,
    ]);
    let expected = TypeProjectedIndexSet::from([
        2371_usize,
        789_usize,
        1205_usize,
    ]);
    set.retain(|v| v % 2 != 0);

    assert_eq!(set.len(), 3);
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_sort1() {
    let mut set = TypeProjectedIndexSet::from([
        6_usize,
        7_usize,
        10_usize,
        17_usize,
        22_usize,
        23_usize,
        47_usize,
        79_usize,
        141_usize,
        176_usize,
        200_usize,
    ]);
    let expected = TypeProjectedIndexSet::from([
        6_usize,
        7_usize,
        10_usize,
        17_usize,
        22_usize,
        23_usize,
        47_usize,
        79_usize,
        141_usize,
        176_usize,
        200_usize,
    ]);
    set.sort();

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_sort2() {
    let mut set = TypeProjectedIndexSet::from([
        10_usize,
        47_usize,
        22_usize,
        17_usize,
        141_usize,
        6_usize,
        176_usize,
        23_usize,
        79_usize,
        200_usize,
        7_usize,
    ]);
    let expected = TypeProjectedIndexSet::from([
        6_usize,
        7_usize,
        10_usize,
        17_usize,
        22_usize,
        23_usize,
        47_usize,
        79_usize,
        141_usize,
        176_usize,
        200_usize,
    ]);
    set.sort();

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_sort3() {
    let mut set = TypeProjectedIndexSet::from([
        200_usize,
        176_usize,
        141_usize,
        79_usize,
        47_usize,
        23_usize,
        22_usize,
        17_usize,
        10_usize,
        7_usize,
        6_usize,
    ]);
    let expected = TypeProjectedIndexSet::from([
        6_usize,
        7_usize,
        10_usize,
        17_usize,
        22_usize,
        23_usize,
        47_usize,
        79_usize,
        141_usize,
        176_usize,
        200_usize,
    ]);
    set.sort();

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_sort_by1() {
    let mut set = TypeProjectedIndexSet::from([
        (1952_usize, 1390_i32),
        (2900_usize, 2846_i32),
        (2999_usize, 760_i32),
        (828_usize,  491_i32),
        (1738_usize, 1984_i32),
        (339_usize,  1996_i32),
    ]);
    let expected = TypeProjectedIndexSet::from([
        (828_usize,  491_i32),
        (2999_usize, 760_i32),
        (1952_usize, 1390_i32),
        (1738_usize, 1984_i32),
        (339_usize,  1996_i32),
        (2900_usize, 2846_i32),
    ]);
    set.sort_by(|v1, v2| v1.1.cmp(&v2.1));

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_sort_by2() {
    let mut set = TypeProjectedIndexSet::from([
        String::from("4"),
        String::from("101"),
        String::from("1"),
        String::from("2"),
        String::from("10"),
        String::from("3"),
    ]);
    let expected = TypeProjectedIndexSet::from([
        String::from("1"),
        String::from("10"),
        String::from("101"),
        String::from("2"),
        String::from("3"),
        String::from("4"),
    ]);
    set.sort_by(|v1, v2| v1.cmp(v2));

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_sort_by3() {
    let mut set = TypeProjectedIndexSet::from([
        String::from("400"),
        String::from("101"),
        String::from("1"),
        String::from("2"),
        String::from("10"),
        String::from("3"),
    ]);
    let expected = TypeProjectedIndexSet::from([
        String::from("1"),
        String::from("2"),
        String::from("3"),
        String::from("10"),
        String::from("400"),
        String::from("101"),
    ]);
    set.sort_by(|v1, v2| v1.len().cmp(&v2.len()));

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_sort_unstable_keys1() {
    let mut set = TypeProjectedIndexSet::from([
        6_usize,
        7_usize,
        10_usize,
        17_usize,
        22_usize,
        23_usize,
        47_usize,
        79_usize,
        141_usize,
        176_usize,
        200_usize,
    ]);
    let expected = TypeProjectedIndexSet::from([
        6_usize,
        7_usize,
        10_usize,
        17_usize,
        22_usize,
        23_usize,
        47_usize,
        79_usize,
        141_usize,
        176_usize,
        200_usize,
    ]);
    set.sort_unstable();

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_sort_unstable_keys2() {
    let mut set = TypeProjectedIndexSet::from([
        10_usize,
        47_usize,
        22_usize,
        17_usize,
        141_usize,
        6_usize,
        176_usize,
        23_usize,
        79_usize,
        200_usize,
        7_usize,
    ]);
    let expected = TypeProjectedIndexSet::from([
        6_usize,
        7_usize,
        10_usize,
        17_usize,
        22_usize,
        23_usize,
        47_usize,
        79_usize,
        141_usize,
        176_usize,
        200_usize,
    ]);
    set.sort_unstable();

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_sort_unstable_keys3() {
    let mut set = TypeProjectedIndexSet::from([
        200_usize,
        176_usize,
        141_usize,
        79_usize,
        47_usize,
        23_usize,
        22_usize,
        17_usize,
        10_usize,
        7_usize,
        6_usize,
    ]);
    let expected = TypeProjectedIndexSet::from([
        6_usize,
        7_usize,
        10_usize,
        17_usize,
        22_usize,
        23_usize,
        47_usize,
        79_usize,
        141_usize,
        176_usize,
        200_usize,
    ]);
    set.sort_unstable();

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_sort_unstable_by1() {
    let mut set = TypeProjectedIndexSet::from([
        (1952_usize, 1390_i32),
        (2900_usize, 2846_i32),
        (2999_usize, 760_i32),
        (828_usize,  491_i32),
        (1738_usize, 1984_i32),
        (339_usize,  1996_i32),
    ]);
    let expected = TypeProjectedIndexSet::from([
        (828_usize,  491_i32),
        (2999_usize, 760_i32),
        (1952_usize, 1390_i32),
        (1738_usize, 1984_i32),
        (339_usize,  1996_i32),
        (2900_usize, 2846_i32),
    ]);
    set.sort_unstable_by(|v1, v2| v1.1.cmp(&v2.1));

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_sort_unstable_by2() {
    let mut set = TypeProjectedIndexSet::from([
        String::from("4"),
        String::from("101"),
        String::from("1"),
        String::from("2"),
        String::from("10"),
        String::from("3"),
    ]);
    let expected = TypeProjectedIndexSet::from([
        String::from("1"),
        String::from("10"),
        String::from("101"),
        String::from("2"),
        String::from("3"),
        String::from("4"),
    ]);
    set.sort_unstable_by(|v1, v2| v1.cmp(v2));

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_sort_unstable_by3() {
    let mut set = TypeProjectedIndexSet::from([
        String::from("400"),
        String::from("101"),
        String::from("1"),
        String::from("2"),
        String::from("10"),
        String::from("3"),
    ]);
    let expected = TypeProjectedIndexSet::from([
        String::from("1"),
        String::from("2"),
        String::from("3"),
        String::from("10"),
        String::from("400"),
        String::from("101"),
    ]);
    set.sort_unstable_by(|v1, v2| v1.len().cmp(&v2.len()));

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_reverse() {
    let mut set = TypeProjectedIndexSet::from([
        (39_usize,   2757_i32),
        (144_usize,  1357_i32),
        (1846_usize, 1138_i32),
        (698_usize,  473_i32),
        (642_usize,  2172_i32),
        (2101_usize, 1894_i32),
    ]);
    let expected = TypeProjectedIndexSet::from([
        (2101_usize, 1894_i32),
        (642_usize,  2172_i32),
        (698_usize,  473_i32),
        (1846_usize, 1138_i32),
        (144_usize,  1357_i32),
        (39_usize,   2757_i32),
    ]);
    set.reverse();

    assert_eq!(set.len(), expected.len());
    assert_eq!(set.as_slice(), expected.as_slice());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_binary_search_by1() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::new();

    for i in -128..128 {
        assert_eq!(set.binary_search_by(|v| v.1.cmp(&i)), Err(0));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_binary_search_by2() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::from([(92_usize, 4_i32)]);

    assert_eq!(set.binary_search_by(|v| v.1.cmp(&0_i32)), Err(0));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&1_i32)), Err(0));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&2_i32)), Err(0));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&3_i32)), Err(0));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&4_i32)), Ok(0));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&5_i32)), Err(1));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&6_i32)), Err(1));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&7_i32)), Err(1));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&8_i32)), Err(1));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_binary_search_by3() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::from([
        (130_usize, 1_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
    ]);

    assert_eq!(set.binary_search_by(|v| v.1.cmp(&0_i32)), Err(0));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&1_i32)), Ok(0));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&2_i32)), Err(1));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&3_i32)), Err(1));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&4_i32)), Ok(1));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&5_i32)), Err(2));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&6_i32)), Err(2));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&7_i32)), Ok(2));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&8_i32)), Err(3));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&9_i32)), Err(3));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_binary_search_by4() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
        (9_usize,   8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(set.binary_search_by(|v| v.1.cmp(&0_i32)),  Err(0));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&1_i32)),  Ok(0));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&2_i32)),  Err(1));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&3_i32)),  Ok(1));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&4_i32)),  Ok(2));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&5_i32)),  Err(3));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&6_i32)),  Err(3));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&7_i32)),  Ok(3));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&8_i32)),  Ok(4));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&9_i32)),  Ok(5));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&10_i32)), Err(6));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&11_i32)), Err(6));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_binary_search_by5() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (60_usize,  4_i32),
        (9_usize,   4_i32),
        (16_usize,  7_i32),
        (19_usize,  8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(set.binary_search_by(|v| v.1.cmp(&0_i32)),  Err(0));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&1_i32)),  Ok(0));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&2_i32)),  Err(1));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&3_i32)),  Ok(1));

    assert!(match set.binary_search_by(|v| v.1.cmp(&4_i32)) {
        Ok(2..=4) => true,
        _ => false,
    });

    assert_eq!(set.binary_search_by(|v| v.1.cmp(&5_i32)),  Err(5));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&6_i32)),  Err(5));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&7_i32)),  Ok(5));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&8_i32)),  Ok(6));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&9_i32)),  Ok(7));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&10_i32)), Err(8));
    assert_eq!(set.binary_search_by(|v| v.1.cmp(&11_i32)), Err(8));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_binary_search_by_key1() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::new();

    for i in -128..128 {
        assert_eq!(set.binary_search_by_key(&i, |v| v.1), Err(0));
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_binary_search_by_key2() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::from([(92_usize, 4_i32)]);

    assert_eq!(set.binary_search_by_key(&0_i32, |v| v.1), Err(0));
    assert_eq!(set.binary_search_by_key(&1_i32, |v| v.1), Err(0));
    assert_eq!(set.binary_search_by_key(&2_i32, |v| v.1), Err(0));
    assert_eq!(set.binary_search_by_key(&3_i32, |v| v.1), Err(0));
    assert_eq!(set.binary_search_by_key(&4_i32, |v| v.1), Ok(0));
    assert_eq!(set.binary_search_by_key(&5_i32, |v| v.1), Err(1));
    assert_eq!(set.binary_search_by_key(&6_i32, |v| v.1), Err(1));
    assert_eq!(set.binary_search_by_key(&7_i32, |v| v.1), Err(1));
    assert_eq!(set.binary_search_by_key(&8_i32, |v| v.1), Err(1));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_binary_search_by_key3() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::from([
        (130_usize, 1_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
    ]);

    assert_eq!(set.binary_search_by_key(&0_i32, |v| v.1), Err(0));
    assert_eq!(set.binary_search_by_key(&1_i32, |v| v.1), Ok(0));
    assert_eq!(set.binary_search_by_key(&2_i32, |v| v.1), Err(1));
    assert_eq!(set.binary_search_by_key(&3_i32, |v| v.1), Err(1));
    assert_eq!(set.binary_search_by_key(&4_i32, |v| v.1), Ok(1));
    assert_eq!(set.binary_search_by_key(&5_i32, |v| v.1), Err(2));
    assert_eq!(set.binary_search_by_key(&6_i32, |v| v.1), Err(2));
    assert_eq!(set.binary_search_by_key(&7_i32, |v| v.1), Ok(2));
    assert_eq!(set.binary_search_by_key(&8_i32, |v| v.1), Err(3));
    assert_eq!(set.binary_search_by_key(&9_i32, |v| v.1), Err(3));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_binary_search_by_key4() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
        (9_usize,   8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(set.binary_search_by_key(&0_i32,  |v| v.1),  Err(0));
    assert_eq!(set.binary_search_by_key(&1_i32,  |v| v.1),  Ok(0));
    assert_eq!(set.binary_search_by_key(&2_i32,  |v| v.1),  Err(1));
    assert_eq!(set.binary_search_by_key(&3_i32,  |v| v.1),  Ok(1));
    assert_eq!(set.binary_search_by_key(&4_i32,  |v| v.1),  Ok(2));
    assert_eq!(set.binary_search_by_key(&5_i32,  |v| v.1),  Err(3));
    assert_eq!(set.binary_search_by_key(&6_i32,  |v| v.1),  Err(3));
    assert_eq!(set.binary_search_by_key(&7_i32,  |v| v.1),  Ok(3));
    assert_eq!(set.binary_search_by_key(&8_i32,  |v| v.1),  Ok(4));
    assert_eq!(set.binary_search_by_key(&9_i32,  |v| v.1),  Ok(5));
    assert_eq!(set.binary_search_by_key(&10_i32, |v| v.1), Err(6));
    assert_eq!(set.binary_search_by_key(&11_i32, |v| v.1), Err(6));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_binary_search_by_key5() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (60_usize,  4_i32),
        (9_usize,   4_i32),
        (16_usize,  7_i32),
        (19_usize,  8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(set.binary_search_by_key(&0_i32, |v| v.1),  Err(0));
    assert_eq!(set.binary_search_by_key(&1_i32, |v| v.1),  Ok(0));
    assert_eq!(set.binary_search_by_key(&2_i32, |v| v.1),  Err(1));
    assert_eq!(set.binary_search_by_key(&3_i32, |v| v.1),  Ok(1));

    assert!(match set.binary_search_by_key(&4_i32, |v| v.1) {
        Ok(2..=4) => true,
        _ => false,
    });

    assert_eq!(set.binary_search_by_key(&5_i32,  |v| v.1), Err(5));
    assert_eq!(set.binary_search_by_key(&6_i32,  |v| v.1), Err(5));
    assert_eq!(set.binary_search_by_key(&7_i32,  |v| v.1), Ok(5));
    assert_eq!(set.binary_search_by_key(&8_i32,  |v| v.1), Ok(6));
    assert_eq!(set.binary_search_by_key(&9_i32,  |v| v.1), Ok(7));
    assert_eq!(set.binary_search_by_key(&10_i32, |v| v.1), Err(8));
    assert_eq!(set.binary_search_by_key(&11_i32, |v| v.1), Err(8));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_partition_point1() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::new();

    for i in -128..128 {
        assert_eq!(set.partition_point(|v| v.1 < i), 0);
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_partition_point2() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::from([(92_usize, 4_i32)]);

    assert_eq!(set.partition_point(|v| v.1 < 0_i32), 0);
    assert_eq!(set.partition_point(|v| v.1 < 1_i32), 0);
    assert_eq!(set.partition_point(|v| v.1 < 2_i32), 0);
    assert_eq!(set.partition_point(|v| v.1 < 3_i32), 0);
    assert_eq!(set.partition_point(|v| v.1 < 4_i32), 0);
    assert_eq!(set.partition_point(|v| v.1 < 5_i32), 1);
    assert_eq!(set.partition_point(|v| v.1 < 6_i32), 1);
    assert_eq!(set.partition_point(|v| v.1 < 7_i32), 1);
    assert_eq!(set.partition_point(|v| v.1 < 8_i32), 1);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_partition_point3() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::from([
        (130_usize, 1_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
    ]);

    assert_eq!(set.partition_point(|v| v.1 < 0_i32), 0);
    assert_eq!(set.partition_point(|v| v.1 < 1_i32), 0);
    assert_eq!(set.partition_point(|v| v.1 < 2_i32), 1);
    assert_eq!(set.partition_point(|v| v.1 < 3_i32), 1);
    assert_eq!(set.partition_point(|v| v.1 < 4_i32), 1);
    assert_eq!(set.partition_point(|v| v.1 < 5_i32), 2);
    assert_eq!(set.partition_point(|v| v.1 < 6_i32), 2);
    assert_eq!(set.partition_point(|v| v.1 < 7_i32), 2);
    assert_eq!(set.partition_point(|v| v.1 < 8_i32), 3);
    assert_eq!(set.partition_point(|v| v.1 < 9_i32), 3);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_partition_point4() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (6_usize,   7_i32),
        (9_usize,   8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(set.partition_point(|v| v.1 < 0_i32),  0);
    assert_eq!(set.partition_point(|v| v.1 < 1_i32),  0);
    assert_eq!(set.partition_point(|v| v.1 < 2_i32),  1);
    assert_eq!(set.partition_point(|v| v.1 < 3_i32),  1);
    assert_eq!(set.partition_point(|v| v.1 < 4_i32),  2);
    assert_eq!(set.partition_point(|v| v.1 < 5_i32),  3);
    assert_eq!(set.partition_point(|v| v.1 < 6_i32),  3);
    assert_eq!(set.partition_point(|v| v.1 < 7_i32),  3);
    assert_eq!(set.partition_point(|v| v.1 < 8_i32),  4);
    assert_eq!(set.partition_point(|v| v.1 < 9_i32),  5);
    assert_eq!(set.partition_point(|v| v.1 < 10_i32), 6);
    assert_eq!(set.partition_point(|v| v.1 < 11_i32), 6);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_partition_point5() {
    let set: TypeProjectedIndexSet<(usize, i32)> = TypeProjectedIndexSet::from([
        (130_usize, 1_i32),
        (45_usize,  3_i32),
        (92_usize,  4_i32),
        (60_usize,  4_i32),
        (9_usize,   4_i32),
        (16_usize,  7_i32),
        (19_usize,  8_i32),
        (10_usize,  9_i32),
    ]);

    assert_eq!(set.partition_point(|v| v.1 < 0_i32), 0);
    assert_eq!(set.partition_point(|v| v.1 < 1_i32), 0);
    assert_eq!(set.partition_point(|v| v.1 < 2_i32), 1);
    assert_eq!(set.partition_point(|v| v.1 < 3_i32), 1);

    assert!(match set.partition_point(|v| v.1 < 4_i32) {
        2..=4 => true,
        _ => false,
    });

    assert_eq!(set.partition_point(|v| v.1 < 5_i32),  5);
    assert_eq!(set.partition_point(|v| v.1 < 6_i32),  5);
    assert_eq!(set.partition_point(|v| v.1 < 7_i32),  5);
    assert_eq!(set.partition_point(|v| v.1 < 8_i32),  6);
    assert_eq!(set.partition_point(|v| v.1 < 9_i32),  7);
    assert_eq!(set.partition_point(|v| v.1 < 10_i32), 8);
    assert_eq!(set.partition_point(|v| v.1 < 11_i32), 8);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_reserve1() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    let additional = 100;

    assert_eq!(set.capacity(), 0);

    set.reserve(additional);

    assert!(set.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_reserve2() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    let additional = 100;

    assert_eq!(set.capacity(), 0);

    set.reserve(additional);

    assert!(set.capacity() >= additional);

    let old_capacity = set.capacity();
    set.insert((0_usize, usize::MAX));
    for i in 1..(set.capacity() - 1) {
        set.insert((i, 0_usize));
    }

    set.insert((set.capacity() - 1, usize::MAX));

    assert_eq!(set.len(), set.capacity());
    assert_eq!(set.capacity(), old_capacity);

    assert_eq!(set[0], (0_usize, usize::MAX));
    for i in 1..(set.len() - 1) {
        assert_eq!(set[i], (i, 0_usize));
    }
    assert_eq!(set[set.len() - 1], (set.len() - 1, usize::MAX));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_reserve3() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    let additional = 100;

    assert_eq!(set.capacity(), 0);
    assert_eq!(set.len(), 0);

    for i in 0..4 {
        let old_capacity = set.capacity();
        set.reserve(additional);

        assert!(set.capacity() >= old_capacity + additional);
        assert!(set.len() <= set.capacity());

        let length = set.len();
        set.insert((length, usize::MAX));
        for j in (length + 1)..(set.capacity() - 1) {
            set.insert((j, i));
        }
        set.insert((set.capacity() - 1, usize::MAX));

        assert_eq!(set.len(), set.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..4 {
        for j in (current_start + 1)..set.len() {
            if set[j].1 == (usize::MAX) {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(set[current_start], (current_start, usize::MAX));
        for value in set[(current_start + 1)..current_end].iter().copied() {
            assert_eq!(value, (value.0, i));
        }
        assert_eq!(set[current_end], (current_end, usize::MAX));

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_reserve_exact1() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    let additional = 100;

    assert_eq!(set.capacity(), 0);

    set.reserve_exact(additional);

    assert!(set.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_reserve_exact2() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    let additional = 100;

    assert_eq!(set.capacity(), 0);

    set.reserve_exact(additional);

    assert!(set.capacity() >= additional);

    let old_capacity = set.capacity();
    set.insert((0_usize, usize::MAX));
    for i in 1..(set.capacity() - 1) {
        set.insert((i, 0_usize));
    }

    set.insert((set.capacity() - 1, usize::MAX));

    assert_eq!(set.len(), set.capacity());
    assert_eq!(set.capacity(), old_capacity);

    assert_eq!(set[0], (0_usize, usize::MAX));
    for i in 1..(set.len() - 1) {
        assert_eq!(set[i], (i, 0_usize));
    }
    assert_eq!(set[set.len() - 1], (set.len() - 1, usize::MAX));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_reserve_exact3() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    let additional = 100;

    assert_eq!(set.capacity(), 0);
    assert_eq!(set.len(), 0);

    for i in 0..32 {
        let old_capacity = set.capacity();
        set.reserve_exact(additional);

        assert!(set.capacity() >= old_capacity + additional);
        assert!(set.len() <= set.capacity());

        let length = set.len();
        set.insert((length, usize::MAX));
        for j in (length + 1)..(set.capacity() - 1) {
            set.insert((j, i));
        }
        set.insert((set.capacity() - 1, usize::MAX));

        assert_eq!(set.len(), set.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..32 {
        for j in (current_start + 1)..set.len() {
            if set[j].1 == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(set[current_start], (current_start, usize::MAX));
        for value in set[(current_start + 1)..current_end].iter().copied() {
            assert_eq!(value, (value.0, i));
        }
        assert_eq!(set[current_end], (current_end, usize::MAX));

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_try_reserve1() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    let additional = 100;

    assert_eq!(set.capacity(), 0);
    assert_eq!(set.try_reserve(additional), Ok(()));
    assert!(set.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_try_reserve2() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    let additional = 100;

    assert_eq!(set.capacity(), 0);
    assert_eq!(set.try_reserve(additional), Ok(()));
    assert!(set.capacity() >= additional);

    let old_capacity = set.capacity();
    set.insert((0_usize, usize::MAX));
    for i in 1..(set.capacity() - 1) {
        set.insert((i, 0_usize));
    }

    set.insert((set.capacity() - 1, usize::MAX));

    assert_eq!(set.len(), set.capacity());
    assert_eq!(set.capacity(), old_capacity);

    assert_eq!(set[0], (0_usize, usize::MAX));
    for i in 1..(set.len() - 1) {
        assert_eq!(set[i], (i, 0_usize));
    }
    assert_eq!(set[set.len() - 1], (set.len() - 1, usize::MAX));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_try_reserve3() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    let additional = 100;

    assert_eq!(set.capacity(), 0);
    assert_eq!(set.len(), 0);

    for i in 0..4 {
        let old_capacity = set.capacity();
        assert_eq!(set.try_reserve(additional), Ok(()));

        assert!(set.capacity() >= old_capacity + additional);
        assert!(set.len() <= set.capacity());

        let length = set.len();
        set.insert((length, usize::MAX));
        for j in (length + 1)..(set.capacity() - 1) {
            set.insert((j, i));
        }
        set.insert((set.capacity() - 1, usize::MAX));

        assert_eq!(set.len(), set.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..4 {
        for j in (current_start + 1)..set.len() {
            if set[j].1 == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(set[current_start], (current_start, usize::MAX));
        for value in set[(current_start + 1)..current_end].iter().copied() {
            assert_eq!(value, (value.0, i));
        }
        assert_eq!(set[current_end], (current_end, usize::MAX));

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_try_reserve_exact1() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    let additional = 100;

    assert_eq!(set.capacity(), 0);
    assert_eq!(set.try_reserve_exact(additional), Ok(()));
    assert!(set.capacity() >= additional);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_try_reserve_exact2() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    let additional = 100;

    assert_eq!(set.capacity(), 0);
    assert_eq!(set.try_reserve_exact(additional), Ok(()));
    assert!(set.capacity() >= additional);

    let old_capacity = set.capacity();
    set.insert((0_usize, usize::MAX));
    for i in 1..(set.capacity() - 1) {
        set.insert((i, 0_usize));
    }

    set.insert((set.capacity() - 1, usize::MAX));

    assert_eq!(set.len(), set.capacity());
    assert_eq!(set.capacity(), old_capacity);

    assert_eq!(set[0], (0_usize, usize::MAX));
    for i in 1..(set.len() - 1) {
        assert_eq!(set[i], (i, 0_usize));
    }
    assert_eq!(set[set.len() - 1], (set.len() - 1, usize::MAX));
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_try_reserve_exact3() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    let additional = 100;

    assert_eq!(set.capacity(), 0);
    assert_eq!(set.len(), 0);

    for i in 0..32 {
        let old_capacity = set.capacity();
        assert_eq!(set.try_reserve_exact(additional), Ok(()));

        assert!(set.capacity() >= old_capacity + additional);
        assert!(set.len() <= set.capacity());

        let length = set.len();
        set.insert((length, usize::MAX));
        for j in (length + 1)..(set.capacity() - 1) {
            set.insert((j, i));
        }
        set.insert((set.capacity() - 1, usize::MAX));

        assert_eq!(set.len(), set.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..32 {
        for j in (current_start + 1)..set.len() {
            if set[j].1 == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(set[current_start], (current_start, usize::MAX));
        for value in set[(current_start + 1)..current_end].iter().copied() {
            assert_eq!(value, (value.0, i));
        }
        assert_eq!(set[current_end], (current_end, usize::MAX));

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shrink_to_fit1() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::with_capacity(10);
    assert_eq!(set.capacity(), 10);

    set.extend([(1_usize, usize::MAX), (2_usize, usize::MAX), (3_usize, usize::MAX)]);
    assert!(set.len() <= set.capacity());
    set.shrink_to_fit();
    assert_eq!(set.len(), set.capacity());
}

#[rustfmt::skip]
#[test]
fn test_type_projected_index_set_shrink_to_fit2() {
    let mut set: TypeProjectedIndexSet<(usize, usize)> = TypeProjectedIndexSet::new();
    for i in 0..128 {
        assert_eq!(set.len(), i);

        set.insert((i, i * i));

        assert_eq!(set.len(), i + 1);
        assert!(set.capacity() >= i + 1);
        assert_eq!(set[i], (i, i * i));
        assert_eq!(set.get(&(i, i * i)), Some(&(i, i * i)));

        set.shrink_to_fit();

        assert_eq!(set.len(), i + 1);
        assert_eq!(set.capacity(), i + 1);
        assert_eq!(set[i], (i, i * i));
        assert_eq!(set.get(&(i, i * i)), Some(&(i, i * i)));
    }
}

#[test]
fn test_type_projected_index_set_difference1() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let result: TypeProjectedIndexSet<u64> = set1.difference(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_difference2() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let mut iter = set1.difference(&set2);

    for i in 0..65536 {
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn test_type_projected_index_set_difference3() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let expected: TypeProjectedIndexSet<u64> = set1.clone();
    let result: TypeProjectedIndexSet<u64> = set1.difference(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_difference4() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let mut iter = set1.difference(&set2);

    assert_eq!(iter.next(), Some(&42_u64));
    assert_eq!(iter.next(), Some(&40_u64));
    assert_eq!(iter.next(), Some(&73_u64));
    assert_eq!(iter.next(), Some(&32_u64));
    assert_eq!(iter.next(), Some(&21_u64));
    assert_eq!(iter.next(), Some(&10_u64));
    assert_eq!(iter.next(), Some(&51_u64));
    assert_eq!(iter.next(), Some(&18_u64));
    assert_eq!(iter.next(), Some(&92_u64));
    assert_eq!(iter.next(), Some(&34_u64));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_set_difference5() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let result: TypeProjectedIndexSet<u64> = set1.difference(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_difference6() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let mut iter = set1.difference(&set2);

    for i in 0..65536 {
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn test_type_projected_index_set_difference7() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        88_u64,
        82_u64,
        98_u64,
        17_u64,
        60_u64,
        62_u64,
        26_u64,
        83_u64,
        19_u64,
    ]);
    let expected: TypeProjectedIndexSet<u64> = set1.clone();
    let result: TypeProjectedIndexSet<u64> = set1.difference(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_difference8() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        88_u64,
        82_u64,
        98_u64,
        17_u64,
        60_u64,
        62_u64,
        26_u64,
        83_u64,
        19_u64,
    ]);
    let mut iter = set1.difference(&set2);

    assert_eq!(iter.next(), Some(&42_u64));
    assert_eq!(iter.next(), Some(&40_u64));
    assert_eq!(iter.next(), Some(&73_u64));
    assert_eq!(iter.next(), Some(&32_u64));
    assert_eq!(iter.next(), Some(&21_u64));
    assert_eq!(iter.next(), Some(&10_u64));
    assert_eq!(iter.next(), Some(&51_u64));
    assert_eq!(iter.next(), Some(&18_u64));
    assert_eq!(iter.next(), Some(&92_u64));
    assert_eq!(iter.next(), Some(&34_u64));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_set_difference9() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        40_u64,
        42_u64,
        51_u64,
        86_u64,
        18_u64,
        70_u64,
    ]);
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        92_u64,
        34_u64,
    ]);
    let result: TypeProjectedIndexSet<u64> = set1.difference(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_difference10() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        40_u64,
        42_u64,
        51_u64,
        86_u64,
        18_u64,
        70_u64,
    ]);
    let mut iter = set1.difference(&set2);

    assert_eq!(iter.next(), Some(&73_u64));
    assert_eq!(iter.next(), Some(&32_u64));
    assert_eq!(iter.next(), Some(&21_u64));
    assert_eq!(iter.next(), Some(&10_u64));
    assert_eq!(iter.next(), Some(&92_u64));
    assert_eq!(iter.next(), Some(&34_u64));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_set_intersection1() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let result: TypeProjectedIndexSet<u64> = set1.intersection(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_intersection2() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let mut iter = set1.intersection(&set2);

    for i in 0..65536 {
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn test_type_projected_index_set_intersection3() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let result: TypeProjectedIndexSet<u64> = set1.intersection(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_intersection4() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let mut iter = set1.intersection(&set2);

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_set_intersection5() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let result: TypeProjectedIndexSet<u64> = set1.intersection(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_intersection6() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let mut iter = set1.intersection(&set2);

    for i in 0..65536 {
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn test_type_projected_index_set_intersection7() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        88_u64,
        82_u64,
        98_u64,
        17_u64,
        60_u64,
        62_u64,
        26_u64,
        83_u64,
        19_u64,
    ]);
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let result: TypeProjectedIndexSet<u64> = set1.intersection(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_intersection8() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        88_u64,
        82_u64,
        98_u64,
        17_u64,
        60_u64,
        62_u64,
        26_u64,
        83_u64,
        19_u64,
    ]);
    let mut iter = set1.intersection(&set2);

    for i in 0..65536 {
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn test_type_projected_index_set_intersection9() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        40_u64,
        42_u64,
        51_u64,
        86_u64,
        18_u64,
        70_u64,
    ]);
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        51_u64,
        18_u64,
    ]);
    let result: TypeProjectedIndexSet<u64> = set1.intersection(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_intersection10() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        40_u64,
        42_u64,
        51_u64,
        86_u64,
        18_u64,
        70_u64,
    ]);
    let mut iter = set1.intersection(&set2);

    assert_eq!(iter.next(), Some(&42_u64));
    assert_eq!(iter.next(), Some(&40_u64));
    assert_eq!(iter.next(), Some(&51_u64));
    assert_eq!(iter.next(), Some(&18_u64));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_set_union1() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let result: TypeProjectedIndexSet<u64> = set1.union(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_union2() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let mut iter = set1.union(&set2);

    for i in 0..65536 {
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn test_type_projected_index_set_union3() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let expected: TypeProjectedIndexSet<u64> = set1.clone();
    let result: TypeProjectedIndexSet<u64> = set1.union(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_union4() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let mut iter = set1.union(&set2);

    assert_eq!(iter.next(), Some(&42_u64));
    assert_eq!(iter.next(), Some(&40_u64));
    assert_eq!(iter.next(), Some(&73_u64));
    assert_eq!(iter.next(), Some(&32_u64));
    assert_eq!(iter.next(), Some(&21_u64));
    assert_eq!(iter.next(), Some(&10_u64));
    assert_eq!(iter.next(), Some(&51_u64));
    assert_eq!(iter.next(), Some(&18_u64));
    assert_eq!(iter.next(), Some(&92_u64));
    assert_eq!(iter.next(), Some(&34_u64));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_set_union5() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let expected: TypeProjectedIndexSet<u64> = set2.clone();
    let result: TypeProjectedIndexSet<u64> = set1.union(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_union6() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let mut iter = set1.union(&set2);

    assert_eq!(iter.next(), Some(&42_u64));
    assert_eq!(iter.next(), Some(&40_u64));
    assert_eq!(iter.next(), Some(&73_u64));
    assert_eq!(iter.next(), Some(&32_u64));
    assert_eq!(iter.next(), Some(&21_u64));
    assert_eq!(iter.next(), Some(&10_u64));
    assert_eq!(iter.next(), Some(&51_u64));
    assert_eq!(iter.next(), Some(&18_u64));
    assert_eq!(iter.next(), Some(&92_u64));
    assert_eq!(iter.next(), Some(&34_u64));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_set_union7() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        88_u64,
        82_u64,
        98_u64,
        17_u64,
        60_u64,
        62_u64,
        26_u64,
        83_u64,
        19_u64,
    ]);
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        42_u64, 40_u64, 73_u64, 32_u64, 21_u64, 10_u64, 51_u64, 18_u64, 92_u64, 34_u64,
        88_u64, 82_u64, 98_u64, 17_u64, 60_u64, 62_u64, 26_u64, 83_u64, 19_u64,
    ]);
    let result: TypeProjectedIndexSet<u64> = set1.union(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_union8() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        88_u64,
        82_u64,
        98_u64,
        17_u64,
        60_u64,
        62_u64,
        26_u64,
        83_u64,
        19_u64,
    ]);
    let mut iter = set1.union(&set2);

    assert_eq!(iter.next(), Some(&42_u64));
    assert_eq!(iter.next(), Some(&40_u64));
    assert_eq!(iter.next(), Some(&73_u64));
    assert_eq!(iter.next(), Some(&32_u64));
    assert_eq!(iter.next(), Some(&21_u64));
    assert_eq!(iter.next(), Some(&10_u64));
    assert_eq!(iter.next(), Some(&51_u64));
    assert_eq!(iter.next(), Some(&18_u64));
    assert_eq!(iter.next(), Some(&92_u64));
    assert_eq!(iter.next(), Some(&34_u64));

    assert_eq!(iter.next(), Some(&88_u64));
    assert_eq!(iter.next(), Some(&82_u64));
    assert_eq!(iter.next(), Some(&98_u64));
    assert_eq!(iter.next(), Some(&17_u64));
    assert_eq!(iter.next(), Some(&60_u64));
    assert_eq!(iter.next(), Some(&62_u64));
    assert_eq!(iter.next(), Some(&26_u64));
    assert_eq!(iter.next(), Some(&83_u64));
    assert_eq!(iter.next(), Some(&19_u64));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_set_union9() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        40_u64,
        42_u64,
        51_u64,
        86_u64,
        18_u64,
        70_u64,
    ]);
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        42_u64, 40_u64, 73_u64, 32_u64, 21_u64, 10_u64, 51_u64, 18_u64, 92_u64, 34_u64,
        86_u64, 70_u64,
    ]);
    let result: TypeProjectedIndexSet<u64> = set1.union(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_union10() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        40_u64,
        42_u64,
        51_u64,
        86_u64,
        18_u64,
        70_u64,
    ]);
    let mut iter = set1.union(&set2);

    assert_eq!(iter.next(), Some(&42_u64));
    assert_eq!(iter.next(), Some(&40_u64));
    assert_eq!(iter.next(), Some(&73_u64));
    assert_eq!(iter.next(), Some(&32_u64));
    assert_eq!(iter.next(), Some(&21_u64));
    assert_eq!(iter.next(), Some(&10_u64));
    assert_eq!(iter.next(), Some(&51_u64));
    assert_eq!(iter.next(), Some(&18_u64));
    assert_eq!(iter.next(), Some(&92_u64));
    assert_eq!(iter.next(), Some(&34_u64));

    assert_eq!(iter.next(), Some(&86_u64));
    assert_eq!(iter.next(), Some(&70_u64));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_set_symmetric_difference1() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let result: TypeProjectedIndexSet<u64> = set1.symmetric_difference(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_symmetric_difference2() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let mut iter = set1.symmetric_difference(&set2);

    for i in 0..65536 {
        assert_eq!(iter.next(), None);
    }
}

#[test]
fn test_type_projected_index_set_symmetric_difference3() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let expected: TypeProjectedIndexSet<u64> = set1.clone();
    let result: TypeProjectedIndexSet<u64> = set1.symmetric_difference(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_symmetric_difference4() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let mut iter = set1.symmetric_difference(&set2);

    assert_eq!(iter.next(), Some(&42_u64));
    assert_eq!(iter.next(), Some(&40_u64));
    assert_eq!(iter.next(), Some(&73_u64));
    assert_eq!(iter.next(), Some(&32_u64));
    assert_eq!(iter.next(), Some(&21_u64));
    assert_eq!(iter.next(), Some(&10_u64));
    assert_eq!(iter.next(), Some(&51_u64));
    assert_eq!(iter.next(), Some(&18_u64));
    assert_eq!(iter.next(), Some(&92_u64));
    assert_eq!(iter.next(), Some(&34_u64));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_set_symmetric_difference5() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let expected: TypeProjectedIndexSet<u64> = set2.clone();
    let result: TypeProjectedIndexSet<u64> = set1.symmetric_difference(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_symmetric_difference6() {
    let set1: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::new();
    let set2 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let mut iter = set1.symmetric_difference(&set2);

    assert_eq!(iter.next(), Some(&42_u64));
    assert_eq!(iter.next(), Some(&40_u64));
    assert_eq!(iter.next(), Some(&73_u64));
    assert_eq!(iter.next(), Some(&32_u64));
    assert_eq!(iter.next(), Some(&21_u64));
    assert_eq!(iter.next(), Some(&10_u64));
    assert_eq!(iter.next(), Some(&51_u64));
    assert_eq!(iter.next(), Some(&18_u64));
    assert_eq!(iter.next(), Some(&92_u64));
    assert_eq!(iter.next(), Some(&34_u64));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_set_symmetric_difference7() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        88_u64,
        82_u64,
        98_u64,
        17_u64,
        60_u64,
        62_u64,
        26_u64,
        83_u64,
        19_u64,
    ]);
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        42_u64, 40_u64, 73_u64, 32_u64, 21_u64, 10_u64, 51_u64, 18_u64, 92_u64, 34_u64,
        88_u64, 82_u64, 98_u64, 17_u64, 60_u64, 62_u64, 26_u64, 83_u64, 19_u64,
    ]);
    let result: TypeProjectedIndexSet<u64> = set1.symmetric_difference(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_symmetric_difference8() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        88_u64,
        82_u64,
        98_u64,
        17_u64,
        60_u64,
        62_u64,
        26_u64,
        83_u64,
        19_u64,
    ]);
    let mut iter = set1.symmetric_difference(&set2);

    assert_eq!(iter.next(), Some(&42_u64));
    assert_eq!(iter.next(), Some(&40_u64));
    assert_eq!(iter.next(), Some(&73_u64));
    assert_eq!(iter.next(), Some(&32_u64));
    assert_eq!(iter.next(), Some(&21_u64));
    assert_eq!(iter.next(), Some(&10_u64));
    assert_eq!(iter.next(), Some(&51_u64));
    assert_eq!(iter.next(), Some(&18_u64));
    assert_eq!(iter.next(), Some(&92_u64));
    assert_eq!(iter.next(), Some(&34_u64));

    assert_eq!(iter.next(), Some(&88_u64));
    assert_eq!(iter.next(), Some(&82_u64));
    assert_eq!(iter.next(), Some(&98_u64));
    assert_eq!(iter.next(), Some(&17_u64));
    assert_eq!(iter.next(), Some(&60_u64));
    assert_eq!(iter.next(), Some(&62_u64));
    assert_eq!(iter.next(), Some(&26_u64));
    assert_eq!(iter.next(), Some(&83_u64));
    assert_eq!(iter.next(), Some(&19_u64));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn test_type_projected_index_set_symmetric_difference9() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        40_u64,
        42_u64,
        51_u64,
        86_u64,
        18_u64,
        70_u64,
    ]);
    let expected: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        73_u64, 32_u64, 21_u64, 10_u64, 92_u64, 34_u64,
        86_u64, 70_u64,
    ]);
    let result: TypeProjectedIndexSet<u64> = set1.symmetric_difference(&set2).cloned().collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_index_set_symmetric_difference10() {
    let set1 = TypeProjectedIndexSet::from([
        42_u64,
        40_u64,
        73_u64,
        32_u64,
        21_u64,
        10_u64,
        51_u64,
        18_u64,
        92_u64,
        34_u64,
    ]);
    let set2: TypeProjectedIndexSet<u64> = TypeProjectedIndexSet::from([
        40_u64,
        42_u64,
        51_u64,
        86_u64,
        18_u64,
        70_u64,
    ]);
    let mut iter = set1.symmetric_difference(&set2);

    assert_eq!(iter.next(), Some(&73_u64));
    assert_eq!(iter.next(), Some(&32_u64));
    assert_eq!(iter.next(), Some(&21_u64));
    assert_eq!(iter.next(), Some(&10_u64));
    assert_eq!(iter.next(), Some(&92_u64));
    assert_eq!(iter.next(), Some(&34_u64));

    assert_eq!(iter.next(), Some(&86_u64));
    assert_eq!(iter.next(), Some(&70_u64));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}
