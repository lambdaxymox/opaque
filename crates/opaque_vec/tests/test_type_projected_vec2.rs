use opaque_vec::TypeProjectedVec;

use std::alloc;

#[test]
fn test_vec_empty_is_empty() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::new();

    assert!(vec.is_empty());
}

#[test]
fn test_vec_empty_len() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::new();

    assert_eq!(vec.len(), 0);
}

#[test]
fn test_vec_empty_capacity() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::new();

    assert_eq!(vec.capacity(), 0);
}

#[test]
fn test_vec_reserve() {
    let mut vec = TypeProjectedVec::new();
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
    let vec: TypeProjectedVec<()> = TypeProjectedVec::new();

    assert_eq!(vec.capacity(), usize::MAX);
}

#[test]
fn test_vec_push_index1() {
    let mut vec = TypeProjectedVec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec[0], 1);
    assert_eq!(vec[1], 2);
    assert_eq!(vec[2], 3);
}

#[test]
fn test_vec_push_index2() {
    let mut vec = TypeProjectedVec::new();
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
    let mut vec = TypeProjectedVec::new();
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
    let mut vec = TypeProjectedVec::new();

    vec.push(1);
    vec.push(2);
    vec.push(3);

    assert_eq!(vec.len(), 3);
}

#[test]
fn test_vec_push_len2() {
    let mut vec = TypeProjectedVec::new();
    let len = 64;

    for i in 0..len {
        vec.push(i);
    }

    assert_eq!(vec.len(), len);
}

#[test]
fn test_vec_push_len3() {
    let mut vec = TypeProjectedVec::new();
    let len = 32;

    for i in 0..len {
        vec.push(usize::MAX);
    }

    assert_eq!(vec.len(), len);
}

#[test]
fn test_vec_push_zst_capacity() {
    for len in [0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
        let vec: TypeProjectedVec<()> = TypeProjectedVec::with_capacity(len);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), usize::MAX);
    }
}

#[test]
fn test_vec_pop_empty() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::new();

    assert_eq!(vec.pop(), None);

    for i in 0..32 {
        assert_eq!(vec.pop(), None);
    }
}

#[test]
fn test_vec_pop1() {
    let mut vec = TypeProjectedVec::new();

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
    let mut vec = TypeProjectedVec::new();
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
    let mut vec = TypeProjectedVec::new();
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
    let mut vec = TypeProjectedVec::new();

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
    let mut vec = TypeProjectedVec::new();
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
    let mut vec = TypeProjectedVec::new();
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
    let mut vec = TypeProjectedVec::new();

    assert_eq!(vec.as_slice(), &[]);

    vec.extend([1, 2, 3]);

    assert_eq!(vec.as_slice(), [1, 2, 3]);
}

#[test]
fn test_vec_extend2() {
    let mut vec1 = TypeProjectedVec::new();

    for i in 0..16 {
        vec1.push(i);
    }

    let mut vec2 = TypeProjectedVec::new();
    vec2.extend(0..16);

    assert_eq!(vec1, vec2);
}

#[test]
fn test_vec_extend3() {
    let mut vec1 = TypeProjectedVec::new();
    let mut vec2 = TypeProjectedVec::new();

    for i in 0..3 {
        vec1.push(i);
    }

    vec1.extend(3..16);

    vec2.extend(0..16);

    assert_eq!(vec1, vec2);
}

#[test]
fn test_vec_clone1() {
    let vec: TypeProjectedVec<isize> = TypeProjectedVec::new();
    let cloned_vec = vec.clone();

    assert_eq!(vec, cloned_vec);
}

#[test]
fn test_vec_clone2() {
    let mut vec: TypeProjectedVec<isize> = TypeProjectedVec::new();
    vec.extend(0..8);

    let cloned_vec = vec.clone();

    assert_eq!(vec, cloned_vec);
}

#[test]
fn test_vec_clone3() {
    let mut vec: TypeProjectedVec<isize> = TypeProjectedVec::new();
    for i in 0..3 {
        vec.push(i);
    }

    vec.extend(0..10);

    let cloned_vec = vec.clone();

    assert_eq!(vec, cloned_vec);
}

#[test]
fn test_vec_clone_len1() {
    let vec: TypeProjectedVec<isize> = TypeProjectedVec::new();
    let cloned_vec = vec.clone();

    assert_eq!(vec.len(), cloned_vec.len());
}

#[test]
fn test_vec_clone_len2() {
    let mut vec: TypeProjectedVec<isize> = TypeProjectedVec::new();
    vec.extend(0..8);

    let cloned_vec = vec.clone();

    assert_eq!(vec.len(), cloned_vec.len());
}

#[test]
fn test_vec_clone_len3() {
    let mut vec: TypeProjectedVec<isize> = TypeProjectedVec::new();
    for i in 0..3 {
        vec.push(i);
    }

    vec.extend(0..10);

    let cloned_vec = vec.clone();

    assert_eq!(vec.len(), cloned_vec.len());
}

