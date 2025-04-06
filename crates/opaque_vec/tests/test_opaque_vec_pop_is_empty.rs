use imgui_vulkan_renderer_opaque_vec::OpaqueVec;

#[test]
fn test_opaque_vec_pop_empty1() {
    let mut vec = OpaqueVec::new::<i32>();

    assert!(vec.is_empty());

    vec.pop::<i32>();

    assert!(vec.is_empty());
}

#[test]
fn test_opaque_vec_pop_is_empty2() {
    let mut vec = OpaqueVec::new::<i32>();

    assert!(vec.is_empty());

    for _ in 0..65536 {
        vec.pop::<i32>();
    }

    assert!(vec.is_empty());
}
