use crate::hasher::TypedProjHasher;
use crate::build_hasher_inner::{OpaqueBuildHasherInner, TypedProjBuildHasherInner};

use core::any;
use core::fmt;
use core::marker;
use alloc_crate::boxed::Box;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

#[repr(transparent)]
pub struct TypedProjBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    inner: TypedProjBuildHasherInner<S>,
}

impl<S> TypedProjBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    #[inline]
    pub const fn build_hasher_type_id(&self) -> any::TypeId {
        self.inner.build_hasher_type_id()
    }

    #[inline]
    pub const fn hasher_type_id(&self) -> any::TypeId {
        self.inner.hasher_type_id()
    }
}

impl<S> TypedProjBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    #[inline]
    pub fn new(build_hasher: S) -> Self {
        let inner = TypedProjBuildHasherInner::new(build_hasher);

        Self { inner, }
    }

    #[inline]
    pub fn from_boxed_build_hasher(build_hasher: Box<S>) -> Self {
        let inner = TypedProjBuildHasherInner::from_boxed_build_hasher(build_hasher);

        Self { inner, }
    }

    pub fn get_build_hasher(&self) -> &S {
        self.inner.get_build_hasher()
    }

    pub fn into_boxed_build_hasher(self) -> Box<S> {
        self.inner.into_boxed_build_hasher()
    }
}

impl<S> hash::BuildHasher for TypedProjBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    type Hasher = S::Hasher;

    fn build_hasher(&self) -> Self::Hasher {
        self.inner.build_hasher()
    }
}

impl<S> TypedProjBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn build_hasher_proj(&self) -> TypedProjHasher<S::Hasher> {
        let hasher = self.inner.build_hasher();

        TypedProjHasher::new(hasher)
    }
}


impl<S> Clone for TypedProjBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<S> Default for TypedProjBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync + Default,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<S> PartialEq for TypedProjBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync + PartialEq,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn eq(&self, other: &Self) -> bool {
        let inner_self = self.get_build_hasher();
        let inner_other = other.get_build_hasher();

        PartialEq::eq(&inner_self, &inner_other)
    }
}

impl<S> fmt::Debug for TypedProjBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync + fmt::Debug,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedProjBuildHasher")
            .field("inner", self.inner.get_build_hasher())
            .finish()
    }
}

impl<S> From<S> for TypedProjBuildHasher<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn from(hasher: S) -> Self {
        Self::new(hasher)
    }
}

#[repr(transparent)]
pub struct OpaqueBuildHasher {
    inner: OpaqueBuildHasherInner,
}

impl OpaqueBuildHasher {
    #[inline]
    pub const fn build_hasher_type_id(&self) -> any::TypeId {
        self.inner.build_hasher_type_id()
    }

    #[inline]
    pub const fn hasher_type_id(&self) -> any::TypeId {
        self.inner.hasher_type_id()
    }
}

impl OpaqueBuildHasher {
    #[inline]
    pub fn has_build_hasher_type<S>(&self) -> bool
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
    {
        self.inner.build_hasher_type_id() == any::TypeId::of::<S>()
    }

    #[inline]
    pub fn has_hasher_type<H>(&self) -> bool
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        self.inner.hasher_type_id() == any::TypeId::of::<H>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<S>(&self)
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_build_hasher_type::<S>() {
            type_check_failed(self.inner.build_hasher_type_id(), any::TypeId::of::<S>());
        }
    }
}

impl OpaqueBuildHasher {
    #[inline]
    pub fn new<S>(build_hasher: S) -> Self
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_build_hasher = TypedProjBuildHasher::<S>::new(build_hasher);

        Self::from_proj(proj_build_hasher)
    }

    #[inline]
    pub fn from_boxed_build_hasher<S>(build_hasher: Box<S>) -> Self
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_build_hasher = TypedProjBuildHasher::<S>::from_boxed_build_hasher(build_hasher);

        Self::from_proj(proj_build_hasher)
    }
}

