use opaque_hash::TypeErasedBuildHasher;

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
fn test_type_erased_hasher_try_into_proj_correct_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    let result = opaque_build_hasher.try_into_proj::<hash::RandomState>();

    assert!(result.is_ok());
}

#[test]
fn test_type_erased_hasher_try_into_proj_correct_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    let result = opaque_build_hasher.try_into_proj::<BuildZeroHasher>();

    assert!(result.is_ok());
}

#[test]
fn test_type_erased_hasher_try_into_proj_correct_type3() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    for _ in 0..65536 {
        let proj_build_hasher = opaque_build_hasher.try_into_proj::<hash::RandomState>();

        assert!(proj_build_hasher.is_ok());

        opaque_build_hasher = TypeErasedBuildHasher::from_proj(proj_build_hasher.unwrap());
    }
}

#[test]
fn test_type_erased_hasher_try_into_proj_correct_type4() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    for _ in 0..65536 {
        let proj_build_hasher = opaque_build_hasher.try_into_proj::<BuildZeroHasher>();

        assert!(proj_build_hasher.is_ok());

        opaque_build_hasher = TypeErasedBuildHasher::from_proj(proj_build_hasher.unwrap());
    }
}

#[test]
fn test_type_erased_hasher_try_into_proj_panics_wrong_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    let result = opaque_build_hasher.try_into_proj::<hash::RandomState>();

    assert!(result.is_err());
}

#[test]
fn test_type_erased_hasher_try_into_proj_panics_wrong_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    let result = opaque_build_hasher.try_into_proj::<BuildZeroHasher>();

    assert!(result.is_err());
}

#[test]
fn test_type_erased_hasher_try_as_proj_correct_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    let result = opaque_build_hasher.try_as_proj::<hash::RandomState>();

    assert!(result.is_ok());
}

#[test]
fn test_type_erased_hasher_try_as_proj_correct_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    let result = opaque_build_hasher.try_as_proj::<BuildZeroHasher>();

    assert!(result.is_ok());
}

#[test]
fn test_type_erased_hasher_try_as_proj_correct_type3() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    for _ in 0..65536 {
        let result = opaque_build_hasher.try_as_proj::<hash::RandomState>();

        assert!(result.is_ok());
    }
}

#[test]
fn test_type_erased_hasher_try_as_proj_correct_type4() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    for _ in 0..65536 {
        let result = opaque_build_hasher.try_as_proj::<BuildZeroHasher>();

        assert!(result.is_ok());
    }
}

#[test]
fn test_type_erased_hasher_try_as_proj_panics_wrong_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    let result = opaque_build_hasher.try_as_proj::<BuildZeroHasher>();

    assert!(result.is_err());
}

#[test]
fn test_type_erased_hasher_try_as_proj_panics_wrong_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    let result = opaque_build_hasher.try_as_proj::<hash::RandomState>();

    assert!(result.is_err());
}

#[test]
fn test_type_erased_hasher_try_as_proj_mut_correct_type1() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    let result = opaque_build_hasher.try_as_proj_mut::<hash::RandomState>();

    assert!(result.is_ok());
}

#[test]
fn test_type_erased_hasher_try_as_proj_mut_correct_type2() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    let result = opaque_build_hasher.try_as_proj_mut::<BuildZeroHasher>();

    assert!(result.is_ok());
}

#[test]
fn test_type_erased_hasher_try_as_proj_mut_correct_type3() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    for _ in 0..65536 {
        let result = opaque_build_hasher.try_as_proj_mut::<hash::RandomState>();

        assert!(result.is_ok());
    }
}

#[test]
fn test_type_erased_hasher_try_as_proj_mut_correct_type4() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    for _ in 0..65536 {
        let result = opaque_build_hasher.try_as_proj_mut::<BuildZeroHasher>();

        assert!(result.is_ok());
    }
}

#[test]
fn test_type_erased_hasher_try_as_proj_mut_panics_wrong_type1() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    let result = opaque_build_hasher.try_as_proj_mut::<BuildZeroHasher>();

    assert!(result.is_err());
}

#[test]
fn test_type_erased_hasher_try_as_proj_mut_panics_wrong_type2() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    let result = opaque_build_hasher.try_as_proj_mut::<hash::RandomState>();

    assert!(result.is_err());
}
