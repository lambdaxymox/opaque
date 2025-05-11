use opaque_hash::OpaqueBuildHasher;

use core::any;
use std::hash;
use std::hash::{BuildHasher, Hasher};

fn run_hashers<T>(value: T) -> (u64, u64)
where
    T: any::Any + hash::Hash,
{
    let default_build_hasher = hash::RandomState::new();
    let mut hasher = default_build_hasher.build_hasher();
    value.hash(&mut hasher);
    let expected = hasher.finish();

    let opaque_build_hasher = OpaqueBuildHasher::new(default_build_hasher);
    let mut opaque_hasher = opaque_build_hasher.build_hasher::<hash::RandomState>();
    value.hash(&mut opaque_hasher);
    let result = opaque_hasher.finish();

    (result, expected)
}

fn run_test_opaque_hasher<T, I>(values: I)
where
    T: any::Any + hash::Hash,
    I: Iterator<Item = T>,
{
    for value in values {
        let (result, expected) = run_hashers(value);

        assert_eq!(result, expected);
    }
}

#[test]
fn test_opaque_hasher_i8() {
    run_test_opaque_hasher((i8::MIN..i8::MAX).into_iter());
}

#[test]
fn test_opaque_hasher_i16() {
    run_test_opaque_hasher((i16::MIN..i16::MAX).into_iter());
}

#[test]
fn test_opaque_hasher_i32() {
    run_test_opaque_hasher((i32::MIN..i32::MAX).step_by(i16::MAX as usize).into_iter());
}

#[test]
fn test_opaque_hasher_u8() {
    run_test_opaque_hasher((u8::MIN..u8::MAX).into_iter());
}

#[test]
fn test_opaque_hasher_u16() {
    run_test_opaque_hasher((u16::MIN..u16::MAX).into_iter());
}

#[test]
fn test_opaque_hasher_u32() {
    run_test_opaque_hasher((u32::MIN..u32::MAX).step_by(u16::MAX as usize).into_iter());
}
