#![feature(optimize_attribute)]
use core::{any, fmt};
use core::any::TypeId;
use core::hash;
use core::marker::PhantomData;

trait AnyHasher: hash::Hasher + any::Any {}

impl<H> AnyHasher for H where H: hash::Hasher + any::Any {}

pub struct OpaqueHasherInner {
    hasher: Box<dyn AnyHasher>,
    type_id: TypeId,
}

impl OpaqueHasherInner {
    #[inline]
    fn new<H>(hasher: H) -> Self
    where
        H: hash::Hasher + any::Any,
    {
        let boxed_hasher = Box::new(hasher);
        let type_id = TypeId::of::<H>();

        Self {
            hasher: boxed_hasher,
            type_id
        }
    }

    #[inline]
    fn from_boxed_hasher<H>(hasher: Box<H>) -> Self
    where
        H: hash::Hasher + any::Any,
    {
        let type_id = TypeId::of::<H>();

        Self {
            hasher,
            type_id,
        }
    }

    #[inline]
    fn is_type<H>(&self) -> bool
    where
        H: hash::Hasher + any::Any,
    {
        self.type_id == TypeId::of::<H>()
    }

    #[inline]
    fn hasher_assuming_type<H>(&self) -> &H
    where
        H: hash::Hasher + any::Any,
    {
        let any_hasher = self.hasher.as_ref() as &dyn any::Any;
        any_hasher.downcast_ref::<H>().unwrap()
    }

    fn into_box_hasher_assuming_type<H>(self) -> Box<H>
    where
        H: hash::Hasher + any::Any,
    {
        let boxed_hasher = unsafe {
            let unboxed_hasher = Box::into_raw(self.hasher);
            Box::from_raw(unboxed_hasher as *mut H)
        };

        boxed_hasher
    }

    fn clone_assuming_type<H>(&self) -> OpaqueHasherInner
    where
        H: hash::Hasher + any::Any + Clone,
    {
        let any_hasher = self.hasher.as_ref() as &dyn any::Any;
        let alloc_ref = any_hasher
            .downcast_ref::<H>()
            .unwrap();
        let cloned_alloc = alloc_ref.clone();

        OpaqueHasherInner::new(cloned_alloc)
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
pub struct TypedProjHasher<H> {
    inner: OpaqueHasherInner,
    _marker: core::marker::PhantomData<H>,
}

impl<H> TypedProjHasher<H>
where
    H: hash::Hasher + any::Any,
{
    #[inline]
    pub fn new(hasher: H) -> Self {
        let inner = OpaqueHasherInner::new::<H>(hasher);

        Self { inner, _marker: PhantomData }
    }

    #[inline]
    pub fn from_boxed_hasher(hasher: Box<H>) -> Self {
        let inner = OpaqueHasherInner::from_boxed_hasher(hasher);

        Self { inner, _marker: PhantomData }
    }

    pub fn hasher(&self) -> &H {
        self.inner.hasher_assuming_type::<H>()
    }

    pub fn into_box_hasher(self) -> Box<H> {
        self.inner.into_box_hasher_assuming_type::<H>()
    }
}

impl<H> hash::Hasher for TypedProjHasher<H>
where
    H: hash::Hasher + any::Any,
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
    H: hash::Hasher + any::Any + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_assuming_type::<H>(),
            _marker: self._marker,
        }
    }
}

impl<H> Default for TypedProjHasher<H>
where
    H: hash::Hasher + any::Any + Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<H> fmt::Debug for TypedProjHasher<H>
where
    H: hash::Hasher + any::Any + fmt::Debug,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TypedProjHasher")
            .field("inner", self.inner.hasher_assuming_type::<H>())
            .finish()
    }
}

