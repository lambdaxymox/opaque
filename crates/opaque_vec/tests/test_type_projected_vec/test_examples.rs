use opaque_vec::TypeProjectedVec;

use std::string::String;
use std::format;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[test]
fn test_type_projected_vec_empty_is_empty() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::new();

    assert!(vec.is_empty());
}

#[test]
fn test_type_projected_vec_empty_len() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::new();

    assert_eq!(vec.len(), 0);
}

#[test]
fn test_type_projected_vec_empty_capacity() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::new();

    assert_eq!(vec.capacity(), 0);
}

#[test]
fn test_type_projected_vec_reserve() {
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
fn test_type_projected_vec_capacity_zst() {
    let vec: TypeProjectedVec<()> = TypeProjectedVec::new();

    assert_eq!(vec.capacity(), usize::MAX);
}

#[test]
fn test_type_projected_vec_push_index1() {
    let mut vec = TypeProjectedVec::new();

    vec.push(1_i32);
    vec.push(2_i32);
    vec.push(3_i32);

    assert_eq!(vec[0], 1_i32);
    assert_eq!(vec[1], 2_i32);
    assert_eq!(vec[2], 3_i32);
}

#[test]
fn test_type_projected_vec_push_index2() {
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
fn test_type_projected_vec_push_index3() {
    let mut vec = TypeProjectedVec::new();
    let len = 32;

    for _ in 0..len {
        vec.push(usize::MAX);
    }

    for i in 0..len {
        assert_eq!(vec[i], usize::MAX);
    }
}

#[test]
fn test_type_projected_vec_push_len1() {
    let mut vec = TypeProjectedVec::new();

    vec.push(1_i32);
    vec.push(2_i32);
    vec.push(3_i32);

    assert_eq!(vec.len(), 3);
}

#[test]
fn test_type_projected_vec_push_len2() {
    let mut vec = TypeProjectedVec::new();
    let len = 64;

    for i in 0..len {
        vec.push(i);
    }

    assert_eq!(vec.len(), len);
}

#[test]
fn test_type_projected_vec_push_len3() {
    let mut vec = TypeProjectedVec::new();
    let len = 32;

    for _ in 0..len {
        vec.push(usize::MAX);
    }

    assert_eq!(vec.len(), len);
}

#[test]
fn test_type_projected_vec_push_zst_capacity() {
    for len in [0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
        let vec: TypeProjectedVec<()> = TypeProjectedVec::with_capacity(len);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), usize::MAX);
    }
}

#[test]
fn test_type_projected_vec_pop_empty() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::new();

    assert_eq!(vec.pop(), None);

    for _ in 0..32 {
        assert_eq!(vec.pop(), None);
    }
}

#[test]
fn test_type_projected_vec_pop1() {
    let mut vec = TypeProjectedVec::new();

    vec.push(1_i32);
    vec.push(2_i32);
    vec.push(3_i32);

    assert_eq!(vec.len(), 3);

    assert_eq!(vec.pop(), Some(3_i32));
    assert_eq!(vec.pop(), Some(2_i32));
    assert_eq!(vec.pop(), Some(1_i32));
    assert_eq!(vec.pop(), None);
}

#[test]
fn test_type_projected_vec_pop2() {
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
fn test_type_projected_vec_pop3() {
    let mut vec = TypeProjectedVec::new();
    let len = 32;

    for _ in 0..len {
        vec.push(usize::MAX);
    }

    for _ in 0..len {
        assert_eq!(vec.pop(), Some(usize::MAX));
    }

    assert_eq!(vec.pop(), None);
}

#[test]
fn test_type_projected_vec_pop_len1() {
    let mut vec = TypeProjectedVec::new();

    vec.push(1_i32);
    vec.push(2_i32);
    vec.push(3_i32);

    assert_eq!(vec.len(), 3);

    let _ = vec.pop();

    assert_eq!(vec.len(), 2);

    let _ = vec.pop();

    assert_eq!(vec.len(), 1);

    let _ = vec.pop();

    assert_eq!(vec.len(), 0);
}

#[test]
fn test_type_projected_vec_pop_len2() {
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
fn test_type_projected_vec_pop_len3() {
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
fn test_type_projected_vec_extend1() {
    let mut vec = TypeProjectedVec::new();

    assert_eq!(vec.as_slice(), &[]);

    vec.extend([1_i32, 2_i32, 3_i32]);

    assert_eq!(vec.as_slice(), [1_i32, 2_i32, 3_i32]);
}

#[test]
fn test_type_projected_vec_extend2() {
    let mut vec1 = TypeProjectedVec::new();

    for i in 0..16 {
        vec1.push(i);
    }

    let mut vec2 = TypeProjectedVec::new();
    vec2.extend(0..16);

    assert_eq!(vec1, vec2);
}

#[test]
fn test_type_projected_vec_extend3() {
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
fn test_type_projected_vec_clear1() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::new();

    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());

    vec.clear();

    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());
}

#[test]
fn test_type_projected_vec_clear2() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([
        444_i32,
        127_i32,
        780_i32,
        59_i32,
        920_i32,
        496_i32,
    ]);

    assert_eq!(vec.len(), 6);
    assert!(!vec.is_empty());

    for value in vec.iter() {
        assert!(vec.contains(value));
    }

    let vec_before_clear = vec.clone();
    vec.clear();

    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());

    for value in vec_before_clear.iter() {
        assert!(!vec.contains(value));
    }
}

#[test]
fn test_type_projected_vec_clone1() {
    let vec: TypeProjectedVec<isize> = TypeProjectedVec::new();
    let cloned_vec = vec.clone();

    assert_eq!(vec, cloned_vec);
}

#[test]
fn test_type_projected_vec_clone2() {
    let mut vec: TypeProjectedVec<isize> = TypeProjectedVec::new();
    vec.extend(0_isize..8_isize);

    let cloned_vec = vec.clone();

    assert_eq!(vec, cloned_vec);
}

#[test]
fn test_type_projected_vec_clone3() {
    let mut vec: TypeProjectedVec<isize> = TypeProjectedVec::new();
    for i in 0..3 {
        vec.push(i);
    }

    vec.extend(0_isize..10_isize);

    let cloned_vec = vec.clone();

    assert_eq!(vec, cloned_vec);
}

