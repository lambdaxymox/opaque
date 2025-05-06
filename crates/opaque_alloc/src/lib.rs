#![feature(allocator_api)]
#![feature(alloc_layout_extra)]
#![feature(optimize_attribute)]
use std::alloc;
use std::alloc::{
    Allocator,
    Layout,
};
use std::any;
use std::any::TypeId;
use std::fmt;
use std::marker::PhantomData;
use std::ptr::NonNull;

pub trait AnyAllocator: Allocator + any::Any {}

impl<A> AnyAllocator for A where A: Allocator + any::Any {}

struct OpaqueAllocInner {
    alloc: Box<dyn AnyAllocator>,
    type_id: TypeId,
}

impl OpaqueAllocInner {
    #[inline]
    fn new<A>(alloc: A) -> Self
    where
        A: Allocator + any::Any,
    {
        let boxed_alloc = Box::new(alloc);
        let type_id: TypeId = TypeId::of::<A>();

        Self { alloc: boxed_alloc, type_id, }
    }

    #[inline]
    fn from_boxed_alloc<A>(alloc: Box<A>) -> Self
    where
        A: Allocator + any::Any,
    {
        let type_id = TypeId::of::<A>();

        Self { alloc, type_id, }
    }

    #[inline]
    fn is_type<A>(&self) -> bool
    where
        A: Allocator + any::Any,
    {
        self.type_id == TypeId::of::<A>()
    }

    #[inline]
    fn allocator_assuming_type<A>(&self) -> &A
    where
        A: Allocator + any::Any,
    {
        let any_alloc = self.alloc.as_ref() as &dyn any::Any;
        any_alloc.downcast_ref::<A>().unwrap()
    }

    fn into_boxed_alloc_assuming_type<A>(self) -> Box<A>
    where
        A: Allocator + any::Any,
    {
        let boxed_alloc = unsafe {
            let unboxed_alloc = Box::into_raw(self.alloc);
            Box::from_raw(unboxed_alloc as *mut A)
        };

        boxed_alloc
    }

    fn clone_assuming_type<A>(&self) -> OpaqueAllocInner
    where
        A: Allocator + any::Any + Clone,
    {
        let any_alloc = self.alloc.as_ref() as &dyn any::Any;
        let alloc_ref = any_alloc
            .downcast_ref::<A>()
            .unwrap();
        let cloned_alloc = alloc_ref.clone();

        OpaqueAllocInner::new(cloned_alloc)
    }
}

unsafe impl alloc::Allocator for OpaqueAllocInner {
    #[inline]
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        self.alloc.allocate(layout)
    }

    #[inline]
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            self.alloc.deallocate(ptr, layout);
        }
    }
}

#[repr(transparent)]
pub struct TypedProjAlloc<A> {
    inner: OpaqueAllocInner,
    _marker: PhantomData<A>,
}

impl<A> TypedProjAlloc<A>
where
    A: Allocator + any::Any,
{
    #[inline]
    pub fn new(alloc: A) -> Self {
        let inner = OpaqueAllocInner::new::<A>(alloc);

        Self { inner, _marker: PhantomData }
    }

    #[inline]
    pub fn from_boxed_alloc(alloc: Box<A>) -> Self {
        let inner = OpaqueAllocInner::from_boxed_alloc(alloc);

        Self { inner, _marker: PhantomData }
    }

    pub fn allocator(&self) -> &A {
        self.inner.allocator_assuming_type::<A>()
    }

    pub fn into_box_alloc(self) -> Box<A> {
        self.inner.into_boxed_alloc_assuming_type::<A>()
    }
}

unsafe impl<A> alloc::Allocator for TypedProjAlloc<A>
where
    A: Allocator + any::Any,
{
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        self.inner.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            self.inner.deallocate(ptr, layout);
        }
    }
}

impl<A> Clone for TypedProjAlloc<A>
where
    A: Allocator + any::Any + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone_assuming_type::<A>(),
            _marker: self._marker,
        }
    }
}

impl<A> fmt::Debug for TypedProjAlloc<A>
where
    A: Allocator + any::Any + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypedProjAlloc")
            .field("inner", &format_args!("{:?}", any::type_name::<Box<A>>()))
            .finish()
    }
}

impl<A> Default for TypedProjAlloc<A>
where
    A: Allocator + any::Any + Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<A> From<A> for TypedProjAlloc<A>
where
    A: Allocator + any::Any,
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
    pub fn new<A>(alloc: A) -> Self
    where
        A: Allocator + any::Any,
    {
        let proj_alloc = TypedProjAlloc::<A>::new(alloc);

        Self::from_proj(proj_alloc)
    }

    #[inline]
    pub fn from_boxed_alloc<A>(alloc: Box<A>) -> Self
    where
        A: Allocator + any::Any,
    {
        let proj_alloc = TypedProjAlloc::<A>::from_boxed_alloc(alloc);

        Self::from_proj(proj_alloc)
    }

    #[inline]
    pub fn is_type<A>(&self) -> bool
    where
        A: Allocator + any::Any,
    {
        self.inner.is_type::<A>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<A>(&self)
    where
        A: Allocator + any::Any,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(type_id_self: TypeId, type_id_other: TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.is_type::<A>() {
            type_check_failed(self.inner.type_id, TypeId::of::<A>());
        }
    }

    pub fn as_proj<A>(&self) -> &TypedProjAlloc<A>
    where
        A: Allocator + any::Any,
    {
        self.assert_type_safety::<A>();

        unsafe { &*(self as *const OpaqueAlloc as *const TypedProjAlloc<A>) }
    }

    pub fn as_proj_mut<A>(&mut self) -> &mut TypedProjAlloc<A>
    where
        A: Allocator + any::Any,
    {
        self.assert_type_safety::<A>();

        unsafe { &mut *(self as *mut OpaqueAlloc as *mut TypedProjAlloc<A>) }
    }

    pub fn into_proj<A>(self) -> TypedProjAlloc<A>
    where
        A: Allocator + any::Any,
    {
        self.assert_type_safety::<A>();

        TypedProjAlloc {
            inner: self.inner,
            _marker: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn from_proj<A>(proj_self: TypedProjAlloc<A>) -> Self
    where
        A: Allocator + any::Any,
    {
        Self {
            inner: proj_self.inner,
        }
    }
}

unsafe impl alloc::Allocator for OpaqueAlloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, std::alloc::AllocError> {
        self.inner.allocate(layout)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        unsafe {
            self.inner.deallocate(ptr, layout);
        }
    }
}

impl fmt::Debug for OpaqueAlloc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OpaqueAlloc").finish()
    }
}
