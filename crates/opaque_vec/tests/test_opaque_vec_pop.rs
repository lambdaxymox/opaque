use opaque_vec::OpaqueVec;

#[test]
fn test_opaque_vec_pop_empty1() {
    let mut vec = OpaqueVec::new::<i32>();

    assert!(vec.pop::<i32>().is_none());
}

#[test]
fn test_opaque_vec_pop_empty2() {
    let mut vec = OpaqueVec::new::<i32>();

    for _ in 0..65536 {
        assert!(vec.pop::<i32>().is_none());
    }
}
