use imgui_vulkan_renderer_opaque_vec::OpaqueVec;

#[test]
fn test_opaque_vec_empty_len() {
    let vec = OpaqueVec::new::<u32>();
    let expected = 0;
    let result = vec.len();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_empty_is_empty() {
    let vec = OpaqueVec::new::<u32>();

    assert!(vec.is_empty());
}

#[test]
fn test_opaque_vec_empty_capacity_should_not_allocate() {
    let vec = OpaqueVec::new::<u32>();
    let expected = 0;
    let result = vec.capacity();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_vec_empty_contains_no_values() {
    let vec = OpaqueVec::new::<u32>();
    for value in 0..65536 {
        assert!(!vec.contains::<u32>(&value));
    }
}

#[test]
fn test_opaque_vec_empty_get() {
    let vec = OpaqueVec::new::<u32>();
    for i in 0..65536 {
        let result = vec.get::<u32>(i);

        assert!(result.is_none());
    }
}
