#![feature(allocator_api)]
#![feature(alloc_layout_extra)]
#![feature(optimize_attribute)]
use core::any;
use core::fmt;
use core::marker;
use core::ptr::NonNull;
use std::alloc;

trait AnyAllocator: any::Any + alloc::Allocator {}

impl<A> AnyAllocator for A where A: any::Any + alloc::Allocator {}

#[repr(C)]
struct TypedProjAllocInner<A> {
    alloc: Box<dyn AnyAllocator>,
    alloc_type_id: any::TypeId,
    _marker: marker::PhantomData<A>,
}

impl<A> TypedProjAllocInner<A> {
    #[inline]
    const fn allocator_type_id(&self) -> any::TypeId {
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
        let alloc_type_id: any::TypeId = any::TypeId::of::<A>();

        Self {
            alloc: boxed_alloc,
            alloc_type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    fn from_boxed_alloc(alloc: Box<A>) -> Self {
        let alloc_type_id = any::TypeId::of::<A>();

        Self {
            alloc,
            alloc_type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline]
    fn allocator(&self) -> &A {
        debug_assert_eq!(self.alloc_type_id, any::TypeId::of::<A>());

        let any_alloc = (&*self.alloc) as &dyn any::Any;

        any_alloc.downcast_ref::<A>().unwrap()
    }

    fn into_boxed_alloc(self) -> Box<A> {
        debug_assert_eq!(self.alloc_type_id, any::TypeId::of::<A>());

        let any_alloc: Box<dyn any::Any> = self.alloc;

        any_alloc.downcast::<A>().unwrap()
    }
}

impl<A> Clone for TypedProjAllocInner<A>
where
    A: any::Any + alloc::Allocator + Clone,
{
    fn clone(&self) -> Self {
        let cloned_alloc = Box::new(self.allocator().clone());

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

#[repr(C)]
struct OpaqueAllocInner {
    alloc: Box<dyn AnyAllocator>,
    alloc_type_id: any::TypeId,
}

impl OpaqueAllocInner {
    #[inline]
    const fn allocator_type_id(&self) -> any::TypeId {
        self.alloc_type_id
    }
}

impl OpaqueAllocInner {
    #[inline(always)]
    pub(crate) fn as_proj_assuming_type<A>(&self) -> &TypedProjAllocInner<A>
    where
        A: any::Any + alloc::Allocator,
    {
        debug_assert_eq!(self.alloc_type_id, any::TypeId::of::<A>());

        unsafe { &*(self as *const OpaqueAllocInner as *const TypedProjAllocInner<A>) }
    }

    #[inline(always)]
    pub(crate) fn as_proj_mut_assuming_type<A>(&mut self) -> &mut TypedProjAllocInner<A>
    where
        A: any::Any + alloc::Allocator,
    {
        debug_assert_eq!(self.alloc_type_id, any::TypeId::of::<A>());

        unsafe { &mut *(self as *mut OpaqueAllocInner as *mut TypedProjAllocInner<A>) }
    }

    #[inline(always)]
    pub(crate) fn into_proj_assuming_type<A>(self) -> TypedProjAllocInner<A>
    where
        A: any::Any + alloc::Allocator,
    {
        debug_assert_eq!(self.alloc_type_id, any::TypeId::of::<A>());

        TypedProjAllocInner {
            alloc: self.alloc,
            alloc_type_id: self.alloc_type_id,
            _marker: marker::PhantomData,
        }
    }

    #[inline(always)]
    pub(crate) fn from_proj<A>(proj_self: TypedProjAllocInner<A>) -> Self
    where
        A: any::Any + alloc::Allocator,
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
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }

    #[inline]
    pub fn has_alloc_type<A>(&self) -> bool
    where
        A: any::Any + alloc::Allocator,
    {
        self.inner.allocator_type_id() == any::TypeId::of::<A>()
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
        fn type_check_failed(type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_alloc_type::<A>() {
            type_check_failed(self.inner.alloc_type_id, any::TypeId::of::<A>());
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
    #[inline]
    pub fn as_proj<A>(&self) -> &TypedProjAlloc<A>
    where
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<A>();

        unsafe { &*(self as *const OpaqueAlloc as *const TypedProjAlloc<A>) }
    }

    #[inline]
    pub fn as_proj_mut<A>(&mut self) -> &mut TypedProjAlloc<A>
    where
        A: any::Any + alloc::Allocator,
    {
        self.assert_type_safety::<A>();

        unsafe { &mut *(self as *mut OpaqueAlloc as *mut TypedProjAlloc<A>) }
    }

    #[inline]
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

#[cfg(test)]
mod alloc_inner_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_opaque_alloc_inner_match_sizes<A>()
    where
        A: any::Any + alloc::Allocator,
    {
        let expected = mem::size_of::<TypedProjAllocInner<A>>();
        let result = mem::size_of::<OpaqueAllocInner>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_alloc_inner_match_alignments<A>()
    where
        A: any::Any + alloc::Allocator,
    {
        let expected = mem::align_of::<TypedProjAllocInner<A>>();
        let result = mem::align_of::<OpaqueAllocInner>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_alloc_inner_match_offsets<A>()
    where
        A: any::Any + alloc::Allocator,
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

    struct DummyAlloc {}

    unsafe impl alloc::Allocator for DummyAlloc {
        fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
            alloc::Global.allocate(layout)
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
            unsafe {
                alloc::Global.deallocate(ptr, layout)
            }
        }
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
    layout_tests!(dummy_alloc, DummyAlloc);
}

#[cfg(test)]
mod alloc_layout_tests {
    use super::*;
    use core::mem;

    fn run_test_opaque_alloc_match_sizes<A>()
    where
        A: any::Any + alloc::Allocator,
    {
        let expected = mem::size_of::<TypedProjAlloc<A>>();
        let result = mem::size_of::<OpaqueAlloc>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types size mismatch");
    }

    fn run_test_opaque_alloc_match_alignments<A>()
    where
        A: any::Any + alloc::Allocator,
    {
        let expected = mem::align_of::<TypedProjAlloc<A>>();
        let result = mem::align_of::<OpaqueAlloc>();

        assert_eq!(result, expected, "Opaque and Typed Projected data types alignment mismatch");
    }

    fn run_test_opaque_alloc_match_offsets<A>()
    where
        A: any::Any + alloc::Allocator,
    {
        assert_eq!(
            mem::offset_of!(TypedProjAlloc<A>, inner),
            mem::offset_of!(OpaqueAlloc, inner),
            "Opaque and Typed Projected data types offsets mismatch"
        );
    }

    struct DummyAlloc {}

    unsafe impl alloc::Allocator for DummyAlloc {
        fn allocate(&self, layout: alloc::Layout) -> Result<NonNull<[u8]>, alloc::AllocError> {
            alloc::Global.allocate(layout)
        }
        unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: alloc::Layout) {
            unsafe {
                alloc::Global.deallocate(ptr, layout)
            }
        }
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
    layout_tests!(dummy_alloc, DummyAlloc);
}
