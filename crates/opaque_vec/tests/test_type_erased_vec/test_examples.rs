use opaque_vec::TypeErasedVec;

use std::string::String;
use std::format;

#[cfg(feature = "nightly")]
use std::alloc;

#[cfg(not(feature = "nightly"))]
use opaque_allocator_api::alloc;

#[test]
fn test_type_erased_vec_empty_is_empty() {
    let vec = TypeErasedVec::new::<i32>();

    assert!(vec.is_empty());
}

#[test]
fn test_type_erased_vec_empty_len() {
    let vec = TypeErasedVec::new::<i32>();

    assert_eq!(vec.len(), 0);
}

#[test]
fn test_type_erased_vec_empty_capacity() {
    let vec = TypeErasedVec::new::<i32>();

    assert_eq!(vec.capacity(), 0);
}

#[test]
fn test_type_erased_vec_reserve() {
    let mut vec = TypeErasedVec::new::<i32>();
    assert_eq!(vec.capacity(), 0);

    vec.reserve::<i32, alloc::Global>(2);
    assert!(vec.capacity() >= 2);

    for i in 0..16 {
        vec.push::<i32, alloc::Global>(i);
    }

    assert!(vec.capacity() >= 16);
    vec.reserve::<i32, alloc::Global>(16);
    assert!(vec.capacity() >= 32);

    vec.push::<i32, alloc::Global>(16);

    vec.reserve::<i32, alloc::Global>(16);
    assert!(vec.capacity() >= 33)
}

#[test]
fn test_type_erased_vec_capacity_zst() {
    let vec = TypeErasedVec::new::<()>();

    assert_eq!(vec.capacity(), usize::MAX);
}

#[test]
fn test_type_erased_vec_push_index1() {
    let mut vec = TypeErasedVec::new::<i32>();

    vec.push::<i32, alloc::Global>(1_i32);
    vec.push::<i32, alloc::Global>(2_i32);
    vec.push::<i32, alloc::Global>(3_i32);

    assert_eq!(vec.as_slice::<i32, alloc::Global>()[0], 1_i32);
    assert_eq!(vec.as_slice::<i32, alloc::Global>()[1], 2_i32);
    assert_eq!(vec.as_slice::<i32, alloc::Global>()[2], 3_i32);
}

#[test]
fn test_type_erased_vec_push_index2() {
    let mut vec = TypeErasedVec::new::<i32>();
    let len: usize = 64;

    for i in 0..len {
        vec.push::<i32, alloc::Global>(i as i32);
    }

    for i in 0..len {
        assert_eq!(vec.as_slice::<i32, alloc::Global>()[i], i as i32);
    }
}

#[test]
fn test_type_erased_vec_push_index3() {
    let mut vec = TypeErasedVec::new::<usize>();
    let len = 32;

    for _ in 0..len {
        vec.push::<usize, alloc::Global>(usize::MAX);
    }

    for i in 0..len {
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[i], usize::MAX);
    }
}

#[test]
fn test_type_erased_vec_push_len1() {
    let mut vec = TypeErasedVec::new::<i32>();

    vec.push::<i32, alloc::Global>(1_i32);
    vec.push::<i32, alloc::Global>(2_i32);
    vec.push::<i32, alloc::Global>(3_i32);

    assert_eq!(vec.len(), 3);
}

#[test]
fn test_type_erased_vec_push_len2() {
    let mut vec = TypeErasedVec::new::<i32>();
    let len: usize = 64;

    for i in 0..len {
        vec.push::<i32, alloc::Global>(i as i32);
    }

    assert_eq!(vec.len(), len);
}

#[test]
fn test_type_erased_vec_push_len3() {
    let mut vec = TypeErasedVec::new::<usize>();
    let len = 32;

    for _ in 0..len {
        vec.push::<usize, alloc::Global>(usize::MAX);
    }

    assert_eq!(vec.len(), len);
}

#[test]
fn test_type_erased_vec_push_zst_capacity() {
    for len in [0, 1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024] {
        let vec = TypeErasedVec::with_capacity::<()>(len);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), usize::MAX);
    }
}

#[test]
fn test_type_erased_vec_pop_empty() {
    let mut vec = TypeErasedVec::new::<i32>();

    assert_eq!(vec.pop::<i32, alloc::Global>(), None);

    for _ in 0..32 {
        assert_eq!(vec.pop::<i32, alloc::Global>(), None);
    }
}

#[test]
fn test_type_erased_vec_pop1() {
    let mut vec = TypeErasedVec::new::<i32>();

    vec.push::<i32, alloc::Global>(1_i32);
    vec.push::<i32, alloc::Global>(2_i32);
    vec.push::<i32, alloc::Global>(3_i32);

    assert_eq!(vec.len(), 3);

    assert_eq!(vec.pop::<i32, alloc::Global>(), Some(3_i32));
    assert_eq!(vec.pop::<i32, alloc::Global>(), Some(2_i32));
    assert_eq!(vec.pop::<i32, alloc::Global>(), Some(1_i32));
    assert_eq!(vec.pop::<i32, alloc::Global>(), None);
}

#[test]
fn test_type_erased_vec_pop2() {
    let mut vec = TypeErasedVec::new::<i32>();
    let len = 64;

    for i in 0..len {
        vec.push::<i32, alloc::Global>(i);
    }

    for i in 0..len {
        assert_eq!(vec.pop::<i32, alloc::Global>(), Some(len - (i + 1)));
    }

    assert_eq!(vec.pop::<i32, alloc::Global>(), None);
}

#[test]
fn test_type_erased_vec_pop3() {
    let mut vec = TypeErasedVec::new::<usize>();
    let len = 32;

    for _ in 0..len {
        vec.push::<usize, alloc::Global>(usize::MAX);
    }

    for _ in 0..len {
        assert_eq!(vec.pop::<usize, alloc::Global>(), Some(usize::MAX));
    }

    assert_eq!(vec.pop::<usize, alloc::Global>(), None);
}

#[test]
fn test_type_erased_vec_pop_len1() {
    let mut vec = TypeErasedVec::new::<i32>();

    vec.push::<i32, alloc::Global>(1_i32);
    vec.push::<i32, alloc::Global>(2_i32);
    vec.push::<i32, alloc::Global>(3_i32);

    assert_eq!(vec.len(), 3);

    let _ = vec.pop::<i32, alloc::Global>();

    assert_eq!(vec.len(), 2);

    let _ = vec.pop::<i32, alloc::Global>();

    assert_eq!(vec.len(), 1);

    let _ = vec.pop::<i32, alloc::Global>();

    assert_eq!(vec.len(), 0);
}

#[test]
fn test_type_erased_vec_pop_len2() {
    let mut vec = TypeErasedVec::new::<usize>();
    let len = 64;

    for i in 0..len {
        vec.push::<usize, alloc::Global>(i);
    }

    for i in 0..len {
        let _ = vec.pop::<usize, alloc::Global>();

        assert_eq!(vec.len(), len - (i + 1));
    }

    let _ = vec.pop::<usize, alloc::Global>();

    assert_eq!(vec.len(), 0);
}

#[test]
fn test_type_erased_vec_pop_len3() {
    let mut vec = TypeErasedVec::new::<usize>();
    let len = 32;

    for i in 0..len {
        vec.push::<usize, alloc::Global>(i);
    }

    for i in 0..len {
        let _ = vec.pop::<usize, alloc::Global>();

        assert_eq!(vec.len(), len - (i + 1));
    }

    let _ = vec.pop::<usize, alloc::Global>();

    assert_eq!(vec.len(), 0);
}

#[test]
fn test_type_erased_vec_extend1() {
    let mut vec = TypeErasedVec::new::<i32>();

    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[]);

    vec.extend::<_, i32, alloc::Global>([1_i32, 2_i32, 3_i32]);

    assert_eq!(vec.as_slice::<i32, alloc::Global>(), [1_i32, 2_i32, 3_i32]);
}

