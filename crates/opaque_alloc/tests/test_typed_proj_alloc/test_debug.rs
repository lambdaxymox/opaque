use opaque_alloc::TypedProjAlloc;
use std::alloc;

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