#[test]
fn test_vec_slice_from_ref() {
    let vec = TypeProjectedVec::from(&[1, 2, 3, 4, 5, 6]);

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
    let mut vec = TypeProjectedVec::from(&[1, 2, 3, 4, 5, 6]);
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
    let mut vec = TypeProjectedVec::from(&[1, 2, 3, 4, 5, 6]);
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
    fn test_case(this: TypeProjectedVec<i32>, that: TypeProjectedVec<i32>) {
        let mut vec = this;
        vec.dedup();
        assert_eq!(vec, that);
    }

    test_case(TypeProjectedVec::from(&[]), TypeProjectedVec::from(&[]));
    test_case(TypeProjectedVec::from(&[1]), TypeProjectedVec::from(&[1]));
    test_case(TypeProjectedVec::from(&[1, 1]), TypeProjectedVec::from(&[1]));
    test_case(TypeProjectedVec::from(&[1, 2, 3]), TypeProjectedVec::from(&[1, 2, 3]));
    test_case(TypeProjectedVec::from(&[1, 1, 2, 3]), TypeProjectedVec::from(&[1, 2, 3]));
    test_case(TypeProjectedVec::from(&[1, 2, 2, 3]), TypeProjectedVec::from(&[1, 2, 3]));
    test_case(TypeProjectedVec::from(&[1, 2, 3, 3]), TypeProjectedVec::from(&[1, 2, 3]));
    test_case(TypeProjectedVec::from(&[1, 1, 2, 2, 2, 3, 3]), TypeProjectedVec::from(&[1, 2, 3]));
    test_case(TypeProjectedVec::from(&[1, 1, 2, 2, 2, 3, 3, 4, 5, 5, 5, 6, 7, 7]), TypeProjectedVec::from(&[1, 2, 3, 4, 5, 6, 7]));
}

#[test]
fn test_vec_dedup_by_key() {
    fn test_case(this: TypeProjectedVec<i32>, that: TypeProjectedVec<i32>) {
        let mut vec = this;
        vec.dedup_by_key(|i| *i / 10);
        assert_eq!(vec, that);
    }

    test_case(TypeProjectedVec::from(&[]), TypeProjectedVec::from(&[]));
    test_case(TypeProjectedVec::from(&[10]), TypeProjectedVec::from(&[10]));
    test_case(TypeProjectedVec::from(&[10, 11]), TypeProjectedVec::from(&[10]));
    test_case(TypeProjectedVec::from(&[10, 20, 30]), TypeProjectedVec::from(&[10, 20, 30]));
    test_case(TypeProjectedVec::from(&[10, 11, 20, 30]), TypeProjectedVec::from(&[10, 20, 30]));
    test_case(TypeProjectedVec::from(&[10, 20, 21, 30]), TypeProjectedVec::from(&[10, 20, 30]));
    test_case(TypeProjectedVec::from(&[10, 20, 30, 31]), TypeProjectedVec::from(&[10, 20, 30]));
    test_case(TypeProjectedVec::from(&[10, 11, 20, 21, 22, 30, 31]), TypeProjectedVec::from(&[10, 20, 30]));
}

#[test]
#[should_panic]
fn test_vec_swap_remove_out_of_bounds1() {
    let mut vec = TypeProjectedVec::from(&[0]);
    vec.swap_remove(1);
}

#[test]
#[should_panic]
fn test_vec_swap_remove_out_of_bounds2() {
    let mut vec = TypeProjectedVec::from(&[0]);
    vec.swap_remove(usize::MAX);
}

#[test]
fn test_vec_truncate_len1() {
    let mut vec = TypeProjectedVec::from([1]);

    vec.truncate(1);
    assert_eq!(vec.len(), 1);
    vec.truncate(0);
    assert_eq!(vec.len(), 0);
}

#[test]
fn test_vec_truncate_len2() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);

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
fn test_vec_truncate_drop1() {
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

    let mut vec = TypeProjectedVec::from([Value::new(1)]);

    vec.truncate(1);
    assert_eq!(get_drop_count(), 0);
    vec.truncate(0);
    assert_eq!(get_drop_count(), 1);
}

#[test]
fn test_vec_truncate_drop2() {
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

    let mut vec = TypeProjectedVec::from([
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

    let mut vec = TypeProjectedVec::from([
        BadValue::new(1),
        BadValue::new(2),
        BadValue::new(0xbadbeef),
        BadValue::new(4),
    ]);

    vec.truncate(0);
}

#[test]
fn test_vec_into_iter_clone_empty() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::with_capacity(10);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.into_iter() {
        cloned_vec.push(value);
    }

    assert!(cloned_vec.is_empty());
}

#[test]
fn test_vec_into_iter_clone1() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1]);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.clone().into_iter() {
        cloned_vec.push(value);
    }

    assert_eq!(cloned_vec, vec);
}

