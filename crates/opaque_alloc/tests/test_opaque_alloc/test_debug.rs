use opaque_alloc::OpaqueAlloc;
use std::alloc;

#[test]
fn test_opaque_alloc_debug1() {
    let alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    let debug_str = format!("{:?}", alloc);

    assert!(debug_str.contains("OpaqueAlloc"));
}

#[test]
fn test_opaque_alloc_debug2() {
    let alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    let debug_str = format!("{:?}", alloc);

    assert!(!debug_str.contains("Global"));
}
