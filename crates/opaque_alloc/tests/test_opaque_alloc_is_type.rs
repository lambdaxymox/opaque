#![feature(allocator_api)]
use std::alloc::{Global, System};
use opaque_alloc::OpaqueAlloc;

#[test]
fn test_opaque_alloc_is_type_global() {
    let opaque_alloc = OpaqueAlloc::new::<Global>(Global);

    assert!(opaque_alloc.is_type::<Global>());
}

#[test]
fn test_opaque_alloc_is_type_system() {
    let opaque_alloc = OpaqueAlloc::new::<System>(System);

    assert!(opaque_alloc.is_type::<System>());
}
