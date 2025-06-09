use crate::hasher_inner::{OpaqueHasherInner, TypedProjHasherInner};

use core::any;
use core::fmt;
use core::marker;
use alloc_crate::boxed::Box;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

#[repr(transparent)]
pub struct TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    inner: TypedProjHasherInner<H>,
}

impl<H> TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    #[inline]
    pub const fn hasher_type_id(&self) -> any::TypeId {
        self.inner.hasher_type_id()
    }
}

impl<H> TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    #[inline]
    pub fn new(hasher: H) -> Self {
        let inner = TypedProjHasherInner::new(hasher);

        Self { inner, }
    }

    #[inline]
    pub fn from_boxed_hasher(hasher: Box<H>) -> Self {
        let inner = TypedProjHasherInner::from_boxed_hasher(hasher);

        Self { inner, }
    }

    pub fn hasher(&self) -> &H {
        self.inner.hasher_assuming_type()
    }

    pub fn into_boxed_hasher(self) -> Box<H> {
        self.inner.into_boxed_hasher_assuming_type()
    }
}

impl<H> hash::Hasher for TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    fn finish(&self) -> u64 {
        self.inner.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.inner.write(bytes)
    }
}

impl<H> Clone for TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<H> Default for TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync + Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<H> fmt::Debug for TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync + fmt::Debug,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TypedProjHasher")
            .field("inner", self.inner.hasher_assuming_type())
            .finish()
    }
}

impl<H> From<H> for TypedProjHasher<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    fn from(hasher: H) -> Self {
        Self::new(hasher)
    }
}

#[repr(transparent)]
pub struct OpaqueHasher {
    inner: OpaqueHasherInner,
}

impl OpaqueHasher {
    #[inline]
    pub const fn hasher_type_id(&self) -> any::TypeId {
        self.inner.hasher_type_id()
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
    fn assert_type_safety<H>(&self)
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_hasher_type::<H>() {
            type_check_failed(self.inner.hasher_type_id(), any::TypeId::of::<H>());
        }
    }
}

impl OpaqueHasher {
    #[inline]
    pub fn new<H>(hasher: H) -> Self
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_alloc = TypedProjHasher::<H>::new(hasher);

        Self::from_proj(proj_alloc)
    }

    #[inline]
    pub fn from_boxed_hasher<H>(hasher: Box<H>) -> Self
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let proj_alloc = TypedProjHasher::<H>::from_boxed_hasher(hasher);

        Self::from_proj(proj_alloc)
    }
}

impl OpaqueHasher {
    #[inline]
    pub fn as_proj<H>(&self) -> &TypedProjHasher<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        self.assert_type_safety::<H>();

        unsafe { &*(self as *const OpaqueHasher as *const TypedProjHasher<H>) }
    }

    #[inline]
    pub fn as_proj_mut<H>(&mut self) -> &mut TypedProjHasher<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        self.assert_type_safety::<H>();

        unsafe { &mut *(self as *mut OpaqueHasher as *mut TypedProjHasher<H>) }
    }

    #[inline]
    pub fn into_proj<H>(self) -> TypedProjHasher<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        self.assert_type_safety::<H>();

        TypedProjHasher {
            inner: self.inner.into_proj(),
        }
    }

    #[inline]
    pub fn from_proj<H>(proj_self: TypedProjHasher<H>) -> Self
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        Self {
            inner: OpaqueHasherInner::from_proj(proj_self.inner),
        }
    }
}

impl hash::Hasher for OpaqueHasher {
    fn finish(&self) -> u64 {
        self.inner.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.inner.write(bytes)
    }
}

impl fmt::Debug for OpaqueHasher {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("OpaqueHasher").finish()
    }
}

mod dummy {
    use super::*;

    #[allow(dead_code)]
    pub(super) struct DummyHasher {
        _do_not_construct: marker::PhantomData<()>,
    }

    impl hash::Hasher for DummyHasher {
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

    impl hash::BuildHasher for DummyBuildHasher {
        type Hasher = DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            panic!("[`DummyBuildHasher::build_hasher`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }
}

#[cfg(test)]
mod hasher_layout_tests {
    use super::*;
    use std::hash;

    fn run_test_opaque_hasher_match_sizes<H>()
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let expected = core::mem::size_of::<TypedProjHasher<H>>();
        let result = core::mem::size_of::<OpaqueHasher>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_hasher_match_alignments<H>()
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let expected = core::mem::align_of::<TypedProjHasher<H>>();
        let result = core::mem::align_of::<OpaqueHasher>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_hasher_match_offsets<H>()
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        assert_eq!(
            core::mem::offset_of!(TypedProjHasher<H>, inner),
            core::mem::offset_of!(OpaqueHasher, inner),
            "Opaque and Typed Projected data types offsets mismatch"
        );
    }

    macro_rules! layout_tests {
        ($module_name:ident, $hasher_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_hasher_layout_match_sizes() {
                    run_test_opaque_hasher_match_sizes::<$hasher_typ>();
                }

                #[test]
                fn test_hasher_layout_match_alignments() {
                    run_test_opaque_hasher_match_alignments::<$hasher_typ>();
                }

                #[test]
                fn test_hasher_layout_match_offsets() {
                    run_test_opaque_hasher_match_offsets::<$hasher_typ>();
                }
            }
        };
    }

    #[cfg(feature = "std")]
    layout_tests!(default_hasher, hash::DefaultHasher);

    layout_tests!(dummy_hasher, dummy::DummyHasher);
}

#[cfg(test)]
mod assert_send_sync {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjHasher<hash::DefaultHasher>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjHasher<dummy::DummyHasher>>();
    }
}