#[test]
fn test_type_erased_vec_extend2() {
    let mut vec1 = TypeErasedVec::new::<i32>();

    for i in 0..16 {
        vec1.push::<i32, alloc::Global>(i);
    }

    let mut vec2 = TypeErasedVec::new::<i32>();
    vec2.extend::<_, i32, alloc::Global>(0..16);

    assert_eq!(vec1.as_proj::<i32, alloc::Global>(), vec2.as_proj::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_extend3() {
    let mut vec1 = TypeErasedVec::new::<i32>();
    let mut vec2 = TypeErasedVec::new::<i32>();

    for i in 0..3 {
        vec1.push::<i32, alloc::Global>(i);
    }

    vec1.extend::<_, i32, alloc::Global>(3..16);

    vec2.extend::<_, i32, alloc::Global>(0..16);

    assert_eq!(vec1.as_slice::<i32, alloc::Global>(), vec2.as_slice::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_clear1() {
    let mut vec = TypeErasedVec::new::<i32>();

    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());

    vec.clear::<i32, alloc::Global>();

    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());
}

#[test]
fn test_type_erased_vec_clear2() {
    let mut vec = TypeErasedVec::from([
        444_i32,
        127_i32,
        780_i32,
        59_i32,
        920_i32,
        496_i32,
    ]);

    assert_eq!(vec.len(), 6);
    assert!(!vec.is_empty());

    for value in vec.iter::<i32, alloc::Global>() {
        assert!(vec.contains::<i32, alloc::Global>(value));
    }

    let vec_before_clear = vec.clone::<i32, alloc::Global>();
    vec.clear::<i32, alloc::Global>();

    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());

    for value in vec_before_clear.iter::<i32, alloc::Global>() {
        assert!(!vec.contains::<i32, alloc::Global>(value));
    }
}

#[test]
fn test_type_erased_vec_clone1() {
    let vec = TypeErasedVec::new::<isize>();
    let cloned_vec = vec.clone::<isize, alloc::Global>();

    assert_eq!(vec.as_slice::<isize, alloc::Global>(), cloned_vec.as_slice::<isize, alloc::Global>());
}

#[test]
fn test_type_erased_vec_clone2() {
    let mut vec = TypeErasedVec::new::<isize>();
    vec.extend::<_, isize, alloc::Global>(0_isize..8_isize);

    let cloned_vec = vec.clone::<isize, alloc::Global>();

    assert_eq!(vec.as_slice::<isize, alloc::Global>(), cloned_vec.as_slice::<isize, alloc::Global>());
}

#[test]
fn test_type_erased_vec_clone3() {
    let mut vec = TypeErasedVec::new::<isize>();
    for i in 0..3 {
        vec.push::<isize, alloc::Global>(i);
    }

    vec.extend::<_, isize, alloc::Global>(0_isize..10_isize);

    let cloned_vec = vec.clone::<isize, alloc::Global>();

    assert_eq!(vec.as_slice::<isize, alloc::Global>(), cloned_vec.as_slice::<isize, alloc::Global>());
}

#[test]
fn test_type_erased_vec_clone_len1() {
    let vec = TypeErasedVec::new::<isize>();
    let cloned_vec = vec.clone::<isize, alloc::Global>();

    assert_eq!(vec.len(), cloned_vec.len());
}

#[test]
fn test_type_erased_vec_clone_len2() {
    let mut vec = TypeErasedVec::new::<isize>();
    vec.extend::<_, isize, alloc::Global>(0_isize..8_isize);

    let cloned_vec = vec.clone::<isize, alloc::Global>();

    assert_eq!(vec.len(), cloned_vec.len());
}

#[test]
fn test_type_erased_vec_clone_len3() {
    let mut vec = TypeErasedVec::new::<isize>();
    for i in 0..3 {
        vec.push::<isize, alloc::Global>(i);
    }

    vec.extend::<_, isize, alloc::Global>(0_isize..10_isize);

    let cloned_vec = vec.clone::<isize, alloc::Global>();

    assert_eq!(vec.len(), cloned_vec.len());
}

#[test]
fn test_type_erased_vec_slice_from_ref() {
    let vec = TypeErasedVec::from(&[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);

    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[0..1], &[1_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[1..2], &[2_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[2..3], &[3_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[3..4], &[4_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[4..5], &[5_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[5..6], &[6_i32]);

    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[0..2], &[1_i32, 2_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[1..3], &[2_i32, 3_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[2..4], &[3_i32, 4_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[3..5], &[4_i32, 5_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[4..6], &[5_i32, 6_i32]);

    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[0..3], &[1_i32, 2_i32, 3_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[1..4], &[2_i32, 3_i32, 4_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[2..5], &[3_i32, 4_i32, 5_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[3..6], &[4_i32, 5_i32, 6_i32]);

    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[0..4], &[1_i32, 2_i32, 3_i32, 4_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[1..5], &[2_i32, 3_i32, 4_i32, 5_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[2..6], &[3_i32, 4_i32, 5_i32, 6_i32]);

    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[0..5], &[1_i32, 2_i32, 3_i32, 4_i32, 5_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[1..6], &[2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);

    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[0..6], &[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    assert_eq!(&vec.as_slice::<i32, alloc::Global>()[..], &[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
}

#[test]
fn test_type_erased_vec_slice_from_mut1() {
    let mut vec = TypeErasedVec::from(&[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    {
        let slice = &mut vec.as_mut_slice::<i32, alloc::Global>()[..2];

        assert_eq!(slice, &[1_i32, 2_i32]);

        for i in 0..slice.len() {
            slice[i] += 20_i32;
        }
    }

    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[21_i32, 22_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
}

#[test]
fn test_type_erased_vec_slice_from_mut2() {
    let mut vec = TypeErasedVec::from(&[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    {
        let slice = &mut vec.as_mut_slice::<i32, alloc::Global>()[2..];

        assert_eq!(slice, &[3_i32, 4_i32, 5_i32, 6_i32]);

        for i in 0..slice.len() {
            slice[i] += 20_i32;
        }
    }

    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[1_i32, 2_i32, 23_i32, 24_i32, 25_i32, 26_i32]);
}

#[test]
fn test_type_erased_vec_dedup() {
    fn test_case(this: TypeErasedVec, that: TypeErasedVec) {
        let mut vec = this;
        vec.dedup::<i32, alloc::Global>();
        assert_eq!(vec.as_slice::<i32, alloc::Global>(), that.as_slice::<i32, alloc::Global>());
    }

    test_case(
        TypeErasedVec::new::<i32>(),
        TypeErasedVec::new::<i32>(),
    );
    test_case(
        TypeErasedVec::from(&[1_i32]),
        TypeErasedVec::from(&[1_i32]),
    );
    test_case(
        TypeErasedVec::from(&[1_i32, 1_i32]),
        TypeErasedVec::from(&[1_i32]),
    );
    test_case(
        TypeErasedVec::from(&[1_i32, 2_i32, 3_i32]),
        TypeErasedVec::from(&[1_i32, 2_i32, 3_i32]),
    );
    test_case(
        TypeErasedVec::from(&[1_i32, 1_i32, 2_i32, 3_i32]),
        TypeErasedVec::from(&[1_i32, 2_i32, 3_i32]),
    );
    test_case(
        TypeErasedVec::from(&[1_i32, 2_i32, 2_i32, 3_i32]),
        TypeErasedVec::from(&[1_i32, 2_i32, 3_i32]),
    );
    test_case(
        TypeErasedVec::from(&[1_i32, 2_i32, 3_i32, 3_i32]),
        TypeErasedVec::from(&[1_i32, 2_i32, 3_i32]),
    );
    test_case(
        TypeErasedVec::from(&[1_i32, 1_i32, 2_i32, 2_i32, 2_i32, 3_i32, 3_i32]),
        TypeErasedVec::from(&[1_i32, 2_i32, 3_i32]),
    );
    test_case(
        TypeErasedVec::from(&[1_i32, 1_i32, 2_i32, 2_i32, 2_i32, 3_i32, 3_i32, 4_i32, 5_i32, 5_i32, 5_i32, 6_i32, 7_i32, 7_i32]),
        TypeErasedVec::from(&[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32]),
    );
}

#[test]
fn test_type_erased_vec_dedup_by_key() {
    fn test_case(this: TypeErasedVec, that: TypeErasedVec) {
        let mut vec = this;
        vec.dedup_by_key::<_, _, i32, alloc::Global>(|i| *i / 10);
        assert_eq!(vec.as_slice::<i32, alloc::Global>(), that.as_slice::<i32, alloc::Global>());
    }

    test_case(
        TypeErasedVec::new::<i32>(),
        TypeErasedVec::new::<i32>(),
    );
    test_case(
        TypeErasedVec::from(&[10_i32]),
        TypeErasedVec::from(&[10_i32]),
    );
    test_case(
        TypeErasedVec::from(&[10_i32, 11_i32]),
        TypeErasedVec::from(&[10_i32]),
    );
    test_case(
        TypeErasedVec::from(&[10_i32, 20_i32, 30_i32]),
        TypeErasedVec::from(&[10_i32, 20_i32, 30_i32]),
    );
    test_case(
        TypeErasedVec::from(&[10_i32, 11_i32, 20_i32, 30_i32]),
        TypeErasedVec::from(&[10_i32, 20_i32, 30_i32]),
    );
    test_case(
        TypeErasedVec::from(&[10_i32, 20_i32, 21_i32, 30_i32]),
        TypeErasedVec::from(&[10_i32, 20_i32, 30_i32]),
    );
    test_case(
        TypeErasedVec::from(&[10_i32, 20_i32, 30_i32, 31_i32]),
        TypeErasedVec::from(&[10_i32, 20_i32, 30_i32]),
    );
    test_case(
        TypeErasedVec::from(&[10_i32, 11_i32, 20_i32, 21_i32, 22_i32, 30_i32, 31_i32]),
        TypeErasedVec::from(&[10_i32, 20_i32, 30_i32]),
    );
}

#[test]
#[should_panic]
fn test_type_erased_vec_swap_remove_out_of_bounds1() {
    let mut vec = TypeErasedVec::from(&[0_i32]);
    vec.swap_remove::<i32, alloc::Global>(1);
}

#[test]
#[should_panic]
fn test_type_erased_vec_swap_remove_out_of_bounds2() {
    let mut vec = TypeErasedVec::from(&[0_i32]);
    vec.swap_remove::<usize, alloc::Global>(usize::MAX);
}

#[test]
fn test_type_erased_vec_truncate1() {
    let mut vec = TypeErasedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected = TypeErasedVec::from([899_i32, 615_i32, 623_i32, 487_i32]);
    vec.truncate::<i32, alloc::Global>(4);

    assert_eq!(vec.as_proj::<i32, alloc::Global>(), expected.as_proj::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_truncate2() {
    let mut vec = TypeErasedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected = TypeErasedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    vec.truncate::<i32, alloc::Global>(vec.len());

    assert_eq!(vec.as_proj::<i32, alloc::Global>(), expected.as_proj::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_truncate3() {
    let mut vec = TypeErasedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected = TypeErasedVec::new::<i32>();
    vec.truncate::<i32, alloc::Global>(0);

    assert_eq!(vec.as_proj::<i32, alloc::Global>(), expected.as_proj::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_truncate_len1() {
    let mut vec = TypeErasedVec::from([1_i32]);

    vec.truncate::<i32, alloc::Global>(1);
    assert_eq!(vec.len(), 1);
    vec.truncate::<i32, alloc::Global>(0);
    assert_eq!(vec.len(), 0);
}

#[test]
fn test_type_erased_vec_truncate_len2() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);

    vec.truncate::<i32, alloc::Global>(6);
    assert_eq!(vec.len(), 6);
    vec.truncate::<i32, alloc::Global>(5);
    assert_eq!(vec.len(), 5);
    vec.truncate::<i32, alloc::Global>(3);
    assert_eq!(vec.len(), 3);
    vec.truncate::<i32, alloc::Global>(0);
    assert_eq!(vec.len(), 0);
}

#[test]
fn test_type_erased_vec_truncate_drop1() {
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

    let mut vec = TypeErasedVec::from([Value::new(1_i32)]);

    vec.truncate::<Value, alloc::Global>(1);
    assert_eq!(get_drop_count(), 0);
    vec.truncate::<Value, alloc::Global>(0);
    assert_eq!(get_drop_count(), 1);
}

#[test]
fn test_type_erased_vec_truncate_drop2() {
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

    let mut vec = TypeErasedVec::from([
        Value::new(1_i32),
        Value::new(2_i32),
        Value::new(3_i32),
        Value::new(4_i32),
        Value::new(5_i32),
        Value::new(6_i32),
    ]);

    vec.truncate::<Value, alloc::Global>(6);
    assert_eq!(get_drop_count(), 0);
    vec.truncate::<Value, alloc::Global>(5);
    assert_eq!(get_drop_count(), 1);
    vec.truncate::<Value, alloc::Global>(3);
    assert_eq!(get_drop_count(), 3);
    vec.truncate::<Value, alloc::Global>(0);
    assert_eq!(get_drop_count(), 6);
}

#[test]
#[should_panic]
fn test_type_erased_vec_truncate_fail() {
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

    let mut vec = TypeErasedVec::from([
        BadValue::new(1_usize),
        BadValue::new(2_usize),
        BadValue::new(0xbadbeef_usize),
        BadValue::new(4_usize),
    ]);

    vec.truncate::<BadValue, alloc::Global>(0);
}

#[test]
fn test_type_erased_vec_into_iter_clone_empty() {
    let vec = TypeErasedVec::with_capacity::<i32>(10);
    let mut cloned_vec = TypeErasedVec::new::<i32>();
    for value in vec.into_iter::<i32, alloc::Global>() {
        cloned_vec.push::<i32, alloc::Global>(value);
    }

    assert!(cloned_vec.is_empty());
}

#[test]
fn test_type_erased_vec_into_iter_clone1() {
    let vec = TypeErasedVec::from([1_i32]);
    let mut cloned_vec = TypeErasedVec::new::<i32>();
    for value in vec.clone::<i32, alloc::Global>().into_iter::<i32, alloc::Global>() {
        cloned_vec.push::<i32, alloc::Global>(value);
    }

    assert_eq!(cloned_vec.as_slice::<i32, alloc::Global>(), vec.as_slice::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_into_iter_clone2() {
    let vec = TypeErasedVec::from([1_i32, 2_i32]);
    let mut cloned_vec = TypeErasedVec::new::<i32>();
    for value in vec.clone::<i32, alloc::Global>().into_iter::<i32, alloc::Global>() {
        cloned_vec.push::<i32, alloc::Global>(value);
    }

    assert_eq!(cloned_vec.as_slice::<i32, alloc::Global>(), vec.as_slice::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_into_iter_clone3() {
    let vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32]);
    let mut cloned_vec = TypeErasedVec::new::<i32>();
    for value in vec.clone::<i32, alloc::Global>().into_iter::<i32, alloc::Global>() {
        cloned_vec.push::<i32, alloc::Global>(value);
    }

    assert_eq!(cloned_vec.as_slice::<i32, alloc::Global>(), vec.as_slice::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_into_iter_clone4() {
    let vec = TypeErasedVec::from([String::from("foo")]);
    let mut cloned_vec = TypeErasedVec::new::<String>();
    for value in vec.clone::<String, alloc::Global>().into_iter::<String, alloc::Global>() {
        cloned_vec.push::<String, alloc::Global>(value);
    }

    assert_eq!(cloned_vec.as_slice::<String, alloc::Global>(), vec.as_slice::<String, alloc::Global>());
}

#[test]
fn test_type_erased_vec_into_iter_clone5() {
    let vec = TypeErasedVec::from([String::from("foo"), String::from("bar")]);
    let mut cloned_vec = TypeErasedVec::new::<String>();
    for value in vec.clone::<String, alloc::Global>().into_iter::<String, alloc::Global>() {
        cloned_vec.push::<String, alloc::Global>(value);
    }

    assert_eq!(cloned_vec.as_slice::<String, alloc::Global>(), vec.as_slice::<String, alloc::Global>());
}

#[test]
fn test_type_erased_vec_into_iter_clone6() {
    let vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let mut cloned_vec = TypeErasedVec::new::<String>();
    for value in vec.clone::<String, alloc::Global>().into_iter::<String, alloc::Global>() {
        cloned_vec.push::<String, alloc::Global>(value);
    }

    assert_eq!(cloned_vec.as_slice::<String, alloc::Global>(), vec.as_slice::<String, alloc::Global>());
}

#[test]
fn test_type_erased_vec_into_iter_partial0() {
    let vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
        String::from("garply"),
    ]);
    let expected = TypeErasedVec::new::<String>();
    let mut result = TypeErasedVec::new::<String>();
    for value in vec
        .clone::<String, alloc::Global>()
        .into_iter::<String, alloc::Global>()
        .take(0)
    {
        result.push::<String, alloc::Global>(value);
    }

    assert_eq!(result.as_slice::<String, alloc::Global>(), expected.as_slice::<String, alloc::Global>());
}

#[test]
fn test_type_erased_vec_into_iter_partial1() {
    let vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
        String::from("garply"),
    ]);
    let expected = TypeErasedVec::from([
        String::from("foo"),
    ]);
    let mut result = TypeErasedVec::new::<String>();
    for value in vec
        .clone::<String, alloc::Global>()
        .into_iter::<String, alloc::Global>()
        .take(1)
    {
        result.push::<String, alloc::Global>(value);
    }

    assert_eq!(result.as_slice::<String, alloc::Global>(), expected.as_slice::<String, alloc::Global>());
}

#[test]
fn test_type_erased_vec_into_iter_partial2() {
    let vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
        String::from("garply"),
    ]);
    let expected = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
    ]);
    let mut result = TypeErasedVec::new::<String>();
    for value in vec
        .clone::<String, alloc::Global>()
        .into_iter::<String, alloc::Global>()
        .take(2)
    {
        result.push::<String, alloc::Global>(value);
    }

    assert_eq!(result.as_slice::<String, alloc::Global>(), expected.as_slice::<String, alloc::Global>());
}

#[test]
fn test_type_erased_vec_into_iter_partial3() {
    let vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
        String::from("garply"),
    ]);
    let expected = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
    ]);
    let mut result = TypeErasedVec::new::<String>();
    for value in vec
        .clone::<String, alloc::Global>()
        .into_iter::<String, alloc::Global>()
        .take(3)
    {
        result.push::<String, alloc::Global>(value);
    }

    assert_eq!(result.as_slice::<String, alloc::Global>(), expected.as_slice::<String, alloc::Global>());
}

#[test]
fn test_type_erased_vec_into_iter_partial4() {
    let vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
        String::from("garply"),
    ]);
    let expected = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let mut result = TypeErasedVec::new::<String>();
    for value in vec
        .clone::<String, alloc::Global>()
        .into_iter::<String, alloc::Global>()
        .take(4)
    {
        result.push::<String, alloc::Global>(value);
    }

    assert_eq!(result.as_slice::<String, alloc::Global>(), expected.as_slice::<String, alloc::Global>());
}

#[test]
fn test_type_erased_vec_into_iter_partial5() {
    let vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
        String::from("garply"),
    ]);
    let expected = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
        String::from("quuz"),
    ]);
    let mut result = TypeErasedVec::new::<String>();
    for value in vec
        .clone::<String, alloc::Global>()
        .into_iter::<String, alloc::Global>()
        .take(5)
    {
        result.push::<String, alloc::Global>(value);
    }

    assert_eq!(result.as_slice::<String, alloc::Global>(), expected.as_slice::<String, alloc::Global>());
}

#[test]
fn test_type_erased_vec_into_iter_as_slice1() {
    let vec = TypeErasedVec::from(["foo", "bar", "baz", "quux"]);
    let mut iter = vec.into_iter::<&str, alloc::Global>();
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
fn test_type_erased_vec_into_iter_as_mut_slice1() {
    let vec = TypeErasedVec::from(["foo", "bar", "baz", "quux"]);
    let mut iter = vec.into_iter::<&str, alloc::Global>();
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
fn test_type_erased_vec_into_iter_as_mut_slice2() {
    let vec = TypeErasedVec::from(["foo", "bar", "baz", "quux"]);
    let mut iter = vec.into_iter::<&str, alloc::Global>();
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
fn test_type_erased_vec_drain_empty() {
    let mut vec = TypeErasedVec::new::<i32>();
    let expected = TypeErasedVec::new::<i32>();
    let result: TypeErasedVec = vec.drain::<_, i32, alloc::Global>(..).collect();

    assert_eq!(result.as_slice::<i32, alloc::Global>(), expected.as_slice::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_drain_entire_range1() {
    let mut vec = TypeErasedVec::from([1_i32]);
    let expected_from_drain = TypeErasedVec::from([1_i32]);
    let result_from_drain: TypeErasedVec = vec.drain::<_, i32, alloc::Global>(..).collect();
    let expected_vec = TypeErasedVec::new::<i32>();

    assert_eq!(
        result_from_drain.as_slice::<i32, alloc::Global>(),
        expected_from_drain.as_slice::<i32, alloc::Global>(),
    );
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), expected_vec.as_slice::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_drain_entire_range2() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32]);
    let expected_from_drain = TypeErasedVec::from([1_i32, 2_i32]);
    let result_from_drain: TypeErasedVec = vec.drain::<_, i32, alloc::Global>(..).collect();
    let expected_vec = TypeErasedVec::new::<i32>();

    assert_eq!(
        result_from_drain.as_slice::<i32, alloc::Global>(),
        expected_from_drain.as_slice::<i32, alloc::Global>(),
    );
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), expected_vec.as_slice::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_drain_entire_range3() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32]);
    let expected_from_drain = TypeErasedVec::from([1_i32, 2_i32, 3_i32]);
    let result_from_drain: TypeErasedVec = vec.drain::<_, i32, alloc::Global>(..).collect();
    let expected_vec = TypeErasedVec::new::<i32>();

    assert_eq!(
        result_from_drain.as_slice::<i32, alloc::Global>(),
        expected_from_drain.as_slice::<i32, alloc::Global>(),
    );
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), expected_vec.as_slice::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_drain_entire_range4() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32]);
    let expected_from_drain = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32]);
    let result_from_drain: TypeErasedVec = vec.drain::<_, i32, alloc::Global>(..).collect();
    let expected_vec = TypeErasedVec::new::<i32>();

    assert_eq!(
        result_from_drain.as_slice::<i32, alloc::Global>(),
        expected_from_drain.as_slice::<i32, alloc::Global>(),
    );
    assert_eq!(
        vec.as_slice::<i32, alloc::Global>(),
        expected_vec.as_slice::<i32, alloc::Global>(),
    );
}

#[test]
fn test_type_erased_vec_drain_entire_range5() {
    let mut vec = TypeErasedVec::from([String::from("foo")]);
    let expected_from_drain = vec.clone::<String, alloc::Global>();
    let result_from_drain: TypeErasedVec = vec.drain::<_, String, alloc::Global>(..).collect();
    let expected_vec = TypeErasedVec::new::<String>();

    assert_eq!(
        result_from_drain.as_slice::<String, alloc::Global>(),
        expected_from_drain.as_slice::<String, alloc::Global>(),
    );
    assert_eq!(
        vec.as_slice::<String, alloc::Global>(),
        expected_vec.as_slice::<String, alloc::Global>(),
    );
}

#[test]
fn test_type_erased_vec_drain_entire_range6() {
    let mut vec = TypeErasedVec::from([String::from("foo"), String::from("bar")]);
    let expected_from_drain = vec.clone::<String, alloc::Global>();
    let result_from_drain: TypeErasedVec = vec.drain::<_, String, alloc::Global>(..).collect();
    let expected_vec = TypeErasedVec::new::<String>();

    assert_eq!(
        result_from_drain.as_slice::<String, alloc::Global>(),
        expected_from_drain.as_slice::<String, alloc::Global>(),
    );
    assert_eq!(
        vec.as_slice::<String, alloc::Global>(),
        expected_vec.as_slice::<String, alloc::Global>(),
    );
}

#[test]
fn test_type_erased_vec_drain_entire_range7() {
    let mut vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let expected_from_drain = vec.clone::<String, alloc::Global>();
    let result_from_drain: TypeErasedVec = vec.drain::<_, String, alloc::Global>(..).collect();
    let expected_vec = TypeErasedVec::new::<String>();

    assert_eq!(
        result_from_drain.as_slice::<String, alloc::Global>(),
        expected_from_drain.as_slice::<String, alloc::Global>(),
    );
    assert_eq!(
        vec.as_slice::<String, alloc::Global>(),
        expected_vec.as_slice::<String, alloc::Global>(),
    );
}

#[test]
fn test_type_erased_vec_drain_partial_range1() {
    let mut vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let expected_from_drain: TypeErasedVec = vec
        .as_slice::<String, alloc::Global>()[0..2]
        .iter()
        .cloned()
        .collect();
    let result_from_drain: TypeErasedVec = vec.drain::<_, String, alloc::Global>(0..2).collect();
    let expected_vec = TypeErasedVec::from([
        String::from("baz"),
        String::from("quux"),
    ]);

    assert_eq!(
        result_from_drain.as_slice::<String, alloc::Global>(),
        expected_from_drain.as_slice::<String, alloc::Global>(),
    );
    assert_eq!(
        vec.as_slice::<String, alloc::Global>(),
        expected_vec.as_slice::<String, alloc::Global>(),
    );
}

#[test]
fn test_type_erased_vec_drain_partial_range2() {
    let mut vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let expected_from_drain: TypeErasedVec = vec
        .as_slice::<String, alloc::Global>()[1..3]
        .iter()
        .cloned()
        .collect();
    let result_from_drain: TypeErasedVec = vec.drain::<_, String, alloc::Global>(1..3).collect();
    let expected_vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("quux"),
    ]);

    assert_eq!(
        result_from_drain.as_slice::<String, alloc::Global>(),
        expected_from_drain.as_slice::<String, alloc::Global>(),
    );
    assert_eq!(
        vec.as_slice::<String, alloc::Global>(),
        expected_vec.as_slice::<String, alloc::Global>(),
    );
}

#[test]
fn test_type_erased_vec_drain_partial_range3() {
    let mut vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let expected_from_drain: TypeErasedVec = vec
        .as_slice::<String, alloc::Global>()[1..]
        .iter()
        .cloned()
        .collect();
    let result_from_drain: TypeErasedVec = vec.drain::<_, String, alloc::Global>(1..).collect();
    let expected_vec = TypeErasedVec::from([
        String::from("foo"),
    ]);

    assert_eq!(
        result_from_drain.as_slice::<String, alloc::Global>(),
        expected_from_drain.as_slice::<String, alloc::Global>(),
    );
    assert_eq!(
        vec.as_slice::<String, alloc::Global>(),
        expected_vec.as_slice::<String, alloc::Global>(),
    );
}

#[test]
fn test_type_erased_vec_drain_partial_range4() {
    let mut vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
        String::from("quux"),
    ]);
    let expected_from_drain: TypeErasedVec = vec
        .as_slice::<String, alloc::Global>()[3..]
        .iter()
        .cloned()
        .collect();
    let result_from_drain: TypeErasedVec = vec
        .drain::<_, String, alloc::Global>(3..)
        .collect();
    let expected_vec = TypeErasedVec::from([
        String::from("foo"),
        String::from("bar"),
        String::from("baz"),
    ]);

    assert_eq!(
        result_from_drain.as_slice::<String, alloc::Global>(),
        expected_from_drain.as_slice::<String, alloc::Global>(),
    );
    assert_eq!(
        vec.as_slice::<String, alloc::Global>(),
        expected_vec.as_slice::<String, alloc::Global>(),
    );
}

