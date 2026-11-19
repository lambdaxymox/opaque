use opaque_alloc::TypeProjectedAlloc;

use alloc_crate::alloc;
use alloc_crate::format;

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
