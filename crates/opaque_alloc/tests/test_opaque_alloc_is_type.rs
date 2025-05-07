#![feature(allocator_api)]
use opaque_alloc::OpaqueAlloc;
use std::alloc;

#[test]
fn test_opaque_alloc_is_type_global() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);

    assert!(opaque_alloc.is_type::<alloc::Global>());
}

#[test]
fn test_opaque_alloc_is_type_system() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);

    assert!(opaque_alloc.is_type::<alloc::System>());
}