impl<H> From<H> for TypedProjHasher<H>
where
    H: hash::Hasher + any::Any,
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
    pub fn new<H>(hasher: H) -> Self
    where
        H: hash::Hasher + any::Any,
    {
        let proj_alloc = TypedProjHasher::<H>::new(hasher);

        Self::from_proj(proj_alloc)
    }

    #[inline]
    pub fn from_boxed_hasher<H>(hasher: Box<H>) -> Self
    where
        H: hash::Hasher + any::Any,
    {
        let proj_alloc = TypedProjHasher::<H>::from_boxed_hasher(hasher);

        Self::from_proj(proj_alloc)
    }

    #[inline]
    pub fn is_type<H>(&self) -> bool
    where
        H: hash::Hasher + any::Any,
    {
        self.inner.is_type::<H>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<H>(&self)
    where
        H: hash::Hasher + any::Any,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(type_id_self: TypeId, type_id_other: TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.is_type::<H>() {
            type_check_failed(self.inner.type_id, TypeId::of::<H>());
        }
    }

    pub fn as_proj<H>(&self) -> &TypedProjHasher<H>
    where
        H: hash::Hasher + any::Any,
    {
        self.assert_type_safety::<H>();

        unsafe { &*(self as *const OpaqueHasher as *const TypedProjHasher<H>) }
    }

    pub fn as_proj_mut<H>(&mut self) -> &mut TypedProjHasher<H>
    where
        H: hash::Hasher + any::Any,
    {
        self.assert_type_safety::<H>();

        unsafe { &mut *(self as *mut OpaqueHasher as *mut TypedProjHasher<H>) }
    }

    pub fn into_proj<H>(self) -> TypedProjHasher<H>
    where
        H: hash::Hasher + any::Any,
    {
        self.assert_type_safety::<H>();

        TypedProjHasher {
            inner: self.inner,
            _marker: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn from_proj<H>(proj_self: TypedProjHasher<H>) -> Self
    where
        H: hash::Hasher + any::Any,
    {
        Self {
            inner: proj_self.inner,
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

pub struct OpaqueBuildHasherInner {
    build_hasher: Box<dyn any::Any>,
    build_hasher_type_id: TypeId,
    hasher_type_id: TypeId,
}

impl OpaqueBuildHasherInner {
    #[inline]
    fn new<S>(build_hasher: S) -> Self
    where
        S: hash::BuildHasher + any::Any,
    {
        let boxed_build_hasher = Box::new(build_hasher);
        let build_hasher_type_id: TypeId = TypeId::of::<S>();
        let hasher_type_id = TypeId::of::<S::Hasher>();

        Self {
            build_hasher: boxed_build_hasher,
            build_hasher_type_id,
            hasher_type_id,
        }
    }

    #[inline]
    fn from_boxed_build_hasher<S>(build_hasher: Box<S>) -> Self
    where
        S: hash::BuildHasher + any::Any,
    {
        let build_hasher_type_id: TypeId = TypeId::of::<S>();
        let hasher_type_id = TypeId::of::<S::Hasher>();

        Self {
            build_hasher,
            build_hasher_type_id,
            hasher_type_id,
        }
    }

    #[inline]
    fn is_build_hasher_type<S>(&self) -> bool
    where
        S: hash::BuildHasher + any::Any,
    {
        self.build_hasher_type_id == TypeId::of::<S>()
    }

    #[inline]
    fn is_hasher_type<S>(&self) -> bool
    where
        S: hash::Hasher + any::Any,
    {
        self.hasher_type_id == TypeId::of::<S>()
    }

    fn get_build_hasher_assuming_type<S>(&self) -> &S
    where
        S: hash::BuildHasher + any::Any,
    {
        let any_build_hasher = self.build_hasher.as_ref() as &dyn any::Any;
        any_build_hasher.downcast_ref::<S>().unwrap()
    }

    fn into_box_build_hasher_assuming_type<S>(self) -> Box<S>
    where
        S: hash::BuildHasher + any::Any,
    {
        let boxed_build_hasher = unsafe {
            let unboxed_build_hasher = Box::into_raw(self.build_hasher);
            Box::from_raw(unboxed_build_hasher as *mut S)
        };

        boxed_build_hasher
    }

    fn clone_assuming_type<A>(&self) -> Self
    where
        A: hash::BuildHasher + any::Any + Clone,
    {
        let any_build_hasher = self.build_hasher.as_ref() as &dyn any::Any;
        let build_hasher_ref = any_build_hasher
            .downcast_ref::<A>()
            .unwrap();
        let cloned_build_hasher = build_hasher_ref.clone();

        Self::new(cloned_build_hasher)
    }

    fn build_hasher_assuming_type<S>(&self) -> TypedProjHasher<S::Hasher>
    where
        S: hash::BuildHasher + any::Any,
    {
        let build_hasher = self.build_hasher.as_ref().downcast_ref::<S>().unwrap();
        let hasher = build_hasher.build_hasher();

        TypedProjHasher::new(hasher)
    }
}

#[repr(transparent)]
pub struct TypedProjBuildHasher<S> {
    inner: OpaqueBuildHasherInner,
    _marker: core::marker::PhantomData<S>,
}

impl<S> TypedProjBuildHasher<S>
where
    S: hash::BuildHasher + any::Any,
{
    #[inline]
    pub fn new(build_hasher: S) -> Self {
        let inner = OpaqueBuildHasherInner::new::<S>(build_hasher);

        Self { inner, _marker: PhantomData }
    }

    #[inline]
    pub fn from_boxed_build_hasher(build_hasher: Box<S>) -> Self {
        let inner = OpaqueBuildHasherInner::from_boxed_build_hasher(build_hasher);

        Self { inner, _marker: PhantomData }
    }

    pub fn get_build_hasher(&self) -> &S {
        self.inner.get_build_hasher_assuming_type::<S>()
    }

    pub fn into_box_build_hasher(self) -> Box<S> {
        self.inner.into_box_build_hasher_assuming_type::<S>()
    }
}

impl<S> hash::BuildHasher for TypedProjBuildHasher<S>
where
    S: hash::BuildHasher + any::Any,
{
    type Hasher = TypedProjHasher<S::Hasher>;

    fn build_hasher(&self) -> Self::Hasher {
        self.inner.build_hasher_assuming_type::<S>()
    }
}

impl<S> Clone for TypedProjBuildHasher<S>
where
    S: hash::BuildHasher + any::Any + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_assuming_type::<S>(),
            _marker: core::marker::PhantomData,
        }
    }
}

impl<S> Default for TypedProjBuildHasher<S>
where
    S: hash::BuildHasher + any::Any + Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<S> PartialEq for TypedProjBuildHasher<S>
where
    S: hash::BuildHasher + any::Any + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let inner_self = self.get_build_hasher();
        let inner_other = other.get_build_hasher();

        PartialEq::eq(&inner_self, &inner_other)
    }
}

