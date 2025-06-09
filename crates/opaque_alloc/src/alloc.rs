use crate::alloc_inner::{TypedProjAllocInner, OpaqueAllocInner};

use core::any;
use core::fmt;
use core::marker;
use core::ptr::NonNull;
use alloc_crate::alloc;
use alloc_crate::boxed::Box;

#[repr(transparent)]
pub struct TypedProjAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    inner: TypedProjAllocInner<A>,
}

impl<A> TypedProjAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
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
    A: any::Any + alloc::Allocator + Send + Sync,
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
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<A> fmt::Debug for TypedProjAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TypedProjAlloc")
            .field("inner", self.inner.allocator())
            .finish()
    }
}

impl<A> Default for TypedProjAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync + Default,
{
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<A> From<A> for TypedProjAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
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
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }

    #[inline]
    pub fn has_alloc_type<A>(&self) -> bool
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.inner.allocator_type_id() == any::TypeId::of::<A>()
    }

    #[inline]
    #[track_caller]
    fn assert_type_safety<A>(&self)
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        #[cold]
        #[optimize(size)]
        #[track_caller]
        fn type_check_failed(type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_alloc_type::<A>() {
            type_check_failed(self.inner.allocator_type_id(), any::TypeId::of::<A>());
        }
    }
}

impl OpaqueAlloc {
    #[inline]
    pub fn new<A>(alloc: A) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_alloc = TypedProjAlloc::<A>::new(alloc);

        Self::from_proj(proj_alloc)
    }

    #[inline]
    pub fn from_boxed_alloc<A>(alloc: Box<A>) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_alloc = TypedProjAlloc::<A>::from_boxed_alloc(alloc);

        Self::from_proj(proj_alloc)
    }
}

impl OpaqueAlloc {
    #[inline]
    pub fn as_proj<A>(&self) -> &TypedProjAlloc<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<A>();

        unsafe { &*(self as *const OpaqueAlloc as *const TypedProjAlloc<A>) }
    }

    #[inline]
    pub fn as_proj_mut<A>(&mut self) -> &mut TypedProjAlloc<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<A>();

        unsafe { &mut *(self as *mut OpaqueAlloc as *mut TypedProjAlloc<A>) }
    }

    #[inline]
    pub fn into_proj<A>(self) -> TypedProjAlloc<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<A>();

        TypedProjAlloc {
            inner: self.inner.into_proj_assuming_type::<A>(),
        }
    }

    #[inline]
    pub fn from_proj<A>(proj_self: TypedProjAlloc<A>) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
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

mod dummy {
    use super::*;
    use core::ptr::NonNull;

    #[allow(dead_code)]
    pub(super) struct DummyAlloc {
        _do_not_construct: marker::PhantomData<()>,
    }

    unsafe impl alloc::Allocator for DummyAlloc {
        fn allocate(&self, _layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
            panic!("[`DummyAlloc::allocate`] should never actually be called. Its purpose is to test struct layouts.");
        }

        unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: alloc::Layout) {
            panic!("[`DummyAlloc::deallocate`] should never actually be called. Its purpose is to test struct layouts.");
        }
    }
}

#[cfg(test)]
mod alloc_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_opaque_alloc_match_sizes<A>()
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::size_of::<TypedProjAlloc<A>>();
        let result = mem::size_of::<OpaqueAlloc>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_alloc_match_alignments<A>()
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::align_of::<TypedProjAlloc<A>>();
        let result = mem::align_of::<OpaqueAlloc>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_alloc_match_offsets<A>()
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        assert_eq!(
            mem::offset_of!(TypedProjAlloc<A>, inner),
            mem::offset_of!(OpaqueAlloc, inner),
            "Opaque and Typed Projected data types offsets mismatch"
        );
    }

    macro_rules! layout_tests {
        ($module_name:ident, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_opaque_alloc_inner_layout_match_sizes() {
                    run_test_opaque_alloc_match_sizes::<$alloc_typ>();
                }

                #[test]
                fn test_opaque_alloc_inner_layout_match_alignments() {
                    run_test_opaque_alloc_match_alignments::<$alloc_typ>();
                }

                #[test]
                fn test_opaque_alloc_inner_layout_match_offsets() {
                    run_test_opaque_alloc_match_offsets::<$alloc_typ>();
                }
            }
        };
    }

    layout_tests!(global, alloc::Global);
    layout_tests!(dummy_alloc, dummy::DummyAlloc);
}

#[cfg(test)]
mod assert_send_sync {
    use super::*;

    #[test]
    fn test_assert_send_sync1() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjAlloc<alloc::Global>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjAlloc<dummy::DummyAlloc>>();
    }
}
