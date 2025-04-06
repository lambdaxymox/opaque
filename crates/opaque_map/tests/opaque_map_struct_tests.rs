use opaque_map::OpaqueMap;

#[derive(Copy, Clone, PartialEq, Debug)]
struct TestStruct {
    id: u32,
    flag: bool,
    value: f32,
}

impl TestStruct {
    fn new(id: u32, flag: bool, value: f32) -> Self {
        Self {
            id,
            flag,
            value,
        }
    }
}

#[test]
fn test_opaque_map_empty_is_empty() {
    let map = OpaqueMap::new::<usize, TestStruct>();

    assert!(map.is_empty());
}

#[test]
fn test_opaque_map_empty_len() {
    let map = OpaqueMap::new::<usize, TestStruct>();

    assert_eq!(map.len(), 0);
}

#[test]
fn test_opaque_map_empty_map_get_none() {
    let map = OpaqueMap::new::<usize, TestStruct>();
    for i in 0..10000 {
        let result = map.get::<usize, usize, TestStruct>(&i);

        assert!(result.is_none());
    }
}

#[test]
fn test_opaque_map_insert_get1() {
    let key = 0;
    let value = TestStruct::new(100, true, f32::MIN_POSITIVE);
    let mut map = OpaqueMap::new::<usize, TestStruct>();
    map.insert(key, value);

    let expected = Some(value);
    let result = map.get::<usize, usize, TestStruct>(&key).copied();

    assert_eq!(result, expected);
}

#[test]
fn test_opaque_map_insert_get2() {
    let value = TestStruct::new(100, true, f32::MIN_POSITIVE);
    let values = [value; 4];
        
    let mut map = OpaqueMap::new::<usize, TestStruct>();
    for (key, value) in values.iter().copied().enumerate() {
        map.insert(key, value);
    }

    for key in map.keys::<usize, TestStruct>().copied() {
        let expected = Some(values[key]);
        let result = map.get::<usize, usize, TestStruct>(&key).copied();
        
        assert_eq!(result, expected);
    }
}

#[test]
fn test_opaque_map_insert_remove_contains_key1() {
    let key = 0;
    let value = TestStruct::new(100, true, f32::MIN_POSITIVE);
    let mut map = OpaqueMap::new::<usize, TestStruct>();
    map.insert(key, value);

    assert!(map.contains_key::<usize, usize, TestStruct>(&key));

    map.swap_remove::<usize, usize, TestStruct>(&key);

    assert!(!map.contains_key::<usize, usize, TestStruct>(&key));
}

#[test]
fn test_opaque_map_as_slice1() {
    let value = TestStruct::new(100, true, f32::MIN_POSITIVE);
    let values = [value; 4];
        
    let mut map = OpaqueMap::new::<usize, TestStruct>();
    for (key, value) in values.iter().copied().enumerate() {
        map.insert(key, value);
    }

    let expected = values.as_slice();
    let values: Vec<TestStruct> = map
        .as_slice::<usize, TestStruct>()
        .values()
        .copied()
        .collect();
    let result = values.as_slice();

    assert_eq!(result, expected);
}
