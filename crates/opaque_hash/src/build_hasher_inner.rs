use core::any;
use core::marker;
use alloc_crate::boxed::Box;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

#[repr(C)]
pub(crate) struct TypedProjBuildHasherInner<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    build_hasher: Box<dyn any::Any + Send + Sync>,
    build_hasher_type_id: any::TypeId,
    hasher_type_id: any::TypeId,
    _marker: marker::PhantomData<S>,
}

impl<S> TypedProjBuildHasherInner<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    #[inline]
    pub(crate) const fn build_hasher_type_id(&self) -> any::TypeId {
        self.build_hasher_type_id
    }

    #[inline]
    pub(crate) const fn hasher_type_id(&self) -> any::TypeId {
        self.hasher_type_id
    }
}

impl<S> TypedProjBuildHasherInner<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    #[inline]
    pub(crate) fn new(build_hasher: S) -> Self {
        let boxed_build_hasher = Box::new(build_hasher);
        let build_hasher_type_id = any::TypeId::of::<S>();
        let hasher_type_id = any::TypeId::of::<S::Hasher>();

        Self {
            build_hasher: boxed_build_hasher,
            build_hasher_type_id,
            hasher_type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    pub(crate) fn from_boxed_build_hasher(build_hasher: Box<S>) -> Self {
        let build_hasher_type_id = any::TypeId::of::<S>();
        let hasher_type_id = any::TypeId::of::<S::Hasher>();

        Self {
            build_hasher,
            build_hasher_type_id,
            hasher_type_id,
            _marker: marker::PhantomData,
        }
    }

    pub(crate) fn get_build_hasher(&self) -> &S {
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<S::Hasher>());

        let any_build_hasher = self.build_hasher.as_ref();
        any_build_hasher.downcast_ref::<S>().unwrap()
    }

    pub(crate) fn into_boxed_build_hasher(self) -> Box<S> {
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<S::Hasher>());

        let boxed_build_hasher = unsafe {
            let unboxed_build_hasher = Box::into_raw(self.build_hasher);
            Box::from_raw(unboxed_build_hasher as *mut S)
        };

        boxed_build_hasher
    }
}

impl<S> TypedProjBuildHasherInner<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    pub(crate) fn build_hasher(&self) -> S::Hasher {
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<S::Hasher>());

        let build_hasher = self.build_hasher.downcast_ref::<S>().unwrap();

        build_hasher.build_hasher()
    }
}

impl<S> Clone for TypedProjBuildHasherInner<S>
where
    S: any::Any + hash::BuildHasher + Send + Sync + Clone,
    S::Hasher: any::Any + hash::Hasher + Send + Sync,
{
    fn clone(&self) -> Self {
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<S::Hasher>());

        let build_hasher_ref = self.build_hasher.downcast_ref::<S>().unwrap();
        let cloned_build_hasher = Box::new(build_hasher_ref.clone());

        Self::from_boxed_build_hasher(cloned_build_hasher)
    }
}

#[repr(C)]
pub(crate) struct OpaqueBuildHasherInner {
    build_hasher: Box<dyn any::Any>,
    build_hasher_type_id: any::TypeId,
    hasher_type_id: any::TypeId,
}

impl OpaqueBuildHasherInner {
    #[inline]
    pub(crate) const fn build_hasher_type_id(&self) -> any::TypeId {
        self.build_hasher_type_id
    }

    #[inline]
    pub(crate) const fn hasher_type_id(&self) -> any::TypeId {
        self.hasher_type_id
    }
}

impl OpaqueBuildHasherInner {
    #[inline]
    pub(crate) fn new<S>(build_hasher: S) -> Self
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
    {
        let boxed_build_hasher = Box::new(build_hasher);
        let build_hasher_type_id = any::TypeId::of::<S>();
        let hasher_type_id = any::TypeId::of::<S::Hasher>();

        Self {
            build_hasher: boxed_build_hasher,
            build_hasher_type_id,
            hasher_type_id,
        }
    }