#[test]
fn test_vec_into_iter_clone2() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1, 2]);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.clone().into_iter() {
        cloned_vec.push(value);
    }

    assert_eq!(cloned_vec, vec);
}

#[test]
fn test_vec_into_iter_clone3() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1, 2, 3, 4]);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.clone().into_iter() {
        cloned_vec.push(value);
    }

    assert_eq!(cloned_vec, vec);
}

#[test]
fn test_vec_into_iter_clone4() {
    let vec = TypeProjectedVec::from([String::from("foo")]);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.clone().into_iter() {
        cloned_vec.push(value);
    }

    assert_eq!(cloned_vec, vec);
}

#[test]
fn test_vec_into_iter_clone5() {
    let vec = TypeProjectedVec::from([String::from("foo"), String::from("bar")]);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.clone().into_iter() {
        cloned_vec.push(value);
    }

    assert_eq!(cloned_vec, vec);
}

#[test]
fn test_vec_into_iter_clone6() {
    let vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.clone().into_iter() {
        cloned_vec.push(value);
    }

    assert_eq!(cloned_vec, vec);
}

#[test]
fn test_vec_into_iter_partial0() {
    let vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
        String::from("garply"),
    ]);
    let expected = TypeProjectedVec::from([]);
    let mut result = TypeProjectedVec::new();
    for value in vec.clone().into_iter().take(0) {
        result.push(value);
    }

    assert_eq!(result, expected);
}


#[test]
fn test_vec_into_iter_partial1() {
    let vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
        String::from("garply"),
    ]);
    let expected = TypeProjectedVec::from([
        String::from("foo"),
    ]);
    let mut result = TypeProjectedVec::new();
    for value in vec.clone().into_iter().take(1) {
        result.push(value);
    }

    assert_eq!(result, expected);
}


#[test]
fn test_vec_into_iter_partial2() {
    let vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
        String::from("garply"),
    ]);
    let expected = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
    ]);
    let mut result = TypeProjectedVec::new();
    for value in vec.clone().into_iter().take(2) {
        result.push(value);
    }

    assert_eq!(result, expected);
}


#[test]
fn test_vec_into_iter_partial3() {
    let vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
        String::from("garply"),
    ]);
    let expected = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
    ]);
    let mut result = TypeProjectedVec::new();
    for value in vec.clone().into_iter().take(3) {
        result.push(value);
    }

    assert_eq!(result, expected);
}


#[test]
fn test_vec_into_iter_partial4() {
    let vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
        String::from("garply"),
    ]);
    let expected = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let mut result = TypeProjectedVec::new();
    for value in vec.clone().into_iter().take(4) {
        result.push(value);
    }

    assert_eq!(result, expected);
}

#[test]
fn test_vec_into_iter_partial5() {
    let vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
        String::from("garply"),
    ]);
    let expected = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
    ]);
    let mut result = TypeProjectedVec::new();
    for value in vec.clone().into_iter().take(5) {
        result.push(value);
    }

    assert_eq!(result, expected);
}

#[test]
fn test_vec_into_iter_as_slice1() {
    let vec = TypeProjectedVec::from(["foo", "bar", "baz", "quux"]);
    let mut iter = vec.into_iter();
    assert_eq!(iter.as_slice(), ["foo", "bar", "baz", "quux"]);
    let _ = iter.next();
    assert_eq!(iter.as_slice(), ["bar", "baz", "quux"]);
    let _ = iter.next();
    assert_eq!(iter.as_slice(), ["baz", "quux"]);
    let _ = iter.next();
    assert_eq!(iter.as_slice(), ["quux"]);
    let _ = iter.next();
    assert!(iter.as_slice().is_empty());
}

#[test]
fn test_vec_into_iter_as_mut_slice1() {
    let vec = TypeProjectedVec::from(["foo", "bar", "baz", "quux"]);
    let mut iter = vec.into_iter();
    assert_eq!(iter.as_mut_slice(), ["foo", "bar", "baz", "quux"]);
    let _ = iter.next();
    assert_eq!(iter.as_mut_slice(), ["bar", "baz", "quux"]);
    let _ = iter.next();
    assert_eq!(iter.as_mut_slice(), ["baz", "quux"]);
    let _ = iter.next();
    assert_eq!(iter.as_mut_slice(), ["quux"]);
    let _ = iter.next();
    assert!(iter.as_mut_slice().is_empty());
}

