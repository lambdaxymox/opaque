#![feature(allocator_api)]
#![feature(alloc_layout_extra)]
#![feature(optimize_attribute)]
use std::alloc;
use std::alloc::Allocator;
use std::any;
use std::any::TypeId;
use std::fmt;
use std::marker::PhantomData;
use std::ptr::NonNull;

#[repr(C)]
struct TypedProjAllocInner<A> {
    alloc: Box<A>,
    alloc_type_id: TypeId,
}

impl<A> TypedProjAllocInner<A> {
    #[inline]
    const fn alloc_type_id(&self) -> TypeId {
        self.alloc_type_id
    }
}

impl<A> TypedProjAllocInner<A>
where
    A: any::Any + alloc::Allocator,
{
    #[inline]
    fn new(alloc: A) -> Self {
        let boxed_alloc = Box::new(alloc);
        let alloc_type_id: TypeId = TypeId::of::<A>();

        Self { alloc: boxed_alloc, alloc_type_id, }
    }

    #[inline]
    fn from_boxed_alloc(alloc: Box<A>) -> Self {
        let alloc_type_id = TypeId::of::<A>();

        Self { alloc, alloc_type_id, }
    }

    #[inline]
    fn allocator(&self) -> &A {
        self.alloc.as_ref()
    }

    fn into_boxed_alloc(self) -> Box<A> {
        self.alloc
    }
}

impl<A> Clone for TypedProjAllocInner<A>
where
    A: any::Any + alloc::Allocator + Clone,
{
    fn clone(&self) -> Self {
        let cloned_alloc = self.alloc.clone();

        Self::from_boxed_alloc(cloned_alloc)
    }
}

unsafe impl<A> alloc::Allocator for TypedProjAllocInner<A>
where
    A: any::Any + alloc::Allocator,
{
    #[inline]
    fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
        self.alloc.allocate(layout)
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
        unsafe {
            self.alloc.deallocate(ptr, layout);
        }
    }
}

trait AnyAllocator: any::Any + alloc::Allocator {}

impl<A> AnyAllocator for A where A: any::Any + alloc::Allocator {}

#[repr(C)]
struct OpaqueAllocInner {
    alloc: Box<dyn AnyAllocator>,
    alloc_type_id: TypeId,
}

impl OpaqueAllocInner {
    #[inline]
    const fn alloc_type_id(&self) -> TypeId {
        self.alloc_type_id
    }
}

impl OpaqueAllocInner {
    pub(crate) fn as_proj_assuming_type<A>(&self) -> &TypedProjAllocInner<A>
    where
        A: any::Any + Allocator,
    {
        unsafe { &*(self as *const OpaqueAllocInner as *const TypedProjAllocInner<A>) }
    }

    pub(crate) fn as_proj_mut_assuming_type<A>(&mut self) -> &mut TypedProjAllocInner<A>
    where
        A: any::Any + Allocator,
    {
        unsafe { &mut *(self as *mut OpaqueAllocInner as *mut TypedProjAllocInner<A>) }
    }

    pub(crate) fn into_proj_assuming_type<A>(self) -> TypedProjAllocInner<A>
    where
        A: any::Any + Allocator,
    {
        let boxed_alloc = unsafe {
            let unboxed_alloc = Box::into_raw(self.alloc);
            Box::from_raw(unboxed_alloc as *mut A)
        };

        TypedProjAllocInner {
            alloc: boxed_alloc,
            alloc_type_id: self.alloc_type_id,
        }
    }

    #[inline]
    pub(crate) fn from_proj<A>(proj_self: TypedProjAllocInner<A>) -> Self
    where
        A: any::Any + Allocator,
    {
        Self {
            alloc: proj_self.alloc,
            alloc_type_id: proj_self.alloc_type_id,
        }
    }
}

unsafe impl alloc::Allocator for OpaqueAllocInner {
    #[inline]
    fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
        self.alloc.allocate(layout)
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
        unsafe {
            self.alloc.deallocate(ptr, layout);
        }
    }
}

#[repr(transparent)]
pub struct TypedProjAlloc<A> {
    inner: TypedProjAllocInner<A>,
}

