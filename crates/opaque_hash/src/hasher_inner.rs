use core::any;
use core::marker;
use alloc_crate::boxed::Box;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

/// This trait exists to define the [`TypedProjHasherInner`] data type. It is not meant for public use.
trait AnyHasher: any::Any + hash::Hasher + Send + Sync {}

impl<H> AnyHasher for H where H: any::Any + hash::Hasher + Send + Sync {}

#[repr(C)]
pub(crate) struct TypedProjHasherInner<H>
where
    H: any::Any + hash::Hasher + Send + Sync,
{
    hasher: Box<dyn AnyHasher>,
    hasher_type_id: any::TypeId,
    _marker: marker::PhantomData<H>,
}

impl<H> TypedProjHasherInner<H>
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
        debug_assert_eq!(self.hasher_type_id, any::TypeId::of::<H>());

        let any_hasher = self.hasher.as_ref() as &dyn any::Any;
        any_hasher.downcast_ref::<H>().unwrap()
    }

    #[inline]
    pub(crate) fn into_boxed_hasher_assuming_type(self) -> Box<H> {
        debug_assert_eq!(self.hasher_type_id, any::TypeId::of::<H>());

        let boxed_hasher = unsafe {
            let unboxed_hasher = Box::into_raw(self.hasher);
            Box::from_raw(unboxed_hasher as *mut H)
        };

        boxed_hasher
    }
}

impl<H> Clone for TypedProjHasherInner<H>
where
    H: any::Any + hash::Hasher + Send + Sync + Clone,
{
    #[inline]
    fn clone(&self) -> TypedProjHasherInner<H> {
        debug_assert_eq!(self.hasher_type_id, any::TypeId::of::<H>());

        let any_hasher = self.hasher.as_ref() as &dyn any::Any;
        let alloc_ref = any_hasher
            .downcast_ref::<H>()
            .unwrap();
        let cloned_alloc = alloc_ref.clone();

        TypedProjHasherInner::new(cloned_alloc)
    }
}

impl<H> hash::Hasher for TypedProjHasherInner<H>
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
pub(crate) struct OpaqueHasherInner {
    hasher: Box<dyn AnyHasher>,
    hasher_type_id: any::TypeId,
}

impl OpaqueHasherInner {
    #[inline]
    pub(crate) const fn hasher_type_id(&self) -> any::TypeId {
        self.hasher_type_id
    }
}

impl OpaqueHasherInner {
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

impl OpaqueHasherInner {
    #[inline(always)]
    pub(crate) fn as_proj<H>(&self) -> &TypedProjHasherInner<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.hasher_type_id, any::TypeId::of::<H>());

        unsafe { &*(self as *const OpaqueHasherInner as *const TypedProjHasherInner<H>) }
    }

    #[inline(always)]
    pub(crate) fn as_proj_mut<H>(&mut self) -> &mut TypedProjHasherInner<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.hasher_type_id, any::TypeId::of::<H>());

        unsafe { &mut *(self as *mut OpaqueHasherInner as *mut TypedProjHasherInner<H>) }
    }

    #[inline(always)]
    pub(crate) fn into_proj<H>(self) -> TypedProjHasherInner<H>
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.hasher_type_id, any::TypeId::of::<H>());

        TypedProjHasherInner {
            hasher: self.hasher,
            hasher_type_id: self.hasher_type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline(always)]
    pub(crate) fn from_proj<H>(proj_self: TypedProjHasherInner<H>) -> Self
    where
        H: any::Any + hash::Hasher + Send + Sync,
    {
        Self {
            hasher: proj_self.hasher,
            hasher_type_id: proj_self.hasher_type_id,
        }
    }
}

impl hash::Hasher for OpaqueHasherInner {
    fn finish(&self) -> u64 {
        self.hasher.finish()
    }

    fn write(&mut self, bytes: &[u8]) {
        self.hasher.write(bytes)
    }
}