#[test]
fn test_type_projected_vec_clone_len1() {
    let vec: TypeProjectedVec<isize> = TypeProjectedVec::new();
    let cloned_vec = vec.clone();

    assert_eq!(vec.len(), cloned_vec.len());
}

#[test]
fn test_type_projected_vec_clone_len2() {
    let mut vec: TypeProjectedVec<isize> = TypeProjectedVec::new();
    vec.extend(0_isize..8_isize);

    let cloned_vec = vec.clone();

    assert_eq!(vec.len(), cloned_vec.len());
}

#[test]
fn test_type_projected_vec_clone_len3() {
    let mut vec: TypeProjectedVec<isize> = TypeProjectedVec::new();
    for i in 0..3 {
        vec.push(i);
    }

    vec.extend(0_isize..10_isize);

    let cloned_vec = vec.clone();

    assert_eq!(vec.len(), cloned_vec.len());
}

#[test]
fn test_type_projected_vec_slice_from_ref() {
    let vec = TypeProjectedVec::from(&[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);

    assert_eq!(&vec[0..1], &[1_i32]);
    assert_eq!(&vec[1..2], &[2_i32]);
    assert_eq!(&vec[2..3], &[3_i32]);
    assert_eq!(&vec[3..4], &[4_i32]);
    assert_eq!(&vec[4..5], &[5_i32]);
    assert_eq!(&vec[5..6], &[6_i32]);

    assert_eq!(&vec[0..2], &[1_i32, 2_i32]);
    assert_eq!(&vec[1..3], &[2_i32, 3_i32]);
    assert_eq!(&vec[2..4], &[3_i32, 4_i32]);
    assert_eq!(&vec[3..5], &[4_i32, 5_i32]);
    assert_eq!(&vec[4..6], &[5_i32, 6_i32]);

    assert_eq!(&vec[0..3], &[1_i32, 2_i32, 3_i32]);
    assert_eq!(&vec[1..4], &[2_i32, 3_i32, 4_i32]);
    assert_eq!(&vec[2..5], &[3_i32, 4_i32, 5_i32]);
    assert_eq!(&vec[3..6], &[4_i32, 5_i32, 6_i32]);

    assert_eq!(&vec[0..4], &[1_i32, 2_i32, 3_i32, 4_i32]);
    assert_eq!(&vec[1..5], &[2_i32, 3_i32, 4_i32, 5_i32]);
    assert_eq!(&vec[2..6], &[3_i32, 4_i32, 5_i32, 6_i32]);

    assert_eq!(&vec[0..5], &[1_i32, 2_i32, 3_i32, 4_i32, 5_i32]);
    assert_eq!(&vec[1..6], &[2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);

    assert_eq!(&vec[0..6], &[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    assert_eq!(&vec[..], &[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    assert_eq!(vec.as_slice(), &[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
}

#[test]
fn test_type_projected_vec_slice_from_mut1() {
    let mut vec = TypeProjectedVec::from(&[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    {
        let slice = &mut vec[..2];

        assert_eq!(slice, &[1_i32, 2_i32]);

        for i in 0..slice.len() {
            slice[i] += 20_i32;
        }
    }

    assert_eq!(vec.as_slice(), &[21_i32, 22_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
}

#[test]
fn test_type_projected_vec_slice_from_mut2() {
    let mut vec = TypeProjectedVec::from(&[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    {
        let slice = &mut vec[2..];

        assert_eq!(slice, &[3_i32, 4_i32, 5_i32, 6_i32]);

        for i in 0..slice.len() {
            slice[i] += 20_i32;
        }
    }

    assert_eq!(vec.as_slice(), &[1_i32, 2_i32, 23_i32, 24_i32, 25_i32, 26_i32]);
}

#[test]
fn test_type_projected_vec_dedup() {
    fn test_case(this: TypeProjectedVec<i32>, that: TypeProjectedVec<i32>) {
        let mut vec = this;
        vec.dedup();
        assert_eq!(vec, that);
    }

    test_case(
        TypeProjectedVec::new(),
        TypeProjectedVec::new(),
    );
    test_case(
        TypeProjectedVec::from(&[1_i32]),
        TypeProjectedVec::from(&[1_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[1_i32, 1_i32]),
        TypeProjectedVec::from(&[1_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[1_i32, 2_i32, 3_i32]),
        TypeProjectedVec::from(&[1_i32, 2_i32, 3_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[1_i32, 1_i32, 2_i32, 3_i32]),
        TypeProjectedVec::from(&[1_i32, 2_i32, 3_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[1_i32, 2_i32, 2_i32, 3_i32]),
        TypeProjectedVec::from(&[1_i32, 2_i32, 3_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[1_i32, 2_i32, 3_i32, 3_i32]),
        TypeProjectedVec::from(&[1_i32, 2_i32, 3_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[1_i32, 1_i32, 2_i32, 2_i32, 2_i32, 3_i32, 3_i32]),
        TypeProjectedVec::from(&[1_i32, 2_i32, 3_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[1_i32, 1_i32, 2_i32, 2_i32, 2_i32, 3_i32, 3_i32, 4_i32, 5_i32, 5_i32, 5_i32, 6_i32, 7_i32, 7_i32]),
        TypeProjectedVec::from(&[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32]),
    );
}

#[test]
fn test_type_projected_vec_dedup_by_key() {
    fn test_case(this: TypeProjectedVec<i32>, that: TypeProjectedVec<i32>) {
        let mut vec = this;
        vec.dedup_by_key(|i| *i / 10);
        assert_eq!(vec, that);
    }

    test_case(
        TypeProjectedVec::new(),
        TypeProjectedVec::new(),
    );
    test_case(
        TypeProjectedVec::from(&[10_i32]),
        TypeProjectedVec::from(&[10_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[10_i32, 11_i32]),
        TypeProjectedVec::from(&[10_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[10_i32, 20_i32, 30_i32]),
        TypeProjectedVec::from(&[10_i32, 20_i32, 30_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[10_i32, 11_i32, 20_i32, 30_i32]),
        TypeProjectedVec::from(&[10_i32, 20_i32, 30_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[10_i32, 20_i32, 21_i32, 30_i32]),
        TypeProjectedVec::from(&[10_i32, 20_i32, 30_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[10_i32, 20_i32, 30_i32, 31_i32]),
        TypeProjectedVec::from(&[10_i32, 20_i32, 30_i32]),
    );
    test_case(
        TypeProjectedVec::from(&[10_i32, 11_i32, 20_i32, 21_i32, 22_i32, 30_i32, 31_i32]),
        TypeProjectedVec::from(&[10_i32, 20_i32, 30_i32]),
    );
}

#[test]
#[should_panic]
fn test_type_projected_vec_swap_remove_out_of_bounds1() {
    let mut vec = TypeProjectedVec::from(&[0_i32]);
    vec.swap_remove(1);
}

#[test]
#[should_panic]
fn test_type_projected_vec_swap_remove_out_of_bounds2() {
    let mut vec = TypeProjectedVec::from(&[0_i32]);
    vec.swap_remove(usize::MAX);
}

#[test]
fn test_type_projected_vec_truncate1() {
    let mut vec = TypeProjectedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected = TypeProjectedVec::from([899_i32, 615_i32, 623_i32, 487_i32]);
    vec.truncate(4);

    assert_eq!(vec, expected);
}

#[test]
fn test_type_projected_vec_truncate2() {
    let mut vec = TypeProjectedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected = TypeProjectedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    vec.truncate(vec.len());

    assert_eq!(vec, expected);
}

#[test]
fn test_type_projected_vec_truncate3() {
    let mut vec = TypeProjectedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected = TypeProjectedVec::new();
    vec.truncate(0);

    assert_eq!(vec, expected);
}

#[test]
fn test_type_projected_vec_truncate_len1() {
    let mut vec = TypeProjectedVec::from([1_i32]);

    vec.truncate(1);
    assert_eq!(vec.len(), 1);
    vec.truncate(0);
    assert_eq!(vec.len(), 0);
}

#[test]
fn test_type_projected_vec_truncate_len2() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);

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
fn test_type_projected_vec_truncate_drop1() {
    static mut DROP_COUNT: usize = 0;

    fn get_drop_count() -> usize { unsafe { DROP_COUNT } }

    struct Value { _data: i32 }

    impl Value {
        fn new(data: i32) -> Self { Self { _data: data, } }
    }

    impl Drop for Value {
        fn drop(&mut self) {
            unsafe { DROP_COUNT += 1; }
        }
    }

    let mut vec = TypeProjectedVec::from([Value::new(1_i32)]);

    vec.truncate(1);
    assert_eq!(get_drop_count(), 0);
    vec.truncate(0);
    assert_eq!(get_drop_count(), 1);
}

#[test]
fn test_type_projected_vec_truncate_drop2() {
    static mut DROP_COUNT: usize = 0;

    fn get_drop_count() -> usize { unsafe { DROP_COUNT } }

    struct Value { _data: i32 }

    impl Value {
        fn new(data: i32) -> Self { Self { _data: data, } }
    }

    impl Drop for Value {
        fn drop(&mut self) {
            unsafe { DROP_COUNT += 1; }
        }
    }

    let mut vec = TypeProjectedVec::from([
        Value::new(1_i32),
        Value::new(2_i32),
        Value::new(3_i32),
        Value::new(4_i32),
        Value::new(5_i32),
        Value::new(6_i32),
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
fn test_type_projected_vec_truncate_fail() {
    struct BadValue { data: usize, }

    impl BadValue {
        fn new(data: usize) -> Self { BadValue { data, } }
    }

    impl Drop for BadValue {
        fn drop(&mut self) {
            let BadValue { ref mut data} = *self;
            if *data == 0xbadbeef_usize {
                panic!("BadElem panic: 0xbadbeef")
            }
        }
    }

    let mut vec = TypeProjectedVec::from([
        BadValue::new(1_usize),
        BadValue::new(2_usize),
        BadValue::new(0xbadbeef_usize),
        BadValue::new(4_usize),
    ]);

    vec.truncate(0);
}

#[test]
fn test_type_projected_vec_into_iter_clone_empty() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::with_capacity(10);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.into_iter() {
        cloned_vec.push(value);
    }

    assert!(cloned_vec.is_empty());
}

#[test]
fn test_type_projected_vec_into_iter_clone1() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1_i32]);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.clone().into_iter() {
        cloned_vec.push(value);
    }

    assert_eq!(cloned_vec, vec);
}

#[test]
fn test_type_projected_vec_into_iter_clone2() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1_i32, 2_i32]);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.clone().into_iter() {
        cloned_vec.push(value);
    }

    assert_eq!(cloned_vec, vec);
}

#[test]
fn test_type_projected_vec_into_iter_clone3() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32]);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.clone().into_iter() {
        cloned_vec.push(value);
    }

    assert_eq!(cloned_vec, vec);
}

#[test]
fn test_type_projected_vec_into_iter_clone4() {
    let vec = TypeProjectedVec::from([String::from("foo")]);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.clone().into_iter() {
        cloned_vec.push(value);
    }

    assert_eq!(cloned_vec, vec);
}

#[test]
fn test_type_projected_vec_into_iter_clone5() {
    let vec = TypeProjectedVec::from([String::from("foo"), String::from("bar")]);
    let mut cloned_vec = TypeProjectedVec::new();
    for value in vec.clone().into_iter() {
        cloned_vec.push(value);
    }

    assert_eq!(cloned_vec, vec);
}

#[test]
fn test_type_projected_vec_into_iter_clone6() {
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
fn test_type_projected_vec_into_iter_partial0() {
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
fn test_type_projected_vec_into_iter_partial1() {
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
fn test_type_projected_vec_into_iter_partial2() {
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
fn test_type_projected_vec_into_iter_partial3() {
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
fn test_type_projected_vec_into_iter_partial4() {
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
fn test_type_projected_vec_into_iter_partial5() {
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
fn test_type_projected_vec_into_iter_as_slice1() {
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
fn test_type_projected_vec_into_iter_as_mut_slice1() {
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
fn test_type_projected_vec_into_iter_as_mut_slice2() {
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
fn test_type_projected_vec_drain_empty() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::new();
    let expected = TypeProjectedVec::new();
    let result: TypeProjectedVec<i32> = vec.drain(..).collect();

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_vec_drain_entire_range1() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1_i32]);
    let expected_from_drain = TypeProjectedVec::from([1_i32]);
    let result_from_drain: TypeProjectedVec<i32> = vec.drain(..).collect();
    let expected_vec = TypeProjectedVec::new();

    assert_eq!(result_from_drain, expected_from_drain);
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_type_projected_vec_drain_entire_range2() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1_i32, 2_i32]);
    let expected_from_drain = TypeProjectedVec::from([1_i32, 2_i32]);
    let result_from_drain: TypeProjectedVec<i32> = vec.drain(..).collect();
    let expected_vec = TypeProjectedVec::new();

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_type_projected_vec_drain_entire_range3() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1_i32, 2_i32, 3_i32]);
    let expected_from_drain = TypeProjectedVec::from([1_i32, 2_i32, 3_i32]);
    let result_from_drain: TypeProjectedVec<i32> = vec.drain(..).collect();
    let expected_vec = TypeProjectedVec::new();

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_type_projected_vec_drain_entire_range4() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32]);
    let expected_from_drain = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32]);
    let result_from_drain: TypeProjectedVec<i32> = vec.drain(..).collect();
    let expected_vec = TypeProjectedVec::new();

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_type_projected_vec_drain_entire_range5() {
    let mut vec: TypeProjectedVec<String> = TypeProjectedVec::from([String::from("foo")]);
    let expected_from_drain = vec.clone();
    let result_from_drain: TypeProjectedVec<String> = vec.drain(..).collect();
    let expected_vec: TypeProjectedVec<String> = TypeProjectedVec::new();

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_type_projected_vec_drain_entire_range6() {
    let mut vec: TypeProjectedVec<String> = TypeProjectedVec::from([String::from("foo"), String::from("bar")]);
    let expected_from_drain = vec.clone();
    let result_from_drain: TypeProjectedVec<String> = vec.drain(..).collect();
    let expected_vec: TypeProjectedVec<String> = TypeProjectedVec::new();

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_type_projected_vec_drain_entire_range7() {
    let mut vec = TypeProjectedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let expected_from_drain = vec.clone();
    let result_from_drain: TypeProjectedVec<String> = vec.drain(..).collect();
    let expected_vec: TypeProjectedVec<String> = TypeProjectedVec::new();

    assert_eq!(result_from_drain.as_slice(), expected_from_drain.as_slice());
    assert_eq!(vec.as_slice(), expected_vec.as_slice());
}

#[test]
fn test_type_projected_vec_drain_partial_range1() {
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
fn test_type_projected_vec_drain_partial_range2() {
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
fn test_type_projected_vec_drain_partial_range3() {
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
fn test_type_projected_vec_drain_partial_range4() {
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
fn test_type_projected_vec_splice1() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let splice_data = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    vec.splice(2..4, splice_data);

    assert_eq!(vec.as_slice(), &[1_i32, 2_i32, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 5_i32, 6_i32]);
}

#[test]
fn test_type_projected_vec_splice2() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let splice_data = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    vec.splice(4.., splice_data);

    assert_eq!(vec.as_slice(), &[1_i32, 2_i32, 3_i32, 4_i32, i32::MAX, i32::MAX, i32::MAX, i32::MAX]);
}

#[test]
fn test_type_projected_vec_splice3() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let splice_data = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    vec.splice(0.., splice_data);

    assert_eq!(vec.as_slice(), &[i32::MAX, i32::MAX, i32::MAX, i32::MAX]);
}

#[test]
fn test_type_projected_vec_splice4() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let splice_data = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    vec.splice(0..1, splice_data);

    assert_eq!(vec.as_slice(), &[i32::MAX, i32::MAX, i32::MAX, i32::MAX, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
}

#[test]
fn test_type_projected_vec_splice5() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    vec.splice(1..3, Some(i32::MAX));

    assert_eq!(vec.as_slice(), &[1_i32, i32::MAX, 4_i32, 5_i32, 6_i32]);
}

#[test]
fn test_type_projected_vec_splice6() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    vec.splice(1..3, None);

    assert_eq!(vec.as_slice(), &[1_i32, 4_i32, 5_i32, 6_i32]);
}

#[test]
fn test_type_projected_vec_debug_fmt_empty_unit() {
    let vec: TypeProjectedVec<()> = TypeProjectedVec::new();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_vec_debug_fmt_empty_u8() {
    let vec: TypeProjectedVec<u8> = TypeProjectedVec::new();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_vec_debug_fmt_empty_u16() {
    let vec: TypeProjectedVec<u16> = TypeProjectedVec::new();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_vec_debug_fmt_empty_u32() {
    let vec: TypeProjectedVec<u32> = TypeProjectedVec::new();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_vec_debug_fmt_empty_u64() {
    let vec: TypeProjectedVec<u64> = TypeProjectedVec::new();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_vec_debug_fmt_empty_usize() {
    let vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_vec_debug_fmt_empty_string() {
    let vec: TypeProjectedVec<u8> = TypeProjectedVec::new();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice());

    assert_eq!(result, expected);
}

#[test]
fn test_type_projected_vec_indexing() {
    let vec = TypeProjectedVec::from([10_i32, 20_i32, 30_i32]);

    assert_eq!(vec[0], 10_i32);
    assert_eq!(vec[1], 20_i32);
    assert_eq!(vec[2], 30_i32);

    let mut idx = 0;

    assert_eq!(vec[idx], 10_i32);
    idx += 1;
    assert_eq!(vec[idx], 20_i32);
    idx += 1;
    assert_eq!(vec[idx], 30_i32);
}

#[test]
#[should_panic]
fn test_type_projected_vec_indexing_out_of_bounds1() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::new();
    let _ = vec[0];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_projected_vec_indexing_out_of_bounds2() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([10_i32]);
    let _ = vec[1];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_projected_vec_indexing_out_of_bounds3() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([10_i32, 20_i32]);
    let _ = vec[2];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_projected_vec_indexing_out_of_bounds4() {
    let vec: TypeProjectedVec<i32> = TypeProjectedVec::from([10_i32, 20_i32, 30_i32]);
    let _ = vec[3];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_projected_vec_slice_out_of_bounds1() {
    let vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let _ = &vec[!0..];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_projected_vec_slice_out_of_bounds2() {
    let vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let _ = &vec[..7];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_projected_vec_slice_out_of_bounds3() {
    let vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let _ = &vec[!0..5];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_projected_vec_slice_out_of_bounds4() {
    let vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let _ = &vec[1..7];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_projected_vec_slice_out_of_bounds5() {
    let vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let _ = &vec[3..2];

    assert!(true);
}

#[cfg(feature = "nightly")]
#[test]
fn test_type_projected_vec_into_boxed_slice() {
    let vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32]);
    let boxed_slice = vec.into_boxed_slice();

    assert_eq!(&*boxed_slice, [1_i32, 2_i32, 3_i32]);
}

#[cfg(feature = "nightly")]
#[test]
fn test_type_projected_vec_into_boxed_slice_from_boxed_slice() {
    let vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32]);
    let boxed_slice = vec.into_boxed_slice();
    let new_vec = TypeProjectedVec::from(boxed_slice);

    assert_eq!(&*new_vec, &[1_i32, 2_i32, 3_i32]);
    assert_eq!(new_vec.as_slice(), &[1_i32, 2_i32, 3_i32]);
}

#[test]
fn test_type_projected_vec_append1() {
    let mut vec1 = TypeProjectedVec::from([1_i32, 2_i32, 3_i32]);
    let mut vec2 = TypeProjectedVec::from([4_i32, 5_i32, 6_i32, 7_i32]);
    vec1.append(&mut vec2);

    assert_eq!(&*vec1, &[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32]);
    assert_eq!(vec1.as_slice(), &[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32]);
    assert!(vec2.is_empty());
}

#[test]
fn test_type_projected_vec_append2() {
    let mut vec1 = TypeProjectedVec::from([1_i32, 2_i32, 3_i32]);
    let mut vec2 = TypeProjectedVec::new();
    vec1.append(&mut vec2);

    assert_eq!(&*vec1, &[1_i32, 2_i32, 3_i32]);
    assert_eq!(vec1.as_slice(), &[1_i32, 2_i32, 3_i32]);
    assert!(vec2.is_empty());
}

#[test]
fn test_type_projected_vec_split_off1() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let vec_ptr = vec.as_ptr();
    let old_capacity = vec.capacity();

    let split_vec = vec.split_off(4);
    assert_eq!(vec.as_slice(), [1_i32, 2_i32, 3_i32, 4_i32]);
    assert_eq!(split_vec.as_slice(), [5_i32, 6_i32]);
    assert_eq!(vec.capacity(), old_capacity);
    assert_eq!(vec.as_ptr(), vec_ptr);
}

#[test]
fn test_type_projected_vec_split_off2() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let vec_ptr = vec.as_ptr();
    let old_capacity = vec.capacity();

    let split_vec = vec.split_off(0);
    assert_eq!(vec.as_slice(), []);
    assert_eq!(split_vec.as_slice(), [1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    assert_eq!(vec.capacity(), old_capacity);
    assert_eq!(vec.as_ptr(), vec_ptr);
}

#[test]
fn test_type_projected_vec_split_off3() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let vec_ptr = vec.as_ptr();
    let old_capacity = vec.capacity();

    let split_vec = vec.split_off(6);
    assert_eq!(vec.as_slice(), [1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    assert_eq!(split_vec.as_slice(), []);
    assert_eq!(vec.capacity(), old_capacity);
    assert_eq!(vec.as_ptr(), vec_ptr);
}

#[test]
fn test_type_projected_vec_split_off4() {
    let mut vec = TypeProjectedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected1 = TypeProjectedVec::from([899_i32, 615_i32, 623_i32, 487_i32]);
    let expected2 = TypeProjectedVec::from([935_i32, 806_i32, 381_i32, 967_i32]);
    let result2 = vec.split_off(4);
    let result1 = vec.clone();

    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);
}

#[test]
fn test_type_projected_vec_split_off5() {
    let mut vec = TypeProjectedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected1 = TypeProjectedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected2 = TypeProjectedVec::new();
    let result2 = vec.split_off(vec.len());
    let result1 = vec.clone();

    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);
}

#[test]
fn test_type_projected_vec_split_off6() {
    let mut vec = TypeProjectedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected1 = TypeProjectedVec::new();
    let expected2 = TypeProjectedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let result2 = vec.split_off(0);
    let result1 = vec.clone();

    assert_eq!(result1, expected1);
    assert_eq!(result2, expected2);
}

#[test]
fn test_type_projected_vec_reserve_exact() {
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
fn test_type_projected_vec_extract_if_empty_true() {
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
fn test_type_projected_vec_extract_if_empty_false() {
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
fn test_type_projected_vec_extract_if_total_true() {
    let mut vec = TypeProjectedVec::from([
        0_i32,
        1_i32,
        2_i32,
        3_i32,
        4_i32,
        5_i32,
        6_i32,
        7_i32,
        8_i32,
        9_i32,
        10_i32,
    ]);
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
fn test_type_projected_vec_extract_if_total_false() {
    let mut vec = TypeProjectedVec::from([
        0_i32,
        1_i32,
        2_i32,
        3_i32,
        4_i32,
        5_i32,
        6_i32,
        7_i32,
        8_i32,
        9_i32,
        10_i32,
    ]);
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
    assert_eq!(vec.as_slice(), [0_i32, 1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32, 8_i32, 9_i32, 10_i32]);
}

#[test]
fn test_type_projected_vec_extract_if_partial_true() {
    let mut vec = TypeProjectedVec::from([
        0_i32,
        1_i32,
        2_i32,
        3_i32,
        4_i32,
        5_i32,
        6_i32,
        7_i32,
        8_i32,
        9_i32,
        10_i32,
    ]);
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
    assert_eq!(vec.len(), old_length - count);
    assert_eq!(vec.as_slice(), [0_i32, 1_i32, 8_i32, 9_i32, 10_i32]);
}

#[test]
fn test_type_projected_vec_extract_if_partial_false() {
    let mut vec = TypeProjectedVec::from([
        0_i32,
        1_i32,
        2_i32,
        3_i32,
        4_i32,
        5_i32,
        6_i32,
        7_i32,
        8_i32,
        9_i32,
        10_i32,
    ]);
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
    assert_eq!(vec.as_slice(), [0_i32, 1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32, 8_i32, 9_i32, 10_i32]);
}

#[test]
#[should_panic]
fn test_type_projected_vec_extract_if_out_of_bounds() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32]);
    let _ = vec.extract_if(10.., |_| true).for_each(drop);

    assert!(true);
}

#[test]
fn test_type_projected_vec_extract_if_retains_unvisited_elements() {
    let mut vec = TypeProjectedVec::from([
        0_i32,
        1_i32,
        2_i32,
        3_i32,
        4_i32,
        5_i32,
        6_i32,
        7_i32,
        8_i32,
        9_i32,
        10_i32,
    ]);
    let mut count = 0;
    {
        let mut iter = vec.extract_if(.., |_| true);
        while count < 3 {
            let _ = iter.next();
            count += 1;
        }
    }

    assert_eq!(vec.as_slice(), [3_i32, 4_i32, 5_i32, 6_i32, 7_i32, 8_i32, 9_i32, 10_i32]);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_vec_extract_if_many1() {
    let mut vec = TypeProjectedVec::from([
        0_i32,    1_i32,    2_i32,    3_i32,    4_i32,    5_i32,    6_i32,    7_i32,    8_i32,
        9_i32,    10_i32,   i32::MAX, i32::MAX, i32::MAX, i32::MAX, 11_i32,   12_i32,   13_i32,
        14_i32,   15_i32,   16_i32,   17_i32,   18_i32,   19_i32,   20_i32,   21_i32,   22_i32,
        23_i32,   24_i32,   25_i32,   26_i32,   27_i32,   28_i32,   29_i32,   30_i32,   31_i32,
        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 32_i32,
    ]);
    let extracted: TypeProjectedVec<i32> = vec
        .extract_if(.., |v| *v == i32::MAX)
        .collect();

    let expected_vec = TypeProjectedVec::from_iter(0_i32..=32_i32);
    let expected_extracted = TypeProjectedVec::from([i32::MAX; 12]);

    assert_eq!(vec, expected_vec);
    assert_eq!(extracted, expected_extracted);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_vec_extract_if_many2() {
    let mut vec = TypeProjectedVec::from([
        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 0_i32,
        1_i32,    2_i32,    3_i32,    4_i32,    5_i32,    6_i32,    7_i32,    8_i32,
        9_i32,    10_i32,   11_i32,   12_i32,   13_i32,   14_i32,   15_i32,   16_i32,
        17_i32,   18_i32,   19_i32,   20_i32,   21_i32,   22_i32,   23_i32,   24_i32,
        25_i32,   26_i32,   27_i32,   28_i32,   29_i32,   30_i32,   31_i32,   32_i32,
        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    ]);
    let extracted: TypeProjectedVec<i32> = vec
        .extract_if(.., |v| *v == i32::MAX)
        .collect();

    let expected_vec = TypeProjectedVec::from_iter(0_i32..=32_i32);
    let expected_extracted = TypeProjectedVec::from([i32::MAX; 15]);

    assert_eq!(vec, expected_vec);
    assert_eq!(extracted, expected_extracted);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_vec_extract_if_many3() {
    let mut vec = TypeProjectedVec::from([
        i32::MAX, 0_i32,    i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 1_i32,
        2_i32,    3_i32,    4_i32,    5_i32,    6_i32,    7_i32,    8_i32,    9_i32,
        10_i32,   i32::MAX, i32::MAX, i32::MAX, i32::MAX, 11_i32,   12_i32,   13_i32,
        14_i32,   15_i32,   16_i32,   17_i32,   18_i32,   19_i32,   20_i32,   21_i32,
        22_i32,   23_i32,   24_i32,   25_i32,   26_i32,   27_i32,   28_i32,   29_i32,
        30_i32,   31_i32,   32_i32,   i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    ]);
    let extracted: TypeProjectedVec<i32> = vec
        .extract_if(.., |v| *v == i32::MAX)
        .collect();

    let expected_vec = TypeProjectedVec::from_iter(0_i32..=32_i32);
    let expected_extracted = TypeProjectedVec::from([i32::MAX; 15]);

    assert_eq!(vec, expected_vec);
    assert_eq!(extracted, expected_extracted);
}

#[rustfmt::skip]
#[test]
fn test_type_projected_vec_extract_if_many4() {
    let mut vec = TypeProjectedVec::from([
        i32::MAX, 0_i32,  i32::MAX, 1_i32,  i32::MAX, 2_i32,  i32::MAX, 3_i32,
        i32::MAX, 4_i32,  i32::MAX, 5_i32,  i32::MAX, 6_i32,  i32::MAX, 7_i32,
        i32::MAX, 8_i32,  i32::MAX, 9_i32,  i32::MAX, 10_i32, i32::MAX, 11_i32,
        i32::MAX, 12_i32, i32::MAX, 13_i32, i32::MAX, 14_i32, i32::MAX, 15_i32,
        i32::MAX, 16_i32, i32::MAX, 17_i32, i32::MAX, 18_i32, i32::MAX, 19_i32,
        i32::MAX, 20_i32, i32::MAX, 21_i32, i32::MAX, 22_i32, i32::MAX, 23_i32,
        i32::MAX, 24_i32, i32::MAX, 25_i32, i32::MAX, 26_i32, i32::MAX, 27_i32,
        i32::MAX, 28_i32, i32::MAX, 29_i32, i32::MAX, 30_i32, i32::MAX, 31_i32,
        i32::MAX, 32_i32,
    ]);
    let extracted: TypeProjectedVec<i32> = vec
        .extract_if(.., |v| *v == i32::MAX)
        .collect();

    let expected_vec = TypeProjectedVec::from_iter(0_i32..=32_i32);
    let expected_extracted = TypeProjectedVec::from([i32::MAX; 33]);

    assert_eq!(vec, expected_vec);
    assert_eq!(extracted, expected_extracted);
}

#[test]
fn test_type_projected_vec_retain1() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    vec.retain(|&x| x % 2_i32 == 0_i32);

    assert_eq!(vec.as_slice(), [2_i32, 4_i32, 6_i32]);
}

#[test]
fn test_type_projected_vec_retain2() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    vec.retain(|_| true);

    assert_eq!(vec.as_slice(), [1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
}

#[test]
fn test_type_projected_vec_retain3() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    vec.retain(|_| false);

    assert_eq!(vec.as_slice(), []);
}

#[test]
fn test_type_projected_vec_shift_insert_first() {
    let mut vec = TypeProjectedVec::new();
    assert!(vec.is_empty());
    vec.shift_insert(0, 1_i32);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.as_slice(), &[1_i32]);
    vec.shift_insert(0, 2_i32);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice(), &[2_i32, 1_i32]);
    vec.shift_insert(0, 3_i32);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.as_slice(), &[3_i32, 2_i32, 1_i32]);
}

#[test]
fn test_type_projected_vec_shift_insert_last() {
    let mut vec = TypeProjectedVec::new();
    assert!(vec.is_empty());
    vec.shift_insert(0, 1_i32);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.as_slice(), &[1_i32]);
    vec.shift_insert(1, 2_i32);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice(), &[1_i32, 2_i32]);
    vec.shift_insert(2, 3_i32);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.as_slice(), &[1_i32, 2_i32, 3_i32]);
}

#[test]
fn test_type_projected_vec_shift_insert_middle() {
    let mut vec = TypeProjectedVec::from([i32::MAX, i32::MAX]);
    assert_eq!(vec.len(), 2);
    vec.shift_insert(1, 1_i32);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.as_slice(), &[i32::MAX, 1_i32, i32::MAX]);
    vec.shift_insert(1, 2_i32);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &[i32::MAX, 2_i32, 1_i32, i32::MAX]);
    vec.shift_insert(1, 3_i32);
    assert_eq!(vec.len(), 5);
    assert_eq!(vec.as_slice(), &[i32::MAX, 3_i32, 2_i32, 1_i32, i32::MAX]);
}

#[test]
#[should_panic]
fn test_type_projected_vec_shift_insert_out_of_bounds() {
    let mut vec = TypeProjectedVec::new();
    vec.shift_insert(2, 1_i32);

    assert!(true);
}

#[test]
fn test_type_projected_vec_swap_remove1() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c']);
    let result = vec.swap_remove(0);

    assert_eq!(result, 'a');
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice(), &['c', 'b']);
}

#[test]
fn test_type_projected_vec_swap_remove2() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c']);
    let result = vec.swap_remove(1);

    assert_eq!(result, 'b');
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice(), &['a', 'c']);
}

#[test]
fn test_type_projected_vec_swap_remove3() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c']);
    let result = vec.swap_remove(2);

    assert_eq!(result, 'c');
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice(), &['a', 'b']);
}

#[test]
#[should_panic]
fn test_type_projected_vec_swap_remove_out_of_bounds() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::new();
    vec.swap_remove(2);

    assert!(true);
}

#[test]
fn test_type_projected_vec_shift_remove1() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove(0);

    assert_eq!(result, 'a');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &['b', 'c', 'd', 'e']);
}

#[test]
fn test_type_projected_vec_shift_remove2() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove(1);

    assert_eq!(result, 'b');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &['a', 'c', 'd', 'e']);
}

#[test]
fn test_type_projected_vec_shift_remove3() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove(2);

    assert_eq!(result, 'c');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &['a', 'b', 'd', 'e']);
}

#[test]
fn test_type_projected_vec_shift_remove4() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove(3);

    assert_eq!(result, 'd');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &['a', 'b', 'c', 'e']);
}

#[test]
fn test_type_projected_vec_shift_remove5() {
    let mut vec = TypeProjectedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove(4);

    assert_eq!(result, 'e');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice(), &['a', 'b', 'c', 'd']);
}

#[test]
fn test_type_projected_vec_shift_remove6() {
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
fn test_type_projected_vec_shift_remove_out_of_bounds() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::new();
    vec.shift_remove(2);

    assert!(true);
}

#[test]
fn test_type_projected_vec_pop_if_empty_true() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::new();

    assert_eq!(vec.pop_if(|_| true), None);
    assert!(vec.is_empty());
}

#[test]
fn test_type_projected_vec_pop_if_empty_false() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::new();

    assert_eq!(vec.pop_if(|_| false), None);
    assert!(vec.is_empty());
}

#[test]
fn test_type_projected_vec_pop_if_true() {
    let mut vec = TypeProjectedVec::from([1_i32, 2_i32, 3_i32]);

    assert_eq!(vec.pop_if(|_| true), Some(3_i32));
    assert_eq!(vec.as_slice(), &[1_i32, 2_i32]);
}

#[test]
fn test_type_projected_vec_pop_if_false() {
    let mut vec: TypeProjectedVec<i32> = TypeProjectedVec::from([1_i32, 2_i32, 3_i32]);

    assert_eq!(vec.pop_if(|_| false), None);
    assert_eq!(vec.as_slice(), &[1_i32, 2_i32, 3_i32]);
}

#[test]
fn test_type_projected_vec_reserve1() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);

    vec.reserve(additional);

    assert!(vec.capacity() >= additional);
}

