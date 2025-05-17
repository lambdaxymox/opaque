#![feature(optimize_attribute)]
use core::{any, fmt};
use core::any::TypeId;
use core::hash;
use core::marker::PhantomData;

trait AnyHasher: hash::Hasher + any::Any {}

impl<H> AnyHasher for H where H: hash::Hasher + any::Any {}

#[repr(C)]
struct TypedProjHasherInner<H> {
    hasher: Box<H>,
    type_id: TypeId,
}

impl<H> TypedProjHasherInner<H> {
    #[inline]
    const fn hasher_type_id(&self) -> TypeId {
        self.type_id
    }
}

impl<H> TypedProjHasherInner<H>
where
    H: any::Any + hash::Hasher,
{
    #[inline]
    fn new(hasher: H) -> Self {
        let boxed_hasher = Box::new(hasher);
        let type_id = TypeId::of::<H>();

        Self {
            hasher: boxed_hasher,
            type_id
        }
    }

    #[inline]
    fn from_boxed_hasher(hasher: Box<H>) -> Self {
        let type_id = TypeId::of::<H>();

        Self {
            hasher,
            type_id,
        }
    }

    #[inline]
    fn hasher_assuming_type(&self) -> &H {
        let any_hasher = self.hasher.as_ref() as &dyn any::Any;
        any_hasher.downcast_ref::<H>().unwrap()
    }

    #[inline]
    fn into_boxed_hasher_assuming_type(self) -> Box<H> {
        let boxed_hasher = unsafe {
            let unboxed_hasher = Box::into_raw(self.hasher);
            Box::from_raw(unboxed_hasher as *mut H)
        };

        boxed_hasher
    }
}

impl<H> TypedProjHasherInner<H> {
    #[inline]
    fn clone_assuming_type(&self) -> TypedProjHasherInner<H>
    where
        H: hash::Hasher + any::Any + Clone,
    {
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
    H: any::Any + hash::Hasher,
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
    type_id: TypeId,
}

impl OpaqueHasherInner {
    #[inline]
    const fn hasher_type_id(&self) -> TypeId {
        self.type_id
    }
}

impl OpaqueHasherInner {
    #[inline]
    fn new<H>(hasher: H) -> Self
    where
        H: any::Any + hash::Hasher,
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
        H: any::Any + hash::Hasher,
    {
        let type_id = TypeId::of::<H>();

        Self {
            hasher,
            type_id,
        }
    }
}

impl OpaqueHasherInner {
    pub(crate) fn as_proj<H>(&self) -> &TypedProjHasherInner<H>
    where
        H: any::Any + hash::Hasher,
    {
        unsafe { &*(self as *const OpaqueHasherInner as *const TypedProjHasherInner<H>) }
    }

    pub(crate) fn as_proj_mut<H>(&mut self) -> &mut TypedProjHasherInner<H>
    where
        H: any::Any + hash::Hasher,
    {
        unsafe { &mut *(self as *mut OpaqueHasherInner as *mut TypedProjHasherInner<H>) }
    }

    pub(crate) fn into_proj<H>(self) -> TypedProjHasherInner<H>
    where
        H: any::Any + hash::Hasher,
    {
        let boxed_hasher = unsafe {
            let unboxed_alloc = Box::into_raw(self.hasher);
            Box::from_raw(unboxed_alloc as *mut H)
        };

        TypedProjHasherInner {
            hasher: boxed_hasher,
            type_id: self.type_id,
        }
    }

    #[inline]
    pub(crate) fn from_proj<H>(proj_self: TypedProjHasherInner<H>) -> Self
    where
        H: any::Any + hash::Hasher,
    {
        Self {
            hasher: proj_self.hasher,
            type_id: proj_self.type_id,
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
pub struct TypedProjHasher<H> {
    inner: TypedProjHasherInner<H>,
}

impl<H> TypedProjHasher<H>
where
    H: any::Any + hash::Hasher,
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
    H: any::Any + hash::Hasher,
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
            inner: self.inner.clone_assuming_type(),
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
            .field("inner", self.inner.hasher_assuming_type())
            .finish()
    }
}

impl<H> From<H> for TypedProjHasher<H>
where
    H: any::Any + hash::Hasher,
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
    pub const fn hasher_type_id(&self) -> TypeId {
        self.inner.hasher_type_id()
    }

    #[inline]
    pub fn has_hasher_type<H>(&self) -> bool
    where
        H: any::Any + hash::Hasher,
    {
        self.inner.hasher_type_id() == TypeId::of::<H>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<H>(&self)
    where
        H: any::Any + hash::Hasher,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(type_id_self: TypeId, type_id_other: TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_hasher_type::<H>() {
            type_check_failed(self.inner.type_id, TypeId::of::<H>());
        }
    }
}

impl OpaqueHasher {
    #[inline]
    pub fn new<H>(hasher: H) -> Self
    where
        H: any::Any + hash::Hasher,
    {
        let proj_alloc = TypedProjHasher::<H>::new(hasher);

        Self::from_proj(proj_alloc)
    }

