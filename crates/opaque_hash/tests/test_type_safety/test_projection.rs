use std::hash;
use opaque_hash::{TypeProjectedBuildHasher, TypeErasedBuildHasher};

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
fn test_opaque_hasher_into_proj_correct_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    let _ = opaque_build_hasher.into_proj::<hash::RandomState>();
}

#[test]
fn test_opaque_hasher_into_proj_correct_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    let _ = opaque_build_hasher.into_proj::<BuildZeroHasher>();
}

#[test]
fn test_opaque_hasher_into_proj_correct_type3() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    for _ in 0..65536 {
        let proj_build_hasher = opaque_build_hasher.into_proj::<hash::RandomState>();
        opaque_build_hasher = TypeErasedBuildHasher::from_proj(proj_build_hasher);
    }
}

#[test]
fn test_opaque_hasher_into_proj_correct_type4() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    for _ in 0..65536 {
        let proj_build_hasher = opaque_build_hasher.into_proj::<BuildZeroHasher>();
        opaque_build_hasher = TypeErasedBuildHasher::from_proj(proj_build_hasher);
    }
}

#[test]
#[should_panic]
fn test_opaque_hasher_into_proj_panics_wrong_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    let _ = opaque_build_hasher.into_proj::<hash::RandomState>();
}

#[test]
#[should_panic]
fn test_opaque_hasher_into_proj_panics_wrong_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    let _ = opaque_build_hasher.into_proj::<BuildZeroHasher>();
}

#[test]
fn test_opaque_hasher_as_proj_correct_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    let _ = opaque_build_hasher.as_proj::<hash::RandomState>();
}

#[test]
fn test_opaque_hasher_as_proj_correct_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    let _ = opaque_build_hasher.as_proj::<BuildZeroHasher>();
}

#[test]
fn test_opaque_hasher_as_proj_correct_type3() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    for _ in 0..65536 {
        let _ = opaque_build_hasher.as_proj::<hash::RandomState>();
    }
}

#[test]
fn test_opaque_hasher_as_proj_correct_type4() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    for _ in 0..65536 {
        let _ = opaque_build_hasher.as_proj::<BuildZeroHasher>();
    }
}

#[test]
#[should_panic]
fn test_opaque_hasher_as_proj_panics_wrong_type1() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    let _ = opaque_build_hasher.as_proj::<BuildZeroHasher>();
}

#[test]
#[should_panic]
fn test_opaque_hasher_as_proj_panics_wrong_type2() {
    let opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    let _ = opaque_build_hasher.as_proj::<hash::RandomState>();
}

#[test]
fn test_opaque_hasher_as_proj_mut_correct_type1() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    let _ = opaque_build_hasher.as_proj_mut::<hash::RandomState>();
}

#[test]
fn test_opaque_hasher_as_proj_mut_correct_type2() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    let _ = opaque_build_hasher.as_proj_mut::<BuildZeroHasher>();
}

#[test]
fn test_opaque_hasher_as_proj_mut_correct_type3() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    for _ in 0..65536 {
        let _ = opaque_build_hasher.as_proj_mut::<hash::RandomState>();
    }
}

#[test]
fn test_opaque_hasher_as_proj_mut_correct_type4() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    for _ in 0..65536 {
        let _ = opaque_build_hasher.as_proj_mut::<BuildZeroHasher>();
    }
}

#[test]
#[should_panic]
fn test_opaque_hasher_as_proj_mut_panics_wrong_type1() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<hash::RandomState>(hash::RandomState::new());
    let _ = opaque_build_hasher.as_proj_mut::<BuildZeroHasher>();
}

#[test]
#[should_panic]
fn test_opaque_hasher_as_proj_mut_panics_wrong_type2() {
    let mut opaque_build_hasher = TypeErasedBuildHasher::new::<BuildZeroHasher>(BuildZeroHasher::new());
    let _ = opaque_build_hasher.as_proj_mut::<hash::RandomState>();
}
