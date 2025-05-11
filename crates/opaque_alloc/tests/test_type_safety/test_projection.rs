use std::alloc;
use opaque_alloc::OpaqueAlloc;

#[test]
fn test_opaque_alloc_into_proj_correct_type1() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    let _ = opaque_alloc.as_proj::<alloc::Global>();
}

#[test]
fn test_opaque_alloc_into_proj_correct_type2() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);
    let _ = opaque_alloc.into_proj::<alloc::System>();
}

#[test]
fn test_opaque_alloc_into_proj_correct_type3() {
    let mut opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    for _ in 0..65536 {
        let proj_alloc = opaque_alloc.into_proj::<alloc::Global>();
        opaque_alloc = OpaqueAlloc::from_proj(proj_alloc);
    }
}

#[test]
fn test_opaque_alloc_into_proj_correct_type4() {
    let mut opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);
    for _ in 0..65536 {
        let proj_alloc = opaque_alloc.into_proj::<alloc::System>();
        opaque_alloc = OpaqueAlloc::from_proj(proj_alloc);
    }
}

#[test]
#[should_panic]
fn test_opaque_alloc_into_proj_panics_wrong_type1() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);
    let _ = opaque_alloc.into_proj::<alloc::Global>();
}

#[test]
#[should_panic]
fn test_opaque_alloc_into_proj_panics_wrong_type2() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    let _ = opaque_alloc.into_proj::<alloc::System>();
}

#[test]
fn test_opaque_alloc_as_proj_correct_type1() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    let _ = opaque_alloc.as_proj::<alloc::Global>();
}

#[test]
fn test_opaque_alloc_as_proj_correct_type2() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);
    let _ = opaque_alloc.as_proj::<alloc::System>();
}

#[test]
fn test_opaque_alloc_as_proj_correct_type3() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    for _ in 0..65536 {
        let _ = opaque_alloc.as_proj::<alloc::Global>();
    }
}

#[test]
fn test_opaque_alloc_as_proj_correct_type4() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);
    for _ in 0..65536 {
        let _ = opaque_alloc.as_proj::<alloc::System>();
    }
}

#[test]
#[should_panic]
fn test_opaque_alloc_as_proj_panics_wrong_type1() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);
    let _ = opaque_alloc.as_proj::<alloc::Global>();
}

#[test]
#[should_panic]
fn test_opaque_alloc_as_proj_panics_wrong_type2() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    let _ = opaque_alloc.as_proj::<alloc::System>();
}

#[test]
fn test_opaque_alloc_as_proj_mut_correct_type1() {
    let mut opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    let _ = opaque_alloc.as_proj_mut::<alloc::Global>();
}

#[test]
fn test_opaque_alloc_as_proj_mut_correct_type2() {
    let mut opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);
    let _ = opaque_alloc.as_proj_mut::<alloc::System>();
}

#[test]
fn test_opaque_alloc_as_proj_mut_correct_type3() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    for _ in 0..65536 {
        let _ = opaque_alloc.as_proj::<alloc::Global>();
    }
}

#[test]
fn test_opaque_alloc_as_proj_mut_correct_type4() {
    let opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);
    for _ in 0..65536 {
        let _ = opaque_alloc.as_proj::<alloc::System>();
    }
}

#[test]
#[should_panic]
fn test_opaque_alloc_as_proj_mut_panics_wrong_type1() {
    let mut opaque_alloc = OpaqueAlloc::new::<alloc::System>(alloc::System);
    let _ = opaque_alloc.as_proj_mut::<alloc::Global>();
}

#[test]
#[should_panic]
fn test_opaque_alloc_as_proj_mut_panics_wrong_type2() {
    let mut opaque_alloc = OpaqueAlloc::new::<alloc::Global>(alloc::Global);
    let _ = opaque_alloc.as_proj_mut::<alloc::System>();
}