#[test]
fn test_type_erased_vec_splice1() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let splice_data = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    vec.splice::<_, _, i32, alloc::Global>(2..4, splice_data);

    assert_eq!(
        vec.as_slice::<i32, alloc::Global>(),
        &[1_i32, 2_i32, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 5_i32, 6_i32],
    );
}

#[test]
fn test_type_erased_vec_splice2() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let splice_data = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    vec.splice::<_, _, i32, alloc::Global>(4.., splice_data);

    assert_eq!(
        vec.as_slice::<i32, alloc::Global>(),
        &[1_i32, 2_i32, 3_i32, 4_i32, i32::MAX, i32::MAX, i32::MAX, i32::MAX],
    );
}

#[test]
fn test_type_erased_vec_splice3() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let splice_data = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    vec.splice::<_, _, i32, alloc::Global>(0.., splice_data);

    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[i32::MAX, i32::MAX, i32::MAX, i32::MAX]);
}

#[test]
fn test_type_erased_vec_splice4() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let splice_data = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
    vec.splice::<_, _, i32, alloc::Global>(0..1, splice_data);

    assert_eq!(
        vec.as_slice::<i32, alloc::Global>(),
        &[i32::MAX, i32::MAX, i32::MAX, i32::MAX, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32],
    );
}

#[test]
fn test_type_erased_vec_splice5() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    vec.splice::<_, _, i32, alloc::Global>(1..3, Some(i32::MAX));

    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[1_i32, i32::MAX, 4_i32, 5_i32, 6_i32]);
}

