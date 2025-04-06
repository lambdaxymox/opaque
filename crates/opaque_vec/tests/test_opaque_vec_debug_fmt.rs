use opaque_vec::OpaqueVec;

#[test]
fn test_opaque_vec_debug_fmt_empty() {
    let vec = OpaqueVec::new::<isize>();

    let expected = "OpaqueVec { element_layout: Layout { size: 8, align: 8 (1 << 3) }, capacity: 0, length: 0, data: [] }";
    let result = format!("{:?}", vec);

    assert_eq!(result, expected);
}