    #[inline]
    pub fn from_boxed_hasher<H>(hasher: Box<H>) -> Self
    where
        H: any::Any + hash::Hasher,
    {
        let proj_alloc = TypedProjHasher::<H>::from_boxed_hasher(hasher);

        Self::from_proj(proj_alloc)
    }
}

impl OpaqueHasher {
    pub fn as_proj<H>(&self) -> &TypedProjHasher<H>
    where
        H: any::Any + hash::Hasher,
    {
        self.assert_type_safety::<H>();

        unsafe { &*(self as *const OpaqueHasher as *const TypedProjHasher<H>) }
    }

    pub fn as_proj_mut<H>(&mut self) -> &mut TypedProjHasher<H>
    where
        H: any::Any + hash::Hasher,
    {
        self.assert_type_safety::<H>();

        unsafe { &mut *(self as *mut OpaqueHasher as *mut TypedProjHasher<H>) }
    }

    pub fn into_proj<H>(self) -> TypedProjHasher<H>
    where
        H: any::Any + hash::Hasher,
    {
        self.assert_type_safety::<H>();

        TypedProjHasher {
            inner: self.inner.into_proj(),
        }
    }

    #[inline]
    pub fn from_proj<H>(proj_self: TypedProjHasher<H>) -> Self
    where
        H: any::Any + hash::Hasher,
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
struct TypedProjBuildHasherInner<S> {
    build_hasher: Box<S>,
    build_hasher_type_id: TypeId,
    hasher_type_id: TypeId,
}

impl<S> TypedProjBuildHasherInner<S>
where
    S: any::Any + hash::BuildHasher,
{
    #[inline]
    fn new(build_hasher: S) -> Self {
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
    fn from_boxed_build_hasher(build_hasher: Box<S>) -> Self {
        let build_hasher_type_id: TypeId = TypeId::of::<S>();
        let hasher_type_id = TypeId::of::<S::Hasher>();

        Self {
            build_hasher,
            build_hasher_type_id,
            hasher_type_id,
        }
    }

    fn get_build_hasher(&self) -> &S {
        let any_build_hasher = self.build_hasher.as_ref() as &dyn any::Any;
        any_build_hasher.downcast_ref::<S>().unwrap()
    }

    fn into_boxed_build_hasher(self) -> Box<S> {
        let boxed_build_hasher = unsafe {
            let unboxed_build_hasher = Box::into_raw(self.build_hasher);
            Box::from_raw(unboxed_build_hasher as *mut S)
        };

        boxed_build_hasher
    }

    fn build_hasher(&self) -> TypedProjHasher<S::Hasher> {
        let hasher = self.build_hasher.build_hasher();

        TypedProjHasher::new(hasher)
    }
}

impl<S> Clone for TypedProjBuildHasherInner<S>
where
    S: any::Any + hash::BuildHasher + Clone,
{
    fn clone(&self) -> Self {
        let cloned_build_hasher = self.build_hasher.clone();

        Self::from_boxed_build_hasher(cloned_build_hasher)
    }
}

#[repr(C)]
struct OpaqueBuildHasherInner {
    build_hasher: Box<dyn any::Any>,
    build_hasher_type_id: TypeId,
    hasher_type_id: TypeId,
}

impl OpaqueBuildHasherInner {
    #[inline]
    const fn build_hasher_type_id(&self) -> TypeId {
        self.build_hasher_type_id
    }

    #[inline]
    const fn hasher_type_id(&self) -> TypeId {
        self.hasher_type_id
    }
}

impl OpaqueBuildHasherInner {
    #[inline]
    fn new<S>(build_hasher: S) -> Self
    where
        S: any::Any + hash::BuildHasher,
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
        S: any::Any + hash::BuildHasher,
    {
        let build_hasher_type_id: TypeId = TypeId::of::<S>();
        let hasher_type_id = TypeId::of::<S::Hasher>();

        Self {
            build_hasher,
            build_hasher_type_id,
            hasher_type_id,
        }
    }
}

impl OpaqueBuildHasherInner {
    pub fn as_proj<S>(&self) -> &TypedProjBuildHasherInner<S>
    where
        S: any::Any + hash::BuildHasher,
    {
        unsafe { &*(self as *const OpaqueBuildHasherInner as *const TypedProjBuildHasherInner<S>) }
    }

    pub fn as_proj_mut<S>(&mut self) -> &mut TypedProjBuildHasherInner<S>
    where
        S: any::Any + hash::BuildHasher,
    {
        unsafe { &mut *(self as *mut OpaqueBuildHasherInner as *mut TypedProjBuildHasherInner<S>) }
    }

    pub fn into_proj<S>(self) -> TypedProjBuildHasherInner<S>
    where
        S: any::Any + hash::BuildHasher,
    {
        let boxed_build_hasher = unsafe {
            let unboxed_build_hasher = Box::into_raw(self.build_hasher);
            Box::from_raw(unboxed_build_hasher as *mut S)
        };

        TypedProjBuildHasherInner {
            build_hasher: boxed_build_hasher,
            build_hasher_type_id: self.build_hasher_type_id,
            hasher_type_id: self.hasher_type_id,
        }
    }

    #[inline]
    pub fn from_proj<S>(proj_self: TypedProjBuildHasherInner<S>) -> Self
    where
        S: any::Any + hash::BuildHasher,
    {
        Self {
            build_hasher: proj_self.build_hasher,
            build_hasher_type_id: proj_self.build_hasher_type_id,
            hasher_type_id: proj_self.hasher_type_id,
        }
    }
}

#[repr(transparent)]
pub struct TypedProjBuildHasher<S> {
    inner: TypedProjBuildHasherInner<S>,
}

impl<S> TypedProjBuildHasher<S>
where
    S: any::Any + hash::BuildHasher,
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
    S: any::Any + hash::BuildHasher,
{
    type Hasher = TypedProjHasher<S::Hasher>;

    fn build_hasher(&self) -> Self::Hasher {
        self.inner.build_hasher()
    }
}

impl<S> Clone for TypedProjBuildHasher<S>
where
    S: hash::BuildHasher + any::Any + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
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
            .field("inner", self.inner.get_build_hasher())
            .finish()
    }
}

impl<S> From<S> for TypedProjBuildHasher<S>
where
    S: any::Any + hash::BuildHasher,
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
    pub const fn build_hasher_type_id(&self) -> TypeId {
        self.inner.build_hasher_type_id()
    }