#[test]
fn test_type_erased_vec_splice6() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    vec.splice::<_, _, i32, alloc::Global>(1..3, None);

    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[1_i32, 4_i32, 5_i32, 6_i32]);
}

#[test]
fn test_type_erased_vec_debug_fmt_empty_unit() {
    let vec = TypeErasedVec::new::<()>();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice::<(), alloc::Global>());

    assert_eq!(result, expected);
}

#[test]
fn test_type_erased_vec_debug_fmt_empty_u8() {
    let vec = TypeErasedVec::new::<u8>();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice::<u8, alloc::Global>());

    assert_eq!(result, expected);
}

#[test]
fn test_type_erased_vec_debug_fmt_empty_u16() {
    let vec = TypeErasedVec::new::<u16>();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice::<u16, alloc::Global>());

    assert_eq!(result, expected);
}

#[test]
fn test_type_erased_vec_debug_fmt_empty_u32() {
    let vec = TypeErasedVec::new::<u32>();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice::<u32, alloc::Global>());

    assert_eq!(result, expected);
}

#[test]
fn test_type_erased_vec_debug_fmt_empty_u64() {
    let vec = TypeErasedVec::new::<u64>();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice::<u64, alloc::Global>());

    assert_eq!(result, expected);
}

#[test]
fn test_type_erased_vec_debug_fmt_empty_usize() {
    let vec = TypeErasedVec::new::<usize>();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice::<usize, alloc::Global>());

    assert_eq!(result, expected);
}

