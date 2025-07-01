use crate::alloc_inner::{TypedProjAllocInner, OpaqueAllocInner};

use core::any;
use core::fmt;
use core::marker;
use core::ptr::NonNull;
use alloc_crate::alloc;
use alloc_crate::boxed::Box;

/// A type-projected memory allocator.
///
/// Wrapping the memory allocator like this allows us to type-erase and type-project allocators
/// as **O(1)** time operations. When passing references to type-projected or type-erased
/// allocators around, type-erasure and type-projection are zero-cost operations, since they have
/// identical layout.
///
/// For a given allocator type `A`, the [`TypedProjAlloc<A>`] and [`OpaqueAlloc`] data types also
/// implement the [`Allocator`] trait, so we can allocate memory with it just as well as the
/// underlying allocator of type `A`.
///
/// # Type Erasure And Type Projection
///
/// This allows for more flexible and dynamic data handling, especially when working with
/// collections of unknown or dynamic types. Some applications of this include implementing
/// heterogeneous data structures, plugin systems, and managing foreign function interface data.
/// There are two data types that are dual to each other: [`TypedProjAlloc`] and [`OpaqueAlloc`].
///
/// # Tradeoffs Compared To A Non-Projected Allocator
///
/// There are some tradeoffs to gaining type-erasability and type-projectability. The projected and
/// erased allocators have identical memory layout to ensure that type projection and type erasure
/// are both **O(1)** time operations. Thus, the underlying memory allocator must be stored in the
/// equivalent of a [`Box`], which carries a small performance penalty. Moreover, the allocators
/// must carry extra metadata about the type of the underlying allocator through its [`TypeId`].
/// Boxing the allocator imposes a small performance penalty at runtime, and the extra metadata
/// makes the allocator itself a little bigger in memory, though this is very minor. This also puts
/// a slight restriction on what kinds of memory allocators can be held inside the container: the
/// underlying memory allocator must be [`any::Any`], i.e. it must have a `'static` lifetime.
///
/// # See Also
///
/// - [`OpaqueAlloc`]: The type-erased counterpart to [`TypedProjAlloc`].
///
/// # Examples
///
/// Using a type-projected allocator.
///
/// ```
/// # #![feature(allocator_api)]
/// # use opaque_alloc::TypedProjAlloc;
/// # use std::any::TypeId;
/// # use std::alloc::Global;
/// #
/// let proj_alloc = TypedProjAlloc::new(Global);
///
/// assert_eq!(proj_alloc.allocator_type_id(), TypeId::of::<Global>());
/// ```
///
/// [`Allocator`]: std::alloc::Allocator
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
    /// Returns the [`TypeId`] of the underlying memory allocator.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    ///
    /// let expected = TypeId::of::<Global>();
    /// let result = proj_alloc.allocator_type_id();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }
}