impl<S> fmt::Debug for TypedProjBuildHasher<S>
where
    S: hash::BuildHasher + any::Any + fmt::Debug,
{
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TypedProjBuildHasher")
            .field("inner", self.inner.get_build_hasher_assuming_type::<S>())
            .finish()
    }
}

impl<S> From<S> for TypedProjBuildHasher<S>
where
    S: hash::BuildHasher + any::Any,
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
    pub fn new<S>(build_hasher: S) -> Self
    where
        S: hash::BuildHasher + any::Any,
    {
        let proj_alloc = TypedProjBuildHasher::<S>::new(build_hasher);

        Self::from_proj(proj_alloc)
    }

    #[inline]
    pub fn from_boxed_build_hasher<S>(build_hasher: Box<S>) -> Self
    where
        S: hash::BuildHasher + any::Any,
    {
        let proj_alloc = TypedProjBuildHasher::<S>::from_boxed_build_hasher(build_hasher);

        Self::from_proj(proj_alloc)
    }

    #[inline]
    pub fn is_build_hasher_type<S>(&self) -> bool
    where
        S: hash::BuildHasher + any::Any,
    {
        self.inner.is_build_hasher_type::<S>()
    }

    #[inline]
    pub fn is_hasher_type<H>(&self) -> bool
    where
        H: hash::Hasher + any::Any,
    {
        self.inner.is_hasher_type::<H>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<S>(&self)
    where
        S: hash::BuildHasher + any::Any,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(type_id_self: TypeId, type_id_other: TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.is_build_hasher_type::<S>() {
            type_check_failed(self.inner.build_hasher_type_id, TypeId::of::<S>());
        }
    }

    pub fn as_proj<S>(&self) -> &TypedProjBuildHasher<S>
    where
        S: hash::BuildHasher + any::Any,
    {
        self.assert_type_safety::<S>();

        unsafe { &*(self as *const OpaqueBuildHasher as *const TypedProjBuildHasher<S>) }
    }

    pub fn as_proj_mut<S>(&mut self) -> &mut TypedProjBuildHasher<S>
    where
        S: hash::BuildHasher + any::Any,
    {
        self.assert_type_safety::<S>();

        unsafe { &mut *(self as *mut OpaqueBuildHasher as *mut TypedProjBuildHasher<S>) }
    }

    pub fn into_proj<S>(self) -> TypedProjBuildHasher<S>
    where
        S: hash::BuildHasher + any::Any,
    {
        self.assert_type_safety::<S>();

        TypedProjBuildHasher {
            inner: self.inner,
            _marker: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn from_proj<S>(proj_self: TypedProjBuildHasher<S>) -> Self
    where
        S: hash::BuildHasher + any::Any,
    {
        Self {
            inner: proj_self.inner,
        }
    }
}

impl OpaqueBuildHasher {
    pub fn build_hasher<S>(&self) -> OpaqueHasher
    where
        S: hash::BuildHasher + any::Any,
    {
        let proj_self = self.as_proj::<S>();
        let proj_hasher = <TypedProjBuildHasher<S> as hash::BuildHasher>::build_hasher(proj_self);

        OpaqueHasher::from_proj(proj_hasher)
    }
}

impl fmt::Debug for OpaqueBuildHasher {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("OpaqueBuildHasher").finish()
    }
}