#[test]
fn test_type_erased_vec_debug_fmt_empty_string() {
    let vec = TypeErasedVec::new::<String>();
    let expected = "[]";
    let result = format!("{:?}", vec.as_slice::<String, alloc::Global>());

    assert_eq!(result, expected);
}

#[test]
fn test_type_erased_vec_indexing() {
    let vec = TypeErasedVec::from([10_i32, 20_i32, 30_i32]);

    assert_eq!(vec.as_slice::<i32, alloc::Global>()[0], 10_i32);
    assert_eq!(vec.as_slice::<i32, alloc::Global>()[1], 20_i32);
    assert_eq!(vec.as_slice::<i32, alloc::Global>()[2], 30_i32);

    let mut idx = 0;

    assert_eq!(vec.as_slice::<i32, alloc::Global>()[idx], 10_i32);
    idx += 1;
    assert_eq!(vec.as_slice::<i32, alloc::Global>()[idx], 20_i32);
    idx += 1;
    assert_eq!(vec.as_slice::<i32, alloc::Global>()[idx], 30_i32);
}

#[test]
#[should_panic]
fn test_type_erased_vec_indexing_out_of_bounds1() {
    let vec = TypeErasedVec::new::<i32>();
    let _ = vec.as_slice::<i32, alloc::Global>()[0];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_erased_vec_indexing_out_of_bounds2() {
    let vec = TypeErasedVec::from([10_i32]);
    let _ = vec.as_slice::<i32, alloc::Global>()[1];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_erased_vec_indexing_out_of_bounds3() {
    let vec = TypeErasedVec::from([10_i32, 20_i32]);
    let _ = vec.as_slice::<i32, alloc::Global>()[2];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_erased_vec_indexing_out_of_bounds4() {
    let vec = TypeErasedVec::from([10_i32, 20_i32, 30_i32]);
    let _ = vec.as_slice::<i32, alloc::Global>()[3];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_erased_vec_slice_out_of_bounds1() {
    let vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let _ = &vec.as_slice::<i32, alloc::Global>()[!0..];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_erased_vec_slice_out_of_bounds2() {
    let vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let _ = &vec.as_slice::<i32, alloc::Global>()[..7];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_erased_vec_slice_out_of_bounds3() {
    let vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let _ = &vec.as_slice::<i32, alloc::Global>()[!0..5];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_erased_vec_slice_out_of_bounds4() {
    let vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let _ = &vec.as_slice::<i32, alloc::Global>()[1..7];

    assert!(true);
}

#[test]
#[should_panic]
fn test_type_erased_vec_slice_out_of_bounds5() {
    let vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let _ = &vec.as_slice::<i32, alloc::Global>()[3..2];

    assert!(true);
}

#[cfg(feature = "nightly")]
#[test]
fn test_type_erased_vec_into_boxed_slice() {
    let vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32]);
    let boxed_slice = vec.into_boxed_slice::<i32, alloc::Global>();

    assert_eq!(&*boxed_slice, [1_i32, 2_i32, 3_i32]);
}

#[cfg(feature = "nightly")]
#[test]
fn test_type_erased_vec_into_boxed_slice_from_boxed_slice() {
    let vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32]);
    let boxed_slice = vec.into_boxed_slice::<i32, alloc::Global>();
    let new_vec = TypeErasedVec::from(boxed_slice);

    assert_eq!(&**new_vec.as_proj::<i32, alloc::Global>(), &[1_i32, 2_i32, 3_i32]);
    assert_eq!(new_vec.as_slice::<i32, alloc::Global>(), &[1_i32, 2_i32, 3_i32]);
}

#[test]
fn test_type_erased_vec_append1() {
    let mut vec1 = TypeErasedVec::from([1_i32, 2_i32, 3_i32]);
    let mut vec2 = TypeErasedVec::from([4_i32, 5_i32, 6_i32, 7_i32]);
    vec1.append::<i32, alloc::Global>(&mut vec2);

    assert_eq!(&**vec1.as_proj::<i32, alloc::Global>(), &[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32]);
    assert_eq!(vec1.as_slice::<i32, alloc::Global>(), &[1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32]);
    assert!(vec2.is_empty());
}

#[test]
fn test_type_erased_vec_append2() {
    let mut vec1 = TypeErasedVec::from([1_i32, 2_i32, 3_i32]);
    let mut vec2 = TypeErasedVec::new::<i32>();
    vec1.append::<i32, alloc::Global>(&mut vec2);

    assert_eq!(&**vec1.as_proj::<i32, alloc::Global>(), &[1_i32, 2_i32, 3_i32]);
    assert_eq!(vec1.as_slice::<i32, alloc::Global>(), &[1_i32, 2_i32, 3_i32]);
    assert!(vec2.is_empty());
}

#[test]
fn test_type_erased_vec_split_off1() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let vec_ptr = vec.as_ptr::<i32, alloc::Global>();
    let old_capacity = vec.capacity();

    let split_vec = vec.split_off::<i32, alloc::Global>(4);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), [1_i32, 2_i32, 3_i32, 4_i32]);
    assert_eq!(split_vec.as_slice::<i32, alloc::Global>(), [5_i32, 6_i32]);
    assert_eq!(vec.capacity(), old_capacity);
    assert_eq!(vec.as_ptr::<i32, alloc::Global>(), vec_ptr);
}

#[test]
fn test_type_erased_vec_split_off2() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let vec_ptr = vec.as_ptr::<i32, alloc::Global>();
    let old_capacity = vec.capacity();

    let split_vec = vec.split_off::<i32, alloc::Global>(0);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), []);
    assert_eq!(split_vec.as_slice::<i32, alloc::Global>(), [1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    assert_eq!(vec.capacity(), old_capacity);
    assert_eq!(vec.as_ptr::<i32, alloc::Global>(), vec_ptr);
}

#[test]
fn test_type_erased_vec_split_off3() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    let vec_ptr = vec.as_ptr::<i32, alloc::Global>();
    let old_capacity = vec.capacity();

    let split_vec = vec.split_off::<i32, alloc::Global>(6);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), [1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    assert_eq!(split_vec.as_slice::<i32, alloc::Global>(), []);
    assert_eq!(vec.capacity(), old_capacity);
    assert_eq!(vec.as_ptr::<i32, alloc::Global>(), vec_ptr);
}

