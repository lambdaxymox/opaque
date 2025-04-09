use opaque_map::OpaqueMap;

#[test]
fn test_opaque_index_map_empty_len() {
    let opaque_map = OpaqueMap::new::<u32, i32>();
    let expected = 0;
    let result = opaque_map.len();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_index_map_empty_is_empty() {
    let opaque_map = OpaqueMap::new::<u32, i32>();

    assert!(opaque_map.is_empty());
}

#[test]
fn test_opaque_index_map_empty_contains_no_values() {
    let opaque_map = OpaqueMap::new::<u32, i32>();
    for key in 0..65536 {
        assert!(!opaque_map.contains_key::<u32, u32, i32>(&key));
    }
}

#[test]
fn test_opaque_index_map_empty_get() {
    let opaque_map = OpaqueMap::new::<u32, i32>();
    for key in 0..65536 {
        let result = opaque_map.get::<u32, u32, i32>(&key);

        assert!(result.is_none());
    }
}
