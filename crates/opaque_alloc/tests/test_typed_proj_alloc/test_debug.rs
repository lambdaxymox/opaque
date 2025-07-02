use opaque_alloc::TypedProjAlloc;

use alloc_crate::format;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use allocator_api2::alloc;

#[test]
fn test_typed_proj_alloc_debug1() {
    let proj_alloc = TypedProjAlloc::new(alloc::Global);
    let debug_str = format!("{:?}", proj_alloc);

    assert!(debug_str.contains("TypedProjAlloc"));
}

#[test]
fn test_typed_proj_alloc_debug2() {
    let proj_alloc = TypedProjAlloc::new(alloc::Global);
    let debug_str = format!("{:?}", proj_alloc);

    assert!(debug_str.contains("Global"));
}