#[test]
fn test_vec_into_iter_as_mut_slice2() {
    let vec = TypeProjectedVec::from(["foo", "bar", "baz", "quux"]);
    let mut iter = vec.into_iter();
    assert_eq!(iter.as_mut_slice(), ["foo", "bar", "baz", "quux"]);
    iter.as_mut_slice()[0] = "FOO";
    assert_eq!(iter.next(), Some("FOO"));
    assert_eq!(iter.as_mut_slice(), ["bar", "baz", "quux"]);
    iter.as_mut_slice()[0] = "BAR";
    assert_eq!(iter.next(), Some("BAR"));
    assert_eq!(iter.as_mut_slice(), ["baz", "quux"]);
    iter.as_mut_slice()[0] = "BAZ";
    assert_eq!(iter.next(), Some("BAZ"));
    assert_eq!(iter.as_mut_slice(), ["quux"]);
    iter.as_mut_slice()[0] = "QUUX";
    assert_eq!(iter.next(), Some("QUUX"));
    assert!(iter.as_mut_slice().is_empty());
    assert_eq!(iter.next(), None);
}

#[test]
fn test_vec_drain_empty() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([]);
    let expected = TypeProjectedVec::new();
    let result: TypeProjectedVec<i32> = vec.drain(..).collect();

    assert_eq!(result, expected);
}

#[test]
fn test_vec_drain_entire_range1() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1]);
    let expected_from_drain = TypeProjectedVec::from([1]);
    let result_from_drain: TypeProjectedVec<i32> = vec.drain(..).collect();
    let expected_vec = TypeProjectedVec::from([]);

    assert_eq!(result_from_drain, expected_from_drain);
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_vec_drain_entire_range2() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1, 2]);
    let expected_from_drain = TypeProjectedVec::from([1, 2]);
    let result_from_drain: TypeProjectedVec<i32> = vec.drain(..).collect();
    let expected_vec = TypeProjectedVec::from([]);

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_vec_drain_entire_range3() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1, 2, 3]);
    let expected_from_drain = TypeProjectedVec::from([1, 2, 3]);
    let result_from_drain: TypeProjectedVec<i32> = vec.drain(..).collect();
    let expected_vec = TypeProjectedVec::from([]);

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_vec_drain_entire_range4() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1, 2, 3, 4]);
    let expected_from_drain = TypeProjectedVec::from([1, 2, 3, 4]);
    let result_from_drain: TypeProjectedVec<i32> = vec.drain(..).collect();
    let expected_vec = TypeProjectedVec::from([]);

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_vec_drain_entire_range5() {
    let mut vec: TypeProjectedVec<String> = TypeProjectedVec::from([String::from("foo")]);
    let expected_from_drain = vec.clone();
    let result_from_drain: TypeProjectedVec<String> = vec.drain(..).collect();
    let expected_vec: TypeProjectedVec<String> = TypeProjectedVec::from([]);

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_vec_drain_entire_range6() {
    let mut vec: TypeProjectedVec<String> = TypeProjectedVec::from([String::from("foo"), String::from("bar")]);
    let expected_from_drain = vec.clone();
    let result_from_drain: TypeProjectedVec<String> = vec.drain(..).collect();
    let expected_vec: TypeProjectedVec<String> = TypeProjectedVec::from([]);

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_vec_drain_entire_range7() {
    let mut vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let expected_from_drain = vec.clone();
    let result_from_drain: TypeProjectedVec<String> = vec.drain(..).collect();
    let expected_vec: TypeProjectedVec<String> = TypeProjectedVec::from([]);

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_vec_drain_partial_range1() {
    let mut vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let expected_from_drain: TypeProjectedVec<String> = vec[0..2].iter().cloned().collect();
    let result_from_drain: TypeProjectedVec<String> = vec.drain(0..2).collect();
    let expected_vec: TypeProjectedVec<String> = TypeProjectedVec::from([
        String::from("baz"),
        String::from("quux"),
    ]);

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_vec_drain_partial_range2() {
    let mut vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let expected_from_drain: TypeProjectedVec<String> = vec[1..3].iter().cloned().collect();
    let result_from_drain: TypeProjectedVec<String> = vec.drain(1..3).collect();
    let expected_vec: TypeProjectedVec<String> = TypeProjectedVec::from([
        String::from("foo"),
        String::from("quux"),
    ]);

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_vec_drain_partial_range3() {
    let mut vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let expected_from_drain: TypeProjectedVec<String> = vec[1..].iter().cloned().collect();
    let result_from_drain: TypeProjectedVec<String> = vec.drain(1..).collect();
    let expected_vec: TypeProjectedVec<String> = TypeProjectedVec::from([
        String::from("foo"),
    ]);

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_vec_drain_partial_range4() {
    let mut vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let expected_from_drain: TypeProjectedVec<String> = vec[3..].iter().cloned().collect();
    let result_from_drain: TypeProjectedVec<String> = vec.drain(3..).collect();
    let expected_vec: TypeProjectedVec<String> = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
    ]);

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_vec_splice1() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    let splice_data = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    vec.splice(2..4, splice_data);

    assert_eq!(vec.as_slice(), &[1, 2, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 5, 6]);
}

#[test]
fn test_vec_splice2() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    let splice_data = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    vec.splice(4.., splice_data);

    assert_eq!(vec.as_slice(), &[1, 2, 3, 4, i32::MAX, i32::MAX, i32::MAX, i32::MAX]);
}

#[test]
fn test_vec_splice3() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    let splice_data = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    vec.splice(0.., splice_data);

    assert_eq!(vec.as_slice(), &[i32::MAX, i32::MAX, i32::MAX, i32::MAX]);
}

#[test]
fn test_vec_splice4() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    let splice_data = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    vec.splice(0..1, splice_data);

    assert_eq!(vec.as_slice(), &[i32::MAX, i32::MAX, i32::MAX, i32::MAX, 2, 3, 4, 5, 6]);
}

#[test]
fn test_vec_splice5() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    vec.splice(1..3, Some(i32::MAX));

    assert_eq!(vec.as_slice(), &[1, i32::MAX, 4, 5, 6]);
}