#[test]
fn test_type_erased_vec_split_off4() {
    let mut vec = TypeErasedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected1 = TypeErasedVec::from([899_i32, 615_i32, 623_i32, 487_i32]);
    let expected2 = TypeErasedVec::from([935_i32, 806_i32, 381_i32, 967_i32]);
    let result2 = vec.split_off::<i32, alloc::Global>(4);
    let result1 = vec.clone::<i32, alloc::Global>();

    assert_eq!(result1.as_slice::<i32, alloc::Global>(), expected1.as_slice::<i32, alloc::Global>());
    assert_eq!(result2.as_slice::<i32, alloc::Global>(), expected2.as_slice::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_split_off5() {
    let mut vec = TypeErasedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected1 = TypeErasedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected2 = TypeErasedVec::new::<i32>();
    let result2 = vec.split_off::<i32, alloc::Global>(vec.len());
    let result1 = vec.clone::<i32, alloc::Global>();

    assert_eq!(result1.as_slice::<i32, alloc::Global>(), expected1.as_slice::<i32, alloc::Global>());
    assert_eq!(result2.as_slice::<i32, alloc::Global>(), expected2.as_slice::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_split_off6() {
    let mut vec = TypeErasedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let expected1 = TypeErasedVec::new::<i32>();
    let expected2 = TypeErasedVec::from([
        899_i32,
        615_i32,
        623_i32,
        487_i32,
        935_i32,
        806_i32,
        381_i32,
        967_i32,
    ]);
    let result2 = vec.split_off::<i32, alloc::Global>(0);
    let result1 = vec.clone::<i32, alloc::Global>();

    assert_eq!(result1.as_slice::<i32, alloc::Global>(), expected1.as_slice::<i32, alloc::Global>());
    assert_eq!(result2.as_slice::<i32, alloc::Global>(), expected2.as_slice::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_reserve_exact() {
    let mut vec = TypeErasedVec::new::<i32>();
    assert_eq!(vec.capacity(), 0);

    vec.reserve_exact::<i32, alloc::Global>(2);
    assert!(vec.capacity() >= 2);

    for i in 0..16 {
        vec.push::<i32, alloc::Global>(i);
    }

    assert!(vec.capacity() >= 16);
    vec.reserve_exact::<i32, alloc::Global>(16);
    assert!(vec.capacity() >= 32);

    vec.push::<i32, alloc::Global>(16);

    vec.reserve_exact::<i32, alloc::Global>(16);
    assert!(vec.capacity() >= 33)
}

#[test]
fn test_type_erased_vec_extract_if_empty_true() {
    let mut vec = TypeErasedVec::new::<i32>();
    assert_eq!(vec.len(), 0);
    {
        let mut iter = vec.extract_if::<_, _, i32, alloc::Global>(.., |_| true);
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
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), []);
}

#[test]
fn test_type_erased_vec_extract_if_empty_false() {
    let mut vec = TypeErasedVec::new::<i32>();
    assert_eq!(vec.len(), 0);
    {
        let mut iter = vec.extract_if::<_, _, i32, alloc::Global>(.., |_| false);
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
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), []);
}

#[test]
fn test_type_erased_vec_extract_if_total_true() {
    let mut vec = TypeErasedVec::from([
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
        let mut iter = vec.extract_if::<_, _, i32, alloc::Global>(.., |_| true);
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
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), []);
}

#[test]
fn test_type_erased_vec_extract_if_total_false() {
    let mut vec = TypeErasedVec::from([
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
        let mut iter = vec.extract_if::<_, _, i32, alloc::Global>(.., |_| false);
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
    assert_eq!(
        vec.as_slice::<i32, alloc::Global>(),
        [0_i32, 1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32, 8_i32, 9_i32, 10_i32],
    );
}

#[test]
fn test_type_erased_vec_extract_if_partial_true() {
    let mut vec = TypeErasedVec::from([
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
        let mut iter = vec.extract_if::<_, _, i32, alloc::Global>(2..8, |_| true);
        while let Some(_) = iter.next() {
            count += 1;
        }

        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    assert_eq!(count, 6);
    assert_eq!(vec.len(), old_length - count);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), [0_i32, 1_i32, 8_i32, 9_i32, 10_i32]);
}

#[test]
fn test_type_erased_vec_extract_if_partial_false() {
    let mut vec = TypeErasedVec::from([
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
        let mut iter = vec.extract_if::<_, _, i32, alloc::Global>(2..8, |_| false);
        while let Some(_) = iter.next() {
            count += 1;
        }

        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.size_hint(), (0, Some(0)));
    }

    assert_eq!(count, 0);
    assert_eq!(vec.len(), old_length);
    assert_eq!(
        vec.as_slice::<i32, alloc::Global>(),
        [0_i32, 1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32, 7_i32, 8_i32, 9_i32, 10_i32],
    );
}

#[test]
#[should_panic]
fn test_type_erased_vec_extract_if_out_of_bounds() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32]);
    let _ = vec.extract_if::<_, _, i32, alloc::Global>(10.., |_| true).for_each(drop);

    assert!(true);
}

#[test]
fn test_type_erased_vec_extract_if_retains_unvisited_elements() {
    let mut vec = TypeErasedVec::from([
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
        let mut iter = vec.extract_if::<_, _, i32, alloc::Global>(.., |_| true);
        while count < 3 {
            let _ = iter.next();
            count += 1;
        }
    }

    assert_eq!(
        vec.as_slice::<i32, alloc::Global>(),
        [3_i32, 4_i32, 5_i32, 6_i32, 7_i32, 8_i32, 9_i32, 10_i32],
    );
}

#[rustfmt::skip]
#[test]
fn test_type_erased_vec_extract_if_many1() {
    let mut vec = TypeErasedVec::from([
        0_i32,    1_i32,    2_i32,    3_i32,    4_i32,    5_i32,    6_i32,    7_i32,    8_i32,
        9_i32,    10_i32,   i32::MAX, i32::MAX, i32::MAX, i32::MAX, 11_i32,   12_i32,   13_i32,
        14_i32,   15_i32,   16_i32,   17_i32,   18_i32,   19_i32,   20_i32,   21_i32,   22_i32,
        23_i32,   24_i32,   25_i32,   26_i32,   27_i32,   28_i32,   29_i32,   30_i32,   31_i32,
        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 32_i32,
    ]);
    let extracted: TypeErasedVec = vec
        .extract_if::<_, _, i32, alloc::Global>(.., |v| *v == i32::MAX)
        .collect();

    let expected_vec = TypeErasedVec::from_iter(0_i32..=32_i32);
    let expected_extracted = TypeErasedVec::from([i32::MAX; 12]);

    assert_eq!(vec.as_proj::<i32, alloc::Global>(), expected_vec.as_proj::<i32, alloc::Global>());
    assert_eq!(extracted.as_proj::<i32, alloc::Global>(), expected_extracted.as_proj::<i32, alloc::Global>());
}

#[rustfmt::skip]
#[test]
fn test_type_erased_vec_extract_if_many2() {
    let mut vec = TypeErasedVec::from([
        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 0_i32,
        1_i32,    2_i32,    3_i32,    4_i32,    5_i32,    6_i32,    7_i32,    8_i32,
        9_i32,    10_i32,   11_i32,   12_i32,   13_i32,   14_i32,   15_i32,   16_i32,
        17_i32,   18_i32,   19_i32,   20_i32,   21_i32,   22_i32,   23_i32,   24_i32,
        25_i32,   26_i32,   27_i32,   28_i32,   29_i32,   30_i32,   31_i32,   32_i32,
        i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    ]);
    let extracted: TypeErasedVec = vec
        .extract_if::<_, _, i32, alloc::Global>(.., |v| *v == i32::MAX)
        .collect();

    let expected_vec = TypeErasedVec::from_iter(0_i32..=32_i32);
    let expected_extracted = TypeErasedVec::from([i32::MAX; 15]);

    assert_eq!(vec.as_proj::<i32, alloc::Global>(), expected_vec.as_proj::<i32, alloc::Global>());
    assert_eq!(extracted.as_proj::<i32, alloc::Global>(), expected_extracted.as_proj::<i32, alloc::Global>());
}

#[rustfmt::skip]
#[test]
fn test_type_erased_vec_extract_if_many3() {
    let mut vec = TypeErasedVec::from([
        i32::MAX, 0_i32,    i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX, 1_i32,
        2_i32,    3_i32,    4_i32,    5_i32,    6_i32,    7_i32,    8_i32,    9_i32,
        10_i32,   i32::MAX, i32::MAX, i32::MAX, i32::MAX, 11_i32,   12_i32,   13_i32,
        14_i32,   15_i32,   16_i32,   17_i32,   18_i32,   19_i32,   20_i32,   21_i32,
        22_i32,   23_i32,   24_i32,   25_i32,   26_i32,   27_i32,   28_i32,   29_i32,
        30_i32,   31_i32,   32_i32,   i32::MAX, i32::MAX, i32::MAX, i32::MAX, i32::MAX,
    ]);
    let extracted: TypeErasedVec = vec
        .extract_if::<_, _, i32, alloc::Global>(.., |v| *v == i32::MAX)
        .collect();

    let expected_vec = TypeErasedVec::from_iter(0_i32..=32_i32);
    let expected_extracted = TypeErasedVec::from([i32::MAX; 15]);

    assert_eq!(vec.as_proj::<i32, alloc::Global>(), expected_vec.as_proj::<i32, alloc::Global>());
    assert_eq!(extracted.as_proj::<i32, alloc::Global>(), expected_extracted.as_proj::<i32, alloc::Global>());
}

#[rustfmt::skip]
#[test]
fn test_type_erased_vec_extract_if_many4() {
    let mut vec = TypeErasedVec::from([
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
    let extracted: TypeErasedVec = vec
        .extract_if::<_, _, i32, alloc::Global>(.., |v| *v == i32::MAX)
        .collect();

    let expected_vec = TypeErasedVec::from_iter(0_i32..=32_i32);
    let expected_extracted = TypeErasedVec::from([i32::MAX; 33]);

    assert_eq!(vec.as_proj::<i32, alloc::Global>(), expected_vec.as_proj::<i32, alloc::Global>());
    assert_eq!(extracted.as_proj::<i32, alloc::Global>(), expected_extracted.as_proj::<i32, alloc::Global>());
}

#[test]
fn test_type_erased_vec_retain1() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    vec.retain::<_, i32, alloc::Global>(|&x| x % 2_i32 == 0_i32);

    assert_eq!(vec.as_slice::<i32, alloc::Global>(), [2_i32, 4_i32, 6_i32]);
}

#[test]
fn test_type_erased_vec_retain2() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    vec.retain::<_, i32, alloc::Global>(|_| true);

    assert_eq!(vec.as_slice::<i32, alloc::Global>(), [1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
}

#[test]
fn test_type_erased_vec_retain3() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32, 4_i32, 5_i32, 6_i32]);
    vec.retain::<_, i32, alloc::Global>(|_| false);

    assert_eq!(vec.as_slice::<i32, alloc::Global>(), []);
}

#[test]
fn test_type_erased_vec_shift_insert_first() {
    let mut vec = TypeErasedVec::new::<i32>();
    assert!(vec.is_empty());
    vec.shift_insert::<i32, alloc::Global>(0, 1_i32);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[1_i32]);
    vec.shift_insert::<i32, alloc::Global>(0, 2_i32);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[2_i32, 1_i32]);
    vec.shift_insert::<i32, alloc::Global>(0, 3_i32);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[3_i32, 2_i32, 1_i32]);
}