    #[inline]
    pub(crate) fn from_boxed_build_hasher<S>(build_hasher: Box<S>) -> Self
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
    {
        let build_hasher_type_id = any::TypeId::of::<S>();
        let hasher_type_id = any::TypeId::of::<S::Hasher>();

        Self {
            build_hasher,
            build_hasher_type_id,
            hasher_type_id,
        }
    }
}

impl OpaqueBuildHasherInner {
    #[inline(always)]
    pub(crate) fn as_proj<S>(&self) -> &TypedProjBuildHasherInner<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<S::Hasher>());

        unsafe { &*(self as *const OpaqueBuildHasherInner as *const TypedProjBuildHasherInner<S>) }
    }

    #[inline(always)]
    pub(crate) fn as_proj_mut<S>(&mut self) -> &mut TypedProjBuildHasherInner<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<S::Hasher>());

        unsafe { &mut *(self as *mut OpaqueBuildHasherInner as *mut TypedProjBuildHasherInner<S>) }
    }

    #[inline(always)]
    pub(crate) fn into_proj<S>(self) -> TypedProjBuildHasherInner<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.build_hasher_type_id(), any::TypeId::of::<S>());
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<S::Hasher>());

        let boxed_build_hasher = unsafe {
            let unboxed_build_hasher = Box::into_raw(self.build_hasher);
            Box::from_raw(unboxed_build_hasher as *mut S)
        };

        TypedProjBuildHasherInner {
            build_hasher: boxed_build_hasher,
            build_hasher_type_id: self.build_hasher_type_id,
            hasher_type_id: self.hasher_type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline(always)]
    pub(crate) fn from_proj<S>(proj_self: TypedProjBuildHasherInner<S>) -> Self
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        Self {
            build_hasher: proj_self.build_hasher,
            build_hasher_type_id: proj_self.build_hasher_type_id,
            hasher_type_id: proj_self.hasher_type_id,
        }
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
mod build_hasher_inner_layout_tests {
    use super::*;
    use std::hash;

    fn run_test_opaque_hasher_match_sizes<S>()
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let expected = core::mem::size_of::<TypedProjBuildHasherInner<S>>();
        let result = core::mem::size_of::<OpaqueBuildHasherInner>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_hasher_match_alignments<S>()
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        let expected = core::mem::align_of::<TypedProjBuildHasherInner<S>>();
        let result = core::mem::align_of::<OpaqueBuildHasherInner>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_hasher_match_offsets<S>()
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        assert_eq!(
            core::mem::offset_of!(TypedProjBuildHasherInner<S>, build_hasher),
            core::mem::offset_of!(OpaqueBuildHasherInner, build_hasher),
            "Opaque and Typed Projected data types offsets mismatch"
        );
        assert_eq!(
            core::mem::offset_of!(TypedProjBuildHasherInner<S>, build_hasher_type_id),
            core::mem::offset_of!(OpaqueBuildHasherInner, build_hasher_type_id),
            "Opaque and Typed Projected data types offsets mismatch"
        );
        assert_eq!(
            core::mem::offset_of!(TypedProjBuildHasherInner<S>, hasher_type_id),
            core::mem::offset_of!(OpaqueBuildHasherInner, hasher_type_id),
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
    layout_tests!(default_hasher, hash::RandomState);

    layout_tests!(dummy_hasher, dummy::DummyBuildHasher);
}

#[cfg(test)]
mod assert_send_sync {
    use super::*;

    #[cfg(feature = "std")]
    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjBuildHasherInner<hash::RandomState>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjBuildHasherInner<dummy::DummyBuildHasher>>();
    }
}

/*
#[cfg(test)]
mod assert_not_send_not_sync {
    use super::*;

    #[test]
    fn test_assert_not_send_not_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<OpaqueBuildHasherInner>();
    }
}
*/
