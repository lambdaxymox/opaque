use std::hash::{BuildHasher, Hash, Hasher, RandomState};
use opaque_hash::OpaqueBuildHasher;

fn run_hashers<T>(value: T) -> (u64, u64)
where
    T: Hash + 'static,
{
    let default_hash_builder = RandomState::new();
    let mut hasher = default_hash_builder.build_hasher();
    value.hash(&mut hasher);
    let expected = hasher.finish();

    let opaque_hash_builder = OpaqueBuildHasher::new(Box::new(default_hash_builder));
    let mut opaque_hasher = opaque_hash_builder.build_hasher();
    value.hash(&mut opaque_hasher);
    let result = hasher.finish();

    (result, expected)
}

fn run_test_opaque_hasher<T, I>(values: I)
where
    T: Hash + 'static,
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
