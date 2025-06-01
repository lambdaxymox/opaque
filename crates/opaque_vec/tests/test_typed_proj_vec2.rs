use opaque_vec::TypedProjVec;

#[test]
fn test_vec_empty_is_empty() {
    let vec: TypedProjVec<i32> = TypedProjVec::new();

    assert!(vec.is_empty());
}

#[test]
fn test_vec_empty_len() {
    let vec: TypedProjVec<i32> = TypedProjVec::new();

    assert_eq!(vec.len(), 0);
}

#[test]
fn test_vec_empty_capacity() {
    let vec: TypedProjVec<i32> = TypedProjVec::new();

    assert_eq!(vec.capacity(), 0);
}

#[test]
fn test_vec_reserve() {
    let mut vec = TypedProjVec::new();
    assert_eq!(vec.capacity(), 0);

    vec.reserve(2);
    assert!(vec.capacity() >= 2);

    for i in 0..16 {
        vec.push(i);
    }

    assert!(vec.capacity() >= 16);
    vec.reserve(16);
    assert!(vec.capacity() >= 32);

    vec.push(16);

    vec.reserve(16);
    assert!(vec.capacity() >= 33)
}

#[test]
fn test_vec_capacity_zst() {
    let vec: TypedProjVec<()> = TypedProjVec::new();

    assert_eq!(vec.capacity(), usize::MAX);
}

#[test]
fn test_vec_push_index1() {
    let mut vec = TypedProjVec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert_eq!(vec[2], 3);
}

#[test]
fn test_vec_push_index2() {
    let mut vec = TypedProjVec::new();
    let len = 64;

    for i in 0..len {
        vec.push(i);
    }

    for i in 0..len {
        assert_eq!(vec[i], i);
    }
}

#[test]
fn test_vec_push_index3() {
    let mut vec = TypedProjVec::new();
    let len = 32;

    for i in 0..len {
        vec.push(usize::MAX);
    }

    for i in 0..len {
        assert_eq!(vec[i], usize::MAX);
    }
}

#[test]
fn test_vec_push_len1() {
    let mut vec = TypedProjVec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec.len(), 3);
}

#[test]
fn test_vec_push_len2() {
    let mut vec = TypedProjVec::new();
    let len = 64;

    for i in 0..len {
        vec.push(i);
    }

    assert_eq!(vec.len(), len);
}

#[test]
fn test_vec_push_len3() {
    let mut vec = TypedProjVec::new();
    let len = 32;

    for i in 0..len {
        vec.push(usize::MAX);
    }

    assert_eq!(vec.len(), len);
}

#[test]
fn test_pop_empty() {
    let mut vec: TypedProjVec<i32> = TypedProjVec::new();

    assert_eq!(vec.pop(), None);

    for i in 0..32 {
        assert_eq!(vec.pop(), None);
    }
}

#[test]
fn test_vec_pop1() {
    let mut vec = TypedProjVec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec.len(), 3);

    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.pop(), Some(1));
    assert_eq!(vec.pop(), None);
}

#[test]
fn test_vec_pop2() {
    let mut vec = TypedProjVec::new();
    let len = 64;

    for i in 0..len {
        vec.push(i);
    }

    for i in 0..len {
        assert_eq!(vec.pop(), Some(len - (i + 1)));
    }

    assert_eq!(vec.pop(), None);
}

#[test]
fn test_vec_pop3() {
    let mut vec = TypedProjVec::new();
    let len = 32;

    for i in 0..len {
        vec.push(usize::MAX);
    }

    for i in 0..len {
        assert_eq!(vec.pop(), Some(usize::MAX));
    }

    assert_eq!(vec.pop(), None);
}

#[test]
fn test_vec_pop_len1() {
    let mut vec = TypedProjVec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec.len(), 3);

    let _ = vec.pop();

    assert_eq!(vec.len(), 2);

    let _ = vec.pop();

    assert_eq!(vec.len(), 1);

    let _ = vec.pop();

    assert_eq!(vec.len(), 0);
}

