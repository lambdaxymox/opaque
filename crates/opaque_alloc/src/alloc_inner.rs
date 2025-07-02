use core::any;
use core::marker;
use core::ptr::NonNull;

use alloc_crate::boxed::Box;

#[cfg(feature = "nightly")]
use alloc_crate::alloc;

#[cfg(not(feature = "nightly"))]
use allocator_api2::alloc;

/// This trait exists to define the [`TypedProjAllocInner`] data type. It is not meant for public use.
trait AnyAllocator: any::Any + alloc::Allocator + Send + Sync {}

impl<A> AnyAllocator for A where A: any::Any + alloc::Allocator + Send + Sync {}

#[repr(C)]
pub(crate) struct TypedProjAllocInner<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    alloc: Box<dyn AnyAllocator>,
    alloc_type_id: any::TypeId,
    _marker: marker::PhantomData<A>,
}

impl<A> TypedProjAllocInner<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    pub(crate) const fn allocator_type_id(&self) -> any::TypeId {
        self.alloc_type_id
    }
}

impl<A> TypedProjAllocInner<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    #[inline]
    pub(crate) fn new(alloc: A) -> Self {
        let boxed_alloc = Box::new(alloc);
        let alloc_type_id: any::TypeId = any::TypeId::of::<A>();

        Self {
            alloc: boxed_alloc,
            alloc_type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    pub(crate) fn from_boxed_alloc(alloc: Box<A>) -> Self {
        let alloc_type_id = any::TypeId::of::<A>();

        Self {
            alloc,
            alloc_type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    pub(crate) fn allocator(&self) -> &A {
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let any_alloc = (&*self.alloc) as &dyn any::Any;

        any_alloc.downcast_ref::<A>().unwrap()
    }

    pub(crate) fn into_boxed_alloc(self) -> Box<A> {
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        let any_alloc: Box<dyn any::Any> = self.alloc;

        any_alloc.downcast::<A>().unwrap()
    }
}

impl<A> Clone for TypedProjAllocInner<A>
where
    A: any::Any + alloc::Allocator + Send + Sync + Clone,
{
    fn clone(&self) -> Self {
        let cloned_alloc = Box::new(self.allocator().clone());

        Self::from_boxed_alloc(cloned_alloc)
    }
}

unsafe impl<A> alloc::Allocator for TypedProjAllocInner<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
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

#[repr(C)]
pub(crate) struct OpaqueAllocInner {
    alloc: Box<dyn AnyAllocator>,
    alloc_type_id: any::TypeId,
}

impl OpaqueAllocInner {
    #[inline]
    pub(crate) const fn allocator_type_id(&self) -> any::TypeId {
        self.alloc_type_id
    }
}

impl OpaqueAllocInner {
    #[inline(always)]
    pub(crate) fn as_proj_assuming_type<A>(&self) -> &TypedProjAllocInner<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe { &*(self as *const OpaqueAllocInner as *const TypedProjAllocInner<A>) }
    }

    #[inline(always)]
    pub(crate) fn as_proj_mut_assuming_type<A>(&mut self) -> &mut TypedProjAllocInner<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        unsafe { &mut *(self as *mut OpaqueAllocInner as *mut TypedProjAllocInner<A>) }
    }

    #[inline(always)]
    pub(crate) fn into_proj_assuming_type<A>(self) -> TypedProjAllocInner<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        debug_assert_eq!(self.allocator_type_id(), any::TypeId::of::<A>());

        TypedProjAllocInner {
            alloc: self.alloc,
            alloc_type_id: self.alloc_type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline(always)]
    pub(crate) fn from_proj<A>(proj_self: TypedProjAllocInner<A>) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
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
mod alloc_inner_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_opaque_alloc_inner_match_sizes<A>()
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::size_of::<TypedProjAllocInner<A>>();
        let result = mem::size_of::<OpaqueAllocInner>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_alloc_inner_match_alignments<A>()
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let expected = mem::align_of::<TypedProjAllocInner<A>>();
        let result = mem::align_of::<OpaqueAllocInner>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_alloc_inner_match_offsets<A>()
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        assert_eq!(
            mem::offset_of!(TypedProjAllocInner<A>, alloc),
            mem::offset_of!(OpaqueAllocInner, alloc),
            "Opaque and Typed Projected data types offsets mismatch"
        );
        assert_eq!(
            mem::offset_of!(TypedProjAllocInner<A>, alloc_type_id),
            mem::offset_of!(OpaqueAllocInner, alloc_type_id),
            "Opaque and Typed Projected data types offsets mismatch"
        );
    }

    macro_rules! layout_tests {
        ($module_name:ident, $alloc_typ:ty) => {
            mod $module_name {
                use super::*;

                #[test]
                fn test_opaque_alloc_inner_layout_match_sizes() {
                    run_test_opaque_alloc_inner_match_sizes::<$alloc_typ>();
                }

                #[test]
                fn test_opaque_alloc_inner_layout_match_alignments() {
                    run_test_opaque_alloc_inner_match_alignments::<$alloc_typ>();
                }

                #[test]
                fn test_opaque_alloc_inner_layout_match_offsets() {
                    run_test_opaque_alloc_inner_match_offsets::<$alloc_typ>();
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

        assert_send_sync::<TypedProjAllocInner<alloc::Global>>();
    }

    #[test]
    fn test_assert_send_sync2() {
        fn assert_send_sync<T: Send + Sync>() {}

        assert_send_sync::<TypedProjAllocInner<dummy::DummyAlloc>>();
    }
}