impl<A> TypedProjAlloc<A>
where
    A: any::Any + alloc::Allocator,
{
    #[inline]
    pub fn new(alloc: A) -> Self {
        let inner = TypedProjAllocInner::new(alloc);

        Self { inner, }
    }

    #[inline]
    pub fn from_boxed_alloc(alloc: Box<A>) -> Self {
        let inner = TypedProjAllocInner::from_boxed_alloc(alloc);

        Self { inner, }
    }

    pub fn allocator(&self) -> &A {
        self.inner.allocator()
    }

    pub fn into_boxed_alloc(self) -> Box<A> {
        self.inner.into_boxed_alloc()
    }
}

unsafe impl<A> alloc::Allocator for TypedProjAlloc<A>
where
    A: any::Any + alloc::Allocator,
{
    fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        self.inner.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
        unsafe {
            self.inner.deallocate(ptr, layout);
        }
    }
}

impl<A> Clone for TypedProjAlloc<A>
where
    A: any::Any + alloc::Allocator + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<A> fmt::Debug for TypedProjAlloc<A>
where
    A: any::Any + alloc::Allocator + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypedProjAlloc")
            .field("inner", self.inner.allocator())
            .finish()
    }
}

impl<A> Default for TypedProjAlloc<A>
where
    A: any::Any + alloc::Allocator + Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<A> From<A> for TypedProjAlloc<A>
where
    A: any::Any + alloc::Allocator,
{
    fn from(alloc: A) -> Self {
        Self::new(alloc)
    }
}

#[repr(transparent)]
pub struct OpaqueAlloc {
    inner: OpaqueAllocInner,
}

impl OpaqueAlloc {
    #[inline]
    pub const fn alloc_type_id(&self) -> TypeId {
        self.inner.alloc_type_id()
    }

    #[inline]
    pub fn has_alloc_type<A>(&self) -> bool
    where
        A: any::Any + alloc::Allocator,
    {
        self.inner.alloc_type_id() == TypeId::of::<A>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<A>(&self)
    where
        A: any::Any + alloc::Allocator,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(type_id_self: TypeId, type_id_other: TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_alloc_type::<A>() {
            type_check_failed(self.inner.alloc_type_id, TypeId::of::<A>());
        }
    }
}

impl OpaqueAlloc {
    #[inline]
    pub fn new<A>(alloc: A) -> Self
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_alloc = TypedProjAlloc::<A>::new(alloc);

        Self::from_proj(proj_alloc)
    }

    #[inline]
    pub fn from_boxed_alloc<A>(alloc: Box<A>) -> Self
    where
        A: any::Any + alloc::Allocator,
    {
        let proj_alloc = TypedProjAlloc::<A>::from_boxed_alloc(alloc);

        Self::from_proj(proj_alloc)
    }
}

impl OpaqueAlloc {
    pub fn as_proj<A>(&self) -> &TypedProjAlloc<A>
    where
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<A>();

        unsafe { &*(self as *const OpaqueAlloc as *const TypedProjAlloc<A>) }
    }

    pub fn as_proj_mut<A>(&mut self) -> &mut TypedProjAlloc<A>
    where
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<A>();

        unsafe { &mut *(self as *mut OpaqueAlloc as *mut TypedProjAlloc<A>) }
    }

    pub fn into_proj<A>(self) -> TypedProjAlloc<A>
    where
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<A>();

        TypedProjAlloc {
            inner: self.inner.into_proj_assuming_type::<A>(),
        }
    }

    #[inline]
    pub fn from_proj<A>(proj_self: TypedProjAlloc<A>) -> Self
    where
        A: any::Any + alloc::Allocator,
    {
        Self {
            inner: OpaqueAllocInner::from_proj(proj_self.inner),
        }
    }
}

unsafe impl alloc::Allocator for OpaqueAlloc {
    fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        self.inner.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
        unsafe {
            self.inner.deallocate(ptr, layout);
        }
    }
}

impl fmt::Debug for OpaqueAlloc {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("OpaqueAlloc").finish()
    }
}