#[test]
fn test_vec_pop_len2() {
    let mut vec = TypedProjVec::new();
    let len = 64;

    for i in 0..len {
        vec.push(i);
    }

    for i in 0..len {
        let _ = vec.pop();

        assert_eq!(vec.len(), len - (i + 1));
    }

    let _ = vec.pop();

    assert_eq!(vec.len(), 0);
}

#[test]
fn test_vec_pop_len3() {
    let mut vec = TypedProjVec::new();
    let len = 32;

    for i in 0..len {
        vec.push(i);
    }

    for i in 0..len {
        let _ = vec.pop();

        assert_eq!(vec.len(), len - (i + 1));
    }

    let _ = vec.pop();

    assert_eq!(vec.len(), 0);
}

#[test]
fn test_vec_extend1() {
    let mut vec = TypedProjVec::new();

    assert_eq!(vec.as_slice(), &[]);

    vec.extend([1, 2, 3]);

    assert_eq!(vec.as_slice(), [1, 2, 3]);
}

#[test]
fn test_vec_extend2() {
    let mut vec1 = TypedProjVec::new();

    for i in 0..16 {
        vec1.push(i);
    }

    let mut vec2 = TypedProjVec::new();
    vec2.extend(0..16);

    assert_eq!(vec1, vec2);
}

#[test]
fn test_vec_extend3() {
    let mut vec1 = TypedProjVec::new();
    let mut vec2 = TypedProjVec::new();

    for i in 0..3 {
        vec1.push(i);
    }

    vec1.extend(3..16);

    vec2.extend(0..16);

    assert_eq!(vec1, vec2);
}

#[test]
fn test_vec_clone1() {
    let vec: TypedProjVec<isize> = TypedProjVec::new();
    let cloned_vec = vec.clone();

    assert_eq!(vec, cloned_vec);
}

#[test]
fn test_vec_clone2() {
    let mut vec: TypedProjVec<isize> = TypedProjVec::new();
    vec.extend(0..8);

    let cloned_vec = vec.clone();

    assert_eq!(vec, cloned_vec);
}

#[test]
fn test_vec_clone3() {
    let mut vec: TypedProjVec<isize> = TypedProjVec::new();
    for i in 0..3 {
        vec.push(i);
    }

    vec.extend(0..10);

    let cloned_vec = vec.clone();

    assert_eq!(vec, cloned_vec);
}

#[test]
fn test_vec_clone_len1() {
    let vec: TypedProjVec<isize> = TypedProjVec::new();
    let cloned_vec = vec.clone();

    assert_eq!(vec.len(), cloned_vec.len());
}

#[test]
fn test_vec_clone_len2() {
    let mut vec: TypedProjVec<isize> = TypedProjVec::new();
    vec.extend(0..8);

    let cloned_vec = vec.clone();

    assert_eq!(vec.len(), cloned_vec.len());
}

#[test]
fn test_vec_clone_len3() {
    let mut vec: TypedProjVec<isize> = TypedProjVec::new();
    for i in 0..3 {
        vec.push(i);
    }

    vec.extend(0..10);

    let cloned_vec = vec.clone();

    assert_eq!(vec.len(), cloned_vec.len());
}

