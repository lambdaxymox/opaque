use opaque_alloc::OpaqueAlloc;

use alloc_crate::format;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[test]
fn test_opaque_alloc_debug1() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    let debug_str = format!("{:?}", opaque_alloc);

    assert!(debug_str.contains("OpaqueAlloc"));
}

#[test]
fn test_opaque_alloc_debug2() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    let debug_str = format!("{:?}", opaque_alloc);

    assert!(!debug_str.contains("Global"));
}