impl<A> TypedProjAlloc<A>
where
    A: any::Any + alloc::Allocator + Send + Sync,
{
    /// Constructs a new type-projected memory allocator.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    ///
    /// assert_eq!(proj_alloc.allocator_type_id(), TypeId::of::<Global>());
    /// assert_ne!(proj_alloc.allocator_type_id(), TypeId::of::<Box<Global>>());
    /// ```
    #[inline]
    pub fn new(alloc: A) -> Self {
        let inner = TypedProjAllocInner::new(alloc);

        Self { inner, }
    }

    /// Constructs a new type-projected memory allocator from a boxed memory allocator.
    ///
    /// The underlying type of the type-projected memory allocator will be the type of the memory
    /// allocator held by the box.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::from_boxed_alloc(Box::new(Global));
    ///
    /// assert_eq!(proj_alloc.allocator_type_id(), TypeId::of::<Global>());
    /// assert_ne!(proj_alloc.allocator_type_id(), TypeId::of::<Box<Global>>());
    /// ```
    #[inline]
    pub fn from_boxed_alloc(alloc: Box<A>) -> Self {
        let inner = TypedProjAllocInner::from_boxed_alloc(alloc);

        Self { inner, }
    }

    /// Returns a reference to the underlying memory allocator.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    ///
    /// let alloc: &Global = proj_alloc.allocator();
    /// ```
    pub fn allocator(&self) -> &A {
        self.inner.allocator()
    }

    /// Converts the type-projected allocator into a boxed memory allocator.
    ///
    /// The resulting boxed memory allocator cannot be type-projected and type-erased again
    /// unless it is converted back via a method like [`TypedProjAlloc::from_boxed_alloc`].
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::TypedProjAlloc;
    /// # use std::any::TypeId;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = TypedProjAlloc::new(Global);
    /// let boxed_alloc: Box<Global> = proj_alloc.into_boxed_alloc();
    ///
    /// let new_proj_alloc = TypedProjAlloc::from_boxed_alloc(boxed_alloc);
    ///
    /// assert_eq!(new_proj_alloc.allocator_type_id(), TypeId::of::<Global>());
    /// assert_ne!(new_proj_alloc.allocator_type_id(), TypeId::of::<Box<Global>>());
    /// ```
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

/// A type-erased memory allocator.
///
/// Wrapping the memory allocator like this allows us to type-erase and type-project allocators
/// as **O(1)** time operations. When passing references to type-projected or type-erased
/// allocators around, type-erasure and type-projection are zero-cost operations, since they have
/// identical layout.
///
/// For a given allocator type `A`, the [`TypedProjAlloc<A>`] and [`OpaqueAlloc`] data types also
/// implement the [`Allocator`] trait, so we can allocate memory with it just as well as the
/// underlying allocator of type `A`.
///
/// # Type Erasure And Type Projection
///
/// This allows for more flexible and dynamic data handling, especially when working with
/// collections of unknown or dynamic types. Some applications of this include implementing
/// heterogeneous data structures, plugin systems, and managing foreign function interface data.
/// There are two data types that are dual to each other: [`TypedProjAlloc`] and [`OpaqueAlloc`].
///
/// # Tradeoffs Compared To A Non-Projected Allocator
///
/// There are some tradeoffs to gaining type-erasability and type-projectability. The projected and
/// erased allocators have identical memory layout to ensure that type projection and type erasure
/// are both **O(1)** time operations. Thus, the underlying memory allocator must be stored in the
/// equivalent of a [`Box`], which carries a small performance penalty. Moreover, the allocators
/// must carry extra metadata about the type of the underlying allocator through its [`TypeId`].
/// Boxing the allocator imposes a small performance penalty at runtime, and the extra metadata
/// makes the allocator itself a little bigger in memory, though this is very minor. This also puts
/// a slight restriction on what kinds of memory allocators can be held inside the container: the
/// underlying memory allocator must be [`any::Any`], i.e. it must have a `'static` lifetime.
///
/// # See Also
///
/// - [`TypedProjAlloc`]: The type-projected counterpart to [`OpaqueAlloc`].
///
/// # Examples
///
/// Using a type-erased allocator.
///
/// ```
/// # #![feature(allocator_api)]
/// # use opaque_alloc::OpaqueAlloc;
/// # use std::any::TypeId;
/// # use std::alloc::Global;
/// #
/// let opaque_alloc = OpaqueAlloc::new::<Global>(Global);
///
/// assert_eq!(opaque_alloc.allocator_type_id(), TypeId::of::<Global>());
/// ```
///
/// [`Allocator`]: std::alloc::Allocator
#[repr(transparent)]
pub struct OpaqueAlloc {
    inner: OpaqueAllocInner,
}

impl OpaqueAlloc {
    /// Returns the [`TypeId`] of the underlying memory allocator.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::OpaqueAlloc;
    /// # use std::any::TypeId;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_alloc = OpaqueAlloc::new::<Global>(Global);
    ///
    /// let expected = TypeId::of::<Global>();
    /// let result = opaque_alloc.allocator_type_id();
    ///
    /// assert_eq!(result, expected);
    /// ```
    #[inline]
    pub const fn allocator_type_id(&self) -> any::TypeId {
        self.inner.allocator_type_id()
    }
}

impl OpaqueAlloc {
    /// Determines whether the underlying memory allocator has the given allocator type.
    ///
    /// Returns `true` if `self` has the specified memory allocator type. Returns `false`
    /// otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::OpaqueAlloc;
    /// # use std::any::{Any, TypeId};
    /// # use std::alloc::{Allocator, AllocError, Layout, Global};
    /// # use std::ptr::NonNull;
    /// #
    /// trait AnyAllocator: Any + Allocator + Send + Sync {}
    ///
    /// impl<A> AnyAllocator for A where A: Any + Allocator + Send + Sync {}
    ///
    /// struct BoxedAllocator(Box<dyn AnyAllocator>);
    /// # unsafe impl Allocator for BoxedAllocator {
    /// #     fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
    /// #         self.0.allocate(layout)
    /// #     }
    /// #
    /// #     unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
    /// #         self.0.deallocate(ptr, layout);
    /// #     }
    /// # }
    /// #
    ///
    /// let opaque_alloc = OpaqueAlloc::new::<Global>(Global);
    ///
    /// assert!(opaque_alloc.has_allocator_type::<Global>());
    /// assert!(!opaque_alloc.has_allocator_type::<BoxedAllocator>());
    /// ```
    #[inline]
    pub fn has_allocator_type<A>(&self) -> bool
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.inner.allocator_type_id() == any::TypeId::of::<A>()
    }

    /// Assert the concrete types underlying a type-erased data type.
    ///
    /// This method's main use case is ensuring the type safety of an operation before projecting
    /// into the type-projected counterpart of the type-erased allocator.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the memory allocator of `self` do not match the
    /// requested allocator type `A`.
    #[inline]
    #[track_caller]
    fn assert_type_safety<A>(&self)
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        #[cold]
        #[cfg_attr(feature = "nightly", optimize(size))]
        #[track_caller]
        fn type_check_failed(type_id_self: any::TypeId, type_id_other: any::TypeId) -> ! {
            panic!("Type mismatch. Need `{:?}`, got `{:?}`", type_id_self, type_id_other);
        }

        if !self.has_allocator_type::<A>() {
            type_check_failed(self.inner.allocator_type_id(), any::TypeId::of::<A>());
        }
    }
}

