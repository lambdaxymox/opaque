use opaque_alloc::TypeErasedAlloc;

use alloc_crate::alloc;
use alloc_crate::format;

#[test]
fn test_type_erased_alloc_debug1() {
    let opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    let debug_str = format!("{:?}", opaque_alloc);

    assert!(debug_str.contains("TypeErasedAlloc"));
}

#[test]
fn test_type_erased_alloc_debug2() {
    let opaque_alloc = TypeErasedAlloc::new::<alloc::Global>(alloc::Global);
    let debug_str = format!("{:?}", opaque_alloc);

    assert!(!debug_str.contains("Global"));
}