#[test]
fn test_type_projected_vec_reserve2() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);

    vec.reserve(additional);

    assert!(vec.capacity() >= additional);

    let old_capacity = vec.capacity();
    vec.push(usize::MAX);
    for _ in 1..(vec.capacity() - 1) {
        vec.push(0_usize);
    }

    vec.push(usize::MAX);

    assert_eq!(vec.len(), vec.capacity());
    assert_eq!(vec.capacity(), old_capacity);

    assert_eq!(vec[0], usize::MAX);
    for i in 1..(vec.len() - 1) {
        assert_eq!(vec[i], 0_usize);
    }
    assert_eq!(vec[vec.len() - 1], usize::MAX);
}

#[test]
fn test_type_projected_vec_reserve3() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.len(), 0);

    for i in 0..4 {
        let old_capacity = vec.capacity();
        vec.reserve(additional);

        assert!(vec.capacity() >= old_capacity + additional);
        assert!(vec.len() <= vec.capacity());

        let length = vec.len();
        vec.push(usize::MAX);
        for _ in (length + 1)..(vec.capacity() - 1) {
            vec.push(i);
        }
        vec.push(usize::MAX);

        assert_eq!(vec.len(), vec.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..4 {
        for j in (current_start + 1)..vec.len() {
            if vec[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(vec[current_start], usize::MAX);
        for value in vec[(current_start + 1)..current_end].iter().copied() {
            assert_eq!(value, i);
        }
        assert_eq!(vec[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[test]
fn test_type_projected_vec_reserve_exact1() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);

    vec.reserve_exact(additional);

    assert!(vec.capacity() >= additional);
}

#[test]
fn test_type_projected_vec_reserve_exact2() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);

    vec.reserve_exact(additional);

    assert!(vec.capacity() >= additional);

    let old_capacity = vec.capacity();
    vec.push(usize::MAX);
    for _ in 1..(vec.capacity() - 1) {
        vec.push(0_usize);
    }

    vec.push(usize::MAX);

    assert_eq!(vec.len(), vec.capacity());
    assert_eq!(vec.capacity(), old_capacity);

    assert_eq!(vec[0], usize::MAX);
    for i in 1..(vec.len() - 1) {
        assert_eq!(vec[i], 0_usize);
    }
    assert_eq!(vec[vec.len() - 1], usize::MAX);
}

#[test]
fn test_type_projected_vec_reserve_exact3() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.len(), 0);

    for i in 0..32 {
        let old_capacity = vec.capacity();
        vec.reserve_exact(additional);

        assert!(vec.capacity() >= old_capacity + additional);
        assert!(vec.len() <= vec.capacity());

        let length = vec.len();
        vec.push(usize::MAX);
        for _ in (length + 1)..(vec.capacity() - 1) {
            vec.push(i);
        }
        vec.push(usize::MAX);

        assert_eq!(vec.len(), vec.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..32 {
        for j in (current_start + 1)..vec.len() {
            if vec[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(vec[current_start], usize::MAX);
        for value in vec[(current_start + 1)..current_end].iter().copied() {
            assert_eq!(value, i);
        }
        assert_eq!(vec[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[test]
fn test_type_projected_vec_try_reserve1() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.try_reserve(additional), Ok(()));
    assert!(vec.capacity() >= additional);
}

#[test]
fn test_type_projected_vec_try_reserve2() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.try_reserve(additional), Ok(()));
    assert!(vec.capacity() >= additional);

    let old_capacity = vec.capacity();
    vec.push(usize::MAX);
    for _ in 1..(vec.capacity() - 1) {
        vec.push(0_usize);
    }

    vec.push(usize::MAX);

    assert_eq!(vec.len(), vec.capacity());
    assert_eq!(vec.capacity(), old_capacity);

    assert_eq!(vec[0], usize::MAX);
    for i in 1..(vec.len() - 1) {
        assert_eq!(vec[i], 0_usize);
    }
    assert_eq!(vec[vec.len() - 1], usize::MAX);
}

#[test]
fn test_type_projected_vec_try_reserve3() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.len(), 0);

    for i in 0..4 {
        let old_capacity = vec.capacity();
        assert_eq!(vec.try_reserve(additional), Ok(()));

        assert!(vec.capacity() >= old_capacity + additional);
        assert!(vec.len() <= vec.capacity());

        let length = vec.len();
        vec.push(usize::MAX);
        for _ in (length + 1)..(vec.capacity() - 1) {
            vec.push(i);
        }
        vec.push(usize::MAX);

        assert_eq!(vec.len(), vec.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..4 {
        for j in (current_start + 1)..vec.len() {
            if vec[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(vec[current_start], usize::MAX);
        for value in vec[(current_start + 1)..current_end].iter().copied() {
            assert_eq!(value, i);
        }
        assert_eq!(vec[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[test]
fn test_type_projected_vec_try_reserve_exact1() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.try_reserve_exact(additional), Ok(()));
    assert!(vec.capacity() >= additional);
}

#[test]
fn test_type_projected_vec_try_reserve_exact2() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.try_reserve_exact(additional), Ok(()));
    assert!(vec.capacity() >= additional);

    let old_capacity = vec.capacity();
    vec.push(usize::MAX);
    for _ in 1..(vec.capacity() - 1) {
        vec.push(0_usize);
    }

    vec.push(usize::MAX);

    assert_eq!(vec.len(), vec.capacity());
    assert_eq!(vec.capacity(), old_capacity);

    assert_eq!(vec[0], usize::MAX);
    for i in 1..(vec.len() - 1) {
        assert_eq!(vec[i], 0_usize);
    }
    assert_eq!(vec[vec.len() - 1], usize::MAX);
}

#[test]
fn test_type_projected_vec_try_reserve_exact3() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.len(), 0);

    for i in 0..32 {
        let old_capacity = vec.capacity();
        assert_eq!(vec.try_reserve_exact(additional), Ok(()));

        assert!(vec.capacity() >= old_capacity + additional);
        assert!(vec.len() <= vec.capacity());

        let length = vec.len();
        vec.push(usize::MAX);
        for _ in (length + 1)..(vec.capacity() - 1) {
            vec.push(i);
        }
        vec.push(usize::MAX);

        assert_eq!(vec.len(), vec.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..32 {
        for j in (current_start + 1)..vec.len() {
            if vec[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(vec[current_start], usize::MAX);
        for value in vec[(current_start + 1)..current_end].iter().copied() {
            assert_eq!(value, i);
        }
        assert_eq!(vec[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[test]
fn test_type_projected_vec_shrink_to_fit1() {
    let mut vec: TypeProjectedVec<(usize, usize)> = TypeProjectedVec::with_capacity(10);
    assert_eq!(vec.capacity(), 10);

    vec.extend([(1_usize, usize::MAX), (2_usize, usize::MAX), (3_usize, usize::MAX)]);
    assert!(vec.len() <= vec.capacity());

    assert_eq!(vec[0], (1_usize, usize::MAX));
    assert_eq!(vec[1], (2_usize, usize::MAX));
    assert_eq!(vec[2], (3_usize, usize::MAX));

    vec.shrink_to_fit();

    assert!(vec.len() <= vec.capacity());
    assert_eq!(vec[0], (1_usize, usize::MAX));
    assert_eq!(vec[1], (2_usize, usize::MAX));
    assert_eq!(vec[2], (3_usize, usize::MAX));
}

#[test]
fn test_type_projected_vec_shrink_to_fit2() {
    let mut vec: TypeProjectedVec<usize> = TypeProjectedVec::new();
    for i in 0..128 {
        assert_eq!(vec.len(), i);

        vec.push(i * i);

        assert_eq!(vec.len(), i + 1);
        assert!(vec.capacity() >= i + 1);
        assert_eq!(vec[i], i * i);
        assert_eq!(vec.get(i), Some(&(i * i)));

        vec.shrink_to_fit();

        assert_eq!(vec.len(), i + 1);
        assert!(vec.capacity() >= i + 1);
        assert_eq!(vec[i], i * i);
        assert_eq!(vec.get(i), Some(&(i * i)));
    }
}