#[test]
fn test_vec_slice_from_ref() {
    let vec = TypedProjVec::from(&[1, 2, 3, 4, 5, 6]);

    assert_eq!(&vec[0..1], &[1]);
    assert_eq!(&vec[1..2], &[2]);
    assert_eq!(&vec[2..3], &[3]);
    assert_eq!(&vec[3..4], &[4]);
    assert_eq!(&vec[4..5], &[5]);
    assert_eq!(&vec[5..6], &[6]);

    assert_eq!(&vec[0..2], &[1, 2]);
    assert_eq!(&vec[1..3], &[2, 3]);
    assert_eq!(&vec[2..4], &[3, 4]);
    assert_eq!(&vec[3..5], &[4, 5]);
    assert_eq!(&vec[4..6], &[5, 6]);

    assert_eq!(&vec[0..3], &[1, 2, 3]);
    assert_eq!(&vec[1..4], &[2, 3, 4]);
    assert_eq!(&vec[2..5], &[3, 4, 5]);
    assert_eq!(&vec[3..6], &[4, 5, 6]);

    assert_eq!(&vec[0..4], &[1, 2, 3, 4]);
    assert_eq!(&vec[1..5], &[2, 3, 4, 5]);
    assert_eq!(&vec[2..6], &[3, 4, 5, 6]);

    assert_eq!(&vec[0..5], &[1, 2, 3, 4, 5]);
    assert_eq!(&vec[1..6], &[2, 3, 4, 5, 6]);

    assert_eq!(&vec[0..6], &[1, 2, 3, 4, 5, 6]);
    assert_eq!(&vec[..], &[1, 2, 3, 4, 5, 6]);
    assert_eq!(vec.as_slice(), &[1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_vec_slice_from_mut1() {
    let mut vec = TypedProjVec::from(&[1, 2, 3, 4, 5, 6]);
    {
        let slice = &mut vec[..2];

        assert_eq!(slice, &[1, 2]);

        for i in 0..slice.len() {
            slice[i] += 20;
        }
    }

    assert_eq!(vec.as_slice(), &[21, 22, 3, 4, 5, 6]);
}

#[test]
fn test_vec_slice_from_mut2() {
    let mut vec = TypedProjVec::from(&[1, 2, 3, 4, 5, 6]);
    {
        let slice = &mut vec[2..];

        assert_eq!(slice, &[3, 4, 5, 6]);

        for i in 0..slice.len() {
            slice[i] += 20;
        }
    }

    assert_eq!(vec.as_slice(), &[1, 2, 23, 24, 25, 26]);
}

#[test]
fn test_vec_dedup() {
    fn test_case(this: TypedProjVec<i32>, that: TypedProjVec<i32>) {
        let mut vec = this;
        vec.dedup();
        assert_eq!(vec, that);
    }

    test_case(TypedProjVec::from(&[]), TypedProjVec::from(&[]));
    test_case(TypedProjVec::from(&[1]), TypedProjVec::from(&[1]));
    test_case(TypedProjVec::from(&[1, 1]), TypedProjVec::from(&[1]));
    test_case(TypedProjVec::from(&[1, 2, 3]), TypedProjVec::from(&[1, 2, 3]));
    test_case(TypedProjVec::from(&[1, 1, 2, 3]), TypedProjVec::from(&[1, 2, 3]));
    test_case(TypedProjVec::from(&[1, 2, 2, 3]), TypedProjVec::from(&[1, 2, 3]));
    test_case(TypedProjVec::from(&[1, 2, 3, 3]), TypedProjVec::from(&[1, 2, 3]));
    test_case(TypedProjVec::from(&[1, 1, 2, 2, 2, 3, 3]), TypedProjVec::from(&[1, 2, 3]));
    test_case(TypedProjVec::from(&[1, 1, 2, 2, 2, 3, 3, 4, 5, 5, 5, 6, 7, 7]), TypedProjVec::from(&[1, 2, 3, 4, 5, 6, 7]));
}

#[test]
fn test_vec_dedup_by_key() {
    fn test_case(this: TypedProjVec<i32>, that: TypedProjVec<i32>) {
        let mut vec = this;
        vec.dedup_by_key(|i| *i / 10);
        assert_eq!(vec, that);
    }

    test_case(TypedProjVec::from(&[]), TypedProjVec::from(&[]));
    test_case(TypedProjVec::from(&[10]), TypedProjVec::from(&[10]));
    test_case(TypedProjVec::from(&[10, 11]), TypedProjVec::from(&[10]));
    test_case(TypedProjVec::from(&[10, 20, 30]), TypedProjVec::from(&[10, 20, 30]));
    test_case(TypedProjVec::from(&[10, 11, 20, 30]), TypedProjVec::from(&[10, 20, 30]));
    test_case(TypedProjVec::from(&[10, 20, 21, 30]), TypedProjVec::from(&[10, 20, 30]));
    test_case(TypedProjVec::from(&[10, 20, 30, 31]), TypedProjVec::from(&[10, 20, 30]));
    test_case(TypedProjVec::from(&[10, 11, 20, 21, 22, 30, 31]), TypedProjVec::from(&[10, 20, 30]));
}

#[test]
#[should_panic]
fn test_vec_swap_remove_out_of_bounds1() {
    let mut vec = TypedProjVec::from(&[0]);
    vec.swap_remove(1);
}

#[test]
#[should_panic]
fn test_vec_swap_remove_out_of_bounds2() {
    let mut vec = TypedProjVec::from(&[0]);
    vec.swap_remove(usize::MAX);
}

#[test]
fn test_truncate_len1() {
    let mut vec = TypedProjVec::from([1]);

    vec.truncate(1);
    assert_eq!(vec.len(), 1);
    vec.truncate(0);
    assert_eq!(vec.len(), 0);
}

#[test]
fn test_truncate_len2() {
    let mut vec = TypedProjVec::from([1, 2, 3, 4, 5, 6]);

    vec.truncate(6);
    assert_eq!(vec.len(), 6);
    vec.truncate(5);
    assert_eq!(vec.len(), 5);
    vec.truncate(3);
    assert_eq!(vec.len(), 3);
    vec.truncate(0);
    assert_eq!(vec.len(), 0);
}


#[test]
fn test_truncate_drop1() {
    static mut DROP_COUNT: usize = 0;

    fn get_drop_count() -> usize { unsafe { DROP_COUNT } }

    struct Value { data: i32 }

    impl Value {
        fn new(data: i32) -> Self { Self { data, } }
    }

    impl Drop for Value {
        fn drop(&mut self) {
            unsafe { DROP_COUNT += 1; }
        }
    }

    let mut vec = TypedProjVec::from([Value::new(1)]);

    vec.truncate(1);
    assert_eq!(get_drop_count(), 0);
    vec.truncate(0);
    assert_eq!(get_drop_count(), 1);
}

#[test]
fn test_truncate_drop2() {
    static mut DROP_COUNT: usize = 0;

    fn get_drop_count() -> usize { unsafe { DROP_COUNT } }

    struct Value { data: i32 }

    impl Value {
        fn new(data: i32) -> Self { Self { data, } }
    }

    impl Drop for Value {
        fn drop(&mut self) {
            unsafe { DROP_COUNT += 1; }
        }
    }

    let mut vec = TypedProjVec::from([
        Value::new(1),
        Value::new(2),
        Value::new(3),
        Value::new(4),
        Value::new(5),
        Value::new(6),
    ]);

    vec.truncate(6);
    assert_eq!(get_drop_count(), 0);
    vec.truncate(5);
    assert_eq!(get_drop_count(), 1);
    vec.truncate(3);
    assert_eq!(get_drop_count(), 3);
    vec.truncate(0);
    assert_eq!(get_drop_count(), 6);
}

#[test]
#[should_panic]
fn test_vec_truncate_fail() {
    struct BadValue { data: usize, }

    impl BadValue {
        fn new(data: usize) -> Self { BadValue { data, } }
    }

    impl Drop for BadValue {
        fn drop(&mut self) {
            let BadValue { ref mut data} = *self;
            if *data == 0xbadbeef {
                panic!("BadElem panic: 0xbadbeef")
            }
        }
    }

    let mut vec = TypedProjVec::from([
        BadValue::new(1),
        BadValue::new(2),
        BadValue::new(0xbadbeef),
        BadValue::new(4),
    ]);

    vec.truncate(0);
}
