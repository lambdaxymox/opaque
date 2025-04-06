use imgui_vulkan_renderer_opaque_map::OpaqueMap;

#[test]
fn test_opaque_map_empty_is_empty() {
    let map = OpaqueMap::new::<usize, u32>();

    assert!(map.is_empty());
}

#[test]
fn test_opaque_map_empty_len() {
    let map = OpaqueMap::new::<usize, u32>();

    assert_eq!(map.len(), 0);
}

#[test]
fn test_opaque_map_empty_get_none() {
    let map = OpaqueMap::new::<usize, u32>();
    for i in 0..10000 {
        let result = map.get::<usize, usize, u32>(&i);

        assert!(result.is_none());
    }
}

#[test]
fn test_opaque_map_push_get1() {
    let key = 0;
    let value: u32 = 1;
    let mut map = OpaqueMap::new::<usize, u32>();
    map.insert(key, value);

    let expected = Some(value);
    let result = map.get::<usize, usize, u32>(&key).copied();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_map_insert_get2() {
    let values = [0, 1, 2, 3];
        
    let mut map = OpaqueMap::new::<usize, i32>();
    for (key, value) in values.iter().copied().enumerate() {
        map.insert(key, value);
    }

    for i in 0..map.len() {
        let expected = Some(values[i]);
        let result = map.get::<usize, usize, i32>(&i).copied();
        
        assert_eq!(result, expected);
    }
}

#[test]
fn test_opaque_map_insert_remove_contains_key1() {
    let key = 0;
    let value: u32 = 1;
    let mut map = OpaqueMap::new::<usize, u32>();
    map.insert(key, value);

    assert!(map.contains_key::<usize, usize, u32>(&key));

    map.swap_remove::<usize, usize, u32>(&key);

    assert!(!map.contains_key::<usize, usize, u32>(&key));
}

#[test]
fn test_opaque_map_as_slice1() {
    let values = [0, 1, 2, 3];
        
    let mut map = OpaqueMap::new::<usize, i32>();
    for (key, value) in values.iter().copied().enumerate() {
        map.insert(key, value);
    }

    let expected = values.as_slice();
    let values: Vec<i32> = map
        .as_slice::<usize, i32>()
        .values()
        .copied()
        .collect();
    let result = values.as_slice();

    assert_eq!(result, expected);
}
