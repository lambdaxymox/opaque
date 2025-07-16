use core::any;
use core::marker;
use alloc_crate::boxed::Box;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

/// This trait exists to define the [`TypeProjectedHasherInner`] data type. It is not meant for public use.
trait AnyHasher: any::Any + hash::Hasher + Send + Sync {}

impl<H> AnyHasher for H where H: any::Any + hash::Hasher + Send + Sync {}

#[repr(C)]
pub(crate) struct TypeProjectedHasherInner<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    hasher: Box<dyn AnyHasher>,
    hasher_type_id: any::TypeId,
    _marker: marker::PhantomData<H>,
}

impl<H> TypeProjectedHasherInner<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    #[inline]
    pub(crate) const fn hasher_type_id(&self) -> any::TypeId {
        self.hasher_type_id
    }
}

impl<H> TypeProjectedHasherInner<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    #[inline]
    pub(crate) fn new(hasher: H) -> Self {
        let boxed_hasher = Box::new(hasher);
        let type_id = any::TypeId::of::<H>();

        Self {
            hasher: boxed_hasher,
            hasher_type_id: type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    pub(crate) fn from_boxed_hasher(hasher: Box<H>) -> Self {
        let type_id = any::TypeId::of::<H>();

        Self {
            hasher,
            hasher_type_id: type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    pub(crate) fn hasher_assuming_type(&self) -> &H {
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<H>());

        let any_hasher = self.hasher.as_ref() as &dyn any::Any;
        any_hasher.downcast_ref::<H>().unwrap()
    }

    #[inline]
    pub(crate) fn into_boxed_hasher_assuming_type(self) -> Box<H> {
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<H>());

        let boxed_hasher = unsafe {
            let unboxed_hasher = Box::into_raw(self.hasher);
            Box::from_raw(unboxed_hasher as *mut H)
        };

        boxed_hasher
    }
}

impl<H> Clone for TypeProjectedHasherInner<H>
where
    H: any::Any + hash::Hasher + Send + Sync + Clone,
{
    #[inline]
    fn clone(&self) -> TypeProjectedHasherInner<H> {
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<H>());

        let any_hasher = self.hasher.as_ref() as &dyn any::Any;
        let alloc_ref = any_hasher
            .downcast_ref::<H>()
            .unwrap();
        let cloned_alloc = alloc_ref.clone();

        TypeProjectedHasherInner::new(cloned_alloc)
    }
}

impl<H> hash::Hasher for TypeProjectedHasherInner<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    fn finish(&self) -> u64 {
        self.hasher.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes)
    }
}

#[repr(C)]
pub(crate) struct TypeErasedHasherInner {
    hasher: Box<dyn AnyHasher>,
    hasher_type_id: any::TypeId,
}

impl TypeErasedHasherInner {
    #[inline]
    pub(crate) const fn hasher_type_id(&self) -> any::TypeId {
        self.hasher_type_id
    }
}

impl TypeErasedHasherInner {
    #[inline]
    fn new<H>(hasher: H) -> Self
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let boxed_hasher = Box::new(hasher);
        let type_id = any::TypeId::of::<H>();

        Self {
            hasher: boxed_hasher,
            hasher_type_id: type_id
        }
    }

    #[inline]
    fn from_boxed_hasher<H>(hasher: Box<H>) -> Self
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let type_id = any::TypeId::of::<H>();

        Self {
            hasher,
            hasher_type_id: type_id,
        }
    }
}

impl TypeErasedHasherInner {
    #[inline(always)]
    pub(crate) fn as_proj<H>(&self) -> &TypeProjectedHasherInner<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<H>());

        unsafe { &*(self as *const TypeErasedHasherInner as *const TypeProjectedHasherInner<H>) }
    }

    #[inline(always)]
    pub(crate) fn as_proj_mut<H>(&mut self) -> &mut TypeProjectedHasherInner<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<H>());

        unsafe { &mut *(self as *mut TypeErasedHasherInner as *mut TypeProjectedHasherInner<H>) }
    }

    #[inline(always)]
    pub(crate) fn into_proj<H>(self) -> TypeProjectedHasherInner<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.hasher_type_id(), any::TypeId::of::<H>());

        TypeProjectedHasherInner {
            hasher: self.hasher,
            hasher_type_id: self.hasher_type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline(always)]
    pub(crate) fn from_proj<H>(proj_self: TypeProjectedHasherInner<H>) -> Self
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        Self {
            hasher: proj_self.hasher,
            hasher_type_id: proj_self.hasher_type_id,
        }
    }
}

impl hash::Hasher for TypeErasedHasherInner {
    fn finish(&self) -> u64 {
        self.hasher.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes)
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
mod hasher_inner_layout_tests {
    use super::*;
    use std::hash;

    fn run_test_type_erased_hasher_match_sizes<H>()
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let expected = core::mem::size_of::<TypeProjectedHasherInner<H>>();
        let result = core::mem::size_of::<TypeErasedHasherInner>();

        assert_eq!(result, expected, "Type Erased and Type Projected data types size mismatch");
    }

    fn run_test_type_erased_hasher_match_alignments<H>()
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        let expected = core::mem::align_of::<TypeProjectedHasherInner<H>>();
        let result = core::mem::align_of::<TypeErasedHasherInner>();

        assert_eq!(result, expected, "Type Erased and Type Projected data types alignment mismatch");
    }

    fn run_test_type_erased_hasher_match_offsets<H>()
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        assert_eq!(
            core::mem::offset_of!(TypeProjectedHasherInner<H>, hasher),
            core::mem::offset_of!(TypeErasedHasherInner, hasher),
            "Type Erased and Type Projected data types offsets mismatch"
        );
        assert_eq!(
            core::mem::offset_of!(TypeProjectedHasherInner<H>, hasher_type_id),
            core::mem::offset_of!(TypeErasedHasherInner, hasher_type_id),
            "Type Erased and Type Projected data types offsets mismatch"
        );
    }

    macro_rules! layout_tests {
        ($module_name:ident, $hasher_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_hasher_layout_match_sizes() {
                    run_test_type_erased_hasher_match_sizes::<$hasher_typ>();
                }

                #[test]
                fn test_hasher_layout_match_alignments() {
                    run_test_type_erased_hasher_match_alignments::<$hasher_typ>();
                }

                #[test]
                fn test_hasher_layout_match_offsets() {
                    run_test_type_erased_hasher_match_offsets::<$hasher_typ>();
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

        assert_send_sync::<TypeProjectedHasherInner<hash::DefaultHasher>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeProjectedHasherInner<dummy::DummyHasher>>();
    }
}

/*
#[cfg(test)]
mod assert_not_send_not_sync {
    use super::*;

    #[test]
    fn test_assert_not_send_not_sync() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypeErasedHasherInner>();
    }
}
*/