impl OpaqueBuildHasher {
    #[inline]
    pub fn as_proj<S>(&self) -> &TypedProjBuildHasher<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        self.assert_type_safety::<S>();

        unsafe { &*(self as *const OpaqueBuildHasher as *const TypedProjBuildHasher<S>) }
    }

    #[inline]
    pub fn as_proj_mut<S>(&mut self) -> &mut TypedProjBuildHasher<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        self.assert_type_safety::<S>();

        unsafe { &mut *(self as *mut OpaqueBuildHasher as *mut TypedProjBuildHasher<S>) }
    }

    #[inline]
    pub fn into_proj<S>(self) -> TypedProjBuildHasher<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        self.assert_type_safety::<S>();

        TypedProjBuildHasher {
            inner: self.inner.into_proj(),
        }
    }

    #[inline]
    pub fn from_proj<S>(proj_self: TypedProjBuildHasher<S>) -> Self
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        Self {
            inner: OpaqueBuildHasherInner::from_proj(proj_self.inner),
        }
    }
}

impl OpaqueBuildHasher {
    pub fn build_hasher<S>(&self) -> S::Hasher
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<S>();

        <TypedProjBuildHasher<S> as hash::BuildHasher>::build_hasher(proj_self)
    }
}

impl OpaqueBuildHasher {
    pub fn build_hasher_proj<S>(&self) -> TypedProjHasher<S::Hasher>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_self = self.as_proj::<S>();

        proj_self.build_hasher_proj()
    }
}

impl fmt::Debug for OpaqueBuildHasher {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("OpaqueBuildHasher").finish()
    }
}

mod dummy {
    use super::*;

    #[allow(dead_code)]
    pub(super) struct DummyHasher {
        _do_not_construct: marker::PhantomData<()>,
    }

    impl hash::Hasher for dummy::DummyHasher {
        #[inline]
        fn finish(&self) -> u64 {
            panic!("[`DummyHasher::finish`] should never actually be called. Its purpose is to test struct layouts.");
        }

        #[inline]
        fn write(&mut self, _bytes: &[u8]) {
            panic!("[`DummyHasher::write`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }

    #[allow(dead_code)]
    pub(super) struct DummyBuildHasher {
        _do_not_construct: marker::PhantomData<()>,
    }

    impl hash::BuildHasher for dummy::DummyBuildHasher {
        type Hasher = dummy::DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            panic!("[`DummyBuildHasher::build_hasher`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }
}

#[cfg(test)]
mod build_hasher_layout_tests {
    use super::*;
    use core::mem;
    use std::hash;

    fn run_test_opaque_build_hasher_match_sizes<S>()
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let expected = mem::size_of::<TypedProjBuildHasher<S>>();
        let result = mem::size_of::<OpaqueBuildHasher>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_build_hasher_match_alignments<S>()
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let expected = mem::align_of::<TypedProjBuildHasher<S>>();
        let result = mem::align_of::<OpaqueBuildHasher>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_build_hasher_match_offsets<S>()
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        assert_eq!(
            mem::offset_of!(TypedProjBuildHasher<S>, inner),
            mem::offset_of!(OpaqueBuildHasher, inner),
            "Opaque and Typed Projected data types offsets mismatch"
        );
    }

    macro_rules! layout_tests {
        ($module_name:ident, $build_hasher_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_build_hasher_layout_match_sizes() {
                    run_test_opaque_build_hasher_match_sizes::<$build_hasher_typ>();
                }

                #[test]
                fn test_build_hasher_layout_match_alignments() {
                    run_test_opaque_build_hasher_match_alignments::<$build_hasher_typ>();
                }

                #[test]
                fn test_build_hasher_layout_match_offsets() {
                    run_test_opaque_build_hasher_match_offsets::<$build_hasher_typ>();
                }
            }
        };
    }

    layout_tests!(random_state, hash::RandomState);
    layout_tests!(dummy_build_hasher, dummy::DummyBuildHasher);
}

#[cfg(test)]
mod assert_send_sync {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjBuildHasher<hash::RandomState>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjBuildHasher<dummy::DummyBuildHasher>>();
    }
}