#[test]
fn test_vec_splice6() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    vec.splice(1..3, None);

    assert_eq!(vec.as_slice(), &[1, 4, 5, 6]);
}

#[test]
fn test_vec_debug_fmt_empty_unit() {
    let vec: TypeProjectedVec<()> = TypeProjectedVec::from([]);
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_vec_debug_fmt_empty_u8() {
    let vec: TypeProjectedVec<u8> = TypeProjectedVec::from([]);
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_vec_debug_fmt_empty_u16() {
    let vec: TypeProjectedVec<u16> = TypeProjectedVec::from([]);
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_vec_debug_fmt_empty_u32() {
    let vec: TypeProjectedVec<u8> = TypeProjectedVec::from([]);
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_vec_debug_fmt_empty_u64() {
    let vec: TypeProjectedVec<u8> = TypeProjectedVec::from([]);
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_vec_debug_fmt_empty_usize() {
    let vec: TypeProjectedVec<u8> = TypeProjectedVec::from([]);
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_vec_debug_fmt_empty_string() {
    let vec: TypeProjectedVec<u8> = TypeProjectedVec::from([]);
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_vec_indexing() {
    let vec = TypeProjectedVec::from([10, 20, 30]);

    assert_eq!(vec[0], 10);
    assert_eq!(vec[1], 20);
    assert_eq!(vec[2], 30);

    let mut idx = 0;

    assert_eq!(vec[idx], 10);
    idx += 1;
    assert_eq!(vec[idx], 20);
    idx += 1;
    assert_eq!(vec[idx], 30);
}

#[test]
#[should_panic]
fn test_vec_indexing_out_of_bounds1() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([]);
    let _ = vec[0];

    assert!(true);
}

#[test]
#[should_panic]
fn test_vec_indexing_out_of_bounds2() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([10]);
    let _ = vec[1];

    assert!(true);
}

#[test]
#[should_panic]
fn test_vec_indexing_out_of_bounds3() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([10, 20]);
    let _ = vec[2];

    assert!(true);
}

#[test]
#[should_panic]
fn test_vec_indexing_out_of_bounds4() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([10, 20, 30]);
    let _ = vec[3];

    assert!(true);
}

#[test]
#[should_panic]
fn test_vec_slice_out_of_bounds1() {
    let vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    let _ = &vec[!0..];
}

#[test]
#[should_panic]
fn test_vec_slice_out_of_bounds2() {
    let vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    let _ = &vec[..7];
}

#[test]
#[should_panic]
fn test_vec_slice_out_of_bounds3() {
    let vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    let _ = &vec[!0..5];
}

#[test]
#[should_panic]
fn test_vec_slice_out_of_bounds4() {
    let vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    let _ = &vec[1..7];
}

#[test]
#[should_panic]
fn test_vec_slice_out_of_bounds5() {
    let vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    let _ = &vec[3..2];
}

#[cfg(feature = "nightly")]
#[test]
fn test_vec_into_boxed_slice() {
    let vec = TypeProjectedVec::from([1, 2, 3]);
    let boxed_slice = vec.into_boxed_slice();

    assert_eq!(&*boxed_slice, [1, 2, 3]);
}

#[cfg(feature = "nightly")]
#[test]
fn test_vec_into_boxed_slice_from_boxed_slice() {
    let vec = TypeProjectedVec::from([1, 2, 3]);
    let boxed_slice = vec.into_boxed_slice();
    let new_vec = TypeProjectedVec::from(boxed_slice);

    assert_eq!(&*new_vec, [1, 2, 3]);
    assert_eq!(new_vec.as_slice(), [1, 2, 3]);
}

#[test]
fn test_vec_append1() {
    let mut vec1 = TypeProjectedVec::from([1, 2, 3]);
    let mut vec2 = TypeProjectedVec::from([4, 5, 6, 7]);
    vec1.append(&mut vec2);

    assert_eq!(&*vec1, [1, 2, 3, 4, 5, 6, 7]);
    assert_eq!(vec1.as_slice(), [1, 2, 3, 4, 5, 6, 7]);
    assert!(vec2.is_empty());
}

#[test]
fn test_vec_append2() {
    let mut vec1 = TypeProjectedVec::from([1, 2, 3]);
    let mut vec2 = TypeProjectedVec::from([]);
    vec1.append(&mut vec2);

    assert_eq!(&*vec1, [1, 2, 3]);
    assert_eq!(vec1.as_slice(), [1, 2, 3]);
    assert!(vec2.is_empty());
}

#[test]
fn test_vec_split_off1() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    let vec_ptr = vec.as_ptr();
    let old_capacity = vec.capacity();

    let split_vec = vec.split_off(4);
    assert_eq!(vec.as_slice(), [1, 2, 3, 4]);
    assert_eq!(split_vec.as_slice(), [5, 6]);
    assert_eq!(vec.capacity(), old_capacity);
    assert_eq!(vec.as_ptr(), vec_ptr);
}

#[test]
fn test_vec_split_off2() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    let vec_ptr = vec.as_ptr();
    let old_capacity = vec.capacity();

    let split_vec = vec.split_off(0);
    assert_eq!(vec.as_slice(), []);
    assert_eq!(split_vec.as_slice(), [1, 2, 3, 4, 5, 6]);
    assert_eq!(vec.capacity(), old_capacity);
    assert_eq!(vec.as_ptr(), vec_ptr);
}

#[test]
fn test_vec_split_off3() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    let vec_ptr = vec.as_ptr();
    let old_capacity = vec.capacity();

    let split_vec = vec.split_off(6);
    assert_eq!(vec.as_slice(), [1, 2, 3, 4, 5, 6]);
    assert_eq!(split_vec.as_slice(), []);
    assert_eq!(vec.capacity(), old_capacity);
    assert_eq!(vec.as_ptr(), vec_ptr);
}

