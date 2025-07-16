use opaque_hash::{
    TypeErasedBuildHasher,
    TypeErasedHasher,
};

use std::hash;

struct ZeroHasher {}

impl hash::Hasher for ZeroHasher {
    fn finish(&self) -> u64 {
        0
    }

    fn write(&mut self, _bytes: &[u8]) {}
}

struct BuildZeroHasher {}

impl hash::BuildHasher for BuildZeroHasher {
    type Hasher = ZeroHasher;

    fn build_hasher(&self) -> Self::Hasher {
        ZeroHasher {}
    }
}

impl BuildZeroHasher {
    fn new() -> Self {
        Self {}
    }
}

#[test]
fn test_type_erased_hasher_has_hasher_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new(hash::RandomState::new());
    let opaque_hasher = TypeErasedHasher::from_proj(opaque_build_hasher.build_hasher_proj::<hash::RandomState>());

    assert!(opaque_hasher.has_hasher_type::<hash::DefaultHasher>());
}

#[test]
fn test_type_erased_hasher_has_hasher_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new(BuildZeroHasher::new());
    let opaque_hasher = TypeErasedHasher::from_proj(opaque_build_hasher.build_hasher_proj::<BuildZeroHasher>());

    assert!(opaque_hasher.has_hasher_type::<ZeroHasher>());
}

#[test]
fn test_type_erased_hasher_has_hasher_type3() {
    let opaque_build_hasher = TypeErasedBuildHasher::new(hash::RandomState::new());
    let opaque_hasher = TypeErasedHasher::from_proj(opaque_build_hasher.build_hasher_proj::<hash::RandomState>());

    assert!(opaque_hasher.has_hasher_type::<hash::DefaultHasher>());
}

#[test]
fn test_type_erased_hasher_has_hasher_type4() {
    let opaque_build_hasher = TypeErasedBuildHasher::new(BuildZeroHasher::new());
    let opaque_hasher = TypeErasedHasher::from_proj(opaque_build_hasher.build_hasher_proj::<BuildZeroHasher>());

    assert!(opaque_hasher.has_hasher_type::<ZeroHasher>());
}

#[test]
fn test_type_erased_hasher_not_has_hasher_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new(hash::RandomState::new());
    let opaque_hasher = TypeErasedHasher::from_proj(opaque_build_hasher.build_hasher_proj::<hash::RandomState>());

    assert!(!opaque_hasher.has_hasher_type::<ZeroHasher>());
}

#[test]
fn test_type_erased_hasher_not_has_hasher_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new(BuildZeroHasher::new());
    let opaque_hasher = TypeErasedHasher::from_proj(opaque_build_hasher.build_hasher_proj::<BuildZeroHasher>());

    assert!(!opaque_hasher.has_hasher_type::<hash::DefaultHasher>());
}

#[test]
fn test_type_erased_build_hasher_has_hasher_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new(hash::RandomState::new());

    assert!(opaque_build_hasher.has_hasher_type::<hash::DefaultHasher>());
}

#[test]
fn test_type_erased_build_hasher_has_hasher_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new(BuildZeroHasher::new());

    assert!(opaque_build_hasher.has_hasher_type::<ZeroHasher>());
}

#[test]
fn test_type_erased_build_hasher_has_build_hasher_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new(hash::RandomState::new());

    assert!(opaque_build_hasher.has_hasher_type::<hash::DefaultHasher>());
}

#[test]
fn test_type_erased_build_hasher_has_build_hasher_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new(BuildZeroHasher::new());

    assert!(opaque_build_hasher.has_hasher_type::<ZeroHasher>());
}

#[test]
fn test_type_erased_build_hasher_not_has_build_hasher_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new(hash::RandomState::new());

    assert!(!opaque_build_hasher.has_hasher_type::<ZeroHasher>());
}

#[test]
fn test_type_erased_build_hasher_not_has_build_hasher_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new(BuildZeroHasher::new());

    assert!(!opaque_build_hasher.has_hasher_type::<hash::DefaultHasher>());
}