impl OpaqueAlloc {
    /// Projects the type-erased allocator reference into a type-projected allocator reference.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the memory allocator of `self` do not match the
    /// requested allocator type `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::{OpaqueAlloc, TypedProjAlloc};
    /// # use std::alloc::Global;
    /// #
    /// let opaque_alloc = OpaqueAlloc::new::<Global>(Global);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// let proj_alloc: &TypedProjAlloc<Global> = opaque_alloc.as_proj::<Global>();
    /// ```
    #[inline]
    pub fn as_proj<A>(&self) -> &TypedProjAlloc<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<A>();

        unsafe { &*(self as *const OpaqueAlloc as *const TypedProjAlloc<A>) }
    }

    /// Projects the mutable type-erased allocator reference into a mutable type-projected
    /// allocator reference.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the memory allocator of `self` do not match the
    /// requested allocator type `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::{OpaqueAlloc, TypedProjAlloc};
    /// # use std::alloc::Global;
    /// #
    /// let mut opaque_alloc = OpaqueAlloc::new::<Global>(Global);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// let proj_alloc: &mut TypedProjAlloc<Global> = opaque_alloc.as_proj_mut::<Global>();
    /// ```
    #[inline]
    pub fn as_proj_mut<A>(&mut self) -> &mut TypedProjAlloc<A>
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        self.assert_type_safety::<A>();

        unsafe { &mut *(self as *mut OpaqueAlloc as *mut TypedProjAlloc<A>) }
    }

    /// Projects the type-erased allocator value into a type-projected allocator value.
    ///
    /// # Panics
    ///
    /// This method panics if the [`TypeId`] of the memory allocator of `self` do not match the
    /// requested allocator type `A`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::{OpaqueAlloc, TypedProjAlloc};
    /// # use std::alloc::Global;
    /// #
    /// let opaque_alloc = OpaqueAlloc::new::<Global>(Global);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// let proj_alloc: TypedProjAlloc<Global> = opaque_alloc.into_proj::<Global>();
    /// ```
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

    /// Erases the type-projected allocator value into a type-erased allocator value.
    ///
    /// Unlike the type projection methods [`as_proj`], [`as_proj_mut`], and [`into_proj`], this
    /// method never panics.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::{OpaqueAlloc, TypedProjAlloc};
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc: TypedProjAlloc<Global> = TypedProjAlloc::new(Global);
    /// let opaque_alloc: OpaqueAlloc = OpaqueAlloc::from_proj(proj_alloc);
    /// #
    /// # assert!(opaque_alloc.has_allocator_type::<Global>());
    /// #
    /// ```
    ///
    /// [`as_proj`]: OpaqueAlloc::as_proj,
    /// [`as_proj_mut`]: OpaqueAlloc::as_proj_mut
    /// [`into_proj`]: OpaqueAlloc::into_proj
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

impl OpaqueAlloc {
    /// Constructs a new type-erased memory allocator.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::OpaqueAlloc;
    /// # use std::any::TypeId;
    /// # use std::alloc::Global;
    /// #
    /// let proj_alloc = OpaqueAlloc::new(Global);
    ///
    /// assert_eq!(proj_alloc.allocator_type_id(), TypeId::of::<Global>());
    /// assert_ne!(proj_alloc.allocator_type_id(), TypeId::of::<Box<Global>>());
    /// ```
    #[inline]
    pub fn new<A>(alloc: A) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_alloc = TypedProjAlloc::<A>::new(alloc);

        Self::from_proj(proj_alloc)
    }

    /// Constructs a new type-erased memory allocator from a boxed memory allocator.
    ///
    /// The underlying type of the type-erased memory allocator will be the type of the memory
    /// allocator held by the box.
    ///
    /// # Examples
    ///
    /// ```
    /// # #![feature(allocator_api)]
    /// # use opaque_alloc::OpaqueAlloc;
    /// # use std::any::TypeId;
    /// # use std::alloc::Global;
    /// #
    /// let opaque_alloc = OpaqueAlloc::from_boxed_alloc(Box::new(Global));
    ///
    /// assert_eq!(opaque_alloc.allocator_type_id(), TypeId::of::<Global>());
    /// assert_ne!(opaque_alloc.allocator_type_id(), TypeId::of::<Box<Global>>());
    /// ```
    #[inline]
    pub fn from_boxed_alloc<A>(alloc: Box<A>) -> Self
    where
        A: any::Any + alloc::Allocator + Send + Sync,
    {
        let proj_alloc = TypedProjAlloc::<A>::from_boxed_alloc(alloc);

        Self::from_proj(proj_alloc)
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