#[test]
fn test_vec_reserve_exact() {
    let mut vec = TypeProjectedVec::new();
    assert_eq!(vec.capacity(), 0);

    vec.reserve_exact(2);
    assert!(vec.capacity() >= 2);

    for i in 0..16 {
        vec.push(i);
    }

    assert!(vec.capacity() >= 16);
    vec.reserve_exact(16);
    assert!(vec.capacity() >= 32);

    vec.push(16);

    vec.reserve_exact(16);
    assert!(vec.capacity() >= 33)
}

#[test]
fn test_vec_extract_if_empty_true() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::new();
    assert_eq!(vec.len(), 0);
    {
        let mut iter = vec.extract_if(.., |_| true);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(),(0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(),(0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(),(0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(),(0, Some(0)));
    }

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.as_slice(), []);
}

#[test]
fn test_vec_extract_if_empty_false() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::new();
    assert_eq!(vec.len(), 0);
    {
        let mut iter = vec.extract_if(.., |_| false);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(),(0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(),(0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(),(0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(),(0, Some(0)));
    }

    assert_eq!(vec.len(), 0);
    assert_eq!(vec.as_slice(), []);
}

#[test]
fn test_vec_extract_if_total_true() {
    let mut vec = TypeProjectedVec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let old_length = vec.len();
    let mut count = 0;
    {
        let mut iter = vec.extract_if(.., |_| true);
        while let Some(_) = iter.next() {
            count += 1;
            assert_eq!(iter.size_hint(), (0, Some(old_length - count)));
        }

        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    assert_eq!(count, 11);
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.as_slice(), []);
}

#[test]
fn test_vec_extract_if_total_false() {
    let mut vec = TypeProjectedVec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let old_length = vec.len();
    let mut count = 0;
    {
        let mut iter = vec.extract_if(.., |_| false);
        while let Some(_) = iter.next() {
            count += 1;
            assert_eq!(iter.size_hint(), (0, Some(old_length - count)));
        }

        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    assert_eq!(count, 0);
    assert_eq!(vec.len(), old_length);
    assert_eq!(vec.as_slice(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_vec_extract_if_partial_true() {
    let mut vec = TypeProjectedVec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let old_length = vec.len();
    let mut count = 0;
    {
        let mut iter = vec.extract_if(2..8, |_| true);
        while let Some(_) = iter.next() {
            count += 1;
        }

        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    assert_eq!(count, 6);
    assert_eq!(vec.len(), 5);
    assert_eq!(vec.as_slice(), [0, 1, 8, 9, 10]);
}

#[test]
fn test_vec_extract_if_partial_false() {
    let mut vec = TypeProjectedVec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let old_length = vec.len();
    let mut count = 0;
    {
        let mut iter = vec.extract_if(2..8, |_| false);
        while let Some(_) = iter.next() {
            count += 1;
        }

        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    assert_eq!(count, 0);
    assert_eq!(vec.len(), old_length);
    assert_eq!(vec.as_slice(), [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
#[should_panic]
fn test_vec_extract_if_out_of_bounds() {
    let mut vec = TypeProjectedVec::from([1, 2, 3]);
    let _ = vec.extract_if(10.., |_| true).for_each(drop);
}

#[test]
fn test_vec_extract_if_retains_unvisited_elements() {
    let mut vec = TypeProjectedVec::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let mut count = 0;
    {
        let mut iter = vec.extract_if(.., |_| true);
        while count < 3 {
            let _ = iter.next();
            count += 1;
        }
    }

    assert_eq!(vec.as_slice(), [3, 4, 5, 6, 7, 8, 9, 10]);
}

#[test]
fn test_extract_if_many1() {
    let mut vec = TypeProjectedVec::from([
        0,        1,        2,        3,        4,        5,        6,        7,        8,
        9,        10,       i32::MAX, i32::MAX, i32::MAX, i32::MAX, 11,       12,       13,
        14,       15,       16,       17,       18,       19,       20,       21,       22,
        23,       24,       25,       26,       27,       28,       29,       30,       31,
        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 32,
    ]);
    let extracted: TypeProjectedVec<i32> = vec
        .extract_if(.., |v| *v == i32::MAX)
        .collect();

    let expected_vec = TypeProjectedVec::from_iter(0..=32);
    let expected_extracted = TypeProjectedVec::from([i32::MAX; 12]);

    assert_eq!(vec, expected_vec);
    assert_eq!(extracted, expected_extracted);
}

#[test]
fn test_extract_if_many2() {
    let mut vec = TypeProjectedVec::from([
        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 0,
        1,        2,        3,        4,        5,        6,        7,        8,
        9,        10,       11,       12,       13,       14,       15,       16,
        17,       18,       19,       20,       21,       22,       23,       24,
        25,       26,       27,       28,       29,       30,       31,       32,
        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    ]);
    let extracted: TypeProjectedVec<i32> = vec
        .extract_if(.., |v| *v == i32::MAX)
        .collect();

    let expected_vec = TypeProjectedVec::from_iter(0..=32);
    let expected_extracted = TypeProjectedVec::from([i32::MAX; 15]);

    assert_eq!(vec, expected_vec);
    assert_eq!(extracted, expected_extracted);
}

#[test]
fn test_extract_if_many3() {
    let mut vec = TypeProjectedVec::from([
        i32::MAX, 0,        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 1,
        2,        3,        4,        5,        6,        7,        8,        9,
        10,       i32::MAX, i32::MAX, i32::MAX, i32::MAX, 11,       12,       13,
        14,       15,       16,       17,       18,       19,       20,       21,
        22,       23,       24,       25,       26,       27,       28,       29,
        30,       31,       32,       i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    ]);
    let extracted: TypeProjectedVec<i32> = vec
        .extract_if(.., |v| *v == i32::MAX)
        .collect();

    let expected_vec = TypeProjectedVec::from_iter(0..=32);
    let expected_extracted = TypeProjectedVec::from([i32::MAX; 15]);

    assert_eq!(vec, expected_vec);
    assert_eq!(extracted, expected_extracted);
}

#[test]
fn test_extract_if_many4() {
    let mut vec = TypeProjectedVec::from([
        i32::MAX, 0,  i32::MAX, 1,  i32::MAX, 2,  i32::MAX, 3,
        i32::MAX, 4,  i32::MAX, 5,  i32::MAX, 6,  i32::MAX, 7,
        i32::MAX, 8,  i32::MAX, 9,  i32::MAX, 10, i32::MAX, 11,
        i32::MAX, 12, i32::MAX, 13, i32::MAX, 14, i32::MAX, 15,
        i32::MAX, 16, i32::MAX, 17, i32::MAX, 18, i32::MAX, 19,
        i32::MAX, 20, i32::MAX, 21, i32::MAX, 22, i32::MAX, 23,
        i32::MAX, 24, i32::MAX, 25, i32::MAX, 26, i32::MAX, 27,
        i32::MAX, 28, i32::MAX, 29, i32::MAX, 30, i32::MAX, 31,
        i32::MAX, 32,
    ]);
    let extracted: TypeProjectedVec<i32> = vec
        .extract_if(.., |v| *v == i32::MAX)
        .collect();

    let expected_vec = TypeProjectedVec::from_iter(0..=32);
    let expected_extracted = TypeProjectedVec::from([i32::MAX; 33]);

    assert_eq!(vec, expected_vec);
    assert_eq!(extracted, expected_extracted);
}

#[test]
fn test_vec_retain1() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    vec.retain(|&x| x % 2 == 0);

    assert_eq!(vec.as_slice(), [2, 4, 6]);
}

#[test]
fn test_vec_retain2() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    vec.retain(|&x| true);

    assert_eq!(vec.as_slice(), [1, 2, 3, 4, 5, 6]);
}

#[test]
fn test_vec_retain3() {
    let mut vec = TypeProjectedVec::from([1, 2, 3, 4, 5, 6]);
    vec.retain(|&x| false);

    assert_eq!(vec.as_slice(), []);
}

#[test]
fn test_vec_shift_insert_first() {
    let mut vec = TypeProjectedVec::new();
    assert!(vec.is_empty());
    vec.shift_insert(0, 1);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.as_slice(), &[1]);
    vec.shift_insert(0, 2);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice(), &[2, 1]);
    vec.shift_insert(0, 3);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.as_slice(), &[3, 2, 1]);
}

#[test]
fn test_vec_shift_insert_last() {
    let mut vec = TypeProjectedVec::new();
    assert!(vec.is_empty());
    vec.shift_insert(0, 1);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.as_slice(), &[1]);
    vec.shift_insert(1, 2);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice(), &[1, 2]);
    vec.shift_insert(2, 3);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.as_slice(), &[1, 2, 3]);
}

#[test]
fn test_vec_shift_insert_middle() {
    let mut vec = TypeProjectedVec::from([i32::MAX, i32::MAX]);
    assert_eq!(vec.len(), 2);
    vec.shift_insert(1, 1);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.as_slice(), &[i32::MAX, 1, i32::MAX]);
    vec.shift_insert(1, 2);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &[i32::MAX, 2, 1, i32::MAX]);
    vec.shift_insert(1, 3);
    assert_eq!(vec.len(), 5);
    assert_eq!(vec.as_slice(), &[i32::MAX, 3, 2, 1, i32::MAX]);
}

