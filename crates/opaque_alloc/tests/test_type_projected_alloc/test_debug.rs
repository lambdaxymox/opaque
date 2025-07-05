use opaque_alloc::TypeProjectedAlloc;

use alloc_crate::format;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[test]
fn test_type_projected_alloc_debug1() {
    let proj_alloc = TypeProjectedAlloc::new(alloc::Global);
    let debug_str = format!("{:?}", proj_alloc);

    assert!(debug_str.contains("TypeProjectedAlloc"));
}

#[test]
fn test_type_projected_alloc_debug2() {
    let proj_alloc = TypeProjectedAlloc::new(alloc::Global);
    let debug_str = format!("{:?}", proj_alloc);

    assert!(debug_str.contains("Global"));
}