#[test]
fn test_type_erased_vec_shift_insert_last() {
    let mut vec = TypeErasedVec::new::<i32>();
    assert!(vec.is_empty());
    vec.shift_insert::<i32, alloc::Global>(0, 1_i32);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[1_i32]);
    vec.shift_insert::<i32, alloc::Global>(1, 2_i32);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[1_i32, 2_i32]);
    vec.shift_insert::<i32, alloc::Global>(2, 3_i32);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[1_i32, 2_i32, 3_i32]);
}

#[test]
fn test_type_erased_vec_shift_insert_middle() {
    let mut vec = TypeErasedVec::from([i32::MAX, i32::MAX]);
    assert_eq!(vec.len(), 2);
    vec.shift_insert::<i32, alloc::Global>(1, 1_i32);
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[i32::MAX, 1_i32, i32::MAX]);
    vec.shift_insert::<i32, alloc::Global>(1, 2_i32);
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[i32::MAX, 2_i32, 1_i32, i32::MAX]);
    vec.shift_insert::<i32, alloc::Global>(1, 3_i32);
    assert_eq!(vec.len(), 5);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[i32::MAX, 3_i32, 2_i32, 1_i32, i32::MAX]);
}

#[test]
#[should_panic]
fn test_type_erased_vec_shift_insert_out_of_bounds() {
    let mut vec = TypeErasedVec::new::<i32>();
    vec.shift_insert::<i32, alloc::Global>(2, 1_i32);

    assert!(true);
}

#[test]
fn test_type_erased_vec_swap_remove1() {
    let mut vec = TypeErasedVec::from(['a', 'b', 'c']);
    let result = vec.swap_remove::<char, alloc::Global>(0);

    assert_eq!(result, 'a');
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice::<char, alloc::Global>(), &['c', 'b']);
}

#[test]
fn test_type_erased_vec_swap_remove2() {
    let mut vec = TypeErasedVec::from(['a', 'b', 'c']);
    let result = vec.swap_remove::<char, alloc::Global>(1);

    assert_eq!(result, 'b');
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice::<char, alloc::Global>(), &['a', 'c']);
}

#[test]
fn test_type_erased_vec_swap_remove3() {
    let mut vec = TypeErasedVec::from(['a', 'b', 'c']);
    let result = vec.swap_remove::<char, alloc::Global>(2);

    assert_eq!(result, 'c');
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice::<char, alloc::Global>(), &['a', 'b']);
}

#[test]
#[should_panic]
fn test_type_erased_vec_swap_remove_out_of_bounds() {
    let mut vec = TypeErasedVec::new::<i32>();
    vec.swap_remove::<i32, alloc::Global>(2);

    assert!(true);
}

#[test]
fn test_type_erased_vec_shift_remove1() {
    let mut vec = TypeErasedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove::<char, alloc::Global>(0);

    assert_eq!(result, 'a');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice::<char, alloc::Global>(), &['b', 'c', 'd', 'e']);
}

#[test]
fn test_type_erased_vec_shift_remove2() {
    let mut vec = TypeErasedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove::<char, alloc::Global>(1);

    assert_eq!(result, 'b');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice::<char, alloc::Global>(), &['a', 'c', 'd', 'e']);
}

#[test]
fn test_type_erased_vec_shift_remove3() {
    let mut vec = TypeErasedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove::<char, alloc::Global>(2);

    assert_eq!(result, 'c');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice::<char, alloc::Global>(), &['a', 'b', 'd', 'e']);
}

#[test]
fn test_type_erased_vec_shift_remove4() {
    let mut vec = TypeErasedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove::<char, alloc::Global>(3);

    assert_eq!(result, 'd');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice::<char, alloc::Global>(), &['a', 'b', 'c', 'e']);
}

#[test]
fn test_type_erased_vec_shift_remove5() {
    let mut vec = TypeErasedVec::from(['a', 'b', 'c', 'd', 'e']);
    let result = vec.shift_remove::<char, alloc::Global>(4);

    assert_eq!(result, 'e');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice::<char, alloc::Global>(), &['a', 'b', 'c', 'd']);
}

#[test]
fn test_type_erased_vec_shift_remove6() {
    let mut vec = TypeErasedVec::from(['a', 'b', 'c', 'd', 'e']);
    assert_eq!(vec.shift_remove::<char, alloc::Global>(0), 'a');
    assert_eq!(vec.len(), 4);
    assert_eq!(vec.as_slice::<char, alloc::Global>(), &['b', 'c', 'd', 'e']);
    assert_eq!(vec.shift_remove::<char, alloc::Global>(0), 'b');
    assert_eq!(vec.len(), 3);
    assert_eq!(vec.as_slice::<char, alloc::Global>(), &['c', 'd', 'e']);
    assert_eq!(vec.shift_remove::<char, alloc::Global>(0), 'c');
    assert_eq!(vec.len(), 2);
    assert_eq!(vec.as_slice::<char, alloc::Global>(), &['d', 'e']);
    assert_eq!(vec.shift_remove::<char, alloc::Global>(0), 'd');
    assert_eq!(vec.len(), 1);
    assert_eq!(vec.as_slice::<char, alloc::Global>(), &['e']);
    assert_eq!(vec.shift_remove::<char, alloc::Global>(0), 'e');
    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());
}

#[test]
#[should_panic]
fn test_type_erased_vec_shift_remove_out_of_bounds() {
    let mut vec = TypeErasedVec::new::<i32>();
    vec.shift_remove::<i32, alloc::Global>(2);

    assert!(true);
}

#[test]
fn test_type_erased_vec_pop_if_empty_true() {
    let mut vec = TypeErasedVec::new::<i32>();

    assert_eq!(vec.pop_if::<_, i32, alloc::Global>(|_| true), None);
    assert!(vec.is_empty());
}

#[test]
fn test_type_erased_vec_pop_if_empty_false() {
    let mut vec = TypeErasedVec::new::<i32>();

    assert_eq!(vec.pop_if::<_, i32, alloc::Global>(|_| false), None);
    assert!(vec.is_empty());
}

#[test]
fn test_type_erased_vec_pop_if_true() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32]);

    assert_eq!(vec.pop_if::<_, i32, alloc::Global>(|_| true), Some(3_i32));
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[1_i32, 2_i32]);
}

#[test]
fn test_type_erased_vec_pop_if_false() {
    let mut vec = TypeErasedVec::from([1_i32, 2_i32, 3_i32]);

    assert_eq!(vec.pop_if::<_, i32, alloc::Global>(|_| false), None);
    assert_eq!(vec.as_slice::<i32, alloc::Global>(), &[1_i32, 2_i32, 3_i32]);
}

#[test]
fn test_type_erased_vec_reserve1() {
    let mut vec = TypeErasedVec::new::<usize>();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);

    vec.reserve::<usize, alloc::Global>(additional);

    assert!(vec.capacity() >= additional);
}

#[test]
fn test_type_erased_vec_reserve2() {
    let mut vec = TypeErasedVec::new::<usize>();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);

    vec.reserve::<usize, alloc::Global>(additional);

    assert!(vec.capacity() >= additional);

    let old_capacity = vec.capacity();
    vec.push::<usize, alloc::Global>(usize::MAX);
    for _ in 1..(vec.capacity() - 1) {
        vec.push::<usize, alloc::Global>(0_usize);
    }

    vec.push::<usize, alloc::Global>(usize::MAX);

    assert_eq!(vec.len(), vec.capacity());
    assert_eq!(vec.capacity(), old_capacity);

    assert_eq!(vec.as_slice::<usize, alloc::Global>()[0], usize::MAX);
    for i in 1..(vec.len() - 1) {
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[i], 0_usize);
    }
    assert_eq!(vec.as_slice::<usize, alloc::Global>()[vec.len() - 1], usize::MAX);
}