#[test]
#[should_panic]
fn test_vec_shift_insert_out_of_bounds() {
    let mut vec = TypeProjectedVec::from([]);
    vec.shift_insert(2, 1);

    assert!(true);
}

#[test]
fn test_vec_swap_remove1() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c']);
    let result = vec.swap_remove(0);

    assert_eq!(result, 'a');
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice(), &['c', 'b']);
}

#[test]
fn test_vec_swap_remove2() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c']);
    let result = vec.swap_remove(1);

    assert_eq!(result, 'b');
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice(), &['a', 'c']);
}

#[test]
fn test_vec_swap_remove3() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c']);
    let result = vec.swap_remove(2);

    assert_eq!(result, 'c');
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice(), &['a', 'b']);
}


#[test]
#[should_panic]
fn test_vec_swap_remove_out_of_bounds() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([]);
    vec.swap_remove(2);

    assert!(true);
}

#[test]
fn test_vec_shift_remove1() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove(0);

    assert_eq!(result, 'a');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &['b', 'c', 'd', 'e']);
}

#[test]
fn test_vec_shift_remove2() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove(1);

    assert_eq!(result, 'b');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &['a', 'c', 'd', 'e']);
}

#[test]
fn test_vec_shift_remove3() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove(2);

    assert_eq!(result, 'c');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &['a', 'b', 'd', 'e']);
}

