#![deny(unsafe_op_in_unsafe_fn)]
#![feature(optimize_attribute)]
#![no_std]
extern crate alloc as alloc_crate;

#[cfg(feature = "std")]
extern crate std;

use core::any;
use core::fmt;
use core::marker;
use alloc_crate::boxed::Box;

#[cfg(feature = "std")]
use std::hash;

#[cfg(not(feature = "std"))]
use core::hash;

trait AnyHasher: any::Any + hash::Hasher + Send + Sync {}

impl<H> AnyHasher for H where H: any::Any + hash::Hasher + Send + Sync {}

#[repr(C)]
struct TypedProjHasherInner<H>
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
    fn new(hasher: H) -> Self {
        let boxed_hasher = Box::new(hasher);
        let type_id = any::TypeId::of::<H>();

        Self {
            hasher: boxed_hasher,
            hasher_type_id: type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    fn from_boxed_hasher(hasher: Box<H>) -> Self {
        let type_id = any::TypeId::of::<H>();

        Self {
            hasher,
            hasher_type_id: type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    fn hasher_assuming_type(&self) -> &H {
        debug_assert_eq!(self.hasher_type_id, any::TypeId::of::<H>());

        let any_hasher = self.hasher.as_ref() as &dyn any::Any;
        any_hasher.downcast_ref::<H>().unwrap()
    }

    #[inline]
    fn into_boxed_hasher_assuming_type(self) -> Box<H> {
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
struct OpaqueHasherInner {
    hasher: Box<dyn AnyHasher>,
    hasher_type_id: any::TypeId,
}

impl OpaqueHasherInner {
    #[inline]
    const fn hasher_type_id(&self) -> any::TypeId {
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
            type_check_failed(self.inner.hasher_type_id, any::TypeId::of::<H>());
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

#[repr(C)]
struct TypedProjBuildHasherInner<S>
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
    fn new(build_hasher: S) -> Self {
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
    fn from_boxed_build_hasher(build_hasher: Box<S>) -> Self {
        let build_hasher_type_id = any::TypeId::of::<S>();
        let hasher_type_id = any::TypeId::of::<S::Hasher>();

        Self {
            build_hasher,
            build_hasher_type_id,
            hasher_type_id,
            _marker: marker::PhantomData,
        }
    }

    fn get_build_hasher(&self) -> &S {
        debug_assert_eq!(self.build_hasher_type_id, any::TypeId::of::<S>());

        let any_build_hasher = self.build_hasher.as_ref();
        any_build_hasher.downcast_ref::<S>().unwrap()
    }

    fn into_boxed_build_hasher(self) -> Box<S> {
        debug_assert_eq!(self.build_hasher_type_id, any::TypeId::of::<S>());

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
    fn build_hasher(&self) -> S::Hasher {
        debug_assert_eq!(self.build_hasher_type_id, any::TypeId::of::<S>());

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
        debug_assert_eq!(self.build_hasher_type_id, any::TypeId::of::<S>());

        let build_hasher_ref = self.build_hasher.downcast_ref::<S>().unwrap();
        let cloned_build_hasher = Box::new(build_hasher_ref.clone());

        Self::from_boxed_build_hasher(cloned_build_hasher)
    }
}

#[repr(C)]
struct OpaqueBuildHasherInner {
    build_hasher: Box<dyn any::Any>,
    build_hasher_type_id: any::TypeId,
    hasher_type_id: any::TypeId,
}

impl OpaqueBuildHasherInner {
    #[inline]
    const fn build_hasher_type_id(&self) -> any::TypeId {
        self.build_hasher_type_id
    }

    #[inline]
    const fn hasher_type_id(&self) -> any::TypeId {
        self.hasher_type_id
    }
}

impl OpaqueBuildHasherInner {
    #[inline]
    fn new<S>(build_hasher: S) -> Self
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
    fn from_boxed_build_hasher<S>(build_hasher: Box<S>) -> Self
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
    pub fn as_proj<S>(&self) -> &TypedProjBuildHasherInner<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.build_hasher_type_id, any::TypeId::of::<S>());

        unsafe { &*(self as *const OpaqueBuildHasherInner as *const TypedProjBuildHasherInner<S>) }
    }

    #[inline(always)]
    pub fn as_proj_mut<S>(&mut self) -> &mut TypedProjBuildHasherInner<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.build_hasher_type_id, any::TypeId::of::<S>());

        unsafe { &mut *(self as *mut OpaqueBuildHasherInner as *mut TypedProjBuildHasherInner<S>) }
    }

    #[inline(always)]
    pub fn into_proj<S>(self) -> TypedProjBuildHasherInner<S>
    where
        S: any::Any + hash::BuildHasher + Send + Sync,
        S::Hasher: any::Any + hash::Hasher + Send + Sync,
    {
        debug_assert_eq!(self.build_hasher_type_id, any::TypeId::of::<S>());

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
    pub fn from_proj<S>(proj_self: TypedProjBuildHasherInner<S>) -> Self
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
            type_check_failed(self.inner.build_hasher_type_id, any::TypeId::of::<S>());
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

    pub(super) struct DummyHasher {}

    impl hash::Hasher for DummyHasher {
        #[inline]
        fn finish(&self) -> u64 {
            panic!("The [`DummyHasher::finish`] should never actually be called. Its purpose is to test struct layouts.");
        }

        #[inline]
        fn write(&mut self, _bytes: &[u8]) {
            panic!("The [`DummyHasher::write`] should never actually be called. Its purpose is for testing struct layouts");
        }
    }

    #[allow(dead_code)]
    pub(super) struct DummyBuildHasher {}

    impl hash::BuildHasher for DummyBuildHasher {
        type Hasher = DummyHasher;
        fn build_hasher(&self) -> Self::Hasher {
            panic!("The [`DummyBuildHasher::build_hasher`] should never actually be called. Its purpose is for testing struct layouts");
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

        assert_send_sync::<TypedProjHasher<hash::DefaultHasher>>();
        assert_send_sync::<TypedProjBuildHasher<hash::RandomState>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjHasher<dummy::DummyHasher>>();
        assert_send_sync::<TypedProjBuildHasher<dummy::DummyBuildHasher>>();
    }
}
