use opaque_alloc::OpaqueAlloc;
use std::alloc;

#[test]
fn test_opaque_alloc_has_alloc_type1() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);

    assert!(opaque_alloc.has_alloc_type::<alloc::Global>());
}

#[test]
fn test_opaque_alloc_has_alloc_type2() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);

    assert!(opaque_alloc.has_alloc_type::<alloc::System>());
}

#[test]
fn test_opaque_alloc_into_proj_has_alloc_type1() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    let proj_alloc = opaque_alloc.into_proj::<alloc::Global>();
    let opaque_alloc = OpaqueAlloc::from_proj(proj_alloc);

    assert!(opaque_alloc.has_alloc_type::<alloc::Global>());
}

#[test]
fn test_opaque_alloc_into_proj_has_alloc_type2() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);
    let proj_alloc = opaque_alloc.into_proj::<alloc::System>();
    let opaque_alloc = OpaqueAlloc::from_proj(proj_alloc);

    assert!(opaque_alloc.has_alloc_type::<alloc::System>());
}

#[test]
fn test_opaque_alloc_not_has_alloc_type1() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);

    assert!(!opaque_alloc.has_alloc_type::<alloc::Global>());
}

#[test]
fn test_opaque_alloc_not_has_alloc_type2() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);

    assert!(!opaque_alloc.has_alloc_type::<alloc::System>());
}

#[test]
fn test_opaque_alloc_into_proj_not_has_alloc_type1() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);
    let proj_alloc = opaque_alloc.into_proj::<alloc::System>();
    let opaque_alloc = OpaqueAlloc::from_proj(proj_alloc);

    assert!(!opaque_alloc.has_alloc_type::<alloc::Global>());
}

#[test]
fn test_opaque_alloc_into_proj_not_has_alloc_type2() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    let proj_alloc = opaque_alloc.into_proj::<alloc::Global>();
    let opaque_alloc = OpaqueAlloc::from_proj(proj_alloc);

    assert!(!opaque_alloc.has_alloc_type::<alloc::System>());
}