#[test]
fn test_vec_shift_remove4() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove(3);

    assert_eq!(result, 'd');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &['a', 'b', 'c', 'e']);
}

#[test]
fn test_vec_shift_remove5() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove(4);

    assert_eq!(result, 'e');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &['a', 'b', 'c', 'd']);
}

#[test]
fn test_vec_shift_remove6() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c', 'd', 'e']);
    assert_eq!(vec.shift_remove(0), 'a');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &['b', 'c', 'd', 'e']);
    assert_eq!(vec.shift_remove(0), 'b');
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.as_slice(), &['c', 'd', 'e']);
    assert_eq!(vec.shift_remove(0), 'c');
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice(), &['d', 'e']);
    assert_eq!(vec.shift_remove(0), 'd');
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.as_slice(), &['e']);
    assert_eq!(vec.shift_remove(0), 'e');
    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());
}

#[test]
#[should_panic]
fn test_vec_shift_remove_out_of_bounds() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([]);
    vec.shift_remove(2);

    assert!(true);
}

#[test]
fn test_vec_pop_if_empty_true() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([]);

    assert_eq!(vec.pop_if(|_| true), None);
    assert!(vec.is_empty());
}

#[test]
fn test_vec_pop_if_empty_false() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([]);

    assert_eq!(vec.pop_if(|_| false), None);
    assert!(vec.is_empty());
}

#[test]
fn test_vec_pop_if_true() {
    let mut vec = TypeProjectedVec::from([1, 2, 3]);

    assert_eq!(vec.pop_if(|_| true), Some(3));
    assert_eq!(vec.as_slice(), &[1, 2]);
}

#[test]
fn test_vec_pop_if_false() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1, 2, 3]);

    assert_eq!(vec.pop_if(|_| false), None);
    assert_eq!(vec.as_slice(), &[1, 2, 3]);
}