#[test]
fn test_type_erased_vec_reserve3() {
    let mut vec = TypeErasedVec::new::<usize>();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.len(), 0);

    for i in 0..4 {
        let old_capacity = vec.capacity();
        vec.reserve::<usize, alloc::Global>(additional);

        assert!(vec.capacity() >= old_capacity + additional);
        assert!(vec.len() <= vec.capacity());

        let length = vec.len();
        vec.push::<usize, alloc::Global>(usize::MAX);
        for _ in (length + 1)..(vec.capacity() - 1) {
            vec.push::<usize, alloc::Global>(i);
        }
        vec.push::<usize, alloc::Global>(usize::MAX);

        assert_eq!(vec.len(), vec.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..4 {
        for j in (current_start + 1)..vec.len() {
            if vec.as_slice::<usize, alloc::Global>()[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[current_start], usize::MAX);
        for value in vec.as_slice::<usize, alloc::Global>()[(current_start + 1)..current_end].iter().copied() {
            assert_eq!(value, i);
        }
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[test]
fn test_type_erased_vec_reserve_exact1() {
    let mut vec = TypeErasedVec::new::<usize>();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);

    vec.reserve_exact::<usize, alloc::Global>(additional);

    assert!(vec.capacity() >= additional);
}

#[test]
fn test_type_erased_vec_reserve_exact2() {
    let mut vec = TypeErasedVec::new::<usize>();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);

    vec.reserve_exact::<usize, alloc::Global>(additional);

    assert!(vec.capacity() >= additional);

    let old_capacity = vec.capacity();
    vec.push::<usize, alloc::Global>(usize::MAX);
    for _ in 1..(vec.capacity() - 1) {
        vec.push::<usize, alloc::Global>(0_usize);
    }

    vec.push::<usize, alloc::Global>(usize::MAX);

    assert_eq!(vec.len(), vec.capacity());
    assert_eq!(vec.capacity(), old_capacity);

    assert_eq!(vec.as_slice::<usize, alloc::Global>()[0], usize::MAX);
    for i in 1..(vec.len() - 1) {
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[i], 0_usize);
    }
    assert_eq!(vec.as_slice::<usize, alloc::Global>()[vec.len() - 1], usize::MAX);
}

#[test]
fn test_type_erased_vec_reserve_exact3() {
    let mut vec = TypeErasedVec::new::<usize>();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.len(), 0);

    for i in 0..32 {
        let old_capacity = vec.capacity();
        vec.reserve_exact::<usize, alloc::Global>(additional);

        assert!(vec.capacity() >= old_capacity + additional);
        assert!(vec.len() <= vec.capacity());

        let length = vec.len();
        vec.push::<usize, alloc::Global>(usize::MAX);
        for _ in (length + 1)..(vec.capacity() - 1) {
            vec.push::<usize, alloc::Global>(i);
        }
        vec.push::<usize, alloc::Global>(usize::MAX);

        assert_eq!(vec.len(), vec.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..32 {
        for j in (current_start + 1)..vec.len() {
            if vec.as_slice::<usize, alloc::Global>()[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[current_start], usize::MAX);
        for value in vec.as_slice::<usize, alloc::Global>()[(current_start + 1)..current_end].iter().copied() {
            assert_eq!(value, i);
        }
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[test]
fn test_type_erased_vec_try_reserve1() {
    let mut vec = TypeErasedVec::new::<usize>();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.try_reserve::<usize, alloc::Global>(additional), Ok(()));
    assert!(vec.capacity() >= additional);
}

#[test]
fn test_type_erased_vec_try_reserve2() {
    let mut vec = TypeErasedVec::new::<usize>();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.try_reserve::<usize, alloc::Global>(additional), Ok(()));
    assert!(vec.capacity() >= additional);

    let old_capacity = vec.capacity();
    vec.push::<usize, alloc::Global>(usize::MAX);
    for _ in 1..(vec.capacity() - 1) {
        vec.push::<usize, alloc::Global>(0_usize);
    }

    vec.push::<usize, alloc::Global>(usize::MAX);

    assert_eq!(vec.len(), vec.capacity());
    assert_eq!(vec.capacity(), old_capacity);

    assert_eq!(vec.as_slice::<usize, alloc::Global>()[0], usize::MAX);
    for i in 1..(vec.len() - 1) {
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[i], 0_usize);
    }
    assert_eq!(vec.as_slice::<usize, alloc::Global>()[vec.len() - 1], usize::MAX);
}

#[test]
fn test_type_erased_vec_try_reserve3() {
    let mut vec = TypeErasedVec::new::<usize>();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.len(), 0);

    for i in 0..4 {
        let old_capacity = vec.capacity();
        assert_eq!(vec.try_reserve::<usize, alloc::Global>(additional), Ok(()));

        assert!(vec.capacity() >= old_capacity + additional);
        assert!(vec.len() <= vec.capacity());

        let length = vec.len();
        vec.push::<usize, alloc::Global>(usize::MAX);
        for _ in (length + 1)..(vec.capacity() - 1) {
            vec.push::<usize, alloc::Global>(i);
        }
        vec.push::<usize, alloc::Global>(usize::MAX);

        assert_eq!(vec.len(), vec.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..4 {
        for j in (current_start + 1)..vec.len() {
            if vec.as_slice::<usize, alloc::Global>()[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[current_start], usize::MAX);
        for value in vec.as_slice::<usize, alloc::Global>()[(current_start + 1)..current_end].iter().copied() {
            assert_eq!(value, i);
        }
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[test]
fn test_type_erased_vec_try_reserve_exact1() {
    let mut vec = TypeErasedVec::new::<usize>();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.try_reserve_exact::<usize, alloc::Global>(additional), Ok(()));
    assert!(vec.capacity() >= additional);
}

#[test]
fn test_type_erased_vec_try_reserve_exact2() {
    let mut vec = TypeErasedVec::new::<usize>();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.try_reserve_exact::<usize, alloc::Global>(additional), Ok(()));
    assert!(vec.capacity() >= additional);

    let old_capacity = vec.capacity();
    vec.push::<usize, alloc::Global>(usize::MAX);
    for _ in 1..(vec.capacity() - 1) {
        vec.push::<usize, alloc::Global>(0_usize);
    }

    vec.push::<usize, alloc::Global>(usize::MAX);

    assert_eq!(vec.len(), vec.capacity());
    assert_eq!(vec.capacity(), old_capacity);

    assert_eq!(vec.as_slice::<usize, alloc::Global>()[0], usize::MAX);
    for i in 1..(vec.len() - 1) {
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[i], 0_usize);
    }
    assert_eq!(vec.as_slice::<usize, alloc::Global>()[vec.len() - 1], usize::MAX);
}

#[test]
fn test_type_erased_vec_try_reserve_exact3() {
    let mut vec = TypeErasedVec::new::<usize>();
    let additional = 100;

    assert_eq!(vec.capacity(), 0);
    assert_eq!(vec.len(), 0);

    for i in 0..32 {
        let old_capacity = vec.capacity();
        assert_eq!(vec.try_reserve_exact::<usize, alloc::Global>(additional), Ok(()));

        assert!(vec.capacity() >= old_capacity + additional);
        assert!(vec.len() <= vec.capacity());

        let length = vec.len();
        vec.push::<usize, alloc::Global>(usize::MAX);
        for _ in (length + 1)..(vec.capacity() - 1) {
            vec.push::<usize, alloc::Global>(i);
        }
        vec.push::<usize, alloc::Global>(usize::MAX);

        assert_eq!(vec.len(), vec.capacity());
    }

    let mut current_start = 0;
    let mut current_end = 1;
    for i in 0..32 {
        for j in (current_start + 1)..vec.len() {
            if vec.as_slice::<usize, alloc::Global>()[j] == usize::MAX {
                break;
            }

            current_end += 1;
        }

        assert!(current_start < current_end);
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[current_start], usize::MAX);
        for value in vec.as_slice::<usize, alloc::Global>()[(current_start + 1)..current_end].iter().copied() {
            assert_eq!(value, i);
        }
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[current_end], usize::MAX);

        current_start = current_end + 1;
        current_end = current_start + 1;
    }
}

#[test]
fn test_type_erased_vec_shrink_to_fit1() {
    let mut vec = TypeErasedVec::with_capacity::<(usize, usize)>(10);
    assert_eq!(vec.capacity(), 10);

    vec.extend::<_, (usize, usize), alloc::Global>([
        (1_usize, usize::MAX),
        (2_usize, usize::MAX),
        (3_usize, usize::MAX),
    ]);
    assert!(vec.len() <= vec.capacity());

    assert_eq!(vec.as_slice::<(usize, usize), alloc::Global>()[0], (1_usize, usize::MAX));
    assert_eq!(vec.as_slice::<(usize, usize), alloc::Global>()[1], (2_usize, usize::MAX));
    assert_eq!(vec.as_slice::<(usize, usize), alloc::Global>()[2], (3_usize, usize::MAX));

    vec.shrink_to_fit::<(usize, usize), alloc::Global>();

    assert!(vec.len() <= vec.capacity());
    assert_eq!(vec.as_slice::<(usize, usize), alloc::Global>()[0], (1_usize, usize::MAX));
    assert_eq!(vec.as_slice::<(usize, usize), alloc::Global>()[1], (2_usize, usize::MAX));
    assert_eq!(vec.as_slice::<(usize, usize), alloc::Global>()[2], (3_usize, usize::MAX));
}

#[test]
fn test_type_erased_vec_shrink_to_fit2() {
    let mut vec = TypeErasedVec::new::<usize>();
    for i in 0..128 {
        assert_eq!(vec.len(), i);

        vec.push::<usize, alloc::Global>(i * i);

        assert_eq!(vec.len(), i + 1);
        assert!(vec.capacity() >= i + 1);
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[i], i * i);
        assert_eq!(vec.get::<_, usize, alloc::Global>(i), Some(&(i * i)));

        vec.shrink_to_fit::<usize, alloc::Global>();

        assert_eq!(vec.len(), i + 1);
        assert!(vec.capacity() >= i + 1);
        assert_eq!(vec.as_slice::<usize, alloc::Global>()[i], i * i);
        assert_eq!(vec.get::<_, usize, alloc::Global>(i), Some(&(i * i)));
    }
}