    #[inline]
    pub const fn hasher_type_id(&self) -> TypeId {
        self.inner.hasher_type_id()
    }
}

impl OpaqueBuildHasher {
    #[inline]
    pub fn has_build_hasher_type<S>(&self) -> bool
    where
        S: any::Any + hash::BuildHasher,
    {
        self.inner.build_hasher_type_id() == TypeId::of::<S>()
    }

    #[inline]
    pub fn has_hasher_type<H>(&self) -> bool
    where
        H: any::Any + hash::Hasher,
    {
        self.inner.hasher_type_id() == TypeId::of::<H>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<S>(&self)
    where
        S: any::Any + hash::BuildHasher,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(type_id_self: TypeId, type_id_other: TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_build_hasher_type::<S>() {
            type_check_failed(self.inner.build_hasher_type_id, TypeId::of::<S>());
        }
    }
}

impl OpaqueBuildHasher {
    #[inline]
    pub fn new<S>(build_hasher: S) -> Self
    where
        S: any::Any + hash::BuildHasher,
    {
        let proj_build_hasher = TypedProjBuildHasher::<S>::new(build_hasher);

        Self::from_proj(proj_build_hasher)
    }

    #[inline]
    pub fn from_boxed_build_hasher<S>(build_hasher: Box<S>) -> Self
    where
        S: any::Any + hash::BuildHasher,
    {
        let proj_build_hasher = TypedProjBuildHasher::<S>::from_boxed_build_hasher(build_hasher);

        Self::from_proj(proj_build_hasher)
    }
}

impl OpaqueBuildHasher {
    pub fn as_proj<S>(&self) -> &TypedProjBuildHasher<S>
    where
        S: any::Any + hash::BuildHasher,
    {
        self.assert_type_safety::<S>();

        unsafe { &*(self as *const OpaqueBuildHasher as *const TypedProjBuildHasher<S>) }
    }

    pub fn as_proj_mut<S>(&mut self) -> &mut TypedProjBuildHasher<S>
    where
        S: any::Any + hash::BuildHasher,
    {
        self.assert_type_safety::<S>();

        unsafe { &mut *(self as *mut OpaqueBuildHasher as *mut TypedProjBuildHasher<S>) }
    }

    pub fn into_proj<S>(self) -> TypedProjBuildHasher<S>
    where
        S: any::Any + hash::BuildHasher,
    {
        self.assert_type_safety::<S>();

        TypedProjBuildHasher {
            inner: self.inner.into_proj(),
        }
    }

    #[inline]
    pub fn from_proj<S>(proj_self: TypedProjBuildHasher<S>) -> Self
    where
        S: any::Any + hash::BuildHasher,
    {
        Self {
            inner: OpaqueBuildHasherInner::from_proj(proj_self.inner),
        }
    }
}

impl OpaqueBuildHasher {
    pub fn build_hasher<S>(&self) -> OpaqueHasher
    where
        S: any::Any + hash::BuildHasher,
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
